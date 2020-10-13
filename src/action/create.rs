use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{debug, info};
use serde_json::json;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "create")]
pub struct Args {
    /// This is the file name to use for this migration. SHOULD BE unique.
    // It is advised that you keep this name unique amongst all of your migrations, however,
    // this is not required. This is because the ame will be prefixed with the current time in an
    // ISO8601 standard format of YYYYMMDDhhmmss followed by an underscore.
    // e.g. 20200916023109_my_migration_name.json
    name: String,
}

pub async fn run(dir: &PathBuf, now: DateTime<Utc>, args: Args) -> Result<()> {
    let file_name = format!("{}_{}.json", now.format("%Y%m%d%H%M%S"), args.name);
    let template = json!({
        "down": {},
        "up": {},
    });
    info!("file = {}/{}", dir.to_string_lossy(), &file_name);
    debug!("template = {:?}", &template);

    let file = File::create(dir.join(file_name))?;
    serde_json::to_writer_pretty(&file, &template)?;

    Ok(())
}
