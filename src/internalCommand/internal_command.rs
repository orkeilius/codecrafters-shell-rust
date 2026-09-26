use crate::internalCommand::cd::Cd;
use crate::internalCommand::type_::{Type_};
use crate::internalCommand::echo::Echo;
use crate::internalCommand::exit::Exit;
use crate::internalCommand::pwd::Pwd;

pub trait InternalCommand {
    fn get_name(&self) -> &'static str;
    fn run(&self,args: &[&str]);
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



