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
const DEFAULT_MODEL: &str = "mixtral-8x7b-32768";

pub struct GroqProvider {
    base_url: String,
    http_client: Arc<Client>,
}

impl GroqProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.groq.com/openai".to_string(),
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

    async fn process_image_content(&self, url: &str) -> Result<String, AppError> {
        self.validate_image(url).await?;

        let response = self.http_client.get(url).send().await.map_err(|e| {
            AppError::ProcessingError(format!("Failed to fetch image: {}", e))
        })?;

        let image_bytes = response.bytes().await.map_err(|e| {
            AppError::ProcessingError(format!("Failed to read image bytes: {}", e))
        })?;

        Ok(BASE64.encode(image_bytes))
    }

    async fn transform_request_body(&self, body: Value) -> Result<Value, AppError> {
        debug!("Transforming request body for Groq: {:#?}", body);


        if body.get("tools").is_some() {
            return Err(AppError::UnsupportedFeature(
                "Groq does not support function calling".to_string(),
            ));
        }

        // Transform the messages to text-only format
        let messages = body.get("messages")
            .and_then(Value::as_array)
            .ok_or_else(|| AppError::InvalidRequestFormat)?;

        let transformed_messages = messages
            .iter()
            .map(|msg| {
                let role = msg.get("role")
                    .and_then(Value::as_str)
                    .unwrap_or("user");

                let content = match msg.get("content") {
                    Some(content_value) if content_value.is_array() => {
                        let mut text_content = String::new();

                        for block in content_value.as_array().unwrap() {
                            match block.get("type").and_then(Value::as_str) {
                                Some("text") => {
                                    if let Some(text) = block.get("text").and_then(Value::as_str) {
                                        if !text_content.is_empty() {
                                            text_content.push('\n');
                                        }
                                        text_content.push_str(text);
                                    }
                                }
                                Some("image_url") => {
                                    if let Some(url) = block
                                        .get("image_url")
                                        .and_then(|u| u.get("url"))
                                        .and_then(Value::as_str)
                                    {
                                        let media_type = block
                                            .get("image_url")
                                            .and_then(|u| u.get("detail"))
                                            .and_then(Value::as_str)
                                            .unwrap_or("auto");

                                        match self.process_image_content(url).await {
                                            Ok(base64_data) => {
                                                if !text_content.is_empty() {
                                                    text_content.push('\n');
                                                }
                                                // For Claude models, use the proper image format
                                                if body.get("model")
                                                    .and_then(Value::as_str)
                                                    .map_or(false, |m| m.contains("claude"))
                                                {
                                                    text_content.push_str(&serde_json::to_string(&json!({
                                                        "type": "image",
                                                        "source": {
                                                            "type": "base64",
                                                            "media_type": media_type,
                                                            "data": base64_data
                                                        }
                                                    }))?);
                                                } else {
                                                    warn!("Image input is only supported for Claude models");
                                                    text_content.push_str(&format!("\n[Image: {}]\n", url));
                                                }
                                            }
                                            Err(e) => {
                                                error!("Failed to process image content: {}", e);
                                                text_content.push_str(&format!("\n[Failed to process image: {}]\n", url));
                                            }
                                        }
                                    }
                                }
                                _ => continue,
                            }
                        }
                        
                        if text_content.is_empty() {
                            text_content = " ".to_string(); // Provide a non-empty default
                        }
                        text_content
                    },
                    Some(content_value) if content_value.is_string() => {
                        content_value.as_str().unwrap_or_default().to_string()
                    },
                    _ => return Err(AppError::InvalidRequestFormat),
                };

                Ok(json!({
                    "role": role,
                    "content": content
                }))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let transformed = json!({
            "model": body.get("model").and_then(Value::as_str).unwrap_or(DEFAULT_MODEL),
            "messages": transformed_messages,
            "stream": body.get("stream").unwrap_or(&json!(false)),
            "max_tokens": body.get("max_tokens").unwrap_or(&json!(null)),
            "temperature": body.get("temperature").unwrap_or(&json!(0.7)),
            "top_p": body.get("top_p").unwrap_or(&json!(1.0)),
        });

        debug!("Transformed body for Groq: {:#?}", transformed);
        Ok(transformed)
    }

    fn transform_streaming_response(&self, chunk: Bytes) -> Result<Bytes, AppError> {
        let text = String::from_utf8(chunk.to_vec())?;
        let lines: Vec<&str> = text.lines().collect();
        
        let mut transformed_lines = Vec::new();
        for line in lines {
            if line.starts_with("data: ") {
                if line == "data: [DONE]" {
                    transformed_lines.push(line.to_string());
                    continue;
                }

                let data = &line["data: ".len()..];
                if let Ok(json) = serde_json::from_str::<Value>(data) {
                    // Groq's response format is already OpenAI-compatible
                    transformed_lines.push(format!("data: {}", json.to_string()));
                } else {
                    transformed_lines.push(line.to_string());
                }
            } else if !line.is_empty() {
                transformed_lines.push(line.to_string());
            }
        }

        Ok(Bytes::from(transformed_lines.join("\n") + "\n"))
    }
}

#[async_trait]
impl Provider for GroqProvider {
    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    fn name(&self) -> &str {
        "groq"
    }

    fn process_headers(&self, original_headers: &HeaderMap) -> Result<HeaderMap, AppError> {
        debug!("Processing Groq request headers");
        let mut headers = HeaderMap::new();

        headers.insert(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth) = original_headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
        {
            debug!("Using provided authorization header for Groq");
            headers.insert(
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(auth).map_err(|_| {
                    error!("Failed to process Groq authorization header");
                    AppError::InvalidHeader
                })?,
            );
        } else {
            error!("No authorization header found for Groq request");
            return Err(AppError::MissingApiKey);
        }

        Ok(headers)
    }

    async fn transform_request(&self, mut request: Request<Bytes>) -> Result<Request<Bytes>, AppError> {
        let body = request.body();
        if let Ok(json_body) = serde_json::from_slice::<Value>(body) {
            let transformed_body = self.transform_request_body(json_body).await?;
            *request.body_mut() = Bytes::from(transformed_body.to_string());
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
