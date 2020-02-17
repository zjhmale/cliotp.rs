use super::{Arg, CliSubCommand, Rtn};
use crate::db::DB;

pub struct DelSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> CliSubCommand for DelSubCommand<'a> {
    fn process(&self, arg: Arg) -> Result<Rtn, String> {
        self.db.delete(&arg)
    }
}
