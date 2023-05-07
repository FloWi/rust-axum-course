#![allow(unused)] // For beginning only.

// re-export (best practice)
pub use self::error::{Error, Result};

use crate::model::ModelController;
use axum::extract::{Path, Query};
use axum::handler::HandlerWithoutStateExt;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router, Server};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
	let mc = ModelController::new().await?;

	let routes_all = Router::new()
		.merge(routes_hello())
		.merge(web::routes_login::routes())
		.nest("/api", web::routes_tickets::routes(mc.clone()))
		.layer(middleware::map_response(main_response_mapper))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static());
	// routes_static() can't be merged with routes_hello(), because path "/" would collide.
	// But static routes usually can be used as a fallback

	//use 127.0.0.1, because using 0.0.0.0 will cause macOS issue a warning at every recompile
	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("->> LISTENING on {addr}\n");
	Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
	//adding just an empty line in the logs for now to separate the different requests
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
	println!();
	res
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
