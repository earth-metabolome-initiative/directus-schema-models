//! Auto-generated crate for the `directus_flows` table.
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
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_flows` table.
# [diesel (belongs_to (emi_deprecated_models_directus_users :: DirectusUser , foreign_key = user_created))]
# [diesel (table_name = directus_flows)]
pub struct DirectusFlow {
    /// Field representing the `id` column in table `directus_flows`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `directus_flows`.
    name: String,
    /// Field representing the `icon` column in table `directus_flows`.
    icon: Option<String>,
    /// Field representing the `color` column in table `directus_flows`.
    color: Option<String>,
    /// Field representing the `description` column in table `directus_flows`.
    description: Option<String>,
    /// Field representing the `status` column in table `directus_flows`.
    #[table_model(default = "active")]
    status: String,
    /// Field representing the `trigger` column in table `directus_flows`.
    trigger: Option<String>,
    /// Field representing the `accountability` column in table
    /// `directus_flows`.
    #[table_model(default = "all")]
    accountability: Option<String>,
    /// Field representing the `options` column in table `directus_flows`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    options: Option<::serde_json::Value>,
    /// Field representing the `operation` column in table `directus_flows`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    operation: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `directus_flows`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_created` column in table `directus_flows`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
}
::diesel_builders::prelude::unique_index!(directus_flows::operation);
:: diesel_builders :: prelude :: fpk ! (directus_flows :: user_created -> :: emi_deprecated_models_directus_users :: directus_users);
