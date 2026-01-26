//! Auto-generated crate for the `directus_versions` table.
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
/// Struct representing a row in the `directus_versions` table.
# [diesel (belongs_to (emi_deprecated_models_directus_collections :: DirectusCollection , foreign_key = collection))]
# [table_model (foreign_key ((collection ,) , (:: emi_deprecated_models_directus_collections :: directus_collections :: collection)))]
# [table_model (foreign_key ((user_created ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
# [table_model (foreign_key ((user_updated ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
# [diesel (table_name = directus_versions)]
pub struct DirectusVersion {
    /// Field representing the `id` column in table `directus_versions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `key` column in table `directus_versions`.
    key: String,
    /// Field representing the `name` column in table `directus_versions`.
    name: Option<String>,
    /// Field representing the `collection` column in table `directus_versions`.
    collection: String,
    /// Field representing the `item` column in table `directus_versions`.
    item: String,
    /// Field representing the `hash` column in table `directus_versions`.
    hash: Option<String>,
    /// Field representing the `date_created` column in table
    /// `directus_versions`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `date_updated` column in table
    /// `directus_versions`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_created` column in table
    /// `directus_versions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `user_updated` column in table
    /// `directus_versions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `delta` column in table `directus_versions`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    delta: Option<::serde_json::Value>,
}
