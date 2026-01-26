#![allow(non_snake_case)]
//! Auto-generated crate for the `MS_Data` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `MS_Data` table.
# [diesel (belongs_to (emi_deprecated_models_batches :: Batch , foreign_key = batch))]
# [diesel (belongs_to (emi_deprecated_models_injection_methods :: InjectionMethod , foreign_key = injection_method))]
# [diesel (belongs_to (emi_deprecated_models_si_units :: SiUnit , foreign_key = injection_volume_unit))]
# [diesel (belongs_to (emi_deprecated_models_instruments :: Instrument , foreign_key = instrument_used))]
# [diesel (belongs_to (emi_deprecated_models_containers :: Container , foreign_key = parent_sample_container))]
#[table_model(surrogate_key)]
# [table_model (foreign_key ((batch ,) , (:: emi_deprecated_models_batches :: Batches :: id)))]
# [table_model (foreign_key ((injection_method ,) , (:: emi_deprecated_models_injection_methods :: Injection_Methods :: id)))]
# [table_model (foreign_key ((injection_volume_unit ,) , (:: emi_deprecated_models_si_units :: SI_Units :: id)))]
# [table_model (foreign_key ((instrument_used ,) , (:: emi_deprecated_models_instruments :: Instruments :: id)))]
# [table_model (foreign_key ((parent_sample_container ,) , (:: emi_deprecated_models_containers :: Containers :: id)))]
# [table_model (foreign_key ((user_created ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
# [table_model (foreign_key ((user_updated ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
# [diesel (table_name = MS_Data)]
pub struct MsDatum {
    /// Field representing the `id` column in table `MS_Data`.
    id: i32,
    /// Field representing the `status` column in table `MS_Data`.
    #[table_model(default = "published")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `MS_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `MS_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `MS_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `MS_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `uuid_ms_file` column in table `MS_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_ms_file: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `status_comment` column in table `MS_Data`.
    status_comment: Option<String>,
    /// Field representing the `filename` column in table `MS_Data`.
    filename: String,
    /// Field representing the `injection_volume` column in table `MS_Data`.
    injection_volume: i32,
    /// Field representing the `injection_volume_unit` column in table
    /// `MS_Data`.
    injection_volume_unit: i32,
    /// Field representing the `injection_method` column in table `MS_Data`.
    injection_method: i32,
    /// Field representing the `instrument_used` column in table `MS_Data`.
    instrument_used: i32,
    /// Field representing the `batch` column in table `MS_Data`.
    batch: Option<i32>,
    /// Field representing the `parent_sample_container` column in table
    /// `MS_Data`.
    parent_sample_container: i32,
    /// Field representing the `converted` column in table `MS_Data`.
    converted: Option<bool>,
    /// Field representing the `processed` column in table `MS_Data`.
    processed: Option<bool>,
}
::diesel_builders::prelude::unique_index!(MS_Data::filename);
