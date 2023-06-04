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

    /// Scan flutter project directory
    Scan,

    /// Get flutter dependecies
    Get(GetArgs),
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// Key corresponding pubspec directory
    #[clap(default_value = "")]
    pub key: String,
}
