#![deny(clippy::pedantic)]

pub mod external_command;
pub mod internal_command;
pub mod logger;

use crate::external_command::find_external_command_path;
use crate::internal_command::internal_command::get_command;
use crate::logger::{DefaultLogger, Logger};
use std::io::{self, Write};
use std::process::Command;

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
    let args = input.split_whitespace().collect::<Vec<&str>>();

    let command = args.first().unwrap_or(&"");
    if command.is_empty() {
        return;
    }

    if let Some(internal_command) = get_command(command) {
        internal_command.run(&args[1..], &DefaultLogger);
        return;
    }

    run_external_command(args[0], &args[1..]);
}

fn run_external_command(command: &str, arg: &[&str], logger: &dyn Logger) {
    let possible_path = find_external_command_path(command);

    if possible_path.is_none() {
        logger.println(&format!("{}: command not found", command.trim()));
        return;
    }
    let output = Command::new(command).args(arg).output().unwrap();
    logger.println(&output.stdout.join("\n"));
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
