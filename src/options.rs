use crate::action::create;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mongo-migrate")]
pub enum MongoMigrate {
    Create(create::Args),
    Down(Down),
    Init(Init),
    Status(Status),
    Up(Up),
}

#[derive(Debug, StructOpt)]
pub struct Down {}

#[derive(Debug, StructOpt)]
pub struct Init {}

#[derive(Debug, StructOpt)]
pub struct Status {}

#[derive(Debug, StructOpt)]
pub struct Up {}
