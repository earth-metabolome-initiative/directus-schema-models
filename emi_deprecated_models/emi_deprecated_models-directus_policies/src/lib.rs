//! Auto-generated crate for the `directus_policies` table.
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
/// Struct representing a row in the `directus_policies` table.
# [diesel (table_name = directus_policies)]
pub struct DirectusPolicy {
    /// Field representing the `id` column in table `directus_policies`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `directus_policies`.
    name: String,
    /// Field representing the `icon` column in table `directus_policies`.
    #[table_model(default = "badge")]
    icon: String,
    /// Field representing the `description` column in table
    /// `directus_policies`.
    description: Option<String>,
    /// Field representing the `ip_access` column in table `directus_policies`.
    ip_access: Option<String>,
    /// Field representing the `enforce_tfa` column in table
    /// `directus_policies`.
    #[table_model(default = false)]
    enforce_tfa: bool,
    /// Field representing the `admin_access` column in table
    /// `directus_policies`.
    #[table_model(default = false)]
    admin_access: bool,
    /// Field representing the `app_access` column in table `directus_policies`.
    #[table_model(default = false)]
    app_access: bool,
}
