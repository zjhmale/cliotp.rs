use super::{Arg, CliSubCommand, Rtn};
use crate::db::DB;

pub struct UpdateSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> CliSubCommand for UpdateSubCommand<'a> {
    fn process(&self, arg: Arg) -> Result<Rtn, String> {
        self.db.update(&arg)
    }
}
