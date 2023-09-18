use std::collections::HashMap;

use actix_web::{HttpResponse, ResponseError};
use reqwest::StatusCode;

#[derive(Debug, Clone)]
pub struct HttpError {
    status_code: StatusCode,
    body: HashMap<String, String>,
}

impl HttpError {
    pub fn new(status_code: StatusCode, body: HashMap<String, String>) -> Self {
        Self { status_code, body }
    }

    pub fn from_message<S: ToString>(
        status_code: StatusCode,
        message: S,
    ) -> Self {
        Self::new(
            status_code,
            HashMap::from([("message".to_string(), message.to_string())]),
        )
    }

    pub fn bad_request<S: ToString>(message: S) -> Self {
        Self::from_message(StatusCode::BAD_REQUEST, message)
    }

    pub fn internal() -> Self {
        Self::from_message(
            StatusCode::INTERNAL_SERVER_ERROR,
            "unknown error".to_string(),
        )
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {:?}", self.status_code, self.body))
    }
}

impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let Self { status_code, body } = self;
        HttpResponse::build(*status_code).json(body)
    }
}
