use crate::modules::file::*;
use crate::structures::{config::Config, error::Error, error::ErrorKind};
use std::env::current_dir;
use std::{env, fs, path::Path};

type Callback<T> = fn() -> Result<T, Error>;

// ----- Project status -----

// Check if project has been initialized
pub fn is_project_initialized() -> Result<bool, Error> {
    let project_alias = get_projec_alias()?;
    let home_path = get_home_path()?;
    let project_path = get_file_path(vec![&home_path, ".yo", &project_alias, "config"])?;

    Ok(does_path_exists(&project_path))
}

// Execute code block only if project has been initialized
pub fn if_project_initialized<T>(callback: Callback<T>) -> Result<T, Error> {
    if !is_project_initialized()? {
        let result = callback()?;
        Ok(result)
    } else {
        Err(Error::new("Project already initialized".to_string()).kind(ErrorKind::Project))
    }
}

// Get project alias
pub fn get_projec_alias() -> Result<String, Error> {
    println!("{}", get_current_path()?);
    let alias = get_current_path()?.replace("/", "_").replace("\\", "_");

    Ok(alias)
}

// ----- Operate on config -----

// Get path of config for current project
pub fn get_config_path() -> Result<String, Error> {
    let project_alias = get_projec_alias()?;
    let home_path = get_home_path()?;
    let config_path = get_file_path(vec![&home_path, ".yo", &project_alias, "config"])?;

    Ok(config_path)
}

// Get config content
pub fn get_config() -> Result<Config, Error> {
    let alias = get_projec_alias()?;
    let content = read_file(&get_config_path()?)?;
    let config: Config = serde_json::from_str(&content)
        .map_err(|e| Error::new(format!("Error when deserializing config for `{alias}`")))?;

    Ok(config)
}

// Set/update config content
pub fn set_config(config: Config) -> Result<(), Error> {
    let alias = get_projec_alias()?;

    write_to_file(&get_config_path()?, &config)
}
