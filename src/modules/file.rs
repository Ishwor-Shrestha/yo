use serde::Serialize;
use std::env;
use std::fmt::format;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::structures::error::*;

// ----- Operate on paths -----

pub fn get_current_path() -> Result<String, Error> {
    let current_path = env::current_dir().map_err(|e| {
        Error::new("Could not fetch current working path".to_string())
            .kind(ErrorKind::FileSystem)
            .source(e)
    })?;

    let current_path = current_path.to_str().ok_or(
        Error::new("Could not fetch current working path".to_string()).kind(ErrorKind::FileSystem),
    )?;

    Ok(current_path.to_string())
}

// Get path of user's home directory
pub fn get_home_path() -> Result<String, Error> {
    let home_path = home::home_dir()
        .ok_or(Error::new("Could not get home path".to_string()).kind(ErrorKind::FileSystem))?;

    let home_path = home_path
        .to_str()
        .ok_or(Error::new("Could not get home path".to_string()).kind(ErrorKind::FileSystem))?;

    Ok(home_path.to_string())
}

// Creates platform specific file path
// from a list of strings
pub fn get_file_path(directories: Vec<&str>) -> Result<String, Error> {
    let dirs = directories.clone().join(",");
    let path: PathBuf = directories.iter().collect();

    let resolved_path = path.to_str().ok_or(
        Error::new(format!(
            "Could not create file path for directories ({dirs})",
        ))
        .kind(ErrorKind::FileSystem),
    )?;

    Ok(resolved_path.to_string())
}

// Checks if given path exists or not
pub fn does_path_exists(path: &String) -> bool {
    Path::new(path).exists()
}

// Clean up user passed path
pub fn clean_path(path: String) -> Result<String, Error> {
    change_directory(&path);
    get_current_path()
}

// ----- Operate on diretory -----

// Create directory in path
pub fn create_dir(path: &String) -> Result<(), Error> {
    if (!does_path_exists(path)) {
        fs::create_dir(path)
            .map_err(|e| Error::new(format!("Could not create `{path}`")).source(e))?;
    }

    Ok(())
}

// Change current working directory
pub fn change_directory(path: &String) {
    let resolved_path = Path::new(path);
    println!("{}", resolved_path.display());
    env::set_current_dir(&resolved_path).unwrap();
    let cd = env::current_dir().unwrap();
    println!("{}", cd.display());
}

// ----- Read/write file -----

// Read content from file
pub fn read_file(path: &String) -> Result<String, Error> {
    if does_path_exists(path) {
        let content = fs::read_to_string(path).map_err(|e| {
            Error::new(format!("Failed to read `{path}`"))
                .code(exitcode::NOINPUT)
                .kind(ErrorKind::FileSystem)
                .source(e)
        })?;

        Ok(content)
    } else {
        Err(Error::new(format!("`{path}` does not exists")).kind(ErrorKind::FileSystem))
    }
}

// Serialize and write to file
pub fn write_to_file<T: Serialize>(path: &String, t: &T) -> Result<(), Error> {
    let contents = serde_json::to_string(t).map_err(|e| {
        Error::new(format!(
            "Failed to serialize file in path `{path}` to write to file"
        ))
        .code(exitcode::IOERR)
        .kind(ErrorKind::FileSystem)
        .source(e)
    })?;

    write_string_to_file(path, contents)
}

// Write string to file
pub fn write_string_to_file(path: &String, contents: String) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .map_err(|e| {
            Error::new(format!("Failed to open file [{}]", path))
                .code(exitcode::IOERR)
                .kind(ErrorKind::FileSystem)
                .source(e)
        })?;

    // Remove all contents from the file
    file.set_len(0);

    file.write_all(contents.as_bytes()).map_err(|e| {
        Error::new(format!("Failed to write to file [{path}]"))
            .code(exitcode::IOERR)
            .kind(ErrorKind::FileSystem)
            .source(e)
    })?;

    file.flush().map_err(|e| {
        Error::new(format!(
            "Something went wrong when writing to file [{path}]"
        ))
        .code(exitcode::IOERR)
        .kind(ErrorKind::FileSystem)
        .source(e)
    })?;

    Ok(())
}
