use std::env;
use std::path::{PathBuf};

pub fn find_external_command_path(name: &str) -> Option<String> {
    for path in get_path_dirs() {

        let found = find_command_in_paths(name, path);
        if found.is_some() {
            return Some(found?.to_str()?.to_string());
        }
    }
    None
}

fn get_path_dirs() -> Vec<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();

    path.split(':').map(PathBuf::from).collect()
}

fn find_command_in_paths(command: &str, path: PathBuf) -> Option<PathBuf> {

    let theorical_path = path.join(command);
    if theorical_path.exists() {
        return Some(theorical_path);
    }
    None
}