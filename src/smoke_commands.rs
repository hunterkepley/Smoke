use parser;
use std::process;

pub fn print(constructor: Vec<parser::ConstructorPart>) {
    if constructor[0].d_type == parser::DataType::VString {
        println!("{}", constructor[0].string); // TODO: deal with print types
    }
}

pub fn quit(constructor: Vec<parser::ConstructorPart>) {
    process::exit(0);
}