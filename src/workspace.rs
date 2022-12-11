use crate::configuration::{Configuration, Workspace};
use std::process::Command;

pub fn go(name: &str) {
    if let Ok(configuration) = Configuration::new_from_file() {
        let workspace = configuration
            .workspaces
            .iter()
            .find(|workspace| workspace.name == name);

        if let Some(workspace) = workspace {
            println!("Going to {}", workspace.name);
            Command::new("sh")
                .arg("-c")
                .arg(format!("cd {} && exec $SHELL", workspace.path))
                .spawn()
                .expect("Failed to execute process")
                .wait_with_output()
                .expect("Failed to wait for the exit");
            println!("Exiting from {}", workspace.name)
        } else {
            println!("Workspace not found");
        }
    }
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

pub fn list() {
    if let Ok(configuration) = Configuration::new_from_file() {
        for workspace in configuration.workspaces {
            println!("{}: \n\t{}", workspace.name, workspace.path);
        }
    } else {
        println!("No workspaces saved");
    }
}
