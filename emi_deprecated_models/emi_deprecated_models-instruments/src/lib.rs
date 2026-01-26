#![allow(non_snake_case)]
//! Auto-generated crate for the `Instruments` table.
#[derive(
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Instruments` table.
# [diesel (belongs_to (emi_deprecated_models_rooms :: Room , foreign_key = instrument_location))]
# [diesel (belongs_to (emi_deprecated_models_instrument_models :: InstrumentModel , foreign_key = instrument_model))]
#[table_model(surrogate_key)]
# [table_model (foreign_key ((instrument_location ,) , (:: emi_deprecated_models_rooms :: Rooms :: id)))]
# [table_model (foreign_key ((instrument_model ,) , (:: emi_deprecated_models_instrument_models :: Instrument_Models :: id)))]
# [table_model (foreign_key ((user_created ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
# [table_model (foreign_key ((user_updated ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
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
