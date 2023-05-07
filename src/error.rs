use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

// convenience for later
pub type Result<T> = core::result::Result<T, Error>;

//Key Idea here: NEVER send internal details about the error to the client

#[derive(Debug)]
pub enum Error {
	LoginFail,
	AuthFailNoAuthTokenCookie,
	TicketDeleteFailedIdNotFound { id: u64 },
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("->> {:<12} - {self:?}", "INTO_RES");

		(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
	}
}
