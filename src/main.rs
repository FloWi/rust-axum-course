#![allow(unused)] // For beginning only.

use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router, Server};
use axum::extract::Query;
use axum::handler::HandlerWithoutStateExt;
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello", get(handler_hello),
    );

    //use 127.0.0.1, because using 0.0.0.0 will cause macOS issue a warning at every recompile
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    Server::bind(&addr).serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!!!</strong>"))
}
