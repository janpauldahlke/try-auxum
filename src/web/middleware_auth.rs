use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("--> {:<12} => mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|t| t.value().to_string());
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
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