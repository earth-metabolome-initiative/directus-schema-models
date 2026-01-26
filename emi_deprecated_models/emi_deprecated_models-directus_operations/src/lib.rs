//! Auto-generated crate for the `directus_operations` table.
#[derive(
    Clone,
    Debug,
    Hash,
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
/// Struct representing a row in the `directus_operations` table.
# [diesel (belongs_to (emi_deprecated_models_directus_flows :: DirectusFlow , foreign_key = flow))]
# [diesel (belongs_to (emi_deprecated_models_directus_users :: DirectusUser , foreign_key = user_created))]
# [table_model (foreign_key ((flow ,) , (:: emi_deprecated_models_directus_flows :: directus_flows :: id)))]
# [table_model (foreign_key ((reject ,) , (directus_operations :: id)))]
# [table_model (foreign_key ((resolve ,) , (directus_operations :: id)))]
# [table_model (foreign_key ((user_created ,) , (:: emi_deprecated_models_directus_users :: directus_users :: id)))]
# [diesel (table_name = directus_operations)]
pub struct DirectusOperation {
    /// Field representing the `id` column in table `directus_operations`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `directus_operations`.
    name: Option<String>,
    /// Field representing the `key` column in table `directus_operations`.
    key: String,
    /// Field representing the `type` column in table `directus_operations`.
    r#type: String,
    /// Field representing the `position_x` column in table
    /// `directus_operations`.
    position_x: i32,
    /// Field representing the `position_y` column in table
    /// `directus_operations`.
    position_y: i32,
    /// Field representing the `options` column in table `directus_operations`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    options: Option<::serde_json::Value>,
    /// Field representing the `resolve` column in table `directus_operations`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    resolve: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `reject` column in table `directus_operations`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    reject: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `flow` column in table `directus_operations`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    flow: ::rosetta_uuid::Uuid,
    /// Field representing the `date_created` column in table
    /// `directus_operations`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_created` column in table
    /// `directus_operations`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
}
::diesel_builders::prelude::unique_index!(directus_operations::reject);
::diesel_builders::prelude::unique_index!(directus_operations::resolve);
