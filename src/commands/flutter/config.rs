use std::env;

use crate::modules::file::{clean_path, get_current_path, get_file_path};
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
    let current_path = get_current_path()?;
    let package_path = clean_path(get_file_path(vec![&current_path, path])?)?;

    config.package(package_path);

    set_config(config)?;

    Ok(String::from("Package path has been set"))
}

fn update_script_path(path: &String) -> Result<String, Error> {
    let mut config = get_config()?;
    let current_path = get_current_path()?;
    let script_path = clean_path(get_file_path(vec![&current_path, path])?)?;

    config.script(script_path);

    set_config(config)?;

    Ok(String::from("Script path has been set"))
}
