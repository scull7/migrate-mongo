use anyhow::Result;
use mongodb::{bson::document::Document, Collection, Database};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Migration {
    pub up: Command,
    pub down: Command,
}

impl Migration {
    pub async fn up(&self, db: &Database) -> Result<()> {
        self.up.run(db).await
    }

    pub async fn down(&self, db: &Database) -> Result<()> {
        self.down.run(db).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Command(Document);

impl Command {
    pub async fn run(&self, db: &Database) -> Result<()> {
        println!("Running command: {:?}", &self.0);

        db.run_command(self.0, None).await
    }
}
