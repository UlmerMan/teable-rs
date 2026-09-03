use reqwest::Method;

use crate::{client::TeableClient, errors::ClientError, models::field::Field};

pub struct FieldsApi<'a> {
    pub(crate) client: &'a TeableClient,
}

impl<'a> FieldsApi<'a> {
    /**
     * Lists all fields in a table by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/field/list-fields) for more information.
     */
    pub async fn list_fields(&self, table_id: &str) -> Result<Vec<Field>, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("table/{table_id}/field"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn create_field(
        &self,
        table_id: &str,
        request: &crate::models::field::Field,
    ) -> Result<Field, ClientError> {
        self.client
            .execute(
                Method::POST,
                &format!("table/{table_id}/field"),
                None::<&()>,
                Some(request),
            )
            .await
    }
}

impl TeableClient {
    pub fn fields(&self) -> FieldsApi<'_> {
        FieldsApi { client: self }
    }
}
