//! Auto-generated crate for the `directus_roles` table.
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
/// Struct representing a row in the `directus_roles` table.
# [diesel (table_name = directus_roles)]
pub struct DirectusRole {
    /// Field representing the `id` column in table `directus_roles`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `directus_roles`.
    name: String,
    /// Field representing the `icon` column in table `directus_roles`.
    #[table_model(default = "supervised_user_circle")]
    icon: String,
    /// Field representing the `description` column in table `directus_roles`.
    description: Option<String>,
    /// Field representing the `parent` column in table `directus_roles`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent: Option<::rosetta_uuid::Uuid>,
}
:: diesel_builders :: prelude :: fk ! ((directus_roles :: parent) -> (directus_roles :: id));
