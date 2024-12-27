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
const DEFAULT_MODEL: &str = "accounts/fireworks/models/mixtral-8x7b";

pub struct FireworksProvider {
    base_url: String,
    http_client: Arc<Client>,
}

impl FireworksProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.fireworks.ai/inference/v1".to_string(),
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
                        "function": {
                            "name": function.get("name")?.as_str()?,
                            "arguments": function.get("arguments")?.as_str()?
                        }
                    }))
                })
                .collect::<Vec<_>>();

            Ok(json!({
                "tool_calls": transformed_calls
            }))
        } else {
            Ok(content.clone())
        }
    }

    async fn transform_request_body(&self, body: Value) -> Result<Value, AppError> {
        debug!("Transforming request body for Fireworks: {:#?}", body);

        let mut transformed = json!({
            "model": body.get("model").and_then(Value::as_str).unwrap_or(DEFAULT_MODEL),
            "max_tokens": body.get("max_tokens").unwrap_or(&json!(2048)),
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
                        let mut text_parts = Vec::new();
                        let mut image_parts = Vec::new();

                        for block in content_value.as_array().unwrap() {
                            match block.get("type").and_then(Value::as_str) {
                                Some("text") => {
                                    if let Some(text) = block.get("text").and_then(Value::as_str) {
                                        text_parts.push(text.to_string());
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
                                                // For Claude models, use the proper image format
                                                if body.get("model")
                                                    .and_then(Value::as_str)
                                                    .map_or(false, |m| m.contains("claude"))
                                                {
                                                    image_parts.push(json!({
                                                        "type": "image",
                                                        "source": {
                                                            "type": "base64",
                                                            "media_type": media_type,
                                                            "data": base64_data
                                                        }
                                                    }));
                                                } else {
                                                    // For other models, pass through URL
                                                    image_parts.push(json!({
                                                        "type": "image",
                                                        "image_url": {
                                                            "url": url,
                                                            "detail": media_type
                                                        }
                                                    }));
                                                }
                                            }
                                            Err(e) => {
                                                error!("Failed to process image content: {}", e);
                                                text_parts.push(format!("[Failed to process image: {}]", url));
                                            }
                                        }
                                    }
                                }
                                Some("tool_call") => {
                                    if let Ok(tool_call) = self.transform_tool_calls(block) {
                                        text_parts.push(serde_json::to_string(&tool_call)?);
                                    }
                                }
                                _ => continue,
                            }
                        }

                        let mut content_parts = Vec::new();
                        if !text_parts.is_empty() {
                            content_parts.push(json!({
                                "type": "text",
                                "text": text_parts.join("\n")
                            }));
                        }
                        content_parts.extend(image_parts);
                        content_parts
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
            let transformed_tools = tools
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|tool| {
                    if tool["type"] == "function" {
                        Some(json!({
                            "type": "function",
                            "function": {
                                "name": tool["function"]["name"].as_str()?,
                                "description": tool["function"]["description"].as_str().unwrap_or(""),
                                "parameters": tool["function"]["parameters"].clone()
                            }
                        }))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            transformed["tools"] = json!(transformed_tools);
        }

        debug!("Transformed body for Fireworks: {:#?}", transformed);
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
                    if let Some(delta) = json.get_mut("choices")
                        .and_then(|choices| choices[0].get_mut("delta"))
                    {
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
impl Provider for FireworksProvider {
    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    fn name(&self) -> &str {
        "fireworks"
    }

    fn process_headers(&self, original_headers: &HeaderMap) -> Result<HeaderMap, AppError> {
        debug!("Processing Fireworks request headers");
        let mut headers = HeaderMap::new();

        headers.insert(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );

        headers.insert(
            http::header::ACCEPT,
            http::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth) = original_headers
            .get(http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
        {
            if auth.trim().is_empty() {
                error!("Empty authorization token provided for Fireworks");
                return Err(AppError::InvalidHeader);
            }

            if !auth.starts_with("Bearer ") {
                error!("Invalid authorization format for Fireworks - must start with 'Bearer'");
                return Err(AppError::InvalidHeader);
            }

            if auth.len() <= 7 {
                error!("Empty Bearer token in Fireworks authorization header");
                return Err(AppError::InvalidHeader);
            }

            headers.insert(
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(auth).map_err(|_| {
                    error!("Invalid characters in Fireworks authorization header");
                    AppError::InvalidHeader
                })?,
            );
        } else {
            error!("Missing 'Authorization' header for Fireworks API request");
            return Err(AppError::MissingApiKey);
        }

        Ok(headers)
    }

    fn transform_path(&self, path: &str) -> String {
        if path.starts_with("/v1/") {
            path.trim_start_matches("/v1").to_string()
        } else {
            path.to_string()
        }
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
