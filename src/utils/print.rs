#![allow(dead_code)]
use colored::Colorize;

pub fn fprint(message: String, message_type: MessageType) {
    match message_type {
        MessageType::Error => println!("{}", message.red().bold(),),
        MessageType::Info => println!("{}", message.blue().bold()),
        MessageType::Success => println!("{}", message.green().bold()),
    }
}

pub enum MessageType {
    Info,
    Error,
    Success,
}
