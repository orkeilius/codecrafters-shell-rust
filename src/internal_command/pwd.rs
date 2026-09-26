use std::env;
use crate::internal_command::internal_command::InternalCommand;

pub struct Pwd;

impl InternalCommand for Pwd {
    fn get_name(&self) -> &'static str {
       "pwd"
    }

    fn run(&self, _: &[&str]) {
        println!("{}", env::current_dir().unwrap_or_default().display()) ;
    }
}