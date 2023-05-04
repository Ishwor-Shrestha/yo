use std::fs;
use std::path::{Path, PathBuf};

use crate::structures::error::*;

pub fn create_yo_dir() -> Result<(), Error> {
    let yo_path = home::home_dir()
        .ok_or(Error::new("Could not get home path".to_string()).kind(ErrorKind::FileSystem))?;

    let yo_path = yo_path
        .to_str()
        .ok_or(Error::new("Could not get home path".to_string()).kind(ErrorKind::FileSystem))?;

    println!("Home path {}", yo_path);

    fs::create_dir(get_file_path(vec![yo_path, ".yo"])?).map_err(|e| {
        Error::new("Could not create `.yox` in home directory".to_string()).source(e)
    })?;

    Ok(())
}

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
