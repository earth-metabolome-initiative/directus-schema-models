#![allow(non_snake_case)]
//! Auto-generated crate for the `Buildings` table.
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
/// Struct representing a row in the `Buildings` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Buildings)]
pub struct Building {
    /// Field representing the `id` column in table `Buildings`.
    id: i32,
    /// Field representing the `status` column in table `Buildings`.
    status: String,
    /// Field representing the `user_created` column in table `Buildings`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Buildings`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Buildings`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Buildings`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `university` column in table `Buildings`.
    university: Option<i32>,
    /// Field representing the `building_name` column in table `Buildings`.
    building_name: Option<String>,
    /// Field representing the `address` column in table `Buildings`.
    address: Option<String>,
}
:: diesel_builders :: prelude :: fk ! ((Buildings :: university) -> (:: emi_deprecated_models_universities :: Universities :: id));
:: diesel_builders :: prelude :: fk ! ((Buildings :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Buildings :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
