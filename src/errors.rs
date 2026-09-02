use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientBuilderError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Missing Token")]
    MissingToken,

    #[error("Invalid URL: {0}")]
    InvalidUrlError(String),

    #[error("HTTP Client Error: {0}")]
    HttpClientError(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),   

    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("HTTP middleware error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),

    #[error("Teable API error {status}: {code} — {message}")]
    Api { status: u16, code: String, message: String },
}