use super::{Arg, CliSubCommand, Rtn};
use crate::db::DB;

pub struct AddSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> CliSubCommand for AddSubCommand<'a> {
    fn process(&self, arg: Arg) -> Result<Rtn, String> {
        self.db.add(&arg)
    }
}
