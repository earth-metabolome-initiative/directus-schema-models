//! Auto-generated crate for the `directus_notifications` table.
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
/// Struct representing a row in the `directus_notifications` table.
#[table_model(surrogate_key)]
# [diesel (table_name = directus_notifications)]
pub struct DirectusNotification {
    /// Field representing the `id` column in table `directus_notifications`.
    id: i32,
    /// Field representing the `timestamp` column in table
    /// `directus_notifications`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    timestamp: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `status` column in table
    /// `directus_notifications`.
    #[table_model(default = "inbox")]
    status: Option<String>,
    /// Field representing the `recipient` column in table
    /// `directus_notifications`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    recipient: ::rosetta_uuid::Uuid,
    /// Field representing the `sender` column in table
    /// `directus_notifications`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sender: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `subject` column in table
    /// `directus_notifications`.
    subject: String,
    /// Field representing the `message` column in table
    /// `directus_notifications`.
    message: Option<String>,
    /// Field representing the `collection` column in table
    /// `directus_notifications`.
    collection: Option<String>,
    /// Field representing the `item` column in table `directus_notifications`.
    item: Option<String>,
}
:: diesel_builders :: prelude :: fk ! ((directus_notifications :: recipient) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((directus_notifications :: sender) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
