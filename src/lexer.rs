#[derive(PartialEq, Clone)]
pub enum Identity {
    Operator, // +, -, /, * etc
    Space, // Just a space denoter
    CParantheses, // )
    OParantheses, // (
    CSquare, // ]
    OSquare, // [
    Comma, // ,
    CCurly, // }
    OCurly, // {
    Newline, // \n
    Equals, // =
    Comparison, // ==, >=, <=, >, <
    Literal, // Any literal like "hi" and 35 and 35.5 and 'g'
    Command, // stl command like print, stop
    Name, // Possible variable and custom function names
    Unknown
}
/* This is all probably unconventional,
** but I wanted to go into this my way with 
** little research on the /correct/ way.
*/
#[derive(Clone)]
pub struct Token {
    pub identity: Identity,
    pub ch: String,
    pub index: u32, // u32 just incase someone goes crazy on a file size
}

fn decide_identity(ch: String) -> Identity {
    use self::Identity::*;
    let ch_slice: &str = &ch[..];
    match ch_slice {
        "{" => OCurly,
        "}" => CCurly,
        "(" => OParantheses,
        ")" => CParantheses,
        "[" => OSquare,
        "]" => CSquare,
        "," => Comma,
        " " => Space,
        "+" | "-" | "*" | "/" => Operator,
        "==" | ">=" | "<=" | ">" | "<" => Comparison,
        "\n" => Newline,
        "=" => Equals,
        _   => Unknown
    }
}

pub fn lex(f: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut commands: Vec<String> = vec!["print".to_string(), "stop".to_string()];

    let mut built_word: Vec<char> = vec![]; // All unknowns to make a literal or word or command
    let mut i = 0;
    for character in f.chars() {
        let _identity = decide_identity(character.clone().to_string());

        if _identity == self::Identity::Unknown {
            built_word.push(character);
            let st: String = built_word.clone().into_iter().collect();
        } else {
            if built_word.len() > 0 {
                let st: String = built_word.clone().into_iter().collect();
                let mut finished = false;
                // Check for command
                for j in commands.clone() {
                    if st.clone() == j {
                        tokens.push(Token {
                            identity: self::Identity::Command,
                            ch: st.clone(),
                            index: i as u32
                        });
                        println!("{} {:?} command", st.clone(), i);
                        built_word.clear();
                        finished = true;
                    }
                }
                if !finished {
                    if built_word[0] == '"' || built_word[0].is_numeric() { // Literals
                        tokens.push(Token {
                            identity: self::Identity::Literal,
                            ch: st.clone(),
                            index: i as u32
                        });
                        println!("{} {:?} literal", st.clone(), i);
                        built_word.clear();
                    } else { // Name
                        tokens.push(Token {
                            identity: self::Identity::Name,
                            ch: st.clone(),
                            index: i as u32
                        });
                        println!("{} {:?} name", st.clone(), i);
                        built_word.clear();
                    }
                    i-=built_word.len()+1;
                }
            } else { // Nothing in the unknown built word, so it's a single character token
                if _identity != self::Identity::Space {
                    tokens.push(Token { // Creates the current 'Token' [full lexed struct of character]
                        identity: _identity.clone(),
                        ch: character.to_string(), 
                        index: i as u32
                    });
                }
            }
        }
        if _identity != self::Identity::Space {
            i+=1;
        }
    }
    for i in tokens.clone() {
        println!("{} {:?}", i.ch, i.index);
    }
    return tokens;
}