//! Auto-generated crate for the `directus_webhooks` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_webhooks` table.
#[table_model(surrogate_key)]
# [diesel (table_name = directus_webhooks)]
pub struct DirectusWebhook {
    /// Field representing the `id` column in table `directus_webhooks`.
    id: i32,
    /// Field representing the `name` column in table `directus_webhooks`.
    name: String,
    /// Field representing the `method` column in table `directus_webhooks`.
    #[table_model(default = "POST")]
    method: String,
    /// Field representing the `url` column in table `directus_webhooks`.
    url: String,
    /// Field representing the `status` column in table `directus_webhooks`.
    #[table_model(default = "active")]
    status: String,
    /// Field representing the `data` column in table `directus_webhooks`.
    #[table_model(default = true)]
    data: bool,
    /// Field representing the `actions` column in table `directus_webhooks`.
    actions: String,
    /// Field representing the `collections` column in table
    /// `directus_webhooks`.
    collections: String,
    /// Field representing the `headers` column in table `directus_webhooks`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    headers: Option<::serde_json::Value>,
    /// Field representing the `was_active_before_deprecation` column in table
    /// `directus_webhooks`.
    #[table_model(default = false)]
    was_active_before_deprecation: bool,
    /// Field representing the `migrated_flow` column in table
    /// `directus_webhooks`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    migrated_flow: Option<::rosetta_uuid::Uuid>,
}
:: diesel_builders :: prelude :: fk ! ((directus_webhooks :: migrated_flow) -> (:: emi_deprecated_models_directus_flows :: directus_flows :: id));
