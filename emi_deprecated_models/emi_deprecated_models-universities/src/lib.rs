#![allow(non_snake_case)]
//! Auto-generated crate for the `Universities` table.
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
/// Struct representing a row in the `Universities` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Universities)]
pub struct University {
    /// Field representing the `id` column in table `Universities`.
    id: i32,
    /// Field representing the `status` column in table `Universities`.
    #[table_model(default = "active")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `Universities`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Universities`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Universities`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Universities`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `uuid_university` column in table `Universities`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_university: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `university_name` column in table `Universities`.
    university_name: String,
    /// Field representing the `country` column in table `Universities`.
    country: String,
    /// Field representing the `alpha_two` column in table `Universities`.
    alpha_two: String,
    /// Field representing the `web_pages` column in table `Universities`.
    web_pages: String,
    /// Field representing the `state` column in table `Universities`.
    state: String,
    /// Field representing the `domains` column in table `Universities`.
    domains: String,
}
::diesel_builders::prelude::unique_index!(Universities::university_name);
:: diesel_builders :: prelude :: fk ! ((Universities :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Universities :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
