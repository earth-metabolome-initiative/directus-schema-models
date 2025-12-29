//! Auto-generated crate for the `directus_collections` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_collections` table.
#[diesel(primary_key(collection))]
# [diesel (table_name = directus_collections)]
pub struct DirectusCollection {
    /// Field representing the `collection` column in table
    /// `directus_collections`.
    collection: String,
    /// Field representing the `icon` column in table `directus_collections`.
    icon: Option<String>,
    /// Field representing the `note` column in table `directus_collections`.
    note: Option<String>,
    /// Field representing the `display_template` column in table
    /// `directus_collections`.
    display_template: Option<String>,
    /// Field representing the `hidden` column in table `directus_collections`.
    #[table_model(default = false)]
    hidden: bool,
    /// Field representing the `singleton` column in table
    /// `directus_collections`.
    #[table_model(default = false)]
    singleton: bool,
    /// Field representing the `translations` column in table
    /// `directus_collections`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    translations: Option<::serde_json::Value>,
    /// Field representing the `archive_field` column in table
    /// `directus_collections`.
    archive_field: Option<String>,
    /// Field representing the `archive_app_filter` column in table
    /// `directus_collections`.
    #[table_model(default = true)]
    archive_app_filter: bool,
    /// Field representing the `archive_value` column in table
    /// `directus_collections`.
    archive_value: Option<String>,
    /// Field representing the `unarchive_value` column in table
    /// `directus_collections`.
    unarchive_value: Option<String>,
    /// Field representing the `sort_field` column in table
    /// `directus_collections`.
    sort_field: Option<String>,
    /// Field representing the `accountability` column in table
    /// `directus_collections`.
    #[table_model(default = "all")]
    accountability: Option<String>,
    /// Field representing the `color` column in table `directus_collections`.
    color: Option<String>,
    /// Field representing the `item_duplication_fields` column in table
    /// `directus_collections`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    item_duplication_fields: Option<::serde_json::Value>,
    /// Field representing the `sort` column in table `directus_collections`.
    sort: Option<i32>,
    /// Field representing the `group` column in table `directus_collections`.
    group: Option<String>,
    /// Field representing the `collapse` column in table
    /// `directus_collections`.
    #[table_model(default = "open")]
    collapse: String,
    /// Field representing the `preview_url` column in table
    /// `directus_collections`.
    preview_url: Option<String>,
    /// Field representing the `versioning` column in table
    /// `directus_collections`.
    #[table_model(default = false)]
    versioning: bool,
}
:: diesel_builders :: prelude :: fk ! ((directus_collections :: group) -> (directus_collections :: collection));
