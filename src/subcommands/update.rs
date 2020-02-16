use super::{Arg, CliSubCommand, Rtn};
use crate::db::DB;

pub struct UpdateSubCommand<'a> {
    db: &'a DB<'a>,
}

impl<'a> CliSubCommand for UpdateSubCommand<'a> {
    fn process(&mut self, arg: Arg) -> Result<Rtn, String> {
        Ok(arg.to_rtn())
    }
}
