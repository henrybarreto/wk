use std::fmt::Display;

use crate::configuration::{Configuration, Workspace};

// TODO: implement std::error::Error.
#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.message);
    }
}

/// Gets a workspace from the configuration file by its name.
pub fn get(name: &str) -> Result<Workspace, Error> {
    let configuration = if let Ok(configuration) = Configuration::new_from_file() {
        configuration
    } else {
        return Err(Error::new("Configuration file not found"));
    };

    return Ok(
        if let Some(workspace) = configuration
            .workspaces
            .iter()
            .find(|workspace| workspace.name == name)
        {
            workspace.clone()
        } else {
            return Err(Error::new("Workspace not found"));
        },
    );
}

pub fn save(name: &str, path: &str) {
    if let Ok(mut configuration) = Configuration::new_from_file() {
        if let Some(_) = configuration
            .workspaces
            .iter()
            .find(|workspace| workspace.name == name)
        {
            println!("Workspace already exists");
        } else {
            configuration.workspaces.push(Workspace {
                name: name.to_string(),
                path: path.to_string(),
            });

            configuration.save_to_file().unwrap();
            println!("Workspace {} saved on path \"{}\"", name, path);
        }
    } else {
        let mut configuration = Configuration::new();
        configuration.workspaces.push(Workspace {
            name: name.to_string(),
            path: path.to_string(),
        });

        configuration.save_to_file().unwrap();
        println!("Workspace {} saved to path \"{}\"", name, path);
    }
}

pub fn remove(name: &str) {
    if let Ok(mut configuration) = Configuration::new_from_file() {
        if let Some(_) = configuration
            .workspaces
            .iter()
            .find(|workspace| workspace.name == name)
        {
            configuration
                .workspaces
                .retain(|workspace| workspace.name != name);
            configuration.save_to_file().unwrap();

            println!("Workspace {} removed", name);
        } else {
            println!("Workspace not found");
        }
    } else {
        println!("Workspace not found");
    }
}

pub fn list() -> Result<(), Error> {
    if let Ok(configuration) = Configuration::new_from_file() {
        configuration.workspaces.iter().for_each(|workspace| {
            println!("{}: \n\t{}", workspace.name, workspace.path);
        });

        return Ok(());
    } else {
        return Err(Error::new("No workspaces saved"));
    }
}
