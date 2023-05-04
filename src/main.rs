#![allow(unused)]
mod structures;

use clap::Parser;
use structures::args::{Command, YoArgs};

fn main() {
    let args = YoArgs::parse();

    match args.command {
        Command::Init(x) => println!("{}", x.alias),
        Command::Workon(x) => println!("{}", x.alias),
    }
}
