#![allow(non_snake_case)]
//! Auto-generated crate for the `Extraction_Methods` table.
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
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `Extraction_Methods` table.
#[table_model(surrogate_key)]
# [diesel (table_name = Extraction_Methods)]
pub struct ExtractionMethod {
    /// Field representing the `id` column in table `Extraction_Methods`.
    id: i32,
    /// Field representing the `status` column in table `Extraction_Methods`.
    status: String,
    /// Field representing the `user_created` column in table
    /// `Extraction_Methods`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_created: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_created` column in table
    /// `Extraction_Methods`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_created: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `user_updated` column in table
    /// `Extraction_Methods`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    user_updated: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `date_updated` column in table
    /// `Extraction_Methods`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `method_name` column in table
    /// `Extraction_Methods`.
    method_name: String,
    /// Field representing the `method_description` column in table
    /// `Extraction_Methods`.
    method_description: String,
}
:: diesel_builders :: prelude :: fpk ! (Extraction_Methods :: user_created -> :: emi_deprecated_models_directus_users :: directus_users);
:: diesel_builders :: prelude :: fpk ! (Extraction_Methods :: user_updated -> :: emi_deprecated_models_directus_users :: directus_users);
