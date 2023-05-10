#![allow(unused)]
mod commands;
mod modules;
mod structures;
mod utils;

use std::process;

use clap::Parser;
use commands::base::init::init;
use structures::args::{Command, YoArgs};
use utils::print::{fprint, MessageType};

fn main() {
    let args = YoArgs::parse();

    let result = match args.command {
        Command::Init => init(),
    };

    match result {
        Ok(success) => fprint(success, MessageType::Success),
        Err(e) => {
            fprint(e.to_string(), MessageType::Error);
            process::exit(e.code);
        }
    }
}
