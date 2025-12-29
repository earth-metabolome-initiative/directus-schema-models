#![allow(non_snake_case)]
//! Auto-generated crate for the `Instruments` table.
#[derive(
    Clone,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Instruments` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Instruments)]
pub struct Instrument {
    /// Field representing the `id` column in table `Instruments`.
    id: i32,
    /// Field representing the `status` column in table `Instruments`.
    status: String,
    /// Field representing the `user_created` column in table `Instruments`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Instruments`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Instruments`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Instruments`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `uuid_instrument` column in table `Instruments`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_instrument: ::rosetta_uuid::Uuid,
    /// Field representing the `instrument_id` column in table `Instruments`.
    instrument_id: String,
    /// Field representing the `instrument_model` column in table `Instruments`.
    instrument_model: i32,
    /// Field representing the `instrument_location` column in table
    /// `Instruments`.
    instrument_location: i32,
    /// Field representing the `grams` column in table `Instruments`.
    grams: Option<f32>,
}
::diesel_builders::prelude::unique_index!(Instruments::instrument_id);
:: diesel_builders :: prelude :: fk ! ((Instruments :: instrument_location) -> (:: emi_deprecated_models_rooms :: Rooms :: id));
:: diesel_builders :: prelude :: fk ! ((Instruments :: instrument_model) -> (:: emi_deprecated_models_instrument_models :: Instrument_Models :: id));
:: diesel_builders :: prelude :: fk ! ((Instruments :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Instruments :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
