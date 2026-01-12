//! Auto-generated crate for the `directus_fields` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_fields` table.
#[table_model(surrogate_key)]
# [diesel (table_name = directus_fields)]
pub struct DirectusField {
    /// Field representing the `id` column in table `directus_fields`.
    id: i32,
    /// Field representing the `collection` column in table `directus_fields`.
    collection: String,
    /// Field representing the `field` column in table `directus_fields`.
    field: String,
    /// Field representing the `special` column in table `directus_fields`.
    special: Option<String>,
    /// Field representing the `interface` column in table `directus_fields`.
    interface: Option<String>,
    /// Field representing the `options` column in table `directus_fields`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    options: Option<::serde_json::Value>,
    /// Field representing the `display` column in table `directus_fields`.
    display: Option<String>,
    /// Field representing the `display_options` column in table
    /// `directus_fields`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    display_options: Option<::serde_json::Value>,
    /// Field representing the `readonly` column in table `directus_fields`.
    #[table_model(default = false)]
    readonly: bool,
    /// Field representing the `hidden` column in table `directus_fields`.
    #[table_model(default = false)]
    hidden: bool,
    /// Field representing the `sort` column in table `directus_fields`.
    sort: Option<i32>,
    /// Field representing the `width` column in table `directus_fields`.
    #[table_model(default = "full")]
    width: Option<String>,
    /// Field representing the `translations` column in table `directus_fields`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    translations: Option<::serde_json::Value>,
    /// Field representing the `note` column in table `directus_fields`.
    note: Option<String>,
    /// Field representing the `conditions` column in table `directus_fields`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    conditions: Option<::serde_json::Value>,
    /// Field representing the `required` column in table `directus_fields`.
    #[table_model(default = false)]
    required: Option<bool>,
    /// Field representing the `group` column in table `directus_fields`.
    group: Option<String>,
    /// Field representing the `validation` column in table `directus_fields`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    validation: Option<::serde_json::Value>,
    /// Field representing the `validation_message` column in table
    /// `directus_fields`.
    validation_message: Option<String>,
}
