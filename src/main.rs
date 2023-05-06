#![allow(unused)] // For beginning only.

use std::net::SocketAddr;
use axum::response::Html;
use axum::{Router, Server};
use axum::handler::HandlerWithoutStateExt;
use axum::routing::get;

#[tokio::main]
async fn main() {
	let routes_hello = Router::new().route(
        "/hello", get(|| async {
            Html("Helo <strong>World!!!</strong>")
        })
    );

    // region:     --- Start Server
    let addr = SocketAddr::from(([0,0,0,0], 8080));
    println!("->> LISTENING on {addr}\n");
    Server::bind(&addr).serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion:  --- Start Server

}

