use crate::commands::flutter::*;
use crate::modules::project::*;
use crate::resources::strings::S_SCANNED_PROJECT_DIRECTORY;
use crate::structures::error::*;

use super::helper::flutter_project::scan_flutter_project;

pub fn scan() -> Result<String, Error> {
    if_project_initialized(&|| {
        scan_flutter_project()?;

        Ok(S_SCANNED_PROJECT_DIRECTORY.to_string())
    })
}
