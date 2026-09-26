use std::env;
use std::path::{Path, PathBuf};
use crate::internalCommand::internal_command::InternalCommand;

pub struct Cd;

impl InternalCommand for Cd {
    fn get_name(&self) -> &'static str {
        "cd"
    }

    fn run(&self, arg: &[&str]) {
        let raw_path = arg.first().unwrap_or(&"");
        if let Err(_) = change_dir(raw_path) {
            println!("cd: {}: No such file or directory", raw_path);
        }
    }
}


fn change_dir(raw_path: &str) -> std::io::Result<()> {
    let path = Path::new(raw_path).canonicalize()?;
    env::set_current_dir(path)
}
