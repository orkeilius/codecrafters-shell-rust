#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    prompt();


}

fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
        Ok(_) => {
            parse_input(input);
        }
    }
}

fn parse_input(input: String) {

    println!("{}: command not found",input.trim());
}