#![allow(non_snake_case)]
//! Auto-generated crate for the `Aliquoting_Data` table.
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
/// Struct representing a row in the `Aliquoting_Data` table.
# [diesel (belongs_to (emi_deprecated_models_si_units :: SiUnit , foreign_key = aliquot_volume_unit))]
#[table_model(surrogate_key)]
# [table_model (foreign_key ((aliquot_volume_unit ,) , (:: emi_deprecated_models_si_units :: SI_Units :: id)))]
# [table_model (foreign_key ((parent_container ,) , (:: emi_deprecated_models_containers :: Containers :: id)))]
# [table_model (foreign_key ((parent_sample_container ,) , (:: emi_deprecated_models_containers :: Containers :: id)))]
# [table_model (foreign_key ((sample_container ,) , (:: emi_deprecated_models_containers :: Containers :: id)))]
# [table_model (foreign_key ((user_created ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
# [table_model (foreign_key ((user_updated ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
# [diesel (table_name = Aliquoting_Data)]
pub struct AliquotingDatum {
    /// Field representing the `id` column in table `Aliquoting_Data`.
    id: i32,
    /// Field representing the `status` column in table `Aliquoting_Data`.
    #[table_model(default = "draft")]
    status: String,
    /// Field representing the `user_created` column in table `Aliquoting_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Aliquoting_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Aliquoting_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Aliquoting_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `sample_container` column in table
    /// `Aliquoting_Data`.
    sample_container: i32,
    /// Field representing the `uuid_aliquot` column in table `Aliquoting_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_aliquot: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `aliquot_volume` column in table
    /// `Aliquoting_Data`.
    aliquot_volume: f32,
    /// Field representing the `aliquot_volume_unit` column in table
    /// `Aliquoting_Data`.
    aliquot_volume_unit: i32,
    /// Field representing the `parent_container` column in table
    /// `Aliquoting_Data`.
    parent_container: i32,
    /// Field representing the `parent_sample_container` column in table
    /// `Aliquoting_Data`.
    parent_sample_container: i32,
}
::diesel_builders::prelude::unique_index!(Aliquoting_Data::sample_container);
