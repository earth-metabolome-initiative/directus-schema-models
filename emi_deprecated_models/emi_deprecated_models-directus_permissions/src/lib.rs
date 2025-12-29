//! Auto-generated crate for the `directus_permissions` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_permissions` table.
#[table_model(surrogate_key)]
# [diesel (table_name = directus_permissions)]
pub struct DirectusPermission {
    /// Field representing the `id` column in table `directus_permissions`.
    id: i32,
    /// Field representing the `collection` column in table
    /// `directus_permissions`.
    collection: String,
    /// Field representing the `action` column in table `directus_permissions`.
    action: String,
    /// Field representing the `permissions` column in table
    /// `directus_permissions`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    permissions: Option<::serde_json::Value>,
    /// Field representing the `validation` column in table
    /// `directus_permissions`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    validation: Option<::serde_json::Value>,
    /// Field representing the `presets` column in table `directus_permissions`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    presets: Option<::serde_json::Value>,
    /// Field representing the `fields` column in table `directus_permissions`.
    fields: Option<String>,
    /// Field representing the `policy` column in table `directus_permissions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    policy: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((directus_permissions :: policy) -> (:: emi_deprecated_models_directus_policies :: directus_policies :: id));
