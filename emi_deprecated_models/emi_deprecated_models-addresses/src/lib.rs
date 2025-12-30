#![allow(non_snake_case)]
//! Auto-generated crate for the `Addresses` table.
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Addresses` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Addresses)]
pub struct Address {
    /// Field representing the `id` column in table `Addresses`.
    id: i32,
    /// Field representing the `country` column in table `Addresses`.
    country: String,
    /// Field representing the `city` column in table `Addresses`.
    city: String,
    /// Field representing the `street` column in table `Addresses`.
    street: String,
    /// Field representing the `street_number` column in table `Addresses`.
    street_number: String,
    /// Field representing the `postal_code` column in table `Addresses`.
    postal_code: String,
    /// Field representing the `geolocation` column in table `Addresses`.
    # [diesel (sql_type = :: postgis_diesel :: sql_types :: Geometry)]
    geolocation: postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>,
    /// Field representing the `city_code` column in table `Addresses`.
    city_code: String,
}
