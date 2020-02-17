#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate google_authenticator;
extern crate r2d2_redis;

mod db;
mod subcommands;

use db::{r2d2, RedisConnectionManager, DB};

fn main() {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    let db = DB {
        db_name: "cliotp",
        pool: &pool,
    };
    let result = subcommands::process(db);

    if let Err(e) = result {
        println!("{:?}", e);
        std::process::exit(1);
    } else {
        println!(">>= {:?}", result);
    }
}
