#![allow(non_snake_case)]
//! Auto-generated crate for the `Brands` table.
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
/// Struct representing a row in the `Brands` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Brands)]
pub struct Brand {
    /// Field representing the `id` column in table `Brands`.
    id: i32,
    /// Field representing the `status` column in table `Brands`.
    status: String,
    /// Field representing the `user_created` column in table `Brands`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Brands`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Brands`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Brands`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `brand` column in table `Brands`.
    brand: String,
}
::diesel_builders::prelude::unique_index!(Brands::brand);
:: diesel_builders :: prelude :: fk ! ((Brands :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Brands :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
