use super::{Arg, CliSubCommand, Rtn};
use crate::db::DB;

pub struct ListSubCommand<'a> {
    db: &'a DB<'a>,
}

impl<'a> CliSubCommand for ListSubCommand<'a> {
    fn process(&mut self, arg: Arg) -> Result<Rtn, String> {
        Ok(arg.to_rtn())
    }
}
