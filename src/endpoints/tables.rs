use reqwest::Method;

use crate::{client::TeableClient, errors::ClientError, models::table::Table};

pub struct TablesApi<'a> {
    pub(crate) client: &'a TeableClient,
}

impl<'a> TablesApi<'a> {
    /**
     * Fetches a table by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/table/get-table-details) for more information.
     */
    pub async fn get_details(&self, base_id: &str, table_id: &str) -> Result<Table, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("base/{base_id}/table/{table_id}"),
                None::<&()>,
                None::<&()>,
        ).await
    }

    /**
     * Lists all tables in a base by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/table/list-tables) for more information.
     */
    pub async fn list_tables(&self, base_id: &str) -> Result<Vec<Table>, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("base/{base_id}/table"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }
}

impl TeableClient {
    pub fn tables(&self) -> TablesApi<'_> {
        TablesApi { client: self }
    }
}