#![allow(unused)]
mod commands;
mod modules;
mod structures;
mod utils;

use std::process;

use clap::Parser;
use commands::base::{init::init, work_on::work_on};
use structures::args::{Command, YoArgs};
use utils::print::{fprint, MessageType};

fn main() {
    let args = YoArgs::parse();

    let result = match args.command {
        Command::Init(x) => init(&(x.alias)),
        Command::Workon(x) => work_on(&(x.alias)),
    };

    match result {
        Ok(success) => fprint(success, MessageType::Success),
        Err(e) => {
            fprint(e.to_string(), MessageType::Error);
            process::exit(e.code);
        }
    }
}
