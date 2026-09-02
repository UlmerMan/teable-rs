use crate::client::TeableClient;
use crate::errors::ClientError;
use crate::models::base::order::Order;
use crate::models::base::{
    Base, CreateBaseRequest, UpdateBaseRequest, UpdateBaseResponse,
};

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

    pub async fn patch(
        &self,
        base_id: &str,
        request: &UpdateBaseRequest,
    ) -> Result<UpdateBaseResponse, ClientError> {
        self.client
            .execute(
                Method::PATCH,
                &format!("base/{base_id}"),
                None::<&()>,
                Some(request),
            )
            .await
    }

    pub async fn update_order(
        &self,
        base_id: &str,
        order: &Order,
    ) -> Result<(), ClientError> {
        self.client
            .execute_empty(
                Method::PUT,
                &format!("base/{base_id}/order"),
                None::<&()>,
                Some(order),
            )
            .await
    }
}

impl TeableClient {
    pub fn bases(&self) -> BasesApi<'_> {
        BasesApi { client: self }
    }
}
