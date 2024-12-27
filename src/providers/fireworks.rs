use super::Provider;
use crate::error::AppError;
use async_trait::async_trait;
use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, HeaderValue, Response, StatusCode},
};
use parking_lot::RwLock;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{debug, error};

const DEFAULT_MODEL: &str = "mixtral-8x7b";
const DEFAULT_MAX_TOKENS: u64 = 1000;
const DEFAULT_TEMPERATURE: f64 = 0.7;
const DEFAULT_TOP_P: f64 = 1.0;

#[derive(Clone)]
pub struct FireworksProvider {
    base_url: Arc<RwLock<String>>,
    current_model: Arc<RwLock<String>>,
    http_client: Arc<Client>,
}

impl FireworksProvider {
    pub fn new() -> Self {
        Self {
            base_url: Arc::new(RwLock::new("https://api.fireworks.ai/inference/v1".to_string())),
            current_model: Arc::new(RwLock::new(DEFAULT_MODEL.to_string())),
            http_client: Arc::new(Client::new()),
        }
    }

    async fn transform_request_body(&self, body: Value) -> Result<Value, AppError> {
        debug!("Transforming request body: {:#?}", body);

        let messages = body
            .get("messages")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                error!("Invalid request format: messages array not found");
                AppError::InvalidRequestFormat
            })?;

        let transformed = json!({
            "model": self.current_model.read().as_str(),
            "messages": messages,
            "max_tokens": body.get("max_tokens")
                .and_then(Value::as_u64)
                .unwrap_or(DEFAULT_MAX_TOKENS),
            "temperature": body.get("temperature")
                .and_then(Value::as_f64)
                .unwrap_or(DEFAULT_TEMPERATURE),
            "top_p": body.get("top_p")
                .and_then(Value::as_f64)
                .unwrap_or(DEFAULT_TOP_P),
            "stream": true
        });

        debug!("Transformed body: {:#?}", transformed);
        Ok(transformed)
    }
}

#[async_trait]
impl Provider for FireworksProvider {
    fn base_url(&self) -> String {
        self.base_url.read().clone()
    }

    fn name(&self) -> &str {
        "fireworks"
    }

    fn process_headers(&self, headers: &HeaderMap) -> Result<HeaderMap, AppError> {
        let mut new_headers = HeaderMap::new();
        
        if let Some(api_key) = headers.get("x-fireworks-api-key") {
            new_headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", api_key.to_str().unwrap_or_default())).unwrap());
        }
        
        Ok(new_headers)
    }

    async fn prepare_request_body(&self, body: Bytes) -> Result<Bytes, AppError> {
        if let Ok(json_body) = serde_json::from_slice::<Value>(&body) {
            let transformed_body = self.transform_request_body(json_body).await?;
            Ok(Bytes::from(transformed_body.to_string()))
        } else {
            Ok(body)
        }
    }

    async fn before_request(&self, _headers: &HeaderMap, body: &Bytes) -> Result<(), AppError> {
        if let Ok(request_body) = serde_json::from_slice::<Value>(body) {
            if let Some(model) = request_body["model"].as_str() {
                debug!("Setting model from before_request: {}", model);
                *self.current_model.write() = model.to_string();
            }
        }
        Ok(())
    }

    fn transform_path(&self, path: &str) -> String {
        if path.contains("/chat/completions") {
            "/chat/completions".to_string()
        } else {
            path.to_string()
        }
    }

    fn requires_signing(&self) -> bool {
        false
    }

    async fn sign_request(
        &self,
        _method: &str,
        _url: &str,
        headers: &HeaderMap,
        _body: &[u8],
    ) -> Result<HeaderMap, AppError> {
        Ok(headers.clone())
    }

    async fn process_response(&self, response: Response<Body>) -> Result<Response<Body>, AppError> {
        if response
            .headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map_or(false, |ct| ct.contains("text/event-stream"))
        {
            debug!("Processing Fireworks event stream response");

            let mut response = response;
            let headers = response.headers_mut();
            headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
            headers.insert(
                "access-control-allow-methods",
                HeaderValue::from_static("POST, OPTIONS"),
            );
            headers.insert(
                "access-control-allow-headers",
                HeaderValue::from_static("content-type, x-provider, x-fireworks-api-key"),
            );
            headers.insert(
                "access-control-expose-headers",
                HeaderValue::from_static("*"),
            );
            headers.insert("x-accel-buffering", HeaderValue::from_static("no"));
            headers.insert("keep-alive", HeaderValue::from_static("timeout=600"));
            Ok(response)
        } else {
            let mut response = response;
            let headers = response.headers_mut();
            headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
            headers.insert(
                "access-control-allow-methods",
                HeaderValue::from_static("POST, OPTIONS"),
            );
            headers.insert(
                "access-control-allow-headers",
                HeaderValue::from_static("content-type, x-provider, x-fireworks-api-key"),
            );
            headers.insert(
                "access-control-expose-headers",
                HeaderValue::from_static("*"),
            );
            Ok(response)
        }
    }
}
