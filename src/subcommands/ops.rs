use google_authenticator::GA_AUTH;

use super::io::{Arg, Rtn};
use crate::db::DB;

pub trait CliSubCommand {
    fn process(&self, arg: Arg) -> Result<Rtn, String>;
}

pub struct AddSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> CliSubCommand for AddSubCommand<'a> {
    fn process(&self, arg: Arg) -> Result<Rtn, String> {
        self.db.add(&arg)
    }
}

pub struct DelSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> CliSubCommand for DelSubCommand<'a> {
    fn process(&self, arg: Arg) -> Result<Rtn, String> {
        self.db.delete(&arg)
    }
}

pub struct ListSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> ListSubCommand<'a> {
    pub fn process(&self, exchange: Option<String>) -> Result<Rtn, String> {
        self.db.list(exchange)
    }
}

pub struct UpdateSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> CliSubCommand for UpdateSubCommand<'a> {
    fn process(&self, arg: Arg) -> Result<Rtn, String> {
        self.db.update(&arg)
    }
}

pub struct NowSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> CliSubCommand for NowSubCommand<'a> {
    fn process(&self, arg: Arg) -> Result<Rtn, String> {
        self.db.get(&arg).and_then(|rtn| match rtn {
            Rtn::Secret { secret } => get_code!(&secret)
                .map_err(|e| format!("{:?}", e))
                .map(|code| Rtn::Code { code }),
            _ => Ok(Rtn::Empty),
        })
    }
}
