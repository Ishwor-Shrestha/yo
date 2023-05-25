use crate::modules::{file::*, project::*};
use crate::resources::strings::*;
use crate::structures::{config::Config, error::Error, error::ErrorKind};

// Initialize project in directory where `yo init` is called
pub fn init() -> Result<String, Error> {
    create_yo_dir()?;

    // check if project is already initialized
    if is_project_initialized()? {
        return Err(Error::new(S_PROJECT_ALREADY_INITIALIZED.to_string()).kind(ErrorKind::Project));
    } else {
        // Create project directory with given alias
        create_project_dir()?;

        // Inside project directory create `config` file with base contents
        create_project_config()?;

        return Ok(String::from(S_PROJECT_INITIALIZED));
    }
}

// Creates `.yo` directory in user's home directory
// if it does not exist
fn create_yo_dir() -> Result<(), Error> {
    let home_path = get_home_path()?;
    let yo_path = get_file_path(vec![&home_path, ".yo"])?;

    create_dir(&yo_path)
}

// Creates project directory with passed alias as name
fn create_project_dir() -> Result<(), Error> {
    let alias = create_project_alias()?;
    let home_path = get_home_path()?;
    let project_path = get_file_path(vec![&home_path, ".yo", &alias])?;

    create_dir(&project_path)
}

// Creates config file under project directory
fn create_project_config() -> Result<(), Error> {
    let config = Config::new(get_current_path()?);
    set_config_x(config)
}
