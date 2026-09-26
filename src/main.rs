#![deny(clippy::pedantic)]

pub mod external_command;

use std::io::{self, Write};
use std::process::Command;
use crate::external_command::find_external_command_path;

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

    let arg = input.trim().split_whitespace().collect::<Vec<&str>>();

    match arg[0]{
        "exit" => std::process::exit(0),
        "echo" => println!("{}", arg[1..].join(" ")),
        "type" => type_command(&arg),
        _ => run_external_command(arg[0],&arg[1..])
    }

}

const LIST_OF_BUILTIN_COMMAND: [&str; 3] = ["exit" ,"echo","type"];
fn type_command(arg: &[&str]){
    let command_name = arg[1..].join(" ");

    if LIST_OF_BUILTIN_COMMAND.contains(&command_name.as_str()){
        println!("{command_name} is a shell builtin");
        return;
    }

    let possible_path = find_external_command_path(&command_name);
    match possible_path {
         Some(path) => println!("{command_name} is {path}"),
         _ => println!("{command_name}: not found")
     }
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

