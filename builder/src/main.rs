//! Builder executable to generate the Directus database code.
use std::path::Path;
mod config;
use config::Config;

use diesel::{Connection, PgConnection};
use pg_diesel::database::{PgDieselDatabase, PgDieselDatabaseBuilder};
use sql_traits::traits::DatabaseLike;
use std::process::Command;
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

    let deny_listed_tables = [
        // Column excluded due to the extremely large number of columns (150)
        // which break `diesel`, which supports only up to 128 columns.
        "Curation_Data",
        // Table excluded due to containing only a single surrogate key column.
        "Test_Connection",
        // Tables excluded as they are just views used by postgis.
        "geography_columns",
        "geometry_columns"
    ].iter().map(|&name| {
        db.table(Some("public"), name).unwrap_or_else(|| {
            eprintln!("The required table '{}' does not exist in the database.", name);
            eprintln!(
                "Please ensure that the database is correctly set up and contains the necessary schema."
            );
            std::process::exit(1);
        })
    }).collect::<Vec<_>>();

    // Generate the code associated with the database
    let synql: SynQL<PgDieselDatabase> =
        SynQL::new_with_crate_base_path(&db, "../".as_ref(), "emi_deprecated_models".as_ref())
            .name("emi_deprecated_models")
            .deny_list(deny_listed_tables)
            .generate_workspace_toml()
            .generate_rustfmt()
            .into();

    tracker.extend(synql.generate().expect("Unable to generate workspace"));

    // Formats the generated code
    let task = Task::new("Code Formatting");
    Command::new("cargo")
        .arg("fmt")
        .current_dir("../")
        .status()
        .expect("Failed to format generated code");
    tracker.add_completed_task(task);

    // Formats the generated TOML using `taplo fmt`
    let task = Task::new("TOML Formatting");
    Command::new("taplo")
        .arg("fmt")
        .current_dir("../")
        .status()
        .expect("Failed to format generated TOML");
    tracker.add_completed_task(task);

    // We print the report
    Report::new(tracker)
        .write(
            Path::new("TIME_REQUIREMENTS.md"),
            Path::new("TIME_REQUIREMENTS.png"),
        )
        .unwrap();
}
