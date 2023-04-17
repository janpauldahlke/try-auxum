#![allow(unused)]

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong> World!!! </strong>") }),
    );

    // region : Server
    //which adress using std
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("--> LISTENING on {address}\n");

    //bind the address to the server
    axum::Server::bind(&address)
        .serve(routes_hello.into_make_service()) //note the .into_make_service() #https://docs.rs/axum/latest/axum/struct.Router.html#method.into_make_service
        .await
        .unwrap();
    // endregion : Server
}
