use std::collections::hash_set::Difference;
use std::collections::HashMap;
use std::fs::{self, DirEntry, ReadDir};
use std::path::Path;
use std::result;
use std::sync::{Arc, Mutex};

use crate::modules::file::{change_directory, does_path_exists, get_current_path, get_file_path};
use crate::modules::project::*;
use crate::resources::strings::*;
use crate::structures::yo_config::{YoConfig, YoFlutterConfig, YoFlutterPubspecDirectory};
use crate::structures::{error::Error, error::ErrorKind};

pub fn run_flutter_command(commmand: String, key: String) -> Result<(), Error> {
    let config = get_config()?;
    let flutter_config = config.flutter_config;

    if flutter_config.pubspec_dirs.is_empty() {
        return Err(Error::new(S_MAKE_SURE_SCAN_THE_PROJECT_FIRST.to_string()));
    }
    Ok(())
}

pub fn scan_flutter_project() -> Result<(), Error> {
    let mut config = get_config()?;
    let mut flutter_config = config.flutter_config;
    let path = flutter_config.package_dir;
    let mut result: Result<(), Error>;

    let root_path = Path::new(&path);
    let mut pubspec_paths = Arc::new(Mutex::new(Vec::<YoFlutterPubspecDirectory>::new()));

    collect_valid_pubspec_dir(root_path, &|file| {
        let value = file.display().to_string();
        let key = value.replace(&path, "");

        pubspec_paths
            .lock()
            .expect(S_FAILED_TO_SCAN_PUBSPEC_DIRECTORIES)
            .push(YoFlutterPubspecDirectory::new(key, value));
    })?;

    let error = Error::new(S_FAILED_TO_SCAN_PUBSPEC_DIRECTORIES.to_string());

    let x_pubspec_paths = Arc::try_unwrap(pubspec_paths)
        .expect(S_FAILED_TO_SCAN_PUBSPEC_DIRECTORIES)
        .into_inner()
        .expect(S_FAILED_TO_SCAN_PUBSPEC_DIRECTORIES);

    let new_flutter_config = YoFlutterConfig::new(path, x_pubspec_paths);
    let new_yo_config = YoConfig::new(config.root, new_flutter_config);

    set_config(new_yo_config);

    Ok(())
}

fn collect_valid_pubspec_dir(dir: &Path, callback: &dyn Fn(&Path)) -> Result<(), Error> {
    if dir.is_dir() {
        let has_pubspec = does_pubspec_exist(dir)?;
        if has_pubspec {
            callback(dir);
        }

        // Read all the files in the given path
        let files = std::fs::read_dir(dir).map_err(|e| {
            Error::new(format!("Could not read `{}`", dir.display().to_string()))
                .kind(ErrorKind::FileSystem)
                .source(e)
        })?;

        for file in files {
            let file = file.map_err(|e| {
                Error::new(format!("Could not read directory"))
                    .kind(ErrorKind::FileSystem)
                    .source(e)
            })?;

            let path = file.path();

            if path.is_file() {
                continue;
            }

            if does_pubspec_exist(&path)? {
                callback(&path);
            } else {
                collect_valid_pubspec_dir(&path, callback)?;
            }
        }
    }

    Ok(())
}

// Checks if given directory has `pubspec.yaml`
fn does_pubspec_exist(dir: &Path) -> Result<bool, Error> {
    let file_path = get_file_path(vec![&dir.display().to_string(), "pubspec.yaml"])?;

    Ok(does_path_exists(&file_path))
}
