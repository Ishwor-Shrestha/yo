use std::env;

use crate::modules::project::{get_config, set_config};
use crate::structures::args::ConfigCommand;
use crate::structures::error::Error;

pub fn update_config(config_command: ConfigCommand) -> Result<String, Error> {
    match config_command {
        ConfigCommand::Package(x) => update_package_path(&x.path),
        ConfigCommand::Script(x) => update_script_path(&x.path),
    }
}

fn update_package_path(path: &String) -> Result<String, Error> {
    let mut config = get_config()?;
    config.package(path.to_string());

    set_config(config)?;

    Ok(String::from("Package path has been set"))
}

fn update_script_path(path: &String) -> Result<String, Error> {
    let mut config = get_config()?;
    config.script(path.to_string());

    set_config(config)?;

    Ok(String::from("Script path has been set"))
}
