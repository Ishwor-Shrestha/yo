use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct YoArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize yo
    Init(ProjectInfo),
    Workon(ProjectInfo),
}

#[derive(Debug, Args)]
pub struct ProjectInfo {
    /// Project alias
    pub alias: String,
}
