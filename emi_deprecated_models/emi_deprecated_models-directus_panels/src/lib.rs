//! Auto-generated crate for the `directus_panels` table.
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
/// Struct representing a row in the `directus_panels` table.
# [diesel (belongs_to (emi_deprecated_models_directus_dashboards :: DirectusDashboard , foreign_key = dashboard))]
# [diesel (belongs_to (emi_deprecated_models_directus_users :: DirectusUser , foreign_key = user_created))]
# [diesel (table_name = directus_panels)]
pub struct DirectusPanel {
    /// Field representing the `id` column in table `directus_panels`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `dashboard` column in table `directus_panels`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    dashboard: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `directus_panels`.
    name: Option<String>,
    /// Field representing the `icon` column in table `directus_panels`.
    icon: Option<String>,
    /// Field representing the `color` column in table `directus_panels`.
    color: Option<String>,
    /// Field representing the `show_header` column in table `directus_panels`.
    #[table_model(default = false)]
    show_header: bool,
    /// Field representing the `note` column in table `directus_panels`.
    note: Option<String>,
    /// Field representing the `type` column in table `directus_panels`.
    r#type: String,
    /// Field representing the `position_x` column in table `directus_panels`.
    position_x: i32,
    /// Field representing the `position_y` column in table `directus_panels`.
    position_y: i32,
    /// Field representing the `width` column in table `directus_panels`.
    width: i32,
    /// Field representing the `height` column in table `directus_panels`.
    height: i32,
    /// Field representing the `options` column in table `directus_panels`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    options: Option<::serde_json::Value>,
    /// Field representing the `date_created` column in table `directus_panels`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_created` column in table `directus_panels`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
}
:: diesel_builders :: prelude :: fpk ! (directus_panels :: dashboard -> :: emi_deprecated_models_directus_dashboards :: directus_dashboards);
:: diesel_builders :: prelude :: fpk ! (directus_panels :: user_created -> :: emi_deprecated_models_directus_users :: directus_users);
