use crate::configuration::{Configuration, Workspace};

pub fn go(name: &str) -> Option<Workspace> {
    if let Ok(configuration) = Configuration::new_from_file() {
        configuration
            .workspaces
            .iter()
            .find(|workspace| workspace.name == name)
            .cloned()
    } else {
        None
    }
}

pub fn save(name: &str, path: &str) {
    if let Ok(mut configuration) = Configuration::new_from_file() {
        configuration.workspaces.push(Workspace {
            name: name.to_string(),
            path: path.to_string(),
        });

        configuration.save_to_file().unwrap();
    } else {
        let mut configuration = Configuration::new();
        configuration.workspaces.push(Workspace {
            name: name.to_string(),
            path: path.to_string(),
        });

        configuration.save_to_file().unwrap();
    }
}

pub fn list() {
    unimplemented!()
}
