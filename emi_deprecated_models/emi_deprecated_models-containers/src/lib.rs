#![allow(non_snake_case)]
//! Auto-generated crate for the `Containers` table.
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
/// Struct representing a row in the `Containers` table.
# [diesel (belongs_to (emi_deprecated_models_container_models :: ContainerModel , foreign_key = container_model))]
# [diesel (belongs_to (emi_deprecated_models_universities :: University , foreign_key = location))]
#[table_model(surrogate_key)]
# [diesel (table_name = Containers)]
pub struct Container {
    /// Field representing the `id` column in table `Containers`.
    id: i32,
    /// Field representing the `status` column in table `Containers`.
    #[table_model(default = "present")]
    status: Option<String>,
    /// Field representing the `user_created` column in table `Containers`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table `Containers`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table `Containers`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table `Containers`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `used` column in table `Containers`.
    used: bool,
    /// Field representing the `reserved` column in table `Containers`.
    reserved: bool,
    /// Field representing the `uuid_container` column in table `Containers`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    uuid_container: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `container_id` column in table `Containers`.
    container_id: String,
    /// Field representing the `container_model` column in table `Containers`.
    container_model: Option<i32>,
    /// Field representing the `is_finite` column in table `Containers`.
    is_finite: Option<bool>,
    /// Field representing the `columns` column in table `Containers`.
    #[table_model(sql_name = "columns")]
    __columns: Option<i32>,
    /// Field representing the `rows` column in table `Containers`.
    rows: Option<i32>,
    /// Field representing the `rows_numeric` column in table `Containers`.
    rows_numeric: Option<bool>,
    /// Field representing the `columns_numeric` column in table `Containers`.
    columns_numeric: Option<bool>,
    /// Field representing the `location` column in table `Containers`.
    location: Option<i32>,
    /// Field representing the `old_id` column in table `Containers`.
    old_id: Option<String>,
    /// Field representing the `parent_container` column in table `Containers`.
    parent_container: Option<i32>,
}
::diesel_builders::prelude::unique_index!(Containers::container_id);
::diesel_builders::prelude::unique_index!(Containers::old_id);
:: diesel_builders :: prelude :: fpk ! (Containers :: container_model -> :: emi_deprecated_models_container_models :: Container_Models);
:: diesel_builders :: prelude :: fpk ! (Containers :: location -> :: emi_deprecated_models_universities :: Universities);
:: diesel_builders :: prelude :: fpk ! (Containers :: parent_container -> Containers);
:: diesel_builders :: prelude :: fpk ! (Containers :: user_created -> :: emi_deprecated_models_directus_users :: directus_users);
:: diesel_builders :: prelude :: fpk ! (Containers :: user_updated -> :: emi_deprecated_models_directus_users :: directus_users);
