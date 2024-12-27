use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub workers: usize,
    pub max_connections: usize,
    pub keep_alive: u64,
    pub request_timeout: u64,
    pub response_timeout: u64,
    pub max_request_size: usize,
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            workers: env::var("WORKERS")
                .ok()
                .and_then(|w| w.parse().ok())
                .unwrap_or_else(num_cpus::get),
            max_connections: env::var("MAX_CONNECTIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(25_000),
            keep_alive: env::var("KEEP_ALIVE")
                .ok()
                .and_then(|k| k.parse().ok())
                .unwrap_or(90),
            request_timeout: env::var("REQUEST_TIMEOUT")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(60),
            response_timeout: env::var("RESPONSE_TIMEOUT")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(60),
            max_request_size: env::var("MAX_REQUEST_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5_242_880), // 5MB
        }
    }
}
