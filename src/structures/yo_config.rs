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
    // 1. Default
    // 2. Monorepo
    pub project_type: String,

    // Package directory
    pub package_dir: String,
}

impl YoConfig {
    pub fn new(root: String) -> YoConfig {
        YoConfig {
            root,
            flutter_config: YoFlutterConfig::new(String::from("default")),
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

impl YoFlutterConfig {
    pub fn new(project_type: String) -> YoFlutterConfig {
        YoFlutterConfig {
            project_type,
            package_dir: String::new(),
        }
    }
}
