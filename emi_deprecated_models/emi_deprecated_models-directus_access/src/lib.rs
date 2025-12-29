//! Auto-generated crate for the `directus_access` table.
#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_access` table.
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
:: diesel_builders :: prelude :: fk ! ((directus_access :: policy) -> (:: emi_deprecated_models_directus_policies :: directus_policies :: id));
:: diesel_builders :: prelude :: fk ! ((directus_access :: role) -> (:: emi_deprecated_models_directus_roles :: directus_roles :: id));
:: diesel_builders :: prelude :: fk ! ((directus_access :: user) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
