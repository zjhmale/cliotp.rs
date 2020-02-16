#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod db;
mod subcommands;

use std::collections::HashMap;
use std::process;
use subcommands::{
    AddSubCommand, Arg, Command, DelSubCommand, ListSubCommand, NowSubCommand, Rtn,
    UpdateSubCommand,
};

fn main() {

    // let result = match Command::from_args() {
    // Command::Add {
    // exchange,
    // name,
    // secret,
    // } => (),
    //
    // Command::Update {
    // exchange,
    // name,
    // secret,
    // } => (),
    // };

    // if let Err(e) = result {
    // e.pretty_print();
    // std::process::exit(1);
    // }
}
