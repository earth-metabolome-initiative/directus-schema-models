//! Configuration module for database connection.
use std::{fmt::Display, path::Path};

use serde::Deserialize;

/// Configuration for database connection.
///
/// This struct is loaded from `config.toml` in the repository root.
/// The config file is gitignored to keep credentials private.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Name of the database to connect to
    database_name: String,
    /// Password for database authentication
    database_password: String,
    /// Username for database authentication
    database_user: String,
    /// Port number where the database server is listening
    database_port: u16,
    /// Hostname or IP address of the database server
    hostname: String,
}

impl Config {
	/// Get the name of the database.
	pub fn database_name(&self) -> &str {
		&self.database_name
	}
}

impl TryFrom<&Path> for Config {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let config_content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}

impl TryFrom<&str> for Config {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::try_from(Path::new(path))
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
			f,
			"postgres://{}:{}@{}:{}/{}",
			self.database_user,
			self.database_password,
			self.hostname,
			self.database_port,
			self.database_name
		)
    }
}
