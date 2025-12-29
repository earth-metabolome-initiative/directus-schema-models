#![allow(non_snake_case)]
//! Auto-generated crate for the `Batches` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Batches` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Batches)]
pub struct Batch {
    /// Field representing the `id` column in table `Batches`.
    id: i32,
    /// Field representing the `status` column in table `Batches`.
    #[table_model(default = "ok")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `Batches`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Batches`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Batches`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Batches`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `uuid_batch` column in table `Batches`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_batch: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `batch_id` column in table `Batches`.
    batch_id: String,
    /// Field representing the `batch_type` column in table `Batches`.
    batch_type: Option<i32>,
    /// Field representing the `comments` column in table `Batches`.
    comments: Option<String>,
    /// Field representing the `old_id` column in table `Batches`.
    old_id: Option<String>,
    /// Field representing the `short_description` column in table `Batches`.
    short_description: String,
    /// Field representing the `description` column in table `Batches`.
    description: String,
}
::diesel_builders::prelude::unique_index!(Batches::batch_id);
::diesel_builders::prelude::unique_index!(Batches::old_id);
:: diesel_builders :: prelude :: fk ! ((Batches :: batch_type) -> (:: emi_deprecated_models_batch_types :: Batch_Types :: id));
:: diesel_builders :: prelude :: fk ! ((Batches :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Batches :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
