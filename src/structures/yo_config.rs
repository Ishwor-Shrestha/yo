use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YoConfig {
    // Root of the project
    pub root: String,

    // ----- For flutter projects -----
    pub flutter_config: YoFlutterConfig,

    // Package directory
    pub package: String,

    // Script directory
    pub script: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoFlutterConfig {
    // Project type
    // 1. default
    // 2. monorepo
    pub project_type: String,

    // Package directory
    pub package_dir: String,
}

impl YoConfig {
    pub fn new(root: String, flutter_config: YoFlutterConfig) -> YoConfig {
        YoConfig {
            root,
            flutter_config,
            package: String::new(),
            script: String::new(),
        }
    }

    pub fn package(&mut self, package: String) -> &YoConfig {
        self.package = package;
        self
    }

    pub fn script(&mut self, script: String) -> &YoConfig {
        self.script = script;
        self
    }
}
