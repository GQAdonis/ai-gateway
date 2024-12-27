use super::Provider;
use crate::error::AppError;
use async_trait::async_trait;
use aws_event_stream_parser::{parse_message, Message};
use aws_sigv4::http_request::{sign, SigningSettings, SigningParams};
use aws_sigv4::sign::v4;
use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, HeaderValue, Response, StatusCode},
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use futures_util::StreamExt;
use http::Request;
use parking_lot::RwLock;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{debug, error, warn};

const DEFAULT_REGION: &str = "us-east-1";
const DEFAULT_MODEL: &str = "amazon.titan-text-express-v1";
const DEFAULT_FALLBACK_MODEL: &str = "anthropic.claude-v2";
const DEFAULT_MAX_TOKENS: u64 = 1000;
const DEFAULT_TEMPERATURE: f64 = 0.7;
const DEFAULT_TOP_P: f64 = 1.0;
const MAX_IMAGE_SIZE: usize = 5_242_880; // 5MB
const SUPPORTED_IMAGE_FORMATS: [&str; 4] = ["image/jpeg", "image/png", "image/gif", "image/webp"];
const MAX_TOOL_CALLS: usize = 15;
const SUPPORTED_TOOL_TYPES: [&str; 1] = ["function"];

#[derive(Clone)]
pub struct BedrockProvider {
    base_url: Arc<RwLock<String>>,
    region: Arc<RwLock<String>>,
    current_model: Arc<RwLock<String>>,
    http_client: Arc<Client>,
}

