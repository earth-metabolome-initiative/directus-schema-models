#![allow(non_snake_case)]
//! Auto-generated crate for the `Rooms` table.
#[derive(
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Rooms` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Rooms)]
pub struct Room {
    /// Field representing the `id` column in table `Rooms`.
    id: i32,
    /// Field representing the `status` column in table `Rooms`.
    #[table_model(default = "active")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `Rooms`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Rooms`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Rooms`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Rooms`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `building` column in table `Rooms`.
    building: i32,
    /// Field representing the `room_name` column in table `Rooms`.
    room_name: String,
    /// Field representing the `comment` column in table `Rooms`.
    comment: String,
    /// Field representing the `address` column in table `Rooms`.
    address: i32,
    /// Field representing the `geolocation` column in table `Rooms`.
    # [diesel (sql_type = :: postgis_diesel :: sql_types :: Geometry)]
    geolocation: postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>,
    /// Field representing the `qr_code` column in table `Rooms`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    qr_code: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((Rooms :: address) -> (:: emi_deprecated_models_addresses :: Addresses :: id));
:: diesel_builders :: prelude :: fk ! ((Rooms :: building) -> (:: emi_deprecated_models_buildings :: Buildings :: id));
:: diesel_builders :: prelude :: fk ! ((Rooms :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Rooms :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
