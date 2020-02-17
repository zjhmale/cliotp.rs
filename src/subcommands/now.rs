use super::{Arg, CliSubCommand, Rtn};
use crate::db::DB;

pub struct NowSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> CliSubCommand for NowSubCommand<'a> {
    fn process(&self, arg: Arg) -> Result<Rtn, String> {
        self.db.get(&arg)
    }
}
