use std::collections::hash_set::Difference;
use std::fs;

use crate::modules::file::{change_directory, does_path_exists, get_file_path};
use crate::modules::project::*;
use crate::structures::{error::Error, error::ErrorKind};

// Find directory where [pubspec.yaml] exists
// Given [FlutterCommandArgs::Monolithic]
// it is expected that all the packages are under a directory named [packages]
pub fn execute_flutter_command(
    command: &String,
    recursion: bool,
    recursion_level: i32,
) -> Result<(), Error> {
    let config = get_config()?;
    let flutter_config = config.flutter_config;
    let path = flutter_config.package_dir;

    return match flutter_config.project_type.as_str() {
        "default" => execute_for_default(&command, &path),
        "monorepo" => {}
        &_ => {
            return Err(Error::new(format!(
                "{} not recognized as a valid project type",
                flutter_config.project_type
            ))
            .kind(ErrorKind::Project));
        }
    }

    match execution_type {
        FlutterCommandExecutionType::Single(path) => {
            let package_path = get_file_path(vec![&command_path, "packages", &path])?;

            if !does_path_exists(&package_path) {
                return Err(
                    Error::new("Could not find `packages` directory".to_string())
                        .kind(ErrorKind::FileSystem),
                );
            }

            change_directory(&package_path);
            run_command(&command)?;
        }
        FlutterCommandExecutionType::Monolithic => {
            let packages_path = get_file_path(vec![&command_path, "packages"])?;

            if !does_path_exists(&packages_path) {
                return Err(
                    Error::new("Could not find `packages` directory".to_string())
                        .kind(ErrorKind::FileSystem),
                );
            }

            // Read all the files in the given path
            let files = std::fs::read_dir(&packages_path).map_err(|e| {
                Error::new(format!("Could not read `{packages_path}`"))
                    .kind(ErrorKind::FileSystem)
                    .source(e)
            })?;

            // Loop over the collected files
            for file in files {
                let file = file.map_err(|e| {
                    Error::new(format!("Could not read `{packages_path}`"))
                        .kind(ErrorKind::FileSystem)
                        .source(e)
                })?;

                let file_metadata = file.metadata().map_err(|e| {
                    Error::new(format!("Could not read metadata for `{packages_path}`"))
                        .kind(ErrorKind::FileSystem)
                        .source(e)
                })?;

                // Skip this iteration if file is not a directory
                if file_metadata.is_file() {
                    continue;
                }

                change_directory(&file.path().display().to_string());
                run_command(command)?;
            }
        }
    }

    Ok(())
}

fn execute_for_default(command: &String, path: &String) -> Result<(), Error> {
    if !does_path_exists(&path) {
        return Err(
            Error::new(format!("Could not find {path}").to_string()).kind(ErrorKind::FileSystem)
        );
    }

    change_directory(&path);
    run_command(command)
}

pub enum FlutterCommandExecutionType {
    Single(String),
    Monolithic,
}
