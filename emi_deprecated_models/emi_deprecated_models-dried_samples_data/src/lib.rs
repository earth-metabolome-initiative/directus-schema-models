#![allow(non_snake_case)]
//! Auto-generated crate for the `Dried_Samples_Data` table.
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
/// Struct representing a row in the `Dried_Samples_Data` table.
# [diesel (belongs_to (emi_deprecated_models_batches :: Batch , foreign_key = batch))]
# [diesel (belongs_to (emi_deprecated_models_field_data :: FieldDatum , foreign_key = field_data))]
#[table_model(surrogate_key)]
# [diesel (table_name = Dried_Samples_Data)]
pub struct DriedSamplesDatum {
    /// Field representing the `id` column in table `Dried_Samples_Data`.
    id: i32,
    /// Field representing the `status` column in table `Dried_Samples_Data`.
    status: String,
    /// Field representing the `user_created` column in table
    /// `Dried_Samples_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table
    /// `Dried_Samples_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table
    /// `Dried_Samples_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table
    /// `Dried_Samples_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `uuid_dried_sample` column in table
    /// `Dried_Samples_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_dried_sample: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `sample_container` column in table
    /// `Dried_Samples_Data`.
    sample_container: i32,
    /// Field representing the `parent_container` column in table
    /// `Dried_Samples_Data`.
    parent_container: Option<i32>,
    /// Field representing the `batch` column in table `Dried_Samples_Data`.
    batch: Option<i32>,
    /// Field representing the `field_data` column in table
    /// `Dried_Samples_Data`.
    field_data: Option<i32>,
}
::diesel_builders::prelude::unique_index!(Dried_Samples_Data::sample_container);
:: diesel_builders :: prelude :: fpk ! (Dried_Samples_Data :: batch -> :: emi_deprecated_models_batches :: Batches);
:: diesel_builders :: prelude :: fpk ! (Dried_Samples_Data :: field_data -> :: emi_deprecated_models_field_data :: Field_Data);
:: diesel_builders :: prelude :: fpk ! (Dried_Samples_Data :: parent_container -> :: emi_deprecated_models_containers :: Containers);
:: diesel_builders :: prelude :: fpk ! (Dried_Samples_Data :: sample_container -> :: emi_deprecated_models_containers :: Containers);
:: diesel_builders :: prelude :: fpk ! (Dried_Samples_Data :: user_created -> :: emi_deprecated_models_directus_users :: directus_users);
:: diesel_builders :: prelude :: fpk ! (Dried_Samples_Data :: user_updated -> :: emi_deprecated_models_directus_users :: directus_users);
