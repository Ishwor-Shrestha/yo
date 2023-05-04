use crate::modules::file::*;
use crate::structures::error::Error;
use std::fs;

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
