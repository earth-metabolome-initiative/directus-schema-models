#![allow(non_snake_case)]
//! Auto-generated crate for the `Instrument_Models` table.
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
/// Struct representing a row in the `Instrument_Models` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Instrument_Models)]
pub struct InstrumentModel {
    /// Field representing the `id` column in table `Instrument_Models`.
    id: i32,
    /// Field representing the `status` column in table `Instrument_Models`.
    #[table_model(default = "present")]
    status: Option<String>,
    /// Field representing the `user_created` column in table
    /// `Instrument_Models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table
    /// `Instrument_Models`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table
    /// `Instrument_Models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table
    /// `Instrument_Models`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `instrument_type` column in table
    /// `Instrument_Models`.
    instrument_type: i32,
    /// Field representing the `instrument_model` column in table
    /// `Instrument_Models`.
    instrument_model: String,
    /// Field representing the `brand` column in table `Instrument_Models`.
    brand: i32,
    /// Field representing the `barcode` column in table `Instrument_Models`.
    barcode: Option<String>,
}
:: diesel_builders :: prelude :: fk ! ((Instrument_Models :: brand) -> (:: emi_deprecated_models_brands :: Brands :: id));
:: diesel_builders :: prelude :: fk ! ((Instrument_Models :: instrument_type) -> (:: emi_deprecated_models_instrument_types :: Instrument_Types :: id));
:: diesel_builders :: prelude :: fk ! ((Instrument_Models :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Instrument_Models :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
