#![allow(non_snake_case)]
//! Auto-generated crate for the `Injection_Methods` table.
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
/// Struct representing a row in the `Injection_Methods` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Injection_Methods)]
pub struct InjectionMethod {
    /// Field representing the `id` column in table `Injection_Methods`.
    id: i32,
    /// Field representing the `status` column in table `Injection_Methods`.
    #[table_model(default = "active")]
    status: Option<String>,
    /// Field representing the `user_created` column in table
    /// `Injection_Methods`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table
    /// `Injection_Methods`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table
    /// `Injection_Methods`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table
    /// `Injection_Methods`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `method_name` column in table
    /// `Injection_Methods`.
    method_name: Option<String>,
    /// Field representing the `method_description` column in table
    /// `Injection_Methods`.
    method_description: Option<String>,
}
::diesel_builders::prelude::unique_index!(Injection_Methods::method_name);
:: diesel_builders :: prelude :: fk ! ((Injection_Methods :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Injection_Methods :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
