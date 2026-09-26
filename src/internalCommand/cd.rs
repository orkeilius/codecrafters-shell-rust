use std::env;
use std::path::Path;
use crate::internalCommand::internal_command::InternalCommand;

pub struct Cd;

impl InternalCommand for Cd {
    fn get_name(&self) -> &'static str {
        "cd"
    }

    fn run(&self, arg: &[&str]) {
        let path = Path::new(arg.first().unwrap_or(&""));

        if let Err(_) =  env::set_current_dir(path) {
            println!("cd: {}: No such file or directory", path.to_str().unwrap_or_default());
        }
    }
}