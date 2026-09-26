#![deny(clippy::pedantic)]

pub mod external_command;
pub mod internalCommand;

use crate::external_command::find_external_command_path;
use std::io::{self, Write};
use std::process::Command;
use crate::internalCommand::internal_command::get_command;

fn main() {
    loop {
        prompt();
    }
}

fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(e) => {
            eprintln!("Error reading input: {e}");
        }
        Ok(_) => {
            parse_input(&input);
        }
    }
}

fn parse_input(input: &str) {

    let args = input.trim().split_whitespace().collect::<Vec<&str>>();

    let command = args.get(0).unwrap_or(&"");
    if command.is_empty() {
        return;
    }

    if let Some(internal_command) = get_command(command) {
        internal_command.run(&args[1..]);
        return;
    }

    run_external_command(args[0],&args[1..])

}


fn run_external_command(command: &str, arg: &[&str]){

    let possible_path = find_external_command_path(&command);

    if possible_path.is_none() {
        println!("{}: command not found",command.trim());
        return;
    }
    let output = Command::new(command).args(arg).output().unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

