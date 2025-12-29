//! Auto-generated crate for the `spatial_ref_sys` table.
#[derive(
    Clone,
    Default,
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
/// Struct representing a row in the `spatial_ref_sys` table.
# [table_model (error = :: validation_errors :: ValidationError)]
#[diesel(primary_key(srid))]
# [diesel (table_name = spatial_ref_sys)]
pub struct SpatialRefSy {
    /// Field representing the `srid` column in table `spatial_ref_sys`.
    srid: i32,
    /// Field representing the `auth_name` column in table `spatial_ref_sys`.
    #[infallible]
    auth_name: Option<String>,
    /// Field representing the `auth_srid` column in table `spatial_ref_sys`.
    #[infallible]
    auth_srid: Option<i32>,
    /// Field representing the `srtext` column in table `spatial_ref_sys`.
    #[infallible]
    srtext: Option<String>,
    /// Field representing the `proj4text` column in table `spatial_ref_sys`.
    #[infallible]
    proj4text: Option<String>,
}
impl ::diesel_builders::ValidateColumn<spatial_ref_sys::srid>
    for <spatial_ref_sys::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(srid: &i32) -> Result<(), Self::Error> {
        use diesel::Column;
        if srid <= &0i32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::spatial_ref_sys::srid::NAME,
                0f64,
            ));
        }
        if srid > &998999i32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::spatial_ref_sys::srid::NAME,
                998999f64,
            ));
        }
        Ok(())
    }
}
