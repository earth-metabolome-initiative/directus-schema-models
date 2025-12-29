//! Auto-generated crate for the `directus_activity` table.
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
/// Struct representing a row in the `directus_activity` table.
#[table_model(surrogate_key)]
# [diesel (table_name = directus_activity)]
pub struct DirectusActivity {
    /// Field representing the `id` column in table `directus_activity`.
    id: i32,
    /// Field representing the `action` column in table `directus_activity`.
    action: String,
    /// Field representing the `user` column in table `directus_activity`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `timestamp` column in table `directus_activity`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    timestamp: ::rosetta_timestamp::TimestampUTC,
    /// Field representing the `ip` column in table `directus_activity`.
    ip: Option<String>,
    /// Field representing the `user_agent` column in table `directus_activity`.
    user_agent: Option<String>,
    /// Field representing the `collection` column in table `directus_activity`.
    collection: String,
    /// Field representing the `item` column in table `directus_activity`.
    item: String,
    /// Field representing the `origin` column in table `directus_activity`.
    origin: Option<String>,
}
