use crate::auth::{AuthProvider, PersonalAccessToken};
use crate::errors::{ClientBuilderError, ClientError};

use std::sync::Arc;
use std::time::Duration;

use reqwest::{Method, Url};
use reqwest_middleware::ClientBuilder as MwClientBuilder;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use secrecy::SecretString;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct TeableClient {
    http: ClientWithMiddleware,
    base_url: Url,
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
        let base_url = self
            .base_url
            .unwrap_or_else(|| Url::parse("https://app.teable.ai/api/").expect("Invalid default URL"));

        let inner_http = reqwest::Client::builder()
            .timeout(self.timeout)
            .user_agent(self.user_agent)
            .tls_danger_accept_invalid_certs(self.danger_accept_invalid_certs)
            .build()?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(self.max_retries);

        let http = MwClientBuilder::new(inner_http)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Ok(TeableClient {
            http,
            base_url,
            auth: Arc::new(PersonalAccessToken::new(token)),
        })
    }
}

#[maybe_async::maybe_async]
impl TeableClient {
    pub fn builder() -> TeableClientBuilder {
        TeableClientBuilder::default()
    }
    pub async fn execute<Resp>(
        &self,
        method: Method,
        path: &str,
        query: Option<&impl Serialize>,
        body: Option<&impl Serialize>,
    ) -> Result<Resp, ClientError>
    where
        Resp: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;

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
            return Err(self.parse_api_error(resp).await);
        }

        Ok(resp.json::<Resp>().await?)
    }

    pub async fn execute_empty(
        &self,
        method: Method,
        path: &str,
        query: Option<&impl Serialize>,
        body: Option<&impl Serialize>,
    ) -> Result<(), ClientError> {
        let url = self.base_url.join(path)?;

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
            return Err(self.parse_api_error(resp).await);
        }

        Ok(())
    }

    async fn parse_api_error(&self, resp: reqwest::Response) -> ClientError {
        let status = resp.status();

        let message = match resp.text().await {
            Ok(body) if !body.is_empty() => body,
            Ok(_) => status.to_string(),
            Err(err) => err.to_string(),
        };

        ClientError::Api {
            status: status.as_u16(),
            code: status.to_string(),
            message,
        }
    }
}
