#![allow(non_snake_case)]
//! Auto-generated crate for the `Container_Models` table.
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
/// Struct representing a row in the `Container_Models` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Container_Models)]
pub struct ContainerModel {
    /// Field representing the `id` column in table `Container_Models`.
    id: i32,
    /// Field representing the `status` column in table `Container_Models`.
    status: String,
    /// Field representing the `user_created` column in table
    /// `Container_Models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table
    /// `Container_Models`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table
    /// `Container_Models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table
    /// `Container_Models`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `container_type` column in table
    /// `Container_Models`.
    container_type: i32,
    /// Field representing the `volume` column in table `Container_Models`.
    volume: f32,
    /// Field representing the `volume_unit` column in table `Container_Models`.
    volume_unit: i32,
    /// Field representing the `brand` column in table `Container_Models`.
    brand: i32,
    /// Field representing the `is_sample_container` column in table
    /// `Container_Models`.
    is_sample_container: bool,
    /// Field representing the `columns` column in table `Container_Models`.
    #[table_model(sql_name = "columns")]
    __columns: i32,
    /// Field representing the `columns_numeric` column in table
    /// `Container_Models`.
    columns_numeric: bool,
    /// Field representing the `rows` column in table `Container_Models`.
    rows: i32,
    /// Field representing the `rows_numeric` column in table
    /// `Container_Models`.
    rows_numeric: bool,
}
:: diesel_builders :: prelude :: fk ! ((Container_Models :: brand) -> (:: emi_deprecated_models_brands :: Brands :: id));
:: diesel_builders :: prelude :: fk ! ((Container_Models :: container_type) -> (:: emi_deprecated_models_container_types :: Container_Types :: id));
:: diesel_builders :: prelude :: fk ! ((Container_Models :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Container_Models :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Container_Models :: volume_unit) -> (:: emi_deprecated_models_si_units :: SI_Units :: id));
