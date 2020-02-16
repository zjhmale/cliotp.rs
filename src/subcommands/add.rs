use std::collections::HashMap;

use super::{Arg, CliSubCommand, Rtn};
use crate::db::DB;

pub struct AddSubCommand<'a> {
    db: &'a DB<'a>,
}

impl<'a> CliSubCommand for AddSubCommand<'a> {
    fn process(&mut self, arg: Arg) -> Result<Rtn, String> {
        Ok(arg.to_rtn())
    }
}
