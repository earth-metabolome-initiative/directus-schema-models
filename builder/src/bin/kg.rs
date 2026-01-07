//! Exports the directus schema as a knowledge graph.

//! Builder executable to generate the Directus database code.
use std::path::Path;

use builder::config::Config;
use diesel::{Connection, PgConnection};
use pg_diesel::database::{PgDieselDatabase, PgDieselDatabaseBuilder};
use sql2kg::prelude::*;
use time_requirements::{prelude::TimeTracker, task::Task};

/// Executable to generate the code for the Directus database.
pub fn main() {
    // Load configuration from file
    let Ok(config) = Config::try_from("config.toml") else {
        eprintln!("Failed to load configuration from 'config.toml'");
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

    let task = Task::new("KG Generation");
    db.write_kg_csvs(&mut conn, Path::new("../kg_data/directus")).expect("Failed to write KG CSVs");
    tracker.add_completed_task(task);

    // We print the report
    tracker.write(Path::new("KG_TIME_REQUIREMENTS.md")).unwrap();
}
