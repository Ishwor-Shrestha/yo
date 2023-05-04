use crate::structures::error::Error;
use crate::{commands::init, modules::file::create_yo_dir};

pub fn init(project_alias: &String) -> Result<(), Error> {
    create_yo_dir()?;

    Ok(())
}
