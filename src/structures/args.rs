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
    Get(FlutterArgs),

    /// Run build runner for flutter projects
    Build(FlutterArgs),
}

#[derive(Debug, Args)]
pub struct FlutterArgs {
    /// Key corresponding pubspec directory
    #[clap(default_value = "")]
    pub key: String,
}
