//! Common setup code for the builder executables.
use crate::config::Config;
use diesel::{Connection, PgConnection};
use pg_diesel::database::{PgDieselDatabase, PgDieselDatabaseBuilder};
use std::path::PathBuf;
use time_requirements::{prelude::TimeTracker, task::Task};

/// Returns the path to the workspace root.
pub fn workspace_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap();
    if cwd.ends_with("builder") {
        cwd.parent().unwrap().to_path_buf()
    } else {
        cwd
    }
}

/// Sets up the database connection and introspection, returning the time tracker,
/// the database connection, and the introspected database.
pub fn setup() -> (TimeTracker, PgConnection, PgDieselDatabase) {
    // Determine the path to config.toml based on workspace root
    let config_path = workspace_root().join("builder").join("config.toml");

    // Load configuration from file
    let Ok(config) = Config::try_from(config_path.to_str().unwrap()) else {
        eprintln!("Failed to load configuration from '{}'", config_path.display());
        eprintln!("Please ensure the file exists and is properly formatted.");
        eprintln!("Refer to 'config.example.toml' for the correct format.");
        std::process::exit(1);
    };

    let mut tracker = TimeTracker::new("Directus Schema Generation");

    let task = Task::new("DB Connection");
    let mut conn =
        PgConnection::establish(&config.to_string()).expect("Failed to connect to database");
    tracker.add_completed_task(task);

    let task = Task::new("DB Introspection");
    let db: PgDieselDatabase = PgDieselDatabaseBuilder::default()
        .connection(&mut conn)
        .schema("public")
        .catalog(config.database_name())
        .try_into()
        .expect("Failed to build database");
    tracker.add_completed_task(task);

    (tracker, conn, db)
}