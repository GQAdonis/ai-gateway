pub mod anthropic;
pub mod bedrock;
pub mod dify;
pub mod fireworks;
pub mod groq;
pub mod openai;
pub mod together;

use crate::error::AppError;
use async_trait::async_trait;
use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, Response},
};

#[async_trait]
pub trait Provider: Send + Sync {
    fn base_url(&self) -> String;

    fn name(&self) -> &str;

    fn process_headers(&self, headers: &HeaderMap) -> Result<HeaderMap, AppError>;

    async fn prepare_request_body(&self, body: Bytes) -> Result<Bytes, AppError>;

    #[allow(unused_variables)]
    async fn sign_request(
        &self,
        method: &str,
        url: &str,
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<HeaderMap, AppError> {
        Ok(headers.clone())
    }

    #[allow(unused_variables)]
    async fn before_request(&self, headers: &HeaderMap, body: &Bytes) -> Result<(), AppError> {
        Ok(())
    }

    fn transform_path(&self, path: &str) -> String {
        path.to_string()
    }

    #[allow(unused_variables)]
    fn transform_path_with_headers(&self, path: &str, headers: &HeaderMap) -> String {
        self.transform_path(path)
    }

    fn requires_signing(&self) -> bool {
        false
    }

    fn get_signing_host(&self) -> String {
        String::new()
    }

    async fn process_response(&self, response: Response<Body>) -> Result<Response<Body>, AppError> {
        Ok(response)
    }
}
