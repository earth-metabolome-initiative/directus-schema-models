#![allow(non_snake_case)]
//! Auto-generated crate for the `Instrument_Types` table.
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
/// Struct representing a row in the `Instrument_Types` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Instrument_Types)]
pub struct InstrumentType {
    /// Field representing the `id` column in table `Instrument_Types`.
    id: i32,
    /// Field representing the `status` column in table `Instrument_Types`.
    #[table_model(default = "active")]
    status: Option<String>,
    /// Field representing the `user_created` column in table
    /// `Instrument_Types`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table
    /// `Instrument_Types`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table
    /// `Instrument_Types`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table
    /// `Instrument_Types`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `instrument_type` column in table
    /// `Instrument_Types`.
    instrument_type: Option<String>,
}
:: diesel_builders :: prelude :: fk ! ((Instrument_Types :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Instrument_Types :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
