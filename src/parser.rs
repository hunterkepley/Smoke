use lexer;
use smoke_commands;
use std::process;

#[derive(PartialEq, Clone)]
pub enum DataType { // Data types in Smoke!
    VString,
    VChar,
    VInt,
    VFloat
}

#[derive(Clone)]
struct CommandToken { // Command combined with lexer token, assigns the two
    token: lexer::Token,
    command: String,
    constructor: Vec<ConstructorPart>
}

#[derive(Clone)]
pub struct ConstructorPart { // Contains finalized constructor value as string,
    pub d_type: DataType, // It's actual data type
    pub string: String,
}

fn parse_constructor(index: u32, tokens:&[lexer::Token]) -> Vec<ConstructorPart> {
    let start_index = index as usize;
    let mut final_constructor: Vec<ConstructorPart> = vec![];
    let mut empty_constructor = false;
    if tokens[start_index+1 as usize].identity == lexer::Identity::OParantheses {
        for i in (start_index+2 as usize)..(tokens.len() as usize) {
            if tokens[i as usize].identity == lexer::Identity::CParantheses { // End of constructor, break down
                break;
            }
            if tokens[i as usize].identity != lexer::Identity::Comma {
                let mut data_type: DataType = DataType::VString;
                let mut token = tokens[i as usize].clone();
                let mut slice: &str = &token.ch;
                if token.identity == lexer::Identity::Literal {
                    if token.ch.chars().nth(0).unwrap() == '"' || (token.ch.chars().nth(0).unwrap() == '\'' && token.ch.len() > 3) { // string
                        println!("{}", slice);
                        slice = &slice[1..token.clone().ch.len()-1];
                        data_type = DataType::VString;
                    }  else if token.ch.chars().nth(0).unwrap() == '\'' && token.ch.len() == 3 { // char
                        slice = &slice[1..token.clone().ch.len()-1];
                        data_type = DataType::VChar;
                    } else { // int or float
                        if token.ch.contains(".") {
                            data_type = DataType::VFloat;
                        } else {
                            data_type = DataType::VInt;
                        }
                    }
                }
                let c_part = ConstructorPart {
                    d_type: data_type,
                    string: slice.to_string()
                };
                final_constructor.push(c_part);
            }
        }
    }
    return final_constructor;
}

fn decide_smoke_command(comm: String, tok: &lexer::Token, tokens:&[lexer::Token]) -> CommandToken { // For smoke stl commands
    match comm.as_ref() {
        "print" => CommandToken{
            token: tok.clone(), 
            command: comm, 
            constructor: parse_constructor(tok.index, tokens)}, // TODO: Actually add constructor
        "println" => CommandToken{
            token: tok.clone(),
            command: comm,
            constructor: parse_constructor(tok.index, tokens)},
        "stop" => CommandToken{
            token: tok.clone(),
            command: comm,
            constructor: parse_constructor(tok.index, tokens)},
        _ => CommandToken{
            token: tok.clone(),
            command: "unknown".to_string(),
            constructor: parse_constructor(tok.index, tokens)}
    }
}

fn run_smoke_command(tok: CommandToken) -> bool { // Smoke STL commands are ran here
    match tok.command.as_ref() {
        "print" => {
            smoke_commands::print(tok.constructor);
            return true;
        },
        "println" => {
            smoke_commands::println(tok.constructor);
            return true;
        },
        "stop" => {
            smoke_commands::quit(tok.constructor);
            return true;
        }
        _ => false
    }
}

pub fn parse(t: &[lexer::Token]) { // Parses, file and token given
    let mut iterator = 0;
    for i in t {
        iterator+=1;
        if i.identity == lexer::Identity::Command || i.identity == lexer::Identity::Name {
            let c = decide_smoke_command(i.clone().ch, i, t);
            if c.command == "unknown" {
                // TODO: Check for user-made command, else error this out
            } else {
                if t[iterator].identity != lexer::Identity::OParantheses {
                    println!("Error, command {} without ( )", c.command); // TODO: MAKE THIS CHECK FOR A CLOSING PARANTHESES.
                    // Make better, idiomatic errors later, including line number
                    process::exit(0);
                }
                run_smoke_command(c.clone());
            }
        }
    }
}