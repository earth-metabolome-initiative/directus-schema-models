//! Auto-generated crate for the `directus_shares` table.
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
/// Struct representing a row in the `directus_shares` table.
# [diesel (belongs_to (emi_deprecated_models_directus_collections :: DirectusCollection , foreign_key = collection))]
# [diesel (belongs_to (emi_deprecated_models_directus_roles :: DirectusRole , foreign_key = role))]
# [diesel (belongs_to (emi_deprecated_models_directus_users :: DirectusUser , foreign_key = user_created))]
# [diesel (table_name = directus_shares)]
pub struct DirectusShare {
    /// Field representing the `id` column in table `directus_shares`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `directus_shares`.
    name: Option<String>,
    /// Field representing the `collection` column in table `directus_shares`.
    collection: String,
    /// Field representing the `item` column in table `directus_shares`.
    item: String,
    /// Field representing the `role` column in table `directus_shares`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    role: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `password` column in table `directus_shares`.
    password: Option<String>,
    /// Field representing the `user_created` column in table `directus_shares`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `directus_shares`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `date_start` column in table `directus_shares`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_start: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `date_end` column in table `directus_shares`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_end: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `times_used` column in table `directus_shares`.
    #[table_model(default = 0i32)]
    times_used: Option<i32>,
    /// Field representing the `max_uses` column in table `directus_shares`.
    max_uses: Option<i32>,
}
:: diesel_builders :: prelude :: fpk ! (directus_shares :: collection -> :: emi_deprecated_models_directus_collections :: directus_collections);
:: diesel_builders :: prelude :: fpk ! (directus_shares :: role -> :: emi_deprecated_models_directus_roles :: directus_roles);
:: diesel_builders :: prelude :: fpk ! (directus_shares :: user_created -> :: emi_deprecated_models_directus_users :: directus_users);
