#![allow(unused)]
mod commands;
mod modules;
mod resources;
mod structures;
mod utils;

use std::process;

use clap::Parser;
use commands::base::init::init;
use commands::flutter::{config::update_config, get::get};
use structures::args::{Command, ConfigArgs, ConfigCommand, YoArgs};
use utils::print::{fprint, MessageType};

fn main() {
    let args = YoArgs::parse();

    let result = match args.command {
        Command::Init => init(),
        Command::Config(x) => update_config(x),
        Command::Get => get(),
    };

    match result {
        Ok(success) => fprint(success, MessageType::Success),
        Err(e) => {
            fprint(e.to_string(), MessageType::Error);
            process::exit(e.code);
        }
    }
}
