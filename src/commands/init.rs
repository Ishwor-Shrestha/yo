use crate::structures::error::Error;
use crate::{commands::init, modules::project::*};

pub fn init(project_alias: &String) -> Result<(), Error> {
    create_yo_dir()?;

    // check if project is already initialized

    Ok(())
}
