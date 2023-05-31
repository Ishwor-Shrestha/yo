use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct YoArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize yo
    Init,

    // ----- Flutter commands -----
    /// Open config
    Config,

    /// Get flutter dependecies
    Get,
}
