//! Auto-generated crate for the `directus_migrations` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_migrations` table.
#[diesel(primary_key(version))]
# [diesel (table_name = directus_migrations)]
pub struct DirectusMigration {
    /// Field representing the `version` column in table `directus_migrations`.
    version: String,
    /// Field representing the `name` column in table `directus_migrations`.
    name: String,
    /// Field representing the `timestamp` column in table
    /// `directus_migrations`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    timestamp: Option<::rosetta_timestamp::TimestampUTC>,
}
