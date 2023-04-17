#![allow(unused)]
// region : Imports
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
// endregion : Imports

// region: modules
mod error;
// note how we re export the error module
pub use self::error::{Error, Result};

mod web;
// endregion: modules

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper)) //note the layer here consuming middleware
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
    println!("----------------------------------------");
    //for now we just return the response itself
    res
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
