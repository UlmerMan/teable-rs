use crate::client::TeableClient;
use crate::errors::ClientError;
use crate::models::base::{Base, CreateBaseRequest};

use reqwest::Method;

pub struct BasesApi<'a> {
    pub(crate) client: &'a TeableClient,
}

#[maybe_async::maybe_async]
impl<'a> BasesApi<'a> {
    pub async fn create(
        &self,
        request: &CreateBaseRequest,
    ) -> Result<Base, ClientError> {
        self.client
            .execute(
                Method::POST,
                "base",
                None::<&()>,
                Some(request),
            )
            .await
    }

    pub async fn get(
        &self,
        base_id: &str,
    ) -> Result<Base, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("base/{base_id}"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn delete(
        &self,
        base_id: &str,
    ) -> Result<(), ClientError> {
        self.client
            .execute_empty(
                Method::DELETE,
                &format!("base/{base_id}"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }
}

impl TeableClient {
    pub fn bases(&self) -> BasesApi<'_> {
        BasesApi { client: self }
    }
}
