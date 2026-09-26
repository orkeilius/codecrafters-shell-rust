#![deny(clippy::pedantic)]

use std::io::{self, Write};

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

    let arg = input.trim().split(' ').collect::<Vec<&str>>();

    match arg[0]{
        "exit" => std::process::exit(0),
        "echo" => println!("{}", arg[1..].join(" ")),
        _ => println!("{}: command not found",input.trim())
    }

}