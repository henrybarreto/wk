use ron;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub name: String,
    pub path: String,
}

// TODO: find a way to make this generic.
const CONFIGURATION_PATH: &str = "/home/henry";
const CONFIGURATION_FILE: &str = "/.wk.ron";

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub workspaces: Vec<Workspace>,
}

impl Configuration {
    pub fn new() -> Configuration {
        if let Ok(configuration) = Self::load_file() {
            configuration
        } else {
            Configuration {
                workspaces: Vec::new(),
            }
        }
    }

    pub fn load_file() -> Result<Configuration, ron::error::Error> {
        Configuration::from_str(&fs::read_to_string(format!(
            "{}{}",
            CONFIGURATION_PATH, CONFIGURATION_FILE
        ))?)
    }

    pub fn save_file(&self) -> Result<(), io::Error> {
        print!("{:#?}", Configuration::to_str(&self).unwrap());
        fs::write(
            format!("{}{}", CONFIGURATION_PATH, CONFIGURATION_FILE),
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
