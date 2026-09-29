use crate::internal_command::internal_command::InternalCommand;
use crate::logger::Logger;

pub struct Echo;

impl InternalCommand for Echo {
    fn get_name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, arg: &[&str],logger: &dyn Logger) {
        logger.println(&format!("{}", arg.join(" ")));
    }

}