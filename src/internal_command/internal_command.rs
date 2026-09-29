use crate::internal_command::cd::Cd;
use crate::internal_command::type_::{Type_};
use crate::internal_command::echo::Echo;
use crate::internal_command::exit::Exit;
use crate::internal_command::pwd::Pwd;
use crate::logger::Logger;

pub trait InternalCommand {
    fn get_name(&self) -> &'static str;
    fn run(&self,args: &[&str],logger: &dyn Logger);
}

pub const COMMAND_LIST: &[&dyn InternalCommand] = &[
    &Cd,
    &Echo,
    &Exit,
    &Pwd,
    &Type_,
];

pub fn get_command(name: &str) -> Option<&'static dyn InternalCommand> {
    COMMAND_LIST.iter()
        .find(|command| command.get_name() == name)
        .copied()
}



