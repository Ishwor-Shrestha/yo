use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Root of the project
    pub root: String,

    // ----- For flutter projects -----

    // Package directory
    pub package: String,

    // Script directory
    pub script: String,
}

impl Config {
    pub fn new(root: String) -> Config {
        Config {
            root,
            package: String::new(),
            script: String::new(),
        }
    }

    pub fn package(&mut self, package: String) -> &Config {
        self.package = package;
        self
    }

    pub fn script(&mut self, script: String) -> &Config {
        self.script = script;
        self
    }
}
