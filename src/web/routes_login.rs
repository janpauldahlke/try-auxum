use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web::AUTH_TOKEN, Error, Result};

#[derive(Debug, Deserialize)]
struct LoginPayLoad {
    username: String,
    pwd: String,
}

async fn api_login(
    cookies: Cookies, //provided by tower cookies, bound in layer in CookieManagerLayer::new()
    payload: Json<LoginPayLoad>, // note one can has only ONE json extractor per route!!
) -> Result<Json<Value>> {
    println!("--> {:<12} - api_login - {payload:?}", "HANDLER");

    //TODO:: here is the real db/auth logic, for tutorial we just return a dummy value
    if payload.username != "jan" || payload.pwd != "123" {
        return Err(Error::LoginFail);
    }
    //

    //TODO: real auth-token generation logic
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    //success body
    let body = Json(json!({
      "result": {"success": true}
    }));
    //ship it
    Ok(body)
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/login", get(api_login))
}
