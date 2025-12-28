//! Builder executable to generate the Directus database code.
use std::path::Path;
mod config;
use config::Config;

use diesel::{Connection, PgConnection};
use pg_diesel::database::{PgDieselDatabase, PgDieselDatabaseBuilder};
use sql_traits::prelude::DatabaseLike;
use synql::prelude::*;
use time_requirements::{prelude::TimeTracker, report::Report, task::Task};

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

    // We write to the target directory the generated structs
    let Some(curation_data) = db.table(Some("public"), "Curation_Data") else {
        eprintln!("The table 'Curation_Data' was not found in the database.");
        eprintln!(
            "Please ensure the database is correctly set up and contains the required tables."
        );
        std::process::exit(1);
    };

    // Generate the code associated with the database
    let synql: SynQL<PgDieselDatabase> = SynQL::new(&db, "../".as_ref())
        .name("directus_schema_models")
        .deny(curation_data)
        .generate_workspace_toml()
        .generate_rustfmt()
        .into();

    tracker.extend(synql.generate().expect("Unable to generate workspace"));

    // We print the report
    Report::new(tracker)
        .write(
            Path::new("TIME_REQUIREMENTS.md"),
            Path::new("TIME_REQUIREMENTS.png"),
        )
        .unwrap();
}
