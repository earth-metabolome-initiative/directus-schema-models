//! Builder executable to generate the Directus database code.
mod common;
mod config;
use std::process::Command;

use pg_diesel::database::PgDieselDatabase;
use sql_traits::traits::{DatabaseLike, TableLike};
use synql::prelude::*;
use time_requirements::task::Task;

/// Executable to generate the code for the Directus database.
pub fn main() {
    let (mut tracker, _conn, db) = common::setup();

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

    // We expect that none of the deny listed tables have other tables depending on
    // them.
    for table in &deny_listed_tables {
        if table.has_dependent_tables(&db) {
            eprintln!(
                "The deny listed table '{}' has dependent tables in the database.",
                table.table_name()
            );
            eprintln!(
                "Please remove dependencies on this table before proceeding with code generation."
            );
            std::process::exit(1);
        }
    }

    // Generate the code associated with the database
    let workspace_root = common::workspace_root();
    let synql: SynQL<PgDieselDatabase> = SynQL::new_with_crate_base_path(
        &db,
        workspace_root.as_path(),
        "emi_deprecated_models".as_ref(),
    )
    .name("emi_deprecated_models")
    .deny_list(deny_listed_tables)
    .sink_crate("emi_deprecated_models")
    .member(TomlDependency::new("builder").path("builder").unwrap())
    .generate_workspace_toml()
    .generate_rustfmt()
    .into();

    tracker.extend(synql.generate().expect("Unable to generate workspace"));

    // Formats the generated code
    let task = Task::new("Code Formatting");
    Command::new("cargo")
        .arg("fmt")
        .current_dir(common::workspace_root())
        .status()
        .expect("Failed to format generated code");
    tracker.add_completed_task(task);

    // Formats the generated TOML using `taplo fmt`
    let task = Task::new("TOML Formatting");
    Command::new("taplo")
        .arg("fmt")
        .current_dir(common::workspace_root())
        .status()
        .expect("Failed to format generated TOML");
    tracker.add_completed_task(task);

    // Resolves the `cargo clippy` code smells
    let task = Task::new("Clippy Linting");
    Command::new("cargo")
        .arg("clippy")
        .arg("--fix")
        .arg("--allow-dirty")
        .current_dir(common::workspace_root())
        .status()
        .expect("Failed to run cargo clippy on generated code");
    tracker.add_completed_task(task);

    // We print the report
    let report_path = common::workspace_root().join("builder/TIME_REQUIREMENTS.md");
    tracker.write(report_path.as_path()).unwrap();
}
