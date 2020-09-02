use anyhow::{anyhow, Result};
use log;
use std::env;
use structopt::StructOpt;

mod options;

use options::MongoMigrate;

fn main() {
    let args = MongoMigrate::from_args();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "mongo_migrate=info");
    }

    match args {
        MongoMigrate::Create(_args) => create(),
        MongoMigrate::Down(_args) => down(),
        MongoMigrate::Init(_args) => init(),
        MongoMigrate::Status(_args) => status(),
        MongoMigrate::Up(_args) => up(),
    }
}

fn create() {
    println!("Create a new migration");
}

fn down() {
    println!("Downgrade the database using the last migration");
}

fn init() {
    println!("Initialize the system");
}

fn status() {
    println!("print current migration status");
}

fn up() {
    println!("Upgrade the database to the latest version specified by the user migrations");
}
