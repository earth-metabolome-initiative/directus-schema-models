//! Auto-generated crate for the `directus_access` table.
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
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
/// Struct representing a row in the `directus_access` table.
# [diesel (belongs_to (emi_deprecated_models_directus_policies :: DirectusPolicy , foreign_key = policy))]
# [diesel (belongs_to (emi_deprecated_models_directus_roles :: DirectusRole , foreign_key = role))]
# [diesel (belongs_to (emi_deprecated_models_directus_users :: DirectusUser , foreign_key = user))]
# [diesel (table_name = directus_access)]
pub struct DirectusAccess {
    /// Field representing the `id` column in table `directus_access`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `role` column in table `directus_access`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    role: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `user` column in table `directus_access`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `policy` column in table `directus_access`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    policy: ::rosetta_uuid::Uuid,
    /// Field representing the `sort` column in table `directus_access`.
    sort: Option<i32>,
}
:: diesel_builders :: prelude :: fpk ! (directus_access :: policy -> :: emi_deprecated_models_directus_policies :: directus_policies);
:: diesel_builders :: prelude :: fpk ! (directus_access :: role -> :: emi_deprecated_models_directus_roles :: directus_roles);
:: diesel_builders :: prelude :: fpk ! (directus_access :: user -> :: emi_deprecated_models_directus_users :: directus_users);
