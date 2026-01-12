//! Auto-generated crate for the `directus_sessions` table.
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
/// Struct representing a row in the `directus_sessions` table.
# [diesel (belongs_to (emi_deprecated_models_directus_shares :: DirectusShare , foreign_key = share))]
# [diesel (belongs_to (emi_deprecated_models_directus_users :: DirectusUser , foreign_key = user))]
#[diesel(primary_key(token))]
# [diesel (table_name = directus_sessions)]
pub struct DirectusSession {
    /// Field representing the `token` column in table `directus_sessions`.
    token: String,
    /// Field representing the `user` column in table `directus_sessions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `expires` column in table `directus_sessions`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    expires: ::rosetta_timestamp::TimestampUTC,
    /// Field representing the `ip` column in table `directus_sessions`.
    ip: Option<String>,
    /// Field representing the `user_agent` column in table `directus_sessions`.
    user_agent: Option<String>,
    /// Field representing the `share` column in table `directus_sessions`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    share: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `origin` column in table `directus_sessions`.
    origin: Option<String>,
    /// Field representing the `next_token` column in table `directus_sessions`.
    next_token: Option<String>,
}
:: diesel_builders :: prelude :: fpk ! (directus_sessions :: share -> :: emi_deprecated_models_directus_shares :: directus_shares);
:: diesel_builders :: prelude :: fpk ! (directus_sessions :: user -> :: emi_deprecated_models_directus_users :: directus_users);
