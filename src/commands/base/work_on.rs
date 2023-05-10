use crate::{
    modules::project::{change_directory, get_config},
    structures::error::Error,
};

use std::process::Command;

pub fn work_on(alias: &String) -> Result<String, Error> {
    let config = get_config()?;
    let project_root = config.root;

    println!("{}", project_root);

    match Command::new("cd").arg(&project_root).spawn() {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    }

    Ok(String::from(format!("Working on `{alias}`")))
}
