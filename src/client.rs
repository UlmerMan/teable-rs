use crate::auth::{AuthProvider, PersonalAccessToken};
use crate::errors::{ClientBuilderError, ClientError};

use std::path;
use std::time::Duration;
use std::sync::Arc;

use reqwest::{Body, Method, Url};
use secrecy::SecretString;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct TeableClient {
    http: reqwest::Client,
    base_url: String,
    auth: Arc<dyn AuthProvider>,
}

pub struct TeableClientBuilder {
    base_url: Option<Url>,
    token: Option<SecretString>,
    timeout: Duration,
    user_agent: String,
    max_retries: u32,
    danger_accept_invalid_certs: bool,
}

impl Default for TeableClientBuilder {
    fn default() -> Self {
        Self {
            base_url: None,
            token: None,
            timeout: Duration::from_secs(30),
            user_agent: format!("teable-rs/{}", env!("CARGO_PKG_VERSION")),
            max_retries: 3,
            danger_accept_invalid_certs: false,
        }
    }
}

impl TeableClientBuilder {
    pub fn base_url(mut self, url: impl Into<String>) -> Result<Self, ClientBuilderError> {
        self.base_url = Some(Url::parse(&url.into())?);
        Ok(self)
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(SecretString::new(token.into().into()));
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.danger_accept_invalid_certs = accept;
        self
    }

    pub fn build(self) -> Result<TeableClient, ClientBuilderError> {
        let token = self.token.ok_or(ClientBuilderError::MissingToken)?;
        let base_url = self.base_url.unwrap_or_else(|| {
            Url::parse("https://app.teable.ai").expect("Invalid default URL")
        });

        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .user_agent(self.user_agent)
            .tls_danger_accept_invalid_certs(self.danger_accept_invalid_certs)
            .build()?;

        Ok(TeableClient {
            http,
            base_url: base_url.to_string(),
            auth: Arc::new(PersonalAccessToken::new(token)),
        })
            
    }

}

impl TeableClient {
    pub async fn execute<Resp: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: Option<&impl Serialize>,
        body: Option<&impl Serialize>,
    ) -> Result<Resp, ClientError> {
        let url = Url::parse(&format!("{}{}", self.base_url, path))?;
        let mut req = self.http.request(method, url);
        req = self.auth.apply(req);

        
        if let Some(query) = query {
            req = req.query(query);
        }

        if let Some(body) = body {
            req = req.json(body);
        }

        let resp = req.send().await?;
        
        if !resp.status().is_success() {
            return Err(ClientError::Api {
                status: resp.status().as_u16(),
                code: resp.status().to_string(),
                message: resp.text().await?,
            });
        }

        let body = resp.json().await?;
        Ok(body)
    }
}