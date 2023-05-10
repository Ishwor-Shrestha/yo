use crate::modules::file::*;
use crate::structures::{config::Config, error::Error, error::ErrorKind};
use std::{env, fs, path::Path};

type Callback<T> = fn() -> Result<T, Error>;

pub fn get_config_path() -> Result<String, Error> {
    let project_alias = get_projec_alias()?;
    let home_path = get_home_path()?;
    let config_path = get_file_path(vec![&home_path, ".yo", &project_alias, "config"])?;

    Ok(config_path)
}

pub fn get_config() -> Result<Config, Error> {
    let alias = get_projec_alias()?;
    let content = read_file(&get_config_path()?)?;
    let config: Config = serde_json::from_str(&content)
        .map_err(|e| Error::new(format!("Error when deserializing config for `{alias}`")))?;

    Ok(config)
}

pub fn is_project_initialized() -> Result<bool, Error> {
    let project_alias = get_projec_alias()?;
    let home_path = get_home_path()?;
    let project_path = get_file_path(vec![&home_path, ".yo", &project_alias, "config"])?;

    Ok(does_path_exists(&project_path))
}

pub fn change_directory(path: &String) {
    let resolved_path = Path::new(path);
    println!("{}", resolved_path.display());
    env::set_current_dir(&resolved_path).unwrap();
    let cd = env::current_dir().unwrap();
    println!("{}", cd.display());
}

pub fn get_projec_alias() -> Result<String, Error> {
    let current_dir = env::current_dir().map_err(|e| {
        Error::new("Could not get current directory".to_string())
            .kind(ErrorKind::FileSystem)
            .source(e)
    })?;

    let current_dir = current_dir
        .display()
        .to_string()
        .replace("/", "_")
        .replace("\\", "_");

    Ok(current_dir)
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
