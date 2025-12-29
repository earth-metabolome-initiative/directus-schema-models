//! Auto-generated crate for the `directus_files` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_files` table.
# [diesel (table_name = directus_files)]
pub struct DirectusFile {
    /// Field representing the `id` column in table `directus_files`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `storage` column in table `directus_files`.
    storage: String,
    /// Field representing the `filename_disk` column in table `directus_files`.
    filename_disk: Option<String>,
    /// Field representing the `filename_download` column in table
    /// `directus_files`.
    filename_download: String,
    /// Field representing the `title` column in table `directus_files`.
    title: Option<String>,
    /// Field representing the `type` column in table `directus_files`.
    r#type: Option<String>,
    /// Field representing the `folder` column in table `directus_files`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    folder: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `uploaded_by` column in table `directus_files`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uploaded_by: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `created_on` column in table `directus_files`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_on: ::rosetta_timestamp::TimestampUTC,
    /// Field representing the `modified_by` column in table `directus_files`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    modified_by: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `modified_on` column in table `directus_files`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    modified_on: ::rosetta_timestamp::TimestampUTC,
    /// Field representing the `charset` column in table `directus_files`.
    charset: Option<String>,
    /// Field representing the `filesize` column in table `directus_files`.
    filesize: Option<i64>,
    /// Field representing the `width` column in table `directus_files`.
    width: Option<i32>,
    /// Field representing the `height` column in table `directus_files`.
    height: Option<i32>,
    /// Field representing the `duration` column in table `directus_files`.
    duration: Option<i32>,
    /// Field representing the `embed` column in table `directus_files`.
    embed: Option<String>,
    /// Field representing the `description` column in table `directus_files`.
    description: Option<String>,
    /// Field representing the `location` column in table `directus_files`.
    location: Option<String>,
    /// Field representing the `tags` column in table `directus_files`.
    tags: Option<String>,
    /// Field representing the `metadata` column in table `directus_files`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    metadata: Option<::serde_json::Value>,
    /// Field representing the `focal_point_x` column in table `directus_files`.
    focal_point_x: Option<i32>,
    /// Field representing the `focal_point_y` column in table `directus_files`.
    focal_point_y: Option<i32>,
    /// Field representing the `tus_id` column in table `directus_files`.
    tus_id: Option<String>,
    /// Field representing the `tus_data` column in table `directus_files`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    tus_data: Option<::serde_json::Value>,
    /// Field representing the `uploaded_on` column in table `directus_files`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    uploaded_on: Option<::rosetta_timestamp::TimestampUTC>,
}
:: diesel_builders :: prelude :: fk ! ((directus_files :: folder) -> (:: emi_deprecated_models_directus_folders :: directus_folders :: id));
:: diesel_builders :: prelude :: fk ! ((directus_files :: modified_by) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((directus_files :: uploaded_by) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
