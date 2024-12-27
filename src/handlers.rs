use crate::{config::AppConfig, error::AppError, proxy};
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::Response,
};
use std::sync::Arc;
use tracing::debug;

pub async fn handle_request(
    state: State<Arc<AppConfig>>,
    request: Request<Body>,
) -> Result<Response<Body>, AppError> {
    debug!("Handling request");
    proxy::proxy_request(state, request).await
}

pub async fn health_check() -> Result<(StatusCode, &'static str), AppError> {
    Ok((StatusCode::OK, "OK"))
}
