use crate::internal_command::internal_command::InternalCommand;
use crate::logger::Logger;

pub struct Exit;

impl InternalCommand for Exit {
    fn get_name(&self) -> &'static str {
        "exit"
    }

    fn run(&self, _: &[&str],_: &dyn Logger) {
        std::process::exit(0)
    }
}