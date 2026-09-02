use reqwest_middleware::RequestBuilder;
use secrecy::{ExposeSecret, SecretString};

pub trait AuthProvider: Send + Sync {
    fn apply(&self, req: RequestBuilder) -> RequestBuilder;
}

pub struct PersonalAccessToken(SecretString);

impl PersonalAccessToken {
    pub fn new(token: SecretString) -> Self {
        Self(token)
    }
}

impl AuthProvider for PersonalAccessToken {
    fn apply(&self, req: RequestBuilder) -> RequestBuilder {
        req.bearer_auth(self.0.expose_secret())
    }
}

impl From<String> for PersonalAccessToken {
    fn from(token: String) -> Self {
        Self(SecretString::new(token.into()))
    }
}
