use crate::commands::flutter::flutter_project::*;
use crate::commands::flutter::*;
use crate::modules::project::{get_project_alias, if_project_initialized, run_command};
use crate::structures::error::{Error, ErrorKind};

pub fn get() -> Result<String, Error> {
    if_project_initialized(|| {
        run_flutter_command("flutter_pub_get".to_string(), "".to_string());

        Ok("Fetched dependecies".to_string())
    })
}
