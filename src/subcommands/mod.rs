pub mod add;
pub mod delete;
pub mod list;
pub mod now;
pub mod update;

use crate::db::DB;
pub use add::AddSubCommand;
pub use delete::DelSubCommand;
pub use list::ListSubCommand;
pub use now::NowSubCommand;
pub use update::UpdateSubCommand;

use std::collections::HashMap;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Arg {
    pub exchange: String,
    pub name: String,
    pub secret: Option<String>,
}

type Exchange = String;
type Name = String;
type Secret = String;
pub type Data = HashMap<Exchange, HashMap<Name, Secret>>;

#[derive(Debug)]
pub enum Rtn {
    Empty,
    Code { code: String },
    Secret { secret: String },
    Single { exchange: String, name: String },
    Multiple { data: Box<Vec<Rtn>> },
}

pub trait CliSubCommand {
    fn process(&self, arg: Arg) -> Result<Rtn, String>;
}

#[derive(StructOpt, Debug)]
#[structopt(global_settings = &[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands])]
pub enum Command {
    #[structopt(name = "add", about = "Create new account")]
    Add {
        #[structopt(short = "e", long = "exchange", help = "exchange name")]
        exchange: String,
        #[structopt(short = "n", long = "name", help = "account name")]
        name: String,
        #[structopt(short = "s", long = "secret", help = "secret key")]
        secret: String,
    },

    #[structopt(name = "delete", about = "Delete new account")]
    Delete {
        #[structopt(short = "e", long = "exchange", help = "exchange name")]
        exchange: String,
        #[structopt(short = "n", long = "name", help = "account name")]
        name: String,
    },

    #[structopt(name = "update", about = "Update new account")]
    Update {
        #[structopt(short = "e", long = "exchange", help = "exchange name")]
        exchange: String,
        #[structopt(short = "n", long = "name", help = "account name")]
        name: String,
        #[structopt(short = "s", long = "secret", help = "secret key")]
        secret: String,
    },

    #[structopt(name = "list", about = "List all accounts")]
    List {
        #[structopt(short = "e", long = "exchange", help = "exchange name")]
        exchange: Option<String>,
    },

    #[structopt(name = "now", about = "Show account GA code")]
    Now {
        #[structopt(short = "e", long = "exchange", help = "exchange name")]
        exchange: String,
        #[structopt(short = "n", long = "name", help = "account name")]
        name: String,
    },
}

pub fn process(db: DB) -> Result<String, String> {
    let result = match Command::from_args() {
        Command::Add {
            exchange,
            name,
            secret,
        } => AddSubCommand { db: &db }.process(Arg {
            exchange: exchange,
            name: name,
            secret: Some(secret),
        }),

        Command::Delete { exchange, name } => DelSubCommand { db: &db }.process(Arg {
            exchange: exchange,
            name: name,
            secret: None,
        }),

        Command::Update {
            exchange,
            name,
            secret,
        } => UpdateSubCommand { db: &db }.process(Arg {
            exchange: exchange,
            name: name,
            secret: Some(secret),
        }),

        Command::List { exchange } => ListSubCommand { db: &db }.process(exchange),

        Command::Now { exchange, name } => NowSubCommand { db: &db }.process(Arg {
            exchange: exchange,
            name: name,
            secret: None,
        }),
    };

    result.map(|rtn| format!("{:?}", rtn))
}
