use google_authenticator::GA_AUTH;

use super::{Arg, CliSubCommand, Rtn};
use crate::db::DB;

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
