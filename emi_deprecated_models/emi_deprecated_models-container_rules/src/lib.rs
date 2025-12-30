#![allow(non_snake_case)]
//! Auto-generated crate for the `Container_Rules` table.
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
/// Struct representing a row in the `Container_Rules` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Container_Rules)]
pub struct ContainerRule {
    /// Field representing the `id` column in table `Container_Rules`.
    id: i32,
    /// Field representing the `status` column in table `Container_Rules`.
    #[table_model(default = "in_use")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `Container_Rules`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Container_Rules`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Container_Rules`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Container_Rules`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `child_container` column in table
    /// `Container_Rules`.
    child_container: i32,
    /// Field representing the `parent_container` column in table
    /// `Container_Rules`.
    parent_container: i32,
    /// Field representing the `rule_name` column in table `Container_Rules`.
    rule_name: String,
}
:: diesel_builders :: prelude :: fk ! ((Container_Rules :: child_container) -> (:: emi_deprecated_models_container_models :: Container_Models :: id));
:: diesel_builders :: prelude :: fk ! ((Container_Rules :: parent_container) -> (:: emi_deprecated_models_container_models :: Container_Models :: id));
:: diesel_builders :: prelude :: fk ! ((Container_Rules :: user_created) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
:: diesel_builders :: prelude :: fk ! ((Container_Rules :: user_updated) -> (:: emi_deprecated_models_directus_users :: directus_users :: id));
