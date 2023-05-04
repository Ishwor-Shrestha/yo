use std::fs;
use std::path::{Path, PathBuf};

use crate::structures::error::*;

pub fn create_dir(path: &String) -> Result<(), Error> {
    if (!does_path_exists(path)) {
        fs::create_dir(path)
            .map_err(|e| Error::new(format!("Could not create `{path}`")).source(e))?;
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
