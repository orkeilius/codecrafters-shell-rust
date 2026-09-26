use crate::external_command::find_external_command_path;
use crate::internal_command::internal_command::{InternalCommand, COMMAND_LIST};

pub struct Type_;

impl InternalCommand for Type_ {
    fn get_name(&self) -> &'static str {
       "type"
    }

    fn run(&self, args: &[&str]) {
        let command_name = args.join(" ");

        if COMMAND_LIST.iter().any(|command| command.get_name() == &command_name) {
            println!("{command_name} is a shell builtin");
            return;
        }

        if let Some(path) = find_external_command_path(&command_name){
            println!("{command_name} is {path}");
            return;
        }

        println!("{command_name}: not found");
    }
}