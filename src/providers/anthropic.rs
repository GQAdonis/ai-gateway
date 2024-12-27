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
const DEFAULT_MODEL: &str = "claude-3-opus-20240229";
const API_VERSION: &str = "2024-03-01";

pub struct AnthropicProvider {
    base_url: String,
    http_client: Arc<Client>,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.anthropic.com".to_string(),
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

    fn transform_tool_calls(&self, content: &Value) -> Result<Value, AppError> {
        if let Some(tool_calls) = content.get("tool_calls") {
            let transformed_calls = tool_calls
                .as_array()
                .ok_or_else(|| AppError::InvalidRequestFormat)?
                .iter()
                .filter_map(|call| {
                    let function = call.get("function")?;
                    Some(json!({
                        "type": "function",
                        "name": function.get("name")?.as_str()?,
                        "parameters": function.get("arguments")?.as_str()?
                    }))
                })
                .collect::<Vec<_>>();

            Ok(json!({
                "type": "tool_calls",
                "tool_calls": transformed_calls
            }))
        } else {
            Ok(content.clone())
        }
    }

    async fn transform_request_body(&self, body: Value) -> Result<Value, AppError> {
        debug!("Transforming request body for Anthropic: {:#?}", body);

        let mut transformed = json!({
            "model": body.get("model").and_then(Value::as_str).unwrap_or(DEFAULT_MODEL),
            "max_tokens": body.get("max_tokens").unwrap_or(&json!(4096)),
            "temperature": body.get("temperature").unwrap_or(&json!(0.7)),
            "top_p": body.get("top_p").unwrap_or(&json!(1.0)),
            "stream": body.get("stream").unwrap_or(&json!(false))
        });

        if let Some(messages) = body.get("messages").and_then(Value::as_array) {
            let mut transformed_messages = Vec::new();

            for msg in messages {
                let role = match msg.get("role").and_then(Value::as_str) {
                    Some("user") => "user",
                    Some("assistant") => "assistant",
                    Some("system") => "system",
                    _ => continue,
                };

                let content = match msg.get("content") {
                    Some(content_value) if content_value.is_array() => {
                        let mut text_content = String::new();
                        let mut image_content = Vec::new();

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

                                        image_content.push(json!({
                                            "type": "image",
                                            "source": {
                                                "type": "base64",
                                                "media_type": media_type,
                                                "data": self.process_image_content(url).await?
                                            }
                                        }));
                                    }
                                }
                                Some("tool_call") => {
                                    if let Ok(tool_call) = self.transform_tool_calls(block) {
                                        text_content.push_str(&serde_json::to_string(&tool_call)?);
                                    }
                                }
                                _ => continue,
                            }
                        }

                        let mut content = Vec::new();
                        if !text_content.is_empty() {
                            content.push(json!({
                                "type": "text",
                                "text": text_content
                            }));
                        }
                        content.extend(image_content);
                        
                        if content.is_empty() {
                            vec![json!({
                                "type": "text",
                                "text": " " // Provide a non-empty default
                            })]
                        } else {
                            content
                        }
                    }
                    Some(content_value) if content_value.is_string() => {
                        vec![json!({
                            "type": "text",
                            "text": content_value.as_str().unwrap_or_default()
                        })]
                    }
                    _ => continue,
                };

                transformed_messages.push(json!({
                    "role": role,
                    "content": content
                }));
            }

            transformed["messages"] = json!(transformed_messages);
        }

        if let Some(tools) = body.get("tools") {
            transformed["tools"] = tools.clone();
        }

        debug!("Transformed body for Anthropic: {:#?}", transformed);
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
                if let Ok(mut json) = serde_json::from_str::<Value>(data) {
                    if let Some(delta) = json.get_mut("delta") {
                        // Transform tool calls if present
                        if let Some(tool_calls) = delta.get("tool_calls") {
                            let transformed_calls = tool_calls
                                .as_array()
                                .unwrap_or(&vec![])
                                .iter()
                                .map(|call| {
                                    json!({
                                        "index": call.get("index").unwrap_or(&json!(0)),
                                        "id": call.get("id").unwrap_or(&json!("call_0")),
                                        "type": "function",
                                        "function": {
                                            "name": call.get("function").and_then(|f| f.get("name")).unwrap_or(&json!("")),
                                            "arguments": call.get("function").and_then(|f| f.get("arguments")).unwrap_or(&json!("{}"))
                                        }
                                    })
                                })
                                .collect::<Vec<_>>();

                            delta["tool_calls"] = json!(transformed_calls);
                        }
                    }
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
impl Provider for AnthropicProvider {
    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    fn name(&self) -> &str {
        "anthropic"
    }

    fn transform_path(&self, path: &str) -> String {
        if path.contains("/chat/completions") {
            "/v1/messages".to_string()
        } else {
            path.to_string()
        }
    }

    fn process_headers(&self, original_headers: &HeaderMap) -> Result<HeaderMap, AppError> {
        debug!("Processing Anthropic request headers");
        let mut headers = HeaderMap::new();

        headers.insert(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );

        headers.insert(
            http::header::HeaderName::from_static("anthropic-version"),
            http::header::HeaderValue::from_static(API_VERSION),
        );

        if let Some(auth) = original_headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
        {
            debug!("Converting Bearer token to x-api-key format");
            let api_key = auth.trim_start_matches("Bearer ");
            headers.insert(
                http::header::HeaderName::from_static("x-api-key"),
                http::header::HeaderValue::from_str(api_key).map_err(|_| {
                    error!("Failed to process Anthropic authorization header");
                    AppError::InvalidHeader
                })?,
            );
        } else {
            error!("No authorization header found for Anthropic request");
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
