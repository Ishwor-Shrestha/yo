use crate::modules::file::*;
use crate::structures::{config::Config, error::Error, error::ErrorKind};
use std::fs;

// Creates `.yo` directory in user's home directory
// if it does not exist
pub fn create_yo_dir() -> Result<(), Error> {
    let home_path = get_home_path()?;
    let yo_path = get_file_path(vec![&home_path, ".yo"])?;

    create_dir(&yo_path)
}

pub fn create_project_dir(alias: &String) -> Result<(), Error> {
    let home_path = get_home_path()?;
    let project_path = get_file_path(vec![&home_path, ".yo", alias])?;

    create_dir(&project_path)
}

pub fn create_project_config(alias: &String) -> Result<(), Error> {
    let config = Config {
        root: get_current_path()?,
    };

    let home_path = get_home_path()?;
    let config_path = get_file_path(vec![&home_path, ".yo", alias, "config"])?;

    write_to_file(&config_path, &config)
}

pub fn is_project_initialized(alias: &String) -> Result<bool, Error> {
    let home_path = get_home_path()?;
    let project_path = get_file_path(vec![&home_path, ".yo", alias, "config"])?;

    Ok(does_path_exists(&project_path))
}
