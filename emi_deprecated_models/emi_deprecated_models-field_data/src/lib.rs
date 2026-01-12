#![allow(non_snake_case)]
//! Auto-generated crate for the `Field_Data` table.
#[derive(
    Clone,
    Debug,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Field_Data` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Field_Data)]
pub struct FieldDatum {
    /// Field representing the `id` column in table `Field_Data`.
    id: i32,
    /// Field representing the `user_created` column in table `Field_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Field_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Field_Data`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Field_Data`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `collector_fullname` column in table
    /// `Field_Data`.
    collector_fullname: Option<String>,
    /// Field representing the `observation_subject` column in table
    /// `Field_Data`.
    observation_subject: Option<String>,
    /// Field representing the `inat_upload` column in table `Field_Data`.
    inat_upload: Option<i32>,
    /// Field representing the `is_wild` column in table `Field_Data`.
    is_wild: Option<i32>,
    /// Field representing the `taxon_name` column in table `Field_Data`.
    taxon_name: Option<String>,
    /// Field representing the `no_name_on_list` column in table `Field_Data`.
    no_name_on_list: Option<i32>,
    /// Field representing the `sample_id` column in table `Field_Data`.
    sample_id: String,
    /// Field representing the `picture_panel` column in table `Field_Data`.
    picture_panel: Option<String>,
    /// Field representing the `picture_general` column in table `Field_Data`.
    picture_general: Option<String>,
    /// Field representing the `picture_detail` column in table `Field_Data`.
    picture_detail: Option<String>,
    /// Field representing the `picture_cut` column in table `Field_Data`.
    picture_cut: Option<String>,
    /// Field representing the `picture_panel_label` column in table
    /// `Field_Data`.
    picture_panel_label: Option<String>,
    /// Field representing the `x_coord` column in table `Field_Data`.
    x_coord: Option<f32>,
    /// Field representing the `y_coord` column in table `Field_Data`.
    y_coord: Option<f32>,
    /// Field representing the `collector_orcid` column in table `Field_Data`.
    collector_orcid: Option<String>,
    /// Field representing the `collector_inat` column in table `Field_Data`.
    collector_inat: Option<String>,
    /// Field representing the `latitude` column in table `Field_Data`.
    latitude: Option<f32>,
    /// Field representing the `longitude` column in table `Field_Data`.
    longitude: Option<f32>,
    /// Field representing the `qfield_project` column in table `Field_Data`.
    qfield_project: String,
    /// Field representing the `picture_free` column in table `Field_Data`.
    picture_free: Option<String>,
    /// Field representing the `comment_eco` column in table `Field_Data`.
    comment_eco: Option<String>,
    /// Field representing the `weather` column in table `Field_Data`.
    weather: Option<String>,
    /// Field representing the `sample_name` column in table `Field_Data`.
    sample_name: Option<String>,
    /// Field representing the `name_proposition` column in table `Field_Data`.
    name_proposition: Option<String>,
    /// Field representing the `ipen` column in table `Field_Data`.
    ipen: Option<String>,
    /// Field representing the `match_name` column in table `Field_Data`.
    match_name: Option<String>,
    /// Field representing the `ott_id` column in table `Field_Data`.
    ott_id: Option<String>,
    /// Field representing the `rank` column in table `Field_Data`.
    rank: Option<String>,
    /// Field representing the `wikidataID` column in table `Field_Data`.
    wikidata_id: Option<String>,
    /// Field representing the `unknown` column in table `Field_Data`.
    unknown: Option<String>,
    /// Field representing the `comment_env` column in table `Field_Data`.
    comment_env: Option<String>,
    /// Field representing the `herbivory_percent` column in table `Field_Data`.
    herbivory_percent: Option<f32>,
    /// Field representing the `temperature_°C` column in table `Field_Data`.
    temperature_c: Option<f32>,
    /// Field representing the `geometry` column in table `Field_Data`.
    # [diesel (sql_type = :: postgis_diesel :: sql_types :: Geometry)]
    geometry: Option<postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>>,
    /// Field representing the `date` column in table `Field_Data`.
    date: Option<i64>,
    /// Field representing the `soil_type` column in table `Field_Data`.
    soil_type: Option<String>,
    /// Field representing the `catalogue_number` column in table `Field_Data`.
    catalogue_number: Option<String>,
    /// Field representing the `extracted_id` column in table `Field_Data`.
    extracted_id: Option<String>,
    /// Field representing the `project` column in table `Field_Data`.
    project: Option<String>,
    /// Field representing the `uuid_qfield` column in table `Field_Data`.
    uuid_qfield: Option<String>,
}
::diesel_builders::prelude::unique_index!(Field_Data::sample_id);
:: diesel_builders :: prelude :: fpk ! (Field_Data :: user_created -> :: emi_deprecated_models_directus_users :: directus_users);
:: diesel_builders :: prelude :: fpk ! (Field_Data :: user_updated -> :: emi_deprecated_models_directus_users :: directus_users);
