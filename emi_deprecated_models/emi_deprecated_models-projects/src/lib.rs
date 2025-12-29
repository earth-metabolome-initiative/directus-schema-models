#![allow(non_snake_case)]
//! Auto-generated crate for the `Projects` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Projects` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Projects)]
pub struct Project {
    /// Field representing the `id` column in table `Projects`.
    id: i32,
    /// Field representing the `status` column in table `Projects`.
    #[table_model(default = "active")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `Projects`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Projects`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Projects`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Projects`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `uuid_project` column in table `Projects`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_project: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `project_id` column in table `Projects`.
    project_id: String,
    /// Field representing the `project_description` column in table `Projects`.
    project_description: String,
    /// Field representing the `parent_project` column in table `Projects`.
    parent_project: Option<i32>,
    /// Field representing the `batch` column in table `Projects`.
    batch: i32,
}
::diesel_builders::prelude::unique_index!(Projects::project_id);
::diesel_builders::prelude::unique_index!(Projects::batch);
:: diesel_builders :: prelude :: fk ! ((Projects :: batch) -> (:: emi_deprecated_models_batches :: Batches :: id));
:: diesel_builders :: prelude :: fk ! ((Projects :: parent_project) -> (Projects :: id));
:: diesel_builders :: prelude :: fk ! ((Projects :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Projects :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
