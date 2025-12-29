//! Auto-generated crate for the `directus_presets` table.
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
/// Struct representing a row in the `directus_presets` table.
#[table_model(surrogate_key)]
# [diesel (table_name = directus_presets)]
pub struct DirectusPreset {
    /// Field representing the `id` column in table `directus_presets`.
    id: i32,
    /// Field representing the `bookmark` column in table `directus_presets`.
    bookmark: Option<String>,
    /// Field representing the `user` column in table `directus_presets`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `role` column in table `directus_presets`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    role: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `collection` column in table `directus_presets`.
    collection: Option<String>,
    /// Field representing the `search` column in table `directus_presets`.
    search: Option<String>,
    /// Field representing the `layout` column in table `directus_presets`.
    #[table_model(default = "tabular")]
    layout: Option<String>,
    /// Field representing the `layout_query` column in table
    /// `directus_presets`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    layout_query: Option<::serde_json::Value>,
    /// Field representing the `layout_options` column in table
    /// `directus_presets`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    layout_options: Option<::serde_json::Value>,
    /// Field representing the `refresh_interval` column in table
    /// `directus_presets`.
    refresh_interval: Option<i32>,
    /// Field representing the `filter` column in table `directus_presets`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    filter: Option<::serde_json::Value>,
    /// Field representing the `icon` column in table `directus_presets`.
    #[table_model(default = "bookmark")]
    icon: Option<String>,
    /// Field representing the `color` column in table `directus_presets`.
    color: Option<String>,
}
:: diesel_builders :: prelude :: fk ! ((directus_presets :: role) -> (:: emi_deprecated_models_directus_roles :: directus_roles :: id));
:: diesel_builders :: prelude :: fk ! ((directus_presets :: user) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
