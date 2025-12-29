//! Auto-generated crate for the `directus_relations` table.
#[derive(
    Clone,
    Default,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_relations` table.
#[table_model(surrogate_key)]
# [diesel (table_name = directus_relations)]
pub struct DirectusRelation {
    /// Field representing the `id` column in table `directus_relations`.
    id: i32,
    /// Field representing the `many_collection` column in table
    /// `directus_relations`.
    many_collection: String,
    /// Field representing the `many_field` column in table
    /// `directus_relations`.
    many_field: String,
    /// Field representing the `one_collection` column in table
    /// `directus_relations`.
    one_collection: Option<String>,
    /// Field representing the `one_field` column in table `directus_relations`.
    one_field: Option<String>,
    /// Field representing the `one_collection_field` column in table
    /// `directus_relations`.
    one_collection_field: Option<String>,
    /// Field representing the `one_allowed_collections` column in table
    /// `directus_relations`.
    one_allowed_collections: Option<String>,
    /// Field representing the `junction_field` column in table
    /// `directus_relations`.
    junction_field: Option<String>,
    /// Field representing the `sort_field` column in table
    /// `directus_relations`.
    sort_field: Option<String>,
    /// Field representing the `one_deselect_action` column in table
    /// `directus_relations`.
    #[table_model(default = "nullify")]
    one_deselect_action: String,
}
