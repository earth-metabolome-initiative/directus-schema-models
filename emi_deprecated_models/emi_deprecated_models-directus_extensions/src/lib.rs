//! Auto-generated crate for the `directus_extensions` table.
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
/// Struct representing a row in the `directus_extensions` table.
# [diesel (table_name = directus_extensions)]
pub struct DirectusExtension {
    /// Field representing the `enabled` column in table `directus_extensions`.
    #[table_model(default = true)]
    enabled: bool,
    /// Field representing the `id` column in table `directus_extensions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `folder` column in table `directus_extensions`.
    folder: String,
    /// Field representing the `source` column in table `directus_extensions`.
    source: String,
    /// Field representing the `bundle` column in table `directus_extensions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    bundle: Option<::rosetta_uuid::Uuid>,
}
