use crate::internal_command::internal_command::InternalCommand;

pub struct Exit;

impl InternalCommand for Exit {
    fn get_name(&self) -> &'static str {
        "exit"
    }

    fn run(&self, _: &[&str]) {
        std::process::exit(0)
    }
}