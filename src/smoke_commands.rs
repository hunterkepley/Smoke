use parser;
use std::process;

pub fn print(constructor: Vec<parser::ConstructorPart>) {
    for i in constructor {
        println!("{}", i.string);
    }
}

pub fn quit(constructor: Vec<parser::ConstructorPart>) {
    process::exit(0);
}