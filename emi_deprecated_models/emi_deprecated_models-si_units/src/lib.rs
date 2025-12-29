#![allow(non_snake_case)]
//! Auto-generated crate for the `SI_Units` table.
#[derive(
    Clone,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `SI_Units` table.
#[table_model(surrogate_key)]
# [diesel (table_name = SI_Units)]
pub struct SiUnit {
    /// Field representing the `id` column in table `SI_Units`.
    id: i32,
    /// Field representing the `status` column in table `SI_Units`.
    #[table_model(default = "active")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `SI_Units`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `SI_Units`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `SI_Units`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `SI_Units`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `unit_name` column in table `SI_Units`.
    unit_name: String,
    /// Field representing the `symbol` column in table `SI_Units`.
    symbol: String,
    /// Field representing the `base_unit` column in table `SI_Units`.
    base_unit: String,
    /// Field representing the `multiplication_factor` column in table
    /// `SI_Units`.
    multiplication_factor: f32,
}
::diesel_builders::prelude::unique_index!(SI_Units::unit_name);
:: diesel_builders :: prelude :: fk ! ((SI_Units :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((SI_Units :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
