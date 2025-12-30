//! Auto-generated crate for the `directus_comments` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_comments` table.
# [diesel (table_name = directus_comments)]
pub struct DirectusComment {
    /// Field representing the `id` column in table `directus_comments`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `collection` column in table `directus_comments`.
    collection: String,
    /// Field representing the `item` column in table `directus_comments`.
    item: String,
    /// Field representing the `comment` column in table `directus_comments`.
    comment: String,
    /// Field representing the `date_created` column in table
    /// `directus_comments`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `date_updated` column in table
    /// `directus_comments`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_created` column in table
    /// `directus_comments`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `user_updated` column in table
    /// `directus_comments`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
}
:: diesel_builders :: prelude :: fk ! ((directus_comments :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((directus_comments :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
