use serde::{Deserialize, Serialize};

use crate::models::{
    ai::config::AiConfig,
    field::{
        field_types::{CellValueType, DBFieldType, FieldType},
        lookup_options::LookupOptions,
        meta::Meta,
        options::FieldOptions,
    },
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Field {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<FieldOptions>,

    #[serde(rename = "cellValueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_value_type: Option<CellValueType>,
    #[serde(rename = "dbFieldType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_field_type: Option<DBFieldType>,
    #[serde(rename = "dbFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_field_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,

    #[serde(rename = "aiConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_config: Option<AiConfig>,

    #[serde(rename = "isLookup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_lookup: Option<bool>,

    #[serde(rename = "isConditionalLookup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_conditional_lookup: Option<bool>,

    #[serde(rename = "lookupOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_options: Option<LookupOptions>,

    #[serde(rename = "notNull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_null: Option<bool>,

    #[serde(rename = "isPrimary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,

    #[serde(rename = "isComputed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_computed: Option<bool>,

    #[serde(rename = "isPending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pending: Option<bool>,

    #[serde(rename = "hasError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,

    #[serde(rename = "isMultipleCellValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multiple_cell_value: Option<bool>,

    #[serde(rename = "recordRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_read: Option<bool>,

    #[serde(rename = "recordCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_create: Option<bool>,
}

/// Fluent builder for creating a [`Field`] value.
///
/// Only the field identity and [`FieldType`] are required. All other values
/// are optional because Teable omits several of them depending on the field
/// type and whether the field is being created or read.
#[derive(Debug)]
pub struct FieldBuilder {
    field: Field,
}

impl Field {
    pub fn builder(id: impl Into<String>, name: impl Into<String>, field_type: FieldType) -> FieldBuilder {
        FieldBuilder::new(id, name, field_type)
    }
}

impl FieldBuilder {
    /// Starts a builder with the required field identity.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        field_type: FieldType,
    ) -> Self {
        Self {
            field: Field {
                id: id.into(),
                name: name.into(),
                field_type,
                options: None,
                cell_value_type: None,
                db_field_type: None,
                db_field_name: None,
                description: None,
                meta: None,
                ai_config: None,
                is_lookup: None,
                is_conditional_lookup: None,
                lookup_options: None,
                not_null: None,
                is_primary: None,
                is_computed: None,
                is_pending: None,
                has_error: None,
                is_multiple_cell_value: None,
                record_read: None,
                record_create: None,
            },
        }
    }

    pub fn options(mut self, value: FieldOptions) -> Self {
        self.field.options = Some(value);
        self
    }

    pub fn cell_value_type(mut self, value: CellValueType) -> Self {
        self.field.cell_value_type = Some(value);
        self
    }

    pub fn db_field_type(mut self, value: DBFieldType) -> Self {
        self.field.db_field_type = Some(value);
        self
    }

    pub fn db_field_name(mut self, value: impl Into<String>) -> Self {
        self.field.db_field_name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.field.description = Some(value.into());
        self
    }

    pub fn meta(mut self, value: Meta) -> Self {
        self.field.meta = Some(value);
        self
    }

    pub fn ai_config(mut self, value: AiConfig) -> Self {
        self.field.ai_config = Some(value);
        self
    }

    pub fn is_lookup(mut self, value: bool) -> Self {
        self.field.is_lookup = Some(value);
        self
    }

    pub fn is_conditional_lookup(mut self, value: bool) -> Self {
        self.field.is_conditional_lookup = Some(value);
        self
    }

    pub fn lookup_options(mut self, value: LookupOptions) -> Self {
        self.field.lookup_options = Some(value);
        self
    }

    pub fn not_null(mut self, value: bool) -> Self {
        self.field.not_null = Some(value);
        self
    }

    pub fn is_primary(mut self, value: bool) -> Self {
        self.field.is_primary = Some(value);
        self
    }

    pub fn is_computed(mut self, value: bool) -> Self {
        self.field.is_computed = Some(value);
        self
    }

    pub fn is_pending(mut self, value: bool) -> Self {
        self.field.is_pending = Some(value);
        self
    }

    pub fn has_error(mut self, value: bool) -> Self {
        self.field.has_error = Some(value);
        self
    }

    pub fn is_multiple_cell_value(mut self, value: bool) -> Self {
        self.field.is_multiple_cell_value = Some(value);
        self
    }

    pub fn record_read(mut self, value: bool) -> Self {
        self.field.record_read = Some(value);
        self
    }

    pub fn record_create(mut self, value: bool) -> Self {
        self.field.record_create = Some(value);
        self
    }

    /// Finishes the builder and returns the configured field.
    pub fn build(self) -> Field {
        self.field
    }
}
