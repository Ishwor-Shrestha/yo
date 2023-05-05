use crate::modules::file::*;
use crate::structures::{config::Config, error::Error, error::ErrorKind};
use std::fs;

pub fn get_config_path(alias: &String) -> Result<String, Error> {
    let home_path = get_home_path()?;
    let config_path = get_file_path(vec![&home_path, ".yo", alias, "config"])?;

    Ok(config_path)
}

pub fn get_config(alias: &String) -> Result<Config, Error> {
    let content = read_file(&get_config_path(alias)?)?;
    let config: Config = serde_json::from_str(&content)
        .map_err(|e| Error::new(format!("Error when deserializing config for `{alias}`")))?;

    Ok(config)
}

pub fn is_project_initialized(alias: &String) -> Result<bool, Error> {
    let home_path = get_home_path()?;
    let project_path = get_file_path(vec![&home_path, ".yo", alias, "config"])?;

    Ok(does_path_exists(&project_path))
}
