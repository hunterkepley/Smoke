use std::env;
use std::fs::File;
use std::io::prelude::*;

mod parser;
mod lexer;
mod smoke_commands;
// What you were workin on: Getting the ID's to be sequential, then parsing the constructors
fn print_help() {
    println!("Use: Smoke FILE.smo");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { // Makes sure {} in ./smoke {FILE.smo}
        print_help(); // TODO: check if it's a .smo file!
    } else {
        let l = &args[1];
        let mut file = File::open(l).expect("Smoke file not found");
        let mut f = String::new();
        file.read_to_string(&mut f).expect("Unable to read file");

        let tokens: Vec<lexer::Token> = lexer::lex(f.clone()); // Not sure if .clone() is proper
        
        parser::parse(&tokens);
    }
}