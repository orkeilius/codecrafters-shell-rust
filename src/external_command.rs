use std::env;
use std::fs::File;
use std::path::{PathBuf};
use std::os::unix::fs::PermissionsExt;

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
    if !theorical_path.exists()  {
        return None
    }
    let metadata = File::open(theorical_path.to_str()?).ok()?.metadata().ok()?;
    let is_executable =  metadata.permissions().mode() & 0o111 != 0;
    if !is_executable {return None}

    Some(theorical_path)
}