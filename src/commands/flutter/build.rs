use crate::commands::flutter::helper::flutter_project::*;
use crate::commands::flutter::*;
use crate::modules::project::{get_project_alias, if_project_initialized, run_command};
use crate::resources::strings::*;
use crate::structures::error::{Error, ErrorKind};

pub fn build(key: &String) -> Result<String, Error> {
    if_project_initialized(&|| {
        let result = run_flutter_command(
            "dart pub run build_runner build --delete-conflicting-outputs".to_string(),
            key,
        )?;

        if !result {
            return Err(
                Error::new(S_FAILED_TO_BUILD_FLUTTER_PROJECT.to_string()).kind(ErrorKind::Project)
            );
        }

        Ok(S_BUILT_FLUTTER_PROJECT.to_string())
    })
}
