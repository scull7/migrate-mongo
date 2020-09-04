use chrono::Utc;
use std::{
    env,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

mod action;
mod options;

use options::MongoMigrate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = MongoMigrate::from_args();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "migrate_mongo=info");
    }
    env_logger::init();

    let now = Utc::now();

    match args {
        MongoMigrate::Create(args) => action::create::run(&migration_dir("."), now, args).await,
        MongoMigrate::Down(_args) => action::down::run().await,
        MongoMigrate::Init(_args) => action::init::run().await,
        MongoMigrate::Status(_args) => action::status::run().await,
        MongoMigrate::Up(_args) => action::up::run().await,
    }
}

fn migration_dir(path: &str) -> PathBuf {
    let migration_dir = Path::new(path)
        .canonicalize()
        .expect("Unable to canonicalize the migration_dir");

    if !migration_dir.is_dir() {
        panic!(
            "Migration path was not a directory, maybe it's missing? - received: \"{}\" => \"{}\"",
            path,
            migration_dir.to_string_lossy(),
        );
    }

    migration_dir
}
