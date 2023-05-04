use crate::structures::error::{Error, ErrorKind};
use crate::{commands::init, modules::project::*};

pub fn init(project_alias: &String) -> Result<(), Error> {
    create_yo_dir()?;

    // check if project is already initialized
    if !is_project_initialized(project_alias)? {
        // Create project directory with given alias
        create_project_dir(project_alias)?;

        // Inside project directory create `config` file with base contents
        create_project_config(project_alias)
    } else {
        return Err(Error::new(format!(
            "Project already initialized with alias `{project_alias}`"
        ))
        .kind(ErrorKind::Project));
    }
}
