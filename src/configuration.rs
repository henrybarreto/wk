use serde::{Deserialize, Serialize};
use std::io;
use std::{env, fs};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workspace {
    pub name: String,
    pub path: String,
}

const CONFIGURATION_FILE: &str = "/.wk.ron";

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub workspaces: Vec<Workspace>,
}

impl Configuration {
    pub fn new() -> Configuration {
        if let Ok(configuration) = Self::new_from_file() {
            configuration
        } else {
            Configuration {
                workspaces: Vec::new(),
            }
        }
    }

    pub fn new_from_file() -> Result<Configuration, ron::error::Error> {
        let mut path = String::new();
        path.push_str(&env::var("HOME").unwrap());
        path.push_str(CONFIGURATION_FILE);

        Configuration::from_str(&fs::read_to_string(path)?)
    }

    pub fn from_str(s: &str) -> Result<Configuration, ron::error::Error> {
        ron::from_str::<Self>(s)
    }

    pub fn save_to_file(&self) -> Result<(), io::Error> {
        let mut path = String::new();
        path.push_str(&env::var("HOME").unwrap());
        path.push_str(CONFIGURATION_FILE);

        fs::write(path, self.to_str().unwrap())
    }

    pub fn to_str(&self) -> Result<String, ron::error::Error> {
        ron::to_string(self)
    }
}
