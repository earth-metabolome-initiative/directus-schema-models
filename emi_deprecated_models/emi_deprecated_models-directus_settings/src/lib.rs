//! Auto-generated crate for the `directus_settings` table.
#[derive(
    Clone,
    Debug,
    Hash,
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
/// Struct representing a row in the `directus_settings` table.
# [diesel (belongs_to (emi_deprecated_models_directus_roles :: DirectusRole , foreign_key = public_registration_role))]
# [diesel (belongs_to (emi_deprecated_models_directus_folders :: DirectusFolder , foreign_key = storage_default_folder))]
#[table_model(surrogate_key)]
# [table_model (foreign_key ((project_logo ,) , (:: emi_deprecated_models_directus_files :: directus_files :: id)))]
# [table_model (foreign_key ((public_background ,) , (:: emi_deprecated_models_directus_files :: directus_files :: id)))]
# [table_model (foreign_key ((public_favicon ,) , (:: emi_deprecated_models_directus_files :: directus_files :: id)))]
# [table_model (foreign_key ((public_foreground ,) , (:: emi_deprecated_models_directus_files :: directus_files :: id)))]
# [table_model (foreign_key ((public_registration_role ,) , (:: emi_deprecated_models_directus_roles :: directus_roles :: id)))]
# [table_model (foreign_key ((storage_default_folder ,) , (:: emi_deprecated_models_directus_folders :: directus_folders :: id)))]
# [diesel (table_name = directus_settings)]
pub struct DirectusSetting {
    /// Field representing the `id` column in table `directus_settings`.
    id: i32,
    /// Field representing the `project_name` column in table
    /// `directus_settings`.
    #[table_model(default = "Directus")]
    project_name: String,
    /// Field representing the `project_url` column in table
    /// `directus_settings`.
    project_url: Option<String>,
    /// Field representing the `project_color` column in table
    /// `directus_settings`.
    #[table_model(default = "#6644FF")]
    project_color: String,
    /// Field representing the `project_logo` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    project_logo: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `public_foreground` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    public_foreground: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `public_background` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    public_background: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `public_note` column in table
    /// `directus_settings`.
    public_note: Option<String>,
    /// Field representing the `auth_login_attempts` column in table
    /// `directus_settings`.
    #[table_model(default = 25i32)]
    auth_login_attempts: Option<i32>,
    /// Field representing the `auth_password_policy` column in table
    /// `directus_settings`.
    auth_password_policy: Option<String>,
    /// Field representing the `storage_asset_transform` column in table
    /// `directus_settings`.
    #[table_model(default = "all")]
    storage_asset_transform: Option<String>,
    /// Field representing the `storage_asset_presets` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    storage_asset_presets: Option<::serde_json::Value>,
    /// Field representing the `custom_css` column in table `directus_settings`.
    custom_css: Option<String>,
    /// Field representing the `storage_default_folder` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    storage_default_folder: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `basemaps` column in table `directus_settings`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    basemaps: Option<::serde_json::Value>,
    /// Field representing the `mapbox_key` column in table `directus_settings`.
    mapbox_key: Option<String>,
    /// Field representing the `module_bar` column in table `directus_settings`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    module_bar: Option<::serde_json::Value>,
    /// Field representing the `project_descriptor` column in table
    /// `directus_settings`.
    project_descriptor: Option<String>,
    /// Field representing the `default_language` column in table
    /// `directus_settings`.
    #[table_model(default = "en-US")]
    default_language: String,
    /// Field representing the `custom_aspect_ratios` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    custom_aspect_ratios: Option<::serde_json::Value>,
    /// Field representing the `public_favicon` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    public_favicon: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `default_appearance` column in table
    /// `directus_settings`.
    #[table_model(default = "auto")]
    default_appearance: String,
    /// Field representing the `default_theme_light` column in table
    /// `directus_settings`.
    default_theme_light: Option<String>,
    /// Field representing the `theme_light_overrides` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    theme_light_overrides: Option<::serde_json::Value>,
    /// Field representing the `default_theme_dark` column in table
    /// `directus_settings`.
    default_theme_dark: Option<String>,
    /// Field representing the `theme_dark_overrides` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    theme_dark_overrides: Option<::serde_json::Value>,
    /// Field representing the `report_error_url` column in table
    /// `directus_settings`.
    report_error_url: Option<String>,
    /// Field representing the `report_bug_url` column in table
    /// `directus_settings`.
    report_bug_url: Option<String>,
    /// Field representing the `report_feature_url` column in table
    /// `directus_settings`.
    report_feature_url: Option<String>,
    /// Field representing the `public_registration` column in table
    /// `directus_settings`.
    #[table_model(default = false)]
    public_registration: bool,
    /// Field representing the `public_registration_verify_email` column in
    /// table `directus_settings`.
    #[table_model(default = true)]
    public_registration_verify_email: bool,
    /// Field representing the `public_registration_role` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    public_registration_role: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `public_registration_email_filter` column in
    /// table `directus_settings`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    public_registration_email_filter: Option<::serde_json::Value>,
    /// Field representing the `visual_editor_urls` column in table
    /// `directus_settings`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    visual_editor_urls: Option<::serde_json::Value>,
}
