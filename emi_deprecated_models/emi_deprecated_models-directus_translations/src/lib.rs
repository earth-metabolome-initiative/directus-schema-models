//! Auto-generated crate for the `directus_translations` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_translations` table.
# [diesel (table_name = directus_translations)]
pub struct DirectusTranslation {
    /// Field representing the `id` column in table `directus_translations`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `language` column in table
    /// `directus_translations`.
    language: String,
    /// Field representing the `key` column in table `directus_translations`.
    key: String,
    /// Field representing the `value` column in table `directus_translations`.
    value: String,
}
