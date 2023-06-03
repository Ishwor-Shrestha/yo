use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YoConfig {
    // Root of the project
    pub root: String,

    // ----- For flutter projects -----
    pub flutter_config: YoFlutterConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YoFlutterConfig {
    // Package directory
    pub package_dir: String,

    // All directories which contains the pubspec.yaml file
    pub pubspec_dirs: Vec<YoFlutterPubspecDirectory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YoFlutterPubspecDirectory {
    // ID
    pub key: String,

    // Path to directory
    pub path: String,
}

impl YoConfig {
    pub fn new(root: String, flutter_config: YoFlutterConfig) -> YoConfig {
        YoConfig {
            root,
            flutter_config,
        }
    }

    pub fn flutter_config(mut self, flutter_config: YoFlutterConfig) -> YoConfig {
        self.flutter_config = flutter_config;
        self
    }
}

impl YoFlutterConfig {
    pub fn new(
        package_dir: String,
        pubspec_dirs: Vec<YoFlutterPubspecDirectory>,
    ) -> YoFlutterConfig {
        YoFlutterConfig {
            package_dir,
            pubspec_dirs,
        }
    }

    pub fn pubspec_dirs(mut self, pubspec_dirs: Vec<YoFlutterPubspecDirectory>) -> YoFlutterConfig {
        self.pubspec_dirs = pubspec_dirs;
        self
    }
}

impl YoFlutterPubspecDirectory {
    pub fn new(key: String, path: String) -> YoFlutterPubspecDirectory {
        YoFlutterPubspecDirectory { key, path }
    }
}
