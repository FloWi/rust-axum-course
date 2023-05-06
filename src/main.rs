#![allow(unused)] // For beginning only.

use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router, Server};
use axum::extract::{Path, Query};
use axum::handler::HandlerWithoutStateExt;
use axum::routing::{get, get_service};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello())
        .fallback_service(routes_static());
    // routes_static() can't be merged with routes_hello(), because path "/" would collide.
    // But static routes usually can be used as a fallback

    //use 127.0.0.1, because using 0.0.0.0 will cause macOS issue a warning at every recompile
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    Server::bind(&addr).serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
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

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}!!!</strong>"))
}
