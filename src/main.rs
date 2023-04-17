#![allow(unused)]

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));

    // region : Server
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("--> LISTENING on {address}\n");

    //bind the address to the server
    axum::Server::bind(&address)
        .serve(routes_hello.into_make_service()) //note the .into_make_service() #https://docs.rs/axum/latest/axum/struct.Router.html#method.into_make_service
        .await
        .unwrap();
    // endregion : Server

    // region : Handler
    #[derive(Debug, Deserialize)] //note the serde deps here
    struct HelloParams {
        name: Option<String>,
    }

    // example is `/hello?name=jan`
    // note the query which is an extractor https://docs.rs/axum/latest/axum/extract/struct.Query.html
    // note `(Query(params): ` which does destructure the query
    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("--> {:<12} - handler_hello - {params:?}", "HANDLER");
        let name = params.name.as_deref().unwrap_or("World"); // deref = this gives Option of Reference of String, unwrap_or provides fallback if no argument was given
        Html(format!("Hello <strong> {name} </strong>!"))
    }

    // example is `/hello?name=jan`
    async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
        println!("--> {:<12} - handler_hello - {name:?}", "HANDLER");
        Html(format!("Hello <strong> {name} </strong>!"))
    }

    // endregion : Handler
}
