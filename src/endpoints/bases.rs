use crate::client::TeableClient;
use crate::errors::ClientError;
use crate::models::base::collaborator::{GetCollaboratorsQuery, GetCollaboratorsResponse};
use crate::models::base::order::Order;
use crate::models::base::{Base, PostBaseRequest, UpdateBaseRequest, UpdateBaseResponse};

use reqwest::Method;

pub struct BasesApi<'a> {
    pub(crate) client: &'a TeableClient,
}

#[maybe_async::maybe_async]
impl<'a> BasesApi<'a> {
    /**
     * Creates a new base in the specified space.
     * [See the API documentation](https://help.teable.ai/en/api-reference/base/post-base) for more information.
     */
    pub async fn post(&self, request: &PostBaseRequest) -> Result<Base, ClientError> {
        self.client
            .execute(Method::POST, "base", None::<&()>, Some(request))
            .await
    }

    /**
     * Fetches a base by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/base/get-base) for more information.
     */
    pub async fn get(&self, base_id: &str) -> Result<Base, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("base/{base_id}"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    /**
     * Deletes a base by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/base/delete-base) for more information.
     */
    pub async fn delete(&self, base_id: &str) -> Result<(), ClientError> {
        self.client
            .execute_empty(
                Method::DELETE,
                &format!("base/{base_id}"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    /**
     * Updates a base by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/base/patch-base) for more information.
     */
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

    /**
     * Updates the order of a base by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/base/put-base-order) for more information.
     */
    pub async fn update_order(&self, base_id: &str, order: &Order) -> Result<(), ClientError> {
        self.client
            .execute_empty(
                Method::PUT,
                &format!("base/{base_id}/order"),
                None::<&()>,
                Some(order),
            )
            .await
    }

    /**
     * Fetches all bases the user has access to.
     * [See the API documentation](https://help.teable.ai/en/api-reference/base/get-baseaccessall) for more information.
     */
    pub async fn get_base_access_all(&self) -> Result<Vec<Base>, ClientError> {
        self.client
            .execute(Method::GET, "base/access/all", None::<&()>, None::<&()>)
            .await
    }

    /**
     * Lists all collaborators of a base by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/base/get-base-collaborators) for more information.
     */
    pub async fn get_collaborators(
        &self,
        base_id: &str,
        query: Option<&GetCollaboratorsQuery>,
    ) -> Result<GetCollaboratorsResponse, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("base/{base_id}/collaborators"),
                query,
                None::<&()>,
            )
            .await
    }

    /**
     * Lists all bases in a space by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/base/get-space-base) for more information.
     */
    pub async fn list_bases(&self, space_id: &str) -> Result<Vec<Base>, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("space/{space_id}/base"),
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
