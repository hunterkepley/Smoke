use lexer;
use smoke_commands;

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
    let final_constructor: Vec<ConstructorPart> = vec![];
    if tokens[start_index].identity == lexer::Identity::OParantheses {
        let mut chars_collected: Vec<char> = vec![];
        for i in start_index..tokens.len() {
            if tokens[i].identity == lexer::Identity::CParantheses {
            } else {
                if tokens[i].identity != lexer::Identity::OParantheses {
                    chars_collected.push(tokens[i].ch);
                }
            }
        }
        // Decide data type and add to constructor vec
        let mut constructor_part = ConstructorPart { // Assume it's a string for setting this up
            d_type: DataType::VString,
            string: "".to_string(),
        };
        if chars_collected[0] == '"' || chars_collected[0] == '\"' {
            // It's a string
            let chars_collected_length = chars_collected.len();
            chars_collected.remove(0);chars_collected.remove(chars_collected_length-1);
            constructor_part.string = chars_collected.into_iter().collect();
        }
        println!("{}", constructor_part.string);
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

    for i in t {
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
                run_smoke_command(c.clone());
            }
            temp_check = vec![]; // Empty the temp command check vec
        }
    }
}