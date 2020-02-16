pub mod add;
pub mod delete;
pub mod list;
pub mod now;
pub mod update;

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

impl Arg {
    fn to_rtn(&self) -> Rtn {
        Rtn::Single {
            exchange: self.exchange.to_owned(),
            name: self.name.to_owned(),
        }
    }
}

type Exchange = String;
type Name = String;
type Secret = String;
pub type Data = HashMap<Exchange, HashMap<Name, Secret>>;

pub enum Rtn {
    Empty,
    Code { code: String },
    Secret { secret: String },
    Single { exchange: String, name: String },
    Multiple { data: Box<Vec<Rtn>> },
}

pub trait CliSubCommand {
    fn process(&mut self, arg: Arg) -> Result<Rtn, String>;
}

pub fn gen_key(exchange: String, name: String) -> String {
    format!("{}, {}", exchange, name)
}

#[derive(StructOpt, Debug)]
#[structopt(global_settings = &[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands])]
pub enum Command {
    #[structopt(name = "add", about = "Create new account")]
    Add {
        #[structopt(short = "e", long = "exchange", help = "exchange name")]
        exchange: Option<String>,
        #[structopt(short = "n", long = "name", help = "account name")]
        name: String,
        #[structopt(short = "s", long = "secret", help = "secret key")]
        secret: String,
    },

    #[structopt(name = "delete", about = "Delete new account")]
    Delete {
        #[structopt(short = "e", long = "exchange", help = "exchange name")]
        exchange: Option<String>,
        #[structopt(short = "n", long = "name", help = "account name")]
        name: String,
    },

    #[structopt(name = "update", about = "Update new account")]
    Update {
        #[structopt(short = "e", long = "exchange", help = "exchange name")]
        exchange: Option<String>,
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
        exchange: Option<String>,
        #[structopt(short = "n", long = "name", help = "account name")]
        name: String,
    },
}

pub fn process() -> String {
    let result = match Command::from_args() {
        Command::Add {
            exchange,
            name,
            secret,
        } => String::from(""),

        Command::Delete { exchange, name } => String::from(""),

        Command::Update {
            exchange,
            name,
            secret,
        } => String::from(""),

        Command::List { exchange } => String::from(""),

        Command::Now { exchange, name } => String::from(""),
    };
    result
}
