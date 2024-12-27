use super::Provider;
use crate::error::AppError;
use async_trait::async_trait;
use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, Request, Response},
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{debug, error, warn};

const SUPPORTED_IMAGE_FORMATS: [&str; 4] = ["image/jpeg", "image/png", "image/gif", "image/webp"];
const MAX_IMAGE_SIZE: usize = 20_971_520; // 20MB
const DEFAULT_MODEL: &str = "gpt-4-turbo-preview";
const ORGANIZATION_HEADER: &str = "OpenAI-Organization";

pub struct OpenAIProvider {
    base_url: String,
    http_client: Arc<Client>,
}

impl OpenAIProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.openai.com".to_string(),
            http_client: Arc::new(Client::new()),
        }
    }

    async fn validate_image(&self, url: &str) -> Result<bool, AppError> {
        let response = self.http_client.head(url).send().await.map_err(|e| {
            AppError::ValidationError(format!("Failed to validate image URL: {}", e))
        })?;

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let content_length = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        if !SUPPORTED_IMAGE_FORMATS.iter().any(|&f| content_type.contains(f)) {
            return Err(AppError::ValidationError(format!(
                "Unsupported image format: {}",
                content_type
            )));
        }

        if content_length > MAX_IMAGE_SIZE {
            return Err(AppError::ValidationError(format!(
                "Image size {} exceeds maximum limit of {} bytes",
                content_length, MAX_IMAGE_SIZE
            )));
        }

        Ok(true)
    }

    async fn validate_request_body(&self, body: &Value) -> Result<(), AppError> {
        // Validate image content if present
        if let Some(messages) = body.get("messages").and_then(Value::as_array) {
            for msg in messages {
                if let Some(content) = msg.get("content").and_then(Value::as_array) {
                    for block in content {
                        if let Some("image_url") = block.get("type").and_then(Value::as_str) {
                            let url = block
                                .get("image_url")
                                .and_then(|u| u.get("url"))
                                .and_then(Value::as_str)
                                .ok_or_else(|| AppError::ValidationError("Invalid image URL".into()))?;
                            
                            self.validate_image(url).await?;
                        }
                    }
                }
            }
        }

        // Validate tool calls format if present
        if let Some(tools) = body.get("tools").and_then(Value::as_array) {
            for tool in tools {
                if tool.get("type").and_then(Value::as_str) != Some("function") {
                    return Err(AppError::ValidationError("Unsupported tool type".into()));
                }

                if let Some(function) = tool.get("function") {
                    if function.get("name").and_then(Value::as_str).is_none() {
                        return Err(AppError::ValidationError(
                            "Function tool must have a name".into(),
                        ));
                    }

                    if function.get("parameters").is_none() {
                        return Err(AppError::ValidationError(
                            "Function tool must have parameters".into(),
                        ));
                    }
                } else {
                    return Err(AppError::ValidationError(
                        "Function tool configuration is invalid".into(),
                    ));
                }
            }
        }

        Ok(())
    }

    async fn preprocess_request_body(&self, body: Value) -> Result<Value, AppError> {
        // Validate the request body
        self.validate_request_body(&body).await?;

        // Set default model if not provided
        let mut processed = body.clone();
        if processed.get("model").is_none() {
            processed["model"] = json!(DEFAULT_MODEL);
        }

        Ok(processed)
    }

    fn transform_streaming_response(&self, chunk: Bytes) -> Result<Bytes, AppError> {
        let text = String::from_utf8(chunk.to_vec())?;
        let lines: Vec<&str> = text.lines().collect();
        
        let mut transformed_lines = Vec::new();
        for line in lines {
            if !line.is_empty() {
                transformed_lines.push(line.to_string());
            }
        }

        Ok(Bytes::from(transformed_lines.join("\n") + "\n"))
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    fn name(&self) -> &str {
        "openai"
    }

    fn process_headers(&self, original_headers: &HeaderMap) -> Result<HeaderMap, AppError> {
        debug!("Processing OpenAI request headers");
        let mut headers = HeaderMap::new();

        headers.insert(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );

        // Process authentication
        if let Some(api_key) = original_headers
            .get("x-magicapi-api-key")
            .and_then(|h| h.to_str().ok())
        {
            debug!("Using x-magicapi-api-key for authentication");
            headers.insert(
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {}", api_key)).map_err(
                    |_| {
                        error!("Failed to create authorization header from x-magicapi-api-key");
                        AppError::InvalidHeader
                    },
                )?,
            );
        } else if let Some(auth) = original_headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
        {
            debug!("Using provided authorization header");
            headers.insert(
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(auth).map_err(|_| {
                    error!("Failed to process authorization header");
                    AppError::InvalidHeader
                })?,
            );
        } else {
            error!("No authorization header found for OpenAI request");
            return Err(AppError::MissingApiKey);
        }

        // Copy OpenAI-Organization header if present
        if let Some(org) = original_headers.get(ORGANIZATION_HEADER) {
            headers.insert(ORGANIZATION_HEADER, org.clone());
        }

        Ok(headers)
    }

    async fn transform_request(&self, mut request: Request<Bytes>) -> Result<Request<Bytes>, AppError> {
        let body = request.body();
        if let Ok(json_body) = serde_json::from_slice::<Value>(body) {
            let processed_body = self.preprocess_request_body(json_body).await?;
            *request.body_mut() = Bytes::from(processed_body.to_string());
        }
        Ok(request)
    }

    async fn process_response(&self, response: Response<Body>) -> Result<Response<Body>, AppError> {
        if response.headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map_or(false, |ct| ct.contains("text/event-stream"))
        {
            let provider = self.clone();
            let stream = response
                .into_body()
                .into_data_stream()
                .map(move |chunk| match chunk {
                    Ok(bytes) => match provider.transform_streaming_response(bytes) {
                        Ok(transformed) => Ok(transformed),
                        Err(e) => {
                            error!("Error transforming response chunk: {}", e);
                            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
                        }
                    },
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
                });

            Ok(Response::builder()
                .status(response.status())
                .header("content-type", "text/event-stream")
                .header("cache-control", "no-cache")
                .header("connection", "keep-alive")
                .body(Body::from_stream(stream))
                .unwrap())
        } else {
            Ok(response)
        }
    }
}