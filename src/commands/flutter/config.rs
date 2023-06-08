use std::env;

use crate::modules::file::{clean_path, get_current_path, get_file_path};
use crate::modules::project::{get_config_path, if_project_initialized};
use crate::resources::strings::*;
use crate::structures::error::Error;
use crate::structures::error::ErrorKind;

pub fn open_config() -> Result<String, Error> {
    if_project_initialized(&|| {
        let config_path = get_config_path()?;
        open::that(config_path).map_err(|x| {
            Error::new(S_FAILED_TO_OPEN_CONFIG.to_string())
                .source(x)
                .kind(ErrorKind::Project)
        });
        Ok(String::new())
    })
}
