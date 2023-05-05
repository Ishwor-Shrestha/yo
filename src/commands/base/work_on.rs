use crate::{modules::project::get_config, structures::error::Error};

pub fn work_on(alias: &String) -> Result<String, Error> {
    let config = get_config(alias)?;

    print!("Config {config:?}");

    Ok(String::from("Working on ..."))
}
