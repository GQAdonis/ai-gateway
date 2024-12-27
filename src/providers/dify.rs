use super::Provider;
use crate::error::AppError;
use async_trait::async_trait;
use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, Response},
};
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tracing::{debug, error};

// File type support varies by endpoint
const CHAT_SUPPORTED_IMAGE_FORMATS: [&str; 4] = ["image/jpeg", "image/png", "image/gif", "image/webp"];
const WORKFLOW_SUPPORTED_DOC_FORMATS: [&str; 3] = ["application/pdf", "application/msword", "text/plain"];
const CHAT_MAX_FILE_SIZE: usize = 10_485_760; // 10MB for images
const WORKFLOW_MAX_FILE_SIZE: usize = 52_428_800; // 50MB for documents

#[derive(Clone)]
pub struct DifyProvider {
    base_url: Arc<Mutex<String>>,
    http_client: Arc<Client>,
}

impl DifyProvider {
    pub fn new() -> Self {
        Self {
            base_url: Arc::new(Mutex::new(String::new())), // Will be set from x-dify-base-url header
            http_client: Arc::new(Client::new()),
        }
    }

    async fn validate_file(&self, url: &str, is_workflow: bool) -> Result<bool, AppError> {
        let response = self.http_client.head(url).send().await.map_err(|e| {
            AppError::ValidationError(format!("Failed to validate file URL: {}", e))
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

        // Check file type based on endpoint
        if is_workflow {
            if !WORKFLOW_SUPPORTED_DOC_FORMATS.iter().any(|&f| content_type.contains(f)) {
                return Err(AppError::ValidationError(format!(
                    "Unsupported document format for workflow: {}",
                    content_type
                )));
            }
            if content_length > WORKFLOW_MAX_FILE_SIZE {
                return Err(AppError::ValidationError(format!(
                    "Document size {} exceeds workflow maximum limit of {} bytes",
                    content_length, WORKFLOW_MAX_FILE_SIZE
                )));
            }
        } else {
            if !CHAT_SUPPORTED_IMAGE_FORMATS.iter().any(|&f| content_type.contains(f)) {
                return Err(AppError::ValidationError(format!(
                    "Unsupported image format for chat: {}",
                    content_type
                )));
            }
            if content_length > CHAT_MAX_FILE_SIZE {
                return Err(AppError::ValidationError(format!(
                    "Image size {} exceeds chat maximum limit of {} bytes",
                    content_length, CHAT_MAX_FILE_SIZE
                )));
            }
        }

        Ok(true)
    }

    async fn validate_request_body(&self, body: &Value, headers: &HeaderMap) -> Result<(), AppError> {
        let is_workflow = headers.get("x-dify-workflow-id").is_some();

        // Validate file content if present
        if let Some(messages) = body.get("messages").and_then(Value::as_array) {
            for msg in messages {
                if let Some(content) = msg.get("content").and_then(Value::as_array) {
                    for block in content {
                        match block.get("type").and_then(Value::as_str) {
                            Some("image_url") if !is_workflow => {
                                // Images only supported in chat flows
                                let url = block
                                    .get("image_url")
                                    .and_then(|u| u.get("url"))
                                    .and_then(Value::as_str)
                                    .ok_or_else(|| AppError::ValidationError("Invalid image URL".into()))?;
                                
                                self.validate_file(url, false).await?;
                            }
                            Some("file") if is_workflow => {
                                // Documents only supported in workflows
                                if let Some(file) = block.get("file") {
                                    let url = file
                                        .get("url")
                                        .and_then(Value::as_str)
                                        .ok_or_else(|| AppError::ValidationError("Invalid file URL".into()))?;
                                    
                                    self.validate_file(url, true).await?;
                                }
                            }
                            Some("file") if !is_workflow => {
                                return Err(AppError::ValidationError(
                                    "Document files not supported in chat flows".into()
                                ));
                            }
                            Some("image_url") if is_workflow => {
                                return Err(AppError::ValidationError(
                                    "Images not supported in workflows".into()
                                ));
                            }
                            _ => continue,
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

    async fn transform_request_body(&self, body: Value, headers: &HeaderMap) -> Result<Value, AppError> {
        debug!("Transforming request body for Dify: {:#?}", body);

        // Validate the request body
        self.validate_request_body(&body, headers).await?;

        let is_workflow = headers.get("x-dify-workflow-id").is_some();
        let mut transformed = json!({
            "response_mode": if body.get("stream").and_then(Value::as_bool).unwrap_or(false) {
                "streaming"
            } else {
                "blocking"
            }
        });

        // Transform messages
        if let Some(messages) = body.get("messages").and_then(Value::as_array) {
            let mut query = String::new();
            let mut files = Vec::new();

            // Process messages to extract query and files
            for msg in messages {
                if msg.get("role").and_then(Value::as_str) == Some("user") {
                    if let Some(content) = msg.get("content") {
                        match content {
                            Value::String(text) => {
                                query = text.clone();
                            }
                            Value::Array(blocks) => {
                                for block in blocks {
                                    match block.get("type").and_then(Value::as_str) {
                                        Some("text") => {
                                            if let Some(text) = block.get("text").and_then(Value::as_str) {
                                                if !query.is_empty() {
                                                    query.push_str("\n");
                                                }
                                                query.push_str(text);
                                            }
                                        }
                                        Some("image_url") if !is_workflow => {
                                            if let Some(url) = block
                                                .get("image_url")
                                                .and_then(|u| u.get("url"))
                                                .and_then(Value::as_str)
                                            {
                                                files.push(json!({
                                                    "type": "image",
                                                    "transfer_method": "remote_url",
                                                    "url": url
                                                }));
                                            }
                                        }
                                        Some("file") if is_workflow => {
                                            if let Some(file) = block.get("file") {
                                                if let Some(url) = file.get("url").and_then(Value::as_str) {
                                                    files.push(json!({
                                                        "type": "document",
                                                        "transfer_method": "remote_url",
                                                        "url": url
                                                    }));
                                                }
                                            }
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }

            transformed["query"] = json!(query);
            if !files.is_empty() {
                transformed["files"] = json!(files);
            }
        }

        // Transform tools
        if let Some(tools) = body.get("tools") {
            transformed["tools"] = tools.clone();
        }

        // Add chat ID if present for chat flows
        if !is_workflow {
            if let Some(chat_id) = headers.get("x-chat-id").and_then(|h| h.to_str().ok()) {
                transformed["conversation_id"] = json!(chat_id);
            }
        }

        // Add user if present
        if let Some(user) = body.get("user") {
            transformed["user"] = user.clone();
        }

        debug!("Transformed body for Dify: {:#?}", transformed);
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
                    // Transform Dify streaming format to OpenAI format
                    let transformed = json!({
                        "id": json.get("id").unwrap_or(&json!("msg")),
                        "object": "chat.completion.chunk",
                        "created": json.get("created").unwrap_or(&json!(0)),
                        "model": "dify",
                        "choices": [{
                            "index": 0,
                            "delta": {
                                "content": json.get("answer").and_then(Value::as_str).unwrap_or("")
                            },
                            "finish_reason": json.get("finish_reason").and_then(Value::as_str).unwrap_or(null)
                        }]
                    });
                    transformed_lines.push(format!("data: {}", transformed.to_string()));
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
impl Provider for DifyProvider {
    fn base_url(&self) -> String {
        self.base_url.lock().unwrap().clone()
    }

    fn name(&self) -> &str {
        "dify"
    }

    fn process_headers(&self, original_headers: &HeaderMap) -> Result<HeaderMap, AppError> {
        debug!("Processing Dify request headers");
        let mut headers = HeaderMap::new();

        headers.insert(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );

        // Set base URL from header
        if let Some(base_url) = original_headers
            .get("x-dify-base-url")
            .and_then(|h| h.to_str().ok())
        {
            debug!("Setting Dify base URL from header: {}", base_url);
            *self.base_url.lock().unwrap() = base_url.trim_end_matches('/').to_string();
        } else {
            error!("No x-dify-base-url header found");
            return Err(AppError::InvalidHeader);
        }

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
            error!("No authorization header found for Dify request");
            return Err(AppError::MissingApiKey);
        }

        Ok(headers)
    }

    async fn prepare_request_body(&self, body: Bytes) -> Result<Bytes, AppError> {
        if let Ok(json_body) = serde_json::from_slice::<Value>(&body) {
            let transformed_body = self.transform_request_body(json_body).await?;
            Ok(Bytes::from(transformed_body.to_string()))
        } else {
            Ok(body)
        }
    }

    fn transform_path(&self, path: &str) -> String {
        path.to_string()
    }

    fn transform_path_with_headers(&self, path: &str, headers: &HeaderMap) -> String {
        if path.contains("/chat/completions") {
            // Check for workflow ID in headers
            if let Some(workflow_id) = headers
                .get("x-dify-workflow-id")
                .and_then(|h| h.to_str().ok())
            {
                debug!("Using workflow endpoint with ID: {}", workflow_id);
                format!("/v1/workflows/{}/run", workflow_id)
            } else {
                "/v1/chat-messages".to_string()
            }
        } else {
            path.to_string()
        }
    }

    async fn process_response(&self, response: Response<Body>) -> Result<Response<Body>, AppError> {
        if response.headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map_or(false, |ct| ct.contains("text/event-stream"))
        {
            let status = response.status();
            let this = self.clone();
            let stream = response
                .into_body()
                .into_data_stream()
                .map(move |chunk| match chunk {
                    Ok(bytes) => match this.transform_streaming_response(bytes) {
                        Ok(transformed) => Ok(transformed),
                        Err(e) => {
                            error!("Error transforming response chunk: {}", e);
                            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
                        }
                    },
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
                });

            Ok(Response::builder()
                .status(status)
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
