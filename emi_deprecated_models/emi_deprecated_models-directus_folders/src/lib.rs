//! Auto-generated crate for the `directus_folders` table.
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
/// Struct representing a row in the `directus_folders` table.
# [diesel (table_name = directus_folders)]
pub struct DirectusFolder {
    /// Field representing the `id` column in table `directus_folders`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `directus_folders`.
    name: String,
    /// Field representing the `parent` column in table `directus_folders`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent: Option<::rosetta_uuid::Uuid>,
}
:: diesel_builders :: prelude :: fk ! ((directus_folders :: parent) -> (directus_folders :: id));
