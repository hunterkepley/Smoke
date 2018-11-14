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
    if tokens[start_index].identity == lexer::Identity::OParantheses {
        let mut chars_collected: Vec<char> = vec![];
        for i in start_index..tokens.len() {
            if tokens[i].identity != lexer::Identity::OParantheses {
                if tokens[i].identity == lexer::Identity::CParantheses {
                    if chars_collected.len() == 0 {
                        // There's nothing in the constructor
                        empty_constructor = true;
                    }
                    break; // Escape when command is closed
                    // TODO: Allow parantheses for functions and math operations and such, make a counter or something
                }
                chars_collected.push(tokens[i].ch);
            }
        }
        // Decide data type and add to constructor vec
        let mut constructor_part = ConstructorPart { // Assume it's a string for setting this up
            d_type: DataType::VString,
            string: "".to_string(),
        };
        if !empty_constructor {
            if chars_collected[0] == '"' || chars_collected[0] == '\"' { // I doubt the '\"' is needed, but until I do testing it's here to stay
                // It's a string
                let chars_collected_length = chars_collected.len();
                chars_collected.remove(0);chars_collected.remove(chars_collected_length-2); // remove quotes
                constructor_part.string = chars_collected.into_iter().collect();
            } else if chars_collected[0].is_numeric() {
                // It's a int/float
                if chars_collected.contains(&'.') {
                    // Float!
                    constructor_part.d_type = DataType::VFloat;
                    constructor_part.string = chars_collected.into_iter().collect();
                } else {
                    // Int!
                    constructor_part.d_type = DataType::VInt;
                    constructor_part.string = chars_collected.into_iter().collect();
                }
            }
        }
        final_constructor.push(constructor_part);
    }
    return final_constructor;
}

fn decide_smoke_command(comm: String, tok: &lexer::Token, tokens:&[lexer::Token]) -> CommandToken { // For smoke stl commands
    match comm.as_ref() {
        "print" => CommandToken{
            token: tok.clone(), 
            command: comm, 
            constructor: parse_constructor(tok.index, tokens)}, // TODO: Actually add constructor
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
        "stop" => {
            smoke_commands::quit(tok.constructor);
            return true;
        }
        _ => false
    }
}

pub fn parse(t: &[lexer::Token]) { // Parses, file and token given
    let mut temp_check: Vec<char> = vec![];
    // List of every command saved in the program including smoke stl commands
    //let commands: Vec<CommandToken> = vec![];
    let mut iterator = 0;
    for i in t {
        iterator+=1;
        if i.identity == lexer::Identity::Unknown {
            if i.ch.is_alphanumeric() { // TODO: Make sure that command[0] isn't numeric!
                temp_check.push(i.ch);
            } else {
                // TODO: Error this out -- not A-z/1-0
            }
        } else {
            let temp_s = temp_check.into_iter().collect(); // String of the maybe command
            let c = decide_smoke_command(temp_s, i, t);
            if c.command == "unknown" {
                // TODO: Check for user-made command, else error this out
            } else {
                if t[iterator-1].identity != lexer::Identity::OParantheses {
                    println!("Error, command {} without ( )", c.command); // TODO: MAKE THIS CHECK FOR A CLOSING PARANTHESES.
                    // Make better, idiomatic errors later, including line number
                    process::exit(0);
                }
                run_smoke_command(c.clone());
            }
            temp_check = vec![]; // Empty the temp command check vec
        }
    }
}