//! Auto-generated crate for the `directus_users` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `directus_users` table.
# [diesel (table_name = directus_users)]
pub struct DirectusUser {
    /// Field representing the `id` column in table `directus_users`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `first_name` column in table `directus_users`.
    first_name: Option<String>,
    /// Field representing the `last_name` column in table `directus_users`.
    last_name: Option<String>,
    /// Field representing the `email` column in table `directus_users`.
    email: Option<String>,
    /// Field representing the `password` column in table `directus_users`.
    password: Option<String>,
    /// Field representing the `location` column in table `directus_users`.
    location: Option<String>,
    /// Field representing the `title` column in table `directus_users`.
    title: Option<String>,
    /// Field representing the `description` column in table `directus_users`.
    description: Option<String>,
    /// Field representing the `tags` column in table `directus_users`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    tags: Option<::serde_json::Value>,
    /// Field representing the `avatar` column in table `directus_users`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    avatar: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `language` column in table `directus_users`.
    language: Option<String>,
    /// Field representing the `tfa_secret` column in table `directus_users`.
    tfa_secret: Option<String>,
    /// Field representing the `status` column in table `directus_users`.
    #[table_model(default = "active")]
    status: String,
    /// Field representing the `role` column in table `directus_users`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    role: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `token` column in table `directus_users`.
    token: Option<String>,
    /// Field representing the `last_access` column in table `directus_users`.
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    last_access: Option<::rosetta_timestamp::TimestampUTC>,
    /// Field representing the `last_page` column in table `directus_users`.
    last_page: Option<String>,
    /// Field representing the `provider` column in table `directus_users`.
    #[table_model(default = "default")]
    provider: String,
    /// Field representing the `external_identifier` column in table
    /// `directus_users`.
    external_identifier: Option<String>,
    /// Field representing the `auth_data` column in table `directus_users`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    auth_data: Option<::serde_json::Value>,
    /// Field representing the `email_notifications` column in table
    /// `directus_users`.
    #[table_model(default = true)]
    email_notifications: Option<bool>,
    /// Field representing the `appearance` column in table `directus_users`.
    appearance: Option<String>,
    /// Field representing the `theme_dark` column in table `directus_users`.
    theme_dark: Option<String>,
    /// Field representing the `theme_light` column in table `directus_users`.
    theme_light: Option<String>,
    /// Field representing the `theme_light_overrides` column in table
    /// `directus_users`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    theme_light_overrides: Option<::serde_json::Value>,
    /// Field representing the `theme_dark_overrides` column in table
    /// `directus_users`.
    # [diesel (sql_type = :: diesel :: sql_types :: Json)]
    theme_dark_overrides: Option<::serde_json::Value>,
}
::diesel_builders::prelude::unique_index!(directus_users::email);
::diesel_builders::prelude::unique_index!(directus_users::external_identifier);
::diesel_builders::prelude::unique_index!(directus_users::token);
:: diesel_builders :: prelude :: fk ! ((directus_users :: role) -> (:: emi_deprecated_models_directus_roles :: directus_roles :: id));
