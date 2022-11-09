use crate::configuration::Configuration;
use crate::configuration::Workspace as WorkspaceConfiguration;

pub fn go(name: &str) -> Option<String> {
    let configuration = Configuration::new();
    for workspace in configuration.workspaces.iter() {
        if workspace.name == name {
            return Some(workspace.path.clone());
        }
    }

    return None;
}

pub fn save(name: &str, path: &str) {
    let mut configuration = Configuration::new();
    configuration.workspaces.push(WorkspaceConfiguration {
        name: name.to_string(),
        path: path.to_string(),
    });
    configuration.save_file().unwrap();
}

pub fn list() {
    unimplemented!()
}
