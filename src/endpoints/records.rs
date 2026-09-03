use reqwest::Method;

use crate::{
    client::TeableClient,
    errors::ClientError,
    models::record::{
        query::GetRecordQuery,
        record_structs::{
            CreateRecordsRequest, CreateRecordsResponse, Record, UpdateRecordRequest,
        },
    },
};

pub struct RecordsApi<'a> {
    pub(crate) client: &'a TeableClient,
}

impl<'a> RecordsApi<'a> {
    /** Creates one or more records in a table.
     *
     * [See the API documentation](https://help.teable.ai/en/api-reference/record/create-records) for more information.
     *
     * The field keys in each record are interpreted according to
     * [`FieldKeyType`](crate::models::record::FieldKeyType).
     */
    pub async fn create_records(
        &self,
        table_id: &str,
        request: &CreateRecordsRequest,
    ) -> Result<CreateRecordsResponse, ClientError> {
        self.client
            .execute(
                Method::POST,
                &format!("table/{table_id}/record"),
                None::<&()>,
                Some(request),
            )
            .await
    }

    /**
     * Get a record by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/record/get-record) for more information.
     */
    pub async fn get_record(
        &self,
        table_id: &str,
        record_id: &str,
        query: GetRecordQuery,
    ) -> Result<Record, ClientError> {
        self.client
            .execute(
                Method::GET,
                &format!("table/{table_id}/record/{record_id}"),
                Some(&query),
                None::<&()>,
            )
            .await
    }

    /**
     * Delete a record by its ID.
     * [See the API documentation](https://help.teable.ai/en/api-reference/record/delete-record) for more information.
     */
    pub async fn delete_record(&self, table_id: &str, record_id: &str) -> Result<(), ClientError> {
        self.client
            .execute_empty(
                Method::DELETE,
                &format!("table/{table_id}/record/{record_id}"),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn update_record(
        &self,
        table_id: &str,
        record_id: &str,
        request: &UpdateRecordRequest,
    ) -> Result<Record, ClientError> {
        self.client
            .execute(
                Method::PATCH,
                &format!("table/{table_id}/record/{record_id}"),
                None::<&()>,
                Some(request),
            )
            .await
    }
}

impl TeableClient {
    pub fn records(&self) -> RecordsApi<'_> {
        RecordsApi { client: self }
    }
}
