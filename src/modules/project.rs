use crate::modules::{file::*, strings_helper::*};
use crate::resources::strings::*;
use crate::resources::*;
use crate::structures::{error::Error, error::ErrorKind, yo_config::YoConfig};
use std::env::current_dir;
use std::process::Command;
use std::{env, fs, path::Path};

pub type Callback<T> = fn() -> Result<T, Error>;

// ----- Project status -----

// Check if project has been initialized
pub fn is_project_initialized() -> Result<bool, Error> {
    let project_alias_result = get_project_alias();

    let project_alias: String;
    match project_alias_result {
        Ok(x) => project_alias = x,
        Err(e) => {
            if e.message == S_PROJECT_NOT_INITIALIZED {
                return Ok(false);
            }

            return Err(e);
        }
    }

    let home_path = get_home_path()?;
    let project_path = get_file_path(vec![&home_path, ".yo", &project_alias, "configx.yaml"])?;

    Ok(does_path_exists(&project_path))
}

// Execute code block only if project has been initialized
pub fn if_project_initialized<T>(callback: Callback<T>) -> Result<T, Error> {
    if is_project_initialized()? {
        let result = callback()?;
        Ok(result)
    } else {
        Err(Error::new(S_PROJECT_NOT_INITIALIZED.to_string()).kind(ErrorKind::Project))
    }
}

// Get project alias
pub fn get_project_alias() -> Result<String, Error> {
    let home_path = get_home_path()?;
    let yo_path = get_file_path(vec![&home_path, ".yo"])?;

    let directories = std::fs::read_dir(&yo_path).map_err(|e| {
        Error::new(S_COULD_NOT_READ_YO_DIR.to_string())
            .kind(ErrorKind::FileSystem)
            .source(e)
    })?;

    let current_path_alias = create_project_alias()?;
    let mut alias = String::new();

    for directory in directories {
        let directory = directory.map_err(|e| {
            Error::new(S_COULD_NOT_READ_YO_DIR.to_string())
                .kind(ErrorKind::FileSystem)
                .source(e)
        })?;

        let directory_alias = directory.path().display().to_string().replace(&yo_path, "");

        let directory_alias = crop_letters(&directory_alias, 1)
            .replace("/", "_")
            .replace("\\", "_");

        if current_path_alias.contains(&directory_alias) {
            alias = directory_alias;
            break;
        }
    }

    if alias.is_empty() {
        return Err(Error::new(S_PROJECT_NOT_INITIALIZED.to_string()).kind(ErrorKind::Project));
    }

    Ok(alias)
}

// Create project alias
pub fn create_project_alias() -> Result<String, Error> {
    Ok(get_current_path()?
        .replace("/", "_")
        .replace("\\", "_")
        .to_string())
}

// ----- Operate on config -----

// Get path of config for current project
pub fn get_config_path() -> Result<String, Error> {
    let project_alias = get_project_alias()?;
    let home_path = get_home_path()?;
    let config_path = get_file_path(vec![&home_path, ".yo", &project_alias, "configx.yaml"])?;

    Ok(config_path)
}

// Get config content
pub fn get_config() -> Result<YoConfig, Error> {
    let alias = get_project_alias()?;
    let content = read_file(&get_config_path()?)?;
    let config: YoConfig = serde_json::from_str(&content)
        .map_err(|e| Error::new(S_ERROR_DESERIALIZING_CONFIG.to_string()))?;

    Ok(config)
}

// Set/update config content
pub fn set_config(config: YoConfig) -> Result<(), Error> {
    let alias = get_project_alias()?;

    write_to_file(&get_config_path()?, &config)
}

// Get config content
pub fn get_config_x() -> Result<YoConfig, Error> {
    let alias = get_project_alias()?;
    let content = read_file(&get_config_path()?)?;
    let config: YoConfig = serde_yaml::from_str(&content)
        .map_err(|e| Error::new(S_ERROR_DESERIALIZING_CONFIG.to_string()))?;

    Ok(config)
}

// Set/update config content
pub fn set_config_x(config: YoConfig) -> Result<(), Error> {
    let alias = get_project_alias()?;

    write_to_file_yaml(&get_config_path()?, &config)
}

pub fn run_command(command: &String) -> Result<(), Error> {
    let is_windows = env::consts::OS == "windows";
    let mut raw_commands: Vec<String> = vec![command.split(" ").collect()];
    let mut base_raw_command = String::new();

    if is_windows {
        base_raw_command = String::from("cmd");
        let mut raw_commands = vec!["/C".to_string()].append(&mut raw_commands);
    } else {
        base_raw_command = raw_commands
            .first()
            .ok_or(Error::new(S_FAILED_TO_COMMAND.to_string()).kind(ErrorKind::Project))?
            .to_string();

        raw_commands.remove(0);
    }

    let mut command = &mut Command::new(base_raw_command);

    for raw_command in raw_commands {
        command = command.arg(&raw_command);
    }

    command
        .output()
        .map_err(|e| Error::new(S_FAILED_TO_COMMAND.to_string()).kind(ErrorKind::Project))?;

    Ok(())
}

fn add_command_args<'a>(m_command: &'a mut Command, arg: &'a String) -> &'a mut Command {
    m_command.arg(arg)
}
