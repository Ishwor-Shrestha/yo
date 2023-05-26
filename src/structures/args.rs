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
    /// Set config
    #[clap(subcommand)]
    Config(ConfigCommand),

    /// Get flutter dependecies
    Get,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Set package path
    Package(ConfigArgs),
    /// Set script path
    Script(ConfigArgs),
}

#[derive(Debug, Args)]
pub struct ConfigArgs {
    /// Path
    pub path: String,
}
