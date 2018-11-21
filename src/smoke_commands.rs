use parser;
use std::process;

pub fn print(constructor: Vec<parser::ConstructorPart>) {
    for i in constructor {
        print!("{}", i.string);
    }
}

pub fn println(constructor: Vec<parser::ConstructorPart>) {
    for i in constructor {
        print!("{}", i.string);
    }
    println!();
}

pub fn quit(constructor: Vec<parser::ConstructorPart>) {
    process::exit(0);
}