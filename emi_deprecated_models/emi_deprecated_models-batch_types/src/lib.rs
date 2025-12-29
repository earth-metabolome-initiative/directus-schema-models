#![allow(non_snake_case)]
//! Auto-generated crate for the `Batch_Types` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Batch_Types` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Batch_Types)]
pub struct BatchType {
    /// Field representing the `id` column in table `Batch_Types`.
    id: i32,
    /// Field representing the `status` column in table `Batch_Types`.
    #[table_model(default = "in_use")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `Batch_Types`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Batch_Types`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Batch_Types`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Batch_Types`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `batch_type` column in table `Batch_Types`.
    batch_type: String,
    /// Field representing the `description` column in table `Batch_Types`.
    description: String,
}
:: diesel_builders :: prelude :: fk ! ((Batch_Types :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Batch_Types :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
