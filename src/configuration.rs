use ron;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

const CONFIGURATION_FILE: &str = "/.wk.ron";

pub type Workspace = (String, String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub workspaces: Vec<Workspace>,
}

impl Configuration {
    pub fn load_file(path: &str) -> Result<Configuration, ron::error::Error> {
        Configuration::from_str(&fs::read_to_string(format!(
            "{}{}",
            path, CONFIGURATION_FILE
        ))?)
    }

    pub fn save_file(&self, path: &str) -> Result<(), io::Error> {
        fs::write(
            format!("{}{}", path, CONFIGURATION_FILE),
            Configuration::to_str(&self).unwrap(),
        )
    }

    pub fn from_str(s: &str) -> Result<Configuration, ron::error::Error> {
        ron::from_str::<Configuration>(s)
    }

    pub fn to_str(&self) -> Result<String, ron::error::Error> {
        ron::to_string(self)
    }
}
