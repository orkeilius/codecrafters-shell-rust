use crate::internal_command::internal_command::InternalCommand;
use std::env;
use std::io::ErrorKind::{InvalidData, NotFound};
use std::path::Path;
use crate::logger::Logger;

pub struct Cd;

impl InternalCommand for Cd {
    fn get_name(&self) -> &'static str {
        "cd"
    }

    fn run(&self, arg: &[&str],logger: &dyn Logger) {
        let raw_path = arg.first().unwrap_or(&"");
        if change_dir(raw_path).is_err() {
            logger.println(&format!("cd: {raw_path}: No such file or directory"));
        }
    }
}

fn change_dir(raw_path: &str) -> std::io::Result<()> {

    let templated_path = add_home_on_tilde(raw_path)?;

    let path = Path::new(&templated_path).canonicalize()?;
    env::set_current_dir(path)
}

fn add_home_on_tilde(path: &str) -> std::io::Result<String> {
    if !path.starts_with('~') {
        return Ok(path.to_string());
    }

    let home_dir = env::home_dir().ok_or(NotFound)?.to_str().ok_or(InvalidData)?.to_string();
    let path_without_tilde = &path.get(1..).unwrap_or("").to_string();

    Ok(format!("{home_dir}{path_without_tilde}"))

}
