#![allow(unused)]
mod commands;
mod modules;
mod resources;
mod structures;
mod utils;

use std::process;

use clap::Parser;
use commands::base::init::init;
use commands::flutter::{
    build::build, clean::clean, config::open_config, get::get, scan::scan, test::test,
};
use structures::args::{Command, YoArgs};
use utils::print::{fprint, MessageType};

fn main() {
    let args = YoArgs::parse();

    let result = match args.command {
        Command::Init => init(),
        Command::Config => open_config(),
        Command::Scan => scan(),
        Command::Get(arg) => get(&arg.key),
        Command::Build(arg) => build(&arg.key),
        Command::Test(arg) => test(&arg.key),
        Command::Clean(arg) => clean(&arg.key),
    };

    match result {
        Ok(success) => fprint(success, MessageType::Success),
        Err(e) => {
            fprint(e.to_string(), MessageType::Error);
            process::exit(e.code);
        }
    }
}
