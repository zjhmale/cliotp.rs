use super::Rtn;
use crate::db::DB;

pub struct ListSubCommand<'a> {
    pub db: &'a DB<'a>,
}

impl<'a> ListSubCommand<'a> {
    pub fn process(&self, exchange: Option<String>) -> Result<Rtn, String> {
        self.db.list(exchange)
    }
}
