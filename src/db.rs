use crate::subcommands::{Arg, Data, Rtn};
use std::collections::HashMap;

pub use r2d2_redis::r2d2::PooledConnection;
pub use r2d2_redis::redis::{Commands, Connection, RedisResult};
pub use r2d2_redis::{r2d2, RedisConnectionManager};

pub struct DB<'a> {
    pub db_name: &'a str,
    pub pool: &'a r2d2::Pool<RedisConnectionManager>,
}

impl<'a> DB<'a> {
    fn after_conn<F>(&self, cb: F) -> Result<Rtn, String>
    where
        F: FnOnce(PooledConnection<RedisConnectionManager>) -> Result<Rtn, String>,
    {
        self.pool.get().map_err(|e| format!("{:?}", e)).and_then(cb)
    }

    fn after_data<F>(&self, cb: F) -> Result<Rtn, String>
    where
        F: FnMut(Data) -> Result<Rtn, String>,
    {
        self.after_conn(|mut conn| {
            conn.get(self.db_name)
                .map_err(|e| format!("{:?}", e))
                .and_then(|data: String| {
                    serde_json::from_str::<Data>(&data).map_err(|e| format!("{:?}", e))
                })
                .and_then(cb)
        })
    }

    pub fn add(&self, arg: &Arg) -> Result<Rtn, String> {
        self.after_data(|mut table| {
            arg.secret
                .to_owned()
                .ok_or(String::from("no secret supplied"))
                .and_then(|secret| match table.get(&arg.exchange) {
                    Some(_) => Err(String::from("account exists already")),
                    None => {
                        let mut exchange_data = HashMap::new();
                        exchange_data.insert(arg.name.to_owned(), secret);
                        table.insert(arg.exchange.to_owned(), exchange_data);
                        serde_json::to_string(&table)
                            .map_err(|e| format!("{:?}", e))
                            .and_then(|t| {
                                self.after_conn(|mut conn| {
                                    let _: RedisResult<String> = conn.set(self.db_name, t);
                                    Ok(Rtn::Empty)
                                })
                            })
                    }
                })
        })
    }

    pub fn update(&self, arg: &Arg) -> Result<Rtn, String> {
        self.after_data(|mut table| {
            arg.secret
                .to_owned()
                .ok_or(String::from("no secret supplied"))
                .and_then(|secret| {
                    table
                        .get_mut(&arg.exchange)
                        .ok_or(String::from("no exchange found"))
                        .and_then(|exchange_data| {
                            if exchange_data.contains_key(&arg.name) {
                                exchange_data.insert(arg.name.to_owned(), secret);
                                Ok(Rtn::Empty)
                            } else {
                                Err(String::from("no account found"))
                            }
                        })
                })
        })
    }

    pub fn delete(&self, arg: &Arg) -> Result<Rtn, String> {
        self.after_data(|mut table| {
            table
                .get_mut(&arg.exchange)
                .ok_or(String::from("no exchange found"))
                .and_then(|exchange_data| {
                    if exchange_data.contains_key(&arg.name) {
                        exchange_data.remove(&arg.name);
                        Ok(Rtn::Empty)
                    } else {
                        Err(String::from("no account found"))
                    }
                })
        })
    }

    pub fn list(&self, exchange: Option<String>) -> Result<Rtn, String> {
        self.after_conn(|mut conn| {
            let mut result = vec![];
            conn.get(self.db_name)
                .map_err(|e| format!("{:?}", e))
                .and_then(|data: String| {
                    serde_json::from_str::<Data>(&data).map_err(|e| format!("{:?}", e))
                })
                .and_then(|table| {
                    match exchange {
                        Some(exchange_name) => {
                            if let Some(exchange_data) = table.get(&exchange_name) {
                                for (name, _) in exchange_data.iter() {
                                    result.push(Rtn::Single {
                                        exchange: exchange_name.to_owned(),
                                        name: name.to_owned(),
                                    })
                                }
                            }
                        }
                        None => {
                            for (exchange_name, exchange_data) in table.iter() {
                                for (name, _) in exchange_data.iter() {
                                    result.push(Rtn::Single {
                                        exchange: exchange_name.to_owned(),
                                        name: name.to_owned(),
                                    })
                                }
                            }
                        }
                    }
                    Ok(Rtn::Multiple {
                        data: Box::new(result),
                    })
                })
        })
    }

    pub fn get(&self, arg: &Arg) -> Result<Rtn, String> {
        self.after_conn(|mut conn| {
            let data: String = conn.get(self.db_name).unwrap();
            let table = serde_json::from_str::<Data>(&data).unwrap();

            table
                .get(&arg.exchange)
                .ok_or(String::from("no exchange found"))
                .and_then(|exchange_data| {
                    exchange_data
                        .get(&arg.name)
                        .ok_or(String::from("no account found"))
                })
                .map(|secret| Rtn::Secret {
                    secret: secret.to_owned(),
                })
        })
    }
}
