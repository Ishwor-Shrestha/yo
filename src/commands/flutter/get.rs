use crate::commands::flutter::flutter_project::*;
use crate::commands::flutter::*;
use crate::modules::project::{get_project_alias, if_project_initialized, run_command};
use crate::resources::strings::*;
use crate::structures::error::{Error, ErrorKind};

pub fn get(key: &String) -> Result<String, Error> {
    if_project_initialized(&|| {
        let result = run_flutter_command("flutter pub get".to_string(), key)?;

        if !result {
            return Err(
                Error::new(S_FAILED_TO_FETCH_FLUTTER_DEPENDENCIES.to_string())
                    .kind(ErrorKind::Project),
            );
        }

        Ok(S_FETCHED_FLUTTER_DEPENDENCIES.to_string())
    })
}
