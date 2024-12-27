use crate::{
    config::AppConfig,
    error::AppError,
    providers::{
        anthropic::AnthropicProvider, bedrock::BedrockProvider, dify::DifyProvider,
        fireworks::FireworksProvider, groq::GroqProvider, openai::OpenAIProvider,
        together::TogetherProvider, Provider,
    },
};
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response},
};
use http_body_util::BodyExt;
use std::sync::Arc;
use tracing::{debug, error};

pub async fn proxy_request(
    _state: State<Arc<AppConfig>>,
    request: Request<Body>,
) -> Result<Response<Body>, AppError> {
    let provider_name = request
        .headers()
        .get("x-provider")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            error!("Missing x-provider header");
            AppError::InvalidRequestFormat
        })?;

    let provider = create_provider(provider_name).ok_or_else(|| {
        error!("Unsupported provider: {}", provider_name);
        AppError::UnsupportedModel
    })?;

    let headers = request.headers().clone();
    let path = request.uri().path().to_string();
    let method = request.method().clone();
    let url = format!(
        "{}{}",
        provider.base_url(),
        provider.transform_path_with_headers(&path, &headers)
    );

    debug!("Proxying request to: {}", url);

    // Convert the request body to bytes using http_body_util
    let (_parts, body) = request.into_parts();
    let bytes = body
        .collect()
        .await
        .map_err(|e| AppError::BodyReadError(e.to_string()))?
        .to_bytes();

    provider.before_request(&headers, &bytes).await?;

    let processed_headers = provider.process_headers(&headers)?;
    let processed_body = provider.prepare_request_body(bytes).await?;

    let mut client_request = reqwest::Request::new(
        method,
        reqwest::Url::parse(&url).map_err(|e| AppError::RequestError(e.to_string()))?,
    );

    *client_request.headers_mut() = processed_headers;
    *client_request.body_mut() = Some(processed_body.to_vec().into());

    if provider.requires_signing() {
        let body_bytes = processed_body.to_vec();
        let signed_headers = provider
            .sign_request(
                client_request.method().as_str(),
                &url,
                client_request.headers(),
                &body_bytes,
            )
            .await?;

        *client_request.headers_mut() = signed_headers;
    }

    let client = reqwest::Client::new();
    let response = client
        .execute(client_request)
        .await
        .map_err(|e| AppError::ProxyError(e.to_string()))?;

    let status = response.status();
    let headers = response.headers().clone();

    let mut builder = Response::builder().status(status);
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    let response = builder.body(Body::from_stream(response.bytes_stream())).unwrap();
    provider.process_response(response).await
}

pub fn create_provider(name: &str) -> Option<Box<dyn Provider>> {
    match name {
        "openai" => Some(Box::new(OpenAIProvider::new())),
        "anthropic" => Some(Box::new(AnthropicProvider::new())),
        "bedrock" => Some(Box::new(BedrockProvider::new())),
        "fireworks" => Some(Box::new(FireworksProvider::new())),
        "groq" => Some(Box::new(GroqProvider::new())),
        "together" => Some(Box::new(TogetherProvider::new())),
        "dify" => Some(Box::new(DifyProvider::new())),
        _ => None,
    }
}

pub async fn initialize_proxy(_config: Arc<AppConfig>) -> Result<(), AppError> {
    // Proxy initialization logic here if needed
    Ok(())
}
