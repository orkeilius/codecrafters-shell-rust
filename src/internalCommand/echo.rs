use crate::internalCommand::internal_command::InternalCommand;

pub struct Echo;

impl InternalCommand for Echo {
    fn get_name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, arg: &[&str]) {
        println!("{}", arg.join(" "))
    }

}