use reqwest::Method;

use crate::{
    client::TeableClient,
    errors::ClientError,
    models::space::space_structs::{PostSpaceRequest, Space},
};

pub struct SpacesApi<'a> {
    pub(crate) client: &'a TeableClient,
}

#[maybe_async::maybe_async]
impl<'a> SpacesApi<'a> {
    /**
     * Lists all spaces the user has access to.
     * [See the API documentation](https://help.teable.ai/en/api-reference/space/get-spacelist) for more information.
     */
    pub async fn get_space_list(&self) -> Result<Vec<Space>, ClientError> {
        self.client
            .execute(Method::GET, "space", None::<&()>, None::<&()>)
            .await
    }

    /**
     * Creates a new space.
     * [See the API documentation](https://help.teable.ai/en/api-reference/space/post-space) for more information.
     */
    pub async fn post(&self, request: &PostSpaceRequest) -> Result<Space, ClientError> {
        self.client
            .execute(Method::POST, "space", None::<&()>, Some(request))
            .await
    }

    /**
     * Fetches a space by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/space/get-space) for more information.
     */
    pub async fn get_space(&self, space_id: &str) -> Result<Space, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("space/{space_id}"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    /**
     * Deletes a space by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/space/delete-space) for more information.
     */
    pub async fn delete(&self, space_id: &str) -> Result<(), ClientError> {
        self.client
            .execute_empty(
                Method::DELETE,
                &format!("space/{space_id}"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    /**
     * Updates a space by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/space/patch-space) for more information.
     */
    pub async fn patch(
        &self,
        space_id: &str,
        request: &PostSpaceRequest,
    ) -> Result<Space, ClientError> {
        self.client
            .execute(
                Method::PATCH,
                &format!("space/{space_id}"),
                None::<&()>,
                Some(request),
            )
            .await
    }
}

impl TeableClient {
    pub fn spaces(&self) -> SpacesApi<'_> {
        SpacesApi { client: self }
    }
}
