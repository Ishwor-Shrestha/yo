use std::fs;
use std::path::{Path, PathBuf};

use crate::structures::error::*;

// Creates `.yo` directory in user's home directory
// if it does not exist
pub fn create_yo_dir() -> Result<(), Error> {
    let home_path = get_home_path()?;
    let yo_path = get_file_path(vec![&home_path, ".yo"])?;

    if (!does_path_exists(&yo_path)) {
        fs::create_dir(yo_path).map_err(|e| {
            Error::new("Could not create `.yo` in home directory".to_string()).source(e)
        })?;
    }

    Ok(())
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