impl BedrockProvider {
    pub fn new() -> Self {
        let region = DEFAULT_REGION.to_string();
        debug!("Initializing BedrockProvider with region: {}", region);

        Self {
            base_url: Arc::new(RwLock::new(format!(
                "https://bedrock-runtime.{}.amazonaws.com",
                region
            ))),
            region: Arc::new(RwLock::new(region)),
            current_model: Arc::new(RwLock::new(DEFAULT_MODEL.to_string())),
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
                .take(MAX_TOOL_CALLS)
                .filter_map(|call| {
                    let call_type = call.get("type")?.as_str()?;
                    if !SUPPORTED_TOOL_TYPES.contains(&call_type) {
                        return None;
                    }

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

            if transformed_calls.is_empty() {
                return Ok(json!({"type": "text", "text": ""}));
            }

            Ok(json!({
                "type": "tool_calls",
                "tool_calls": transformed_calls
            }))
        } else {
            Ok(content.clone())
        }
    }

    fn get_model_name(&self, path: &str) -> String {
        path.split('/')
            .last()
            .map(ToString::to_string)
            .unwrap_or_else(|| DEFAULT_FALLBACK_MODEL.to_string())
    }

    fn transform_request_body(&self, body: Value) -> Result<Value, AppError> {
        debug!("Transforming request body: {:#?}", body);

        if body.get("inferenceConfig").is_some() {
            return Ok(body);
        }

        let messages = body
            .get("messages")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                error!("Invalid request format: messages array not found");
                AppError::InvalidRequestFormat
            })?;

        let mut transformed_messages = Vec::new();
        for msg in messages {
            let role = msg["role"].as_str().unwrap_or("user");
            let content = msg.get("content");

            let transformed_content = match content {
                Some(content_value) => {
                    if content_value.is_array() {
                        content_value
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|block| {
                                match block.get("type").and_then(Value::as_str) {
                                    Some("text") => json!({
                                        "type": "text",
                                        "text": block.get("text").and_then(Value::as_str).unwrap_or_default()
                                    }),
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
                                            
                                            match self.process_image_content(url) {
                                                Ok(base64_data) => json!({
                                                    "type": "image",
                                                    "source": {
                                                        "type": "base64",
                                                        "media_type": media_type,
                                                        "data": base64_data
                                                    }
                                                }),
                                                Err(e) => {
                                                    error!("Failed to process image content: {}", e);
                                                    json!({"type": "text", "text": ""})
                                                }
                                            }
                                        } else {
                                            json!({"type": "text", "text": ""})
                                        }
                                    }
                                    Some("tool_call") => self.transform_tool_calls(block)
                                        .unwrap_or_else(|_| json!({"type": "text", "text": ""})),
                                    _ => json!({"type": "text", "text": ""})
                                }
                            })
                            .collect::<Vec<_>>()
                    } else if content_value.is_string() {
                        vec![json!({
                            "type": "text",
                            "text": content_value.as_str().unwrap_or_default()
                        })]
                    } else {
                        vec![json!({"type": "text", "text": ""})]
                    }
                }
                None => vec![json!({"type": "text", "text": ""})],
            };

            transformed_messages.push(json!({
                "role": role,
                "content": transformed_content
            }));
        }

        let transformed = json!({
            "messages": transformed_messages,
            "inferenceConfig": {
                "maxTokens": body.get("max_tokens")
                    .and_then(Value::as_u64)
                    .unwrap_or(DEFAULT_MAX_TOKENS),
                "temperature": body.get("temperature")
                    .and_then(Value::as_f64)
                    .unwrap_or(DEFAULT_TEMPERATURE),
                "topP": body.get("top_p")
                    .and_then(Value::as_f64)
                    .unwrap_or(DEFAULT_TOP_P)
            }
        });

        debug!("Transformed body: {:#?}", transformed);
        Ok(transformed)
    }

    fn transform_bedrock_chunk(&self, chunk: Bytes) -> Result<Bytes, AppError> {
        debug!("Processing chunk of size: {}", chunk.len());
        let mut remaining = chunk.as_ref();
        let mut response_events = Vec::new();

        while !remaining.is_empty() {
            match self.process_message(remaining) {
                Ok((rest, events)) => {
                    remaining = rest;
                    response_events.extend(events);
                }
                Err(e) => {
                    debug!("Failed to parse message: {:?}", e);
                    break;
                }
            }
        }

        Ok(Bytes::from(response_events.join("")))
    }

    fn process_message<'a>(&self, data: &'a [u8]) -> Result<(&'a [u8], Vec<String>), AppError> {
        let (rest, message) =
            parse_message(data).map_err(|e| AppError::EventStreamError(e.to_string()))?;

        let event_type = self.get_event_type(&message);
        let events = match event_type.as_deref() {
            Some("contentBlockDelta") => self.handle_content_block(&message)?,
            Some("metadata") => self.handle_metadata(&message)?,
            _ => {
                debug!("Skipping event type: {:?}", event_type);
                vec![]
            }
        };

        if !message.valid() {
            warn!("Invalid message checksum detected");
        }

        Ok((rest, events))
    }

    fn get_event_type(&self, message: &Message) -> Option<String> {
        message
            .headers
            .headers
            .iter()
            .find(|h| h.key == ":event-type")
            .and_then(|h| match &h.value {
                aws_event_stream_parser::HeaderValue::String(s) => Some(s.to_string()),
                _ => None,
            })
    }

    fn handle_content_block(&self, message: &Message) -> Result<Vec<String>, AppError> {
        let body_str = String::from_utf8(message.body.to_vec())?;
        let json: Value = serde_json::from_str(&body_str)?;

        if let Some(delta) = json.get("delta") {
            let response = if let Some(tool_calls) = delta.get("tool_calls") {
                self.create_tool_call_response(tool_calls)
            } else if let Some(text) = delta.get("text").and_then(Value::as_str) {
                self.create_delta_response(text)
            } else {
                return Ok(vec![]);
            };
            Ok(vec![format!("data: {}\n\n", response.to_string())])
        } else {
            Ok(vec![])
        }
    }

    fn handle_metadata(&self, message: &Message) -> Result<Vec<String>, AppError> {
        let body_str = String::from_utf8(message.body.to_vec())?;
        let json: Value = serde_json::from_str(&body_str)?;

        if let Some(usage) = json.get("usage") {
            let final_message = self.create_final_response(usage);
            Ok(vec![format!(
                "data: {}\ndata: [DONE]\n\n",
                final_message.to_string()
            )])
        } else {
            Ok(vec![])
        }
    }

    fn create_delta_response(&self, delta: &str) -> Value {
        json!({
            "id": "chatcmpl-bedrock",
            "object": "chat.completion.chunk",
            "created": chrono::Utc::now().timestamp(),
            "model": self.current_model.read().as_str(),
            "choices": [{
                "index": 0,
                "delta": {
                    "content": delta
                },
                "finish_reason": null
            }]
        })
    }

    fn create_tool_call_response(&self, tool_calls: &Value) -> Value {
        json!({
            "id": "chatcmpl-bedrock",
            "object": "chat.completion.chunk",
            "created": chrono::Utc::now().timestamp(),
            "model": self.current_model.read().as_str(),
            "choices": [{
                "index": 0,
                "delta": {
                    "tool_calls": tool_calls
                },
                "finish_reason": null
            }]
        })
    }

    fn create_final_response(&self, usage: &Value) -> Value {
        json!({
            "id": "chatcmpl-bedrock",
            "object": "chat.completion.chunk",
            "created": chrono::Utc::now().timestamp(),
            "model": self.current_model.read().as_str(),
            "choices": [{
                "index": 0,
                "delta": {},
                "finish_reason": "stop"
            }],
            "usage": usage
        })
    }

    async fn sign_request(&self, mut request: Request<Bytes>, credentials: (String, String, String)) -> Result<Request<Bytes>, AppError> {
        let (access_key, secret_key, region) = credentials;
        
        let signing_settings = SigningSettings::default();
        let signing_params = SigningParams::builder()
            .access_key(access_key)
            .secret_key(secret_key)
            .region(region)
            .service_name("bedrock")
            .time(OffsetDateTime::now_utc())
            .settings(signing_settings)
            .build()
            .map_err(|e| AppError::SigningError(e.to_string()))?;

        let (signing_instructions, _signature) = v4::sign(request.clone(), &signing_params)
            .map_err(|e| AppError::SigningError(e.to_string()))?;

        for (key, value) in signing_instructions.headers {
            request.headers_mut().insert(
                key.clone(),
                HeaderValue::from_str(&value).map_err(|e| AppError::SigningError(e.to_string()))?,
            );
        }

        Ok(request)
    }
}

