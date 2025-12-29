//! Auto-generated crate for the `directus_revisions` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_revisions` table.
#[table_model(surrogate_key)]
# [diesel (table_name = directus_revisions)]
pub struct DirectusRevision {
    /// Field representing the `id` column in table `directus_revisions`.
    id: i32,
    /// Field representing the `activity` column in table `directus_revisions`.
    activity: i32,
    /// Field representing the `collection` column in table
    /// `directus_revisions`.
    collection: String,
    /// Field representing the `item` column in table `directus_revisions`.
    item: String,
    /// Field representing the `data` column in table `directus_revisions`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    data: Option<::serde_json::Value>,
    /// Field representing the `delta` column in table `directus_revisions`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    delta: Option<::serde_json::Value>,
    /// Field representing the `parent` column in table `directus_revisions`.
    parent: Option<i32>,
    /// Field representing the `version` column in table `directus_revisions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    version: Option<::rosetta_uuid::Uuid>,
}
:: diesel_builders :: prelude :: fk ! ((directus_revisions :: activity) -> (:: emi_deprecated_models_directus_activity :: directus_activity :: id));
:: diesel_builders :: prelude :: fk ! ((directus_revisions :: parent) -> (directus_revisions :: id));
:: diesel_builders :: prelude :: fk ! ((directus_revisions :: version) -> (:: emi_deprecated_models_directus_versions :: directus_versions :: id));
