//! Exports the directus schema as a knowledge graph.

//! Builder executable to generate the Directus database code.

use builder::common;
use sql2kg::prelude::*;
use time_requirements::task::Task;

/// Executable to generate the code for the Directus database.
pub fn main() {
    let (mut tracker, mut conn, db) = common::setup();

    let task = Task::new("KG Generation");
    let kg_path = common::workspace_root().join("kg_data/directus");
    db.write_kg_csvs(&mut conn, kg_path.as_path(), false).expect("Failed to write KG CSVs");
    tracker.add_completed_task(task);

    // We print the report
    let report_path = common::workspace_root().join("builder/KG_TIME_REQUIREMENTS.md");
    tracker.write(report_path.as_path()).unwrap();
}