#[async_trait]
impl Provider for BedrockProvider {
    fn base_url(&self) -> String {
        self.base_url.read().clone()
    }

    fn name(&self) -> &str {
        "bedrock"
    }

    async fn before_request(&self, headers: &HeaderMap, body: &Bytes) -> Result<(), AppError> {
        if let Ok(request_body) = serde_json::from_slice::<Value>(body) {
            if let Some(model) = request_body["model"].as_str() {
                debug!("Setting model from before_request: {}", model);
                *self.current_model.write() = model.to_string();
            }
        }

        if let Some(region) = headers.get("x-aws-region").and_then(|h| h.to_str().ok()) {
            debug!("Setting region from before_request: {}", region);
            *self.region.write() = region.to_string();
            *self.base_url.write() = format!("https://bedrock-runtime.{}.amazonaws.com", region);
        }

        Ok(())
    }

    fn transform_path(&self, path: &str) -> String {
        if path.contains("/chat/completions") {
            let model = self.current_model.read();
            format!("/model/{}/invoke", model)
        } else {
            path.to_string()
        }
    }

    fn requires_signing(&self) -> bool {
        true
    }

    fn get_signing_credentials(&self, headers: &HeaderMap) -> Option<(String, String, String)> {
        let access_key = headers.get("x-aws-access-key-id")?.to_str().ok()?;
        let secret_key = headers.get("x-aws-secret-access-key")?.to_str().ok()?;
        let region = headers
            .get("x-aws-region")
            .and_then(|h| h.to_str().ok())
            .map(String::from)
            .unwrap_or_else(|| self.region.read().clone());

        Some((access_key.to_string(), secret_key.to_string(), region))
    }

    fn get_signing_host(&self) -> String {
        let region = self.region.read().clone();
        format!("bedrock-runtime.{}.amazonaws.com", region)
    }

    async fn transform_request(&self, mut request: Request<Bytes>) -> Result<Request<Bytes>, AppError> {
        let body = request.body();
        if let Ok(json_body) = serde_json::from_slice::<Value>(body) {
            let transformed_body = self.transform_request_body(json_body)?;
            *request.body_mut() = Bytes::from(transformed_body.to_string());
        }
        Ok(request)
    }

    async fn process_response(&self, response: Response<Body>) -> Result<Response<Body>, AppError> {
        if response
            .headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map_or(false, |ct| ct.contains("application/vnd.amazon.eventstream"))
        {
            debug!("Processing Bedrock event stream response");

            let provider = self.clone();
            let stream = response
                .into_body()
                .into_data_stream()
                .map(move |chunk| match chunk {
                    Ok(bytes) => match provider.transform_bedrock_chunk(bytes) {
                        Ok(transformed) => Ok(transformed),
                        Err(e) => {
                            error!("Error transforming chunk: {}", e);
                            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
                        }
                    },
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
                });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/event-stream")
                .header("cache-control", "no-cache")
                .header("connection", "keep-alive")
                .header("transfer-encoding", "chunked")
                .header("access-control-allow-origin", "*")
                .header("access-control-allow-methods", "POST, OPTIONS")
                .header(
                    "access-control-allow-headers",
                    "content-type, x-provider, x-aws-access-key-id, x-aws-secret-access-key, x-aws-region"
                )
                .header("access-control-expose-headers", "*")
                .header("x-accel-buffering", "no")
                .header("keep-alive", "timeout=600")
                .body(Body::from_stream(stream))
                .unwrap())
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
                HeaderValue::from_static(
                    "content-type, x-provider, x-aws-access-key-id, x-aws-secret-access-key, x-aws-region"
                ),
            );
            headers.insert(
                "access-control-expose-headers",
                HeaderValue::from_static("*"),
            );
            Ok(response)
        }
    }
}
