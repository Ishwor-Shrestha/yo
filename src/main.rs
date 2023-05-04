#![allow(unused)]
mod commands;
mod modules;
mod structures;
mod utils;

use clap::Parser;
use commands::init;
use structures::args::{Command, YoArgs};
use utils::print::{fprint, MessageType};

fn main() {
    let args = YoArgs::parse();

    match args.command {
        Command::Init(x) => match init::init(&(x.alias)) {
            Ok(_) => fprint("Initialized".to_string(), MessageType::Success),
            Err(e) => fprint(e.to_string(), MessageType::Error),
        },
        Command::Workon(x) => println!("{}", x.alias),
    }
}
