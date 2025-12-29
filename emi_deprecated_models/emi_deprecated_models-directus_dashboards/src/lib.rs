//! Auto-generated crate for the `directus_dashboards` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_dashboards` table.
# [diesel (table_name = directus_dashboards)]
pub struct DirectusDashboard {
    /// Field representing the `id` column in table `directus_dashboards`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `directus_dashboards`.
    name: String,
    /// Field representing the `icon` column in table `directus_dashboards`.
    #[table_model(default = "dashboard")]
    icon: String,
    /// Field representing the `note` column in table `directus_dashboards`.
    note: Option<String>,
    /// Field representing the `date_created` column in table
    /// `directus_dashboards`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_created` column in table
    /// `directus_dashboards`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `color` column in table `directus_dashboards`.
    color: Option<String>,
}
:: diesel_builders :: prelude :: fk ! ((directus_dashboards :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
