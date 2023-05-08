use crate::ctx::Ctx;
use crate::error::ClientError;
use crate::{Error, Result};
use axum::http::{Method, Uri};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use std::time::SystemTime;
use uuid::Uuid;

pub async fn log_request(
	uuid: Uuid,
	req_method: Method,
	uri: Uri,
	ctx: Option<Ctx>,
	service_error: Option<&Error>,
	client_error: Option<ClientError>,
) -> Result<()> {
	let now = SystemTime::now();
	let now: DateTime<Utc> = now.into();
	let now = now.to_rfc3339();

	let error_type = service_error.map(|se| se.as_ref().to_string());
	let error_data = serde_json::to_value(service_error)
		.ok()
		.and_then(|mut v| v.get_mut("data").map(|v| v.take()));

	let log_line = RequestLogLine {
		uuid: uuid.to_string(),
		timestamp: now,

		req_path: uri.to_string(),
		req_method: req_method.to_string(),

		client_error_type: client_error.map(|e| e.as_ref().to_string()),
		user_id: ctx.map(|c| c.user_id()),
		error_type,
		error_data,
	};

	println!("   ->> log_request: \n{}", json!(log_line));

	// TODO: send to cloudwatch like service
	Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
	uuid: String,      //uuid string formatted
	timestamp: String, //(should be iso 8601)

	// User and context attributes
	user_id: Option<u64>,

	// http request attributes
	req_path: String,
	req_method: String,

	// Error attributes
	client_error_type: Option<String>,
	error_type: Option<String>,
	error_data: Option<Value>,
}
