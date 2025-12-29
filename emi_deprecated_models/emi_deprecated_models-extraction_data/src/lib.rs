#![allow(non_snake_case)]
//! Auto-generated crate for the `Extraction_Data` table.
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
/// Struct representing a row in the `Extraction_Data` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Extraction_Data)]
pub struct ExtractionDatum {
    /// Field representing the `id` column in table `Extraction_Data`.
    id: i32,
    /// Field representing the `status` column in table `Extraction_Data`.
    #[table_model(default = "present")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `Extraction_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Extraction_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Extraction_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Extraction_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `dried_weight` column in table `Extraction_Data`.
    dried_weight: f32,
    /// Field representing the `dried_weight_unit` column in table
    /// `Extraction_Data`.
    dried_weight_unit: i32,
    /// Field representing the `extraction_method` column in table
    /// `Extraction_Data`.
    extraction_method: Option<i32>,
    /// Field representing the `batch` column in table `Extraction_Data`.
    batch: Option<i32>,
    /// Field representing the `solvent_volume` column in table
    /// `Extraction_Data`.
    solvent_volume: Option<f32>,
    /// Field representing the `solvent_volume_unit` column in table
    /// `Extraction_Data`.
    solvent_volume_unit: Option<i32>,
    /// Field representing the `uuid_extraction` column in table
    /// `Extraction_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_extraction: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `sample_container` column in table
    /// `Extraction_Data`.
    sample_container: i32,
    /// Field representing the `parent_container` column in table
    /// `Extraction_Data`.
    parent_container: Option<i32>,
    /// Field representing the `parent_sample_container` column in table
    /// `Extraction_Data`.
    parent_sample_container: i32,
    /// Field representing the `extraction_container` column in table
    /// `Extraction_Data`.
    extraction_container: Option<i32>,
    /// Field representing the `altemis_tube_id` column in table
    /// `Extraction_Data`.
    altemis_tube_id: Option<String>,
    /// Field representing the `altemis_rack_id` column in table
    /// `Extraction_Data`.
    altemis_rack_id: Option<String>,
}
::diesel_builders::prelude::unique_index!(Extraction_Data::sample_container);
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: batch) -> (:: emi_deprecated_models_batches :: Batches :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: dried_weight_unit) -> (:: emi_deprecated_models_si_units :: SI_Units :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: extraction_container) -> (:: emi_deprecated_models_container_models :: Container_Models :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: extraction_method) -> (:: emi_deprecated_models_extraction_methods :: Extraction_Methods :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: parent_container) -> (:: emi_deprecated_models_containers :: Containers :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: parent_sample_container) -> (:: emi_deprecated_models_containers :: Containers :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: sample_container) -> (:: emi_deprecated_models_containers :: Containers :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: solvent_volume_unit) -> (:: emi_deprecated_models_si_units :: SI_Units :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Extraction_Data :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
