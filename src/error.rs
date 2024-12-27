use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::{fmt, string::FromUtf8Error};

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    InvalidMethod,
    InvalidRequestFormat,
    ValidationError(String),
    ProcessingError(String),
    ProxyError(String),
    BodyReadError(String),
    UnsupportedModel,
    RequestError(String),
    EventStreamError(String),
    InvalidHeaderValue(axum::http::header::InvalidHeaderValue),
    AwsSigningError(aws_sigv4::http_request::SigningError),
    InvalidHeader,
    MissingApiKey,
    SerdeError(serde_json::Error),
    Utf8Error(FromUtf8Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidMethod => write!(f, "Invalid HTTP method"),
            AppError::InvalidRequestFormat => write!(f, "Invalid request format"),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            AppError::ProxyError(msg) => write!(f, "Proxy error: {}", msg),
            AppError::BodyReadError(msg) => write!(f, "Body read error: {}", msg),
            AppError::UnsupportedModel => write!(f, "Unsupported model"),
            AppError::RequestError(msg) => write!(f, "Request error: {}", msg),
            AppError::EventStreamError(msg) => write!(f, "Event stream error: {}", msg),
            AppError::InvalidHeaderValue(e) => write!(f, "Invalid header value: {}", e),
            AppError::AwsSigningError(e) => write!(f, "AWS signing error: {}", e),
            AppError::InvalidHeader => write!(f, "Invalid header"),
            AppError::MissingApiKey => write!(f, "Missing API key"),
            AppError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            AppError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerdeError(err)
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(err: FromUtf8Error) -> Self {
        AppError::Utf8Error(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::InvalidMethod => (StatusCode::METHOD_NOT_ALLOWED, self.to_string()),
            AppError::InvalidRequestFormat => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::ProcessingError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::ProxyError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::BodyReadError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::UnsupportedModel => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::RequestError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::EventStreamError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::InvalidHeaderValue(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::AwsSigningError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::InvalidHeader => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::MissingApiKey => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::SerdeError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Utf8Error(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "type": format!("{:?}", self),
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}
