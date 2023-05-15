use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::RequestPartsExt;

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("--> {:<12} => mw_require_auth", "MIDDLEWARE");
    ctx?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|t| t.value().to_string());
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) =>
        //here can happen the expensive db call to get the user and other token validation
        {
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    //remove the cookies if something went wrong
    //copilot very strong here
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    //store the cty_result in the extension
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

//TODO: parse token of format `user-[user-id].[expiration].[signature]`
//returns (user_id, expiration, signature)
// we do this by using RegEx and bring a new dependency Lazy-regex crate
//check on cargo toml
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

// Region: Ctx Extractor
//impl saync trait on ctx
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("--> {:<12} - Ctx", "CTX XTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}
// endregion: Ctx Extractor
