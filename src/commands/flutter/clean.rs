use crate::commands::flutter::helper::flutter_project::*;
use crate::commands::flutter::*;
use crate::modules::project::{get_project_alias, if_project_initialized, run_command};
use crate::resources::strings::*;
use crate::structures::error::{Error, ErrorKind};

pub fn clean(key: &String) -> Result<String, Error> {
    if_project_initialized(&|| {
        let result = run_flutter_command("flutter clean".to_string(), key)?;

        if !result {
            return Err(
                Error::new(S_FAILED_TO_CLEAN_FLUTTER_PROJECT.to_string()).kind(ErrorKind::Project)
            );
        }

        Ok(S_CLEANED_FLUTTER_PROJECT.to_string())
    })
}
