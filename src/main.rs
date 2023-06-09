#![allow(unused)]
// region : Imports
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use uuid::Uuid;
use web::AUTH_TOKEN;

use crate::{
    model::ModelController,
    web::{middleware_auth::mw_require_auth, routes_tickets},
};

// note how we re export the error module
pub use self::error::{Error, Result};

// endregion : Imports

// region: modules
mod ctx;
mod error;
mod model;
mod web;
// endregion: modules

#[tokio::main]
async fn main() -> Result<()> {
    let model_controller = ModelController::new().await?;

    let routes_apis = routes_tickets::routes(model_controller.clone())
        .route_layer(middleware::from_fn(mw_require_auth)); //this ensures the cookie paser middleware is only applied to api routes

    let routes_all = Router::new()
        .merge(routes_cookies()) //only for testing TODO: remove
        .merge(routes_hello())
        .nest("/api", routes_apis) // nest is powerful, since it allow to nest under given prefix, without the routes themself need to know about it
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper)) //note the layer here consuming middleware
        .layer(CookieManagerLayer::new()) //using tower-cookies
        .fallback_service(routes_static()); // / collides with our "/hello" route, so we need to use fallback_service

    // region : Server
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("--> LISTENING on {address}\n");

    //bind the address to the server
    axum::Server::bind(&address)
        .serve(routes_all.into_make_service()) //note the .into_make_service() #https://docs.rs/axum/latest/axum/struct.Router.html#method.into_make_service
        .await
        .unwrap();
    // endregion : Server

    Ok(())
}

#[derive(Debug, Deserialize)] //note the serde deps here
struct HelloParams {
    name: Option<String>,
}

// special middleware layer in axum, that takes response from the router and maps it to another response
// will become important when we need to distinc between different errors
async fn main_response_mapper(res: Response) -> Response {
    println!(
        "--> {:<12} - main_response_mapper - {res:?}",
        "RES_MAPPER\n"
    );
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
              "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
              }
            });
            println!("    ->> client_error_body: {client_error_body}");
            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    println!("--> server log line - {uuid} - Error : {service_error:?}");
    error_response.unwrap_or(res)
}

// region : Handler
// example is `/hello?name=jan`
// note the query which is an extractor https://docs.rs/axum/latest/axum/extract/struct.Query.html
// note `(Query(params): ` which does destructure the query
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER\n");
    let name = params.name.as_deref().unwrap_or("World"); // deref = this gives Option of Reference of String, unwrap_or provides fallback if no argument was given
    Html(format!("Hello <strong> {name} </strong>!"))
}
// example is `/hello?name=jan`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {name:?}", "HANDLER");
    Html(format!("Hello <strong> {name} </strong>!"))
}
// endregion : Handler

// region : Routes
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}
// endregion : Routes

// region cookies

async fn cookie_handler(cookies: Cookies) -> String {
    let visited = cookies
        .get(AUTH_TOKEN)
        .and_then(|c| c.value().parse().ok())
        .unwrap_or(0);
    if visited > 5 {
        cookies.remove(Cookie::new(AUTH_TOKEN, ""));
        "Counter has been reset".into()
    } else {
        cookies.add(Cookie::new(AUTH_TOKEN, (visited + 1).to_string()));
        format!("You've been here {} times before", visited)
    }
}

// cookie routes
fn routes_cookies() -> Router {
    Router::new().route("/api/cookies", get(cookie_handler))
}

// endregion cookies
