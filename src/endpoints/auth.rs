use crate::client::TeableClient;
use crate::errors::ClientError;
use crate::models::user::User;

use reqwest::Method;

pub struct AuthApi<'a> {
    pub(crate) client: &'a TeableClient,
}

#[maybe_async::maybe_async]
impl<'a> AuthApi<'a> {
    /**
     * Fetches the currently authenticated user via access token.
     * [See the API documentation](https://help.teable.ai/en/api-reference/auth/get-authuser) for more information.
     */
    pub async fn get_user(&self) -> Result<User, ClientError> {
        self.client
            .execute(Method::GET, "auth/user", None::<&()>, None::<&()>)
            .await
    }

    /**
     * Fetches the currently authenticated user.
     * [See the API documentation](https://help.teable.ai/en/api-reference/auth/get-authuser) for more information.
     */
    pub async fn get_user_me(&self) -> Result<User, ClientError> {
        self.get_user().await
    }
}

impl TeableClient {
    pub fn auth(&self) -> AuthApi<'_> {
        AuthApi { client: self }
    }
}
