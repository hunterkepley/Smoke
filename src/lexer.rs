#[derive(PartialEq, Clone)]
pub enum Identity {
    Operator, // +, +=, ==, etc
    Space, // Just a space denoter
    CParantheses, // )
    OParantheses, // (
    CSquare, // ]
    OSquare, // [
    Comma, // ,
    CCurly, // }
    OCurly, // {
    Newline, // \n
    Unknown
}
/* This is all probably unconventional,
** but I wanted to go into this my way with 
** little research on the /correct/ way.
*/
#[derive(Clone)]
pub struct Token {
    pub identity: Identity,
    pub ch: char,
    pub index: u32, // u32 just incase someone goes crazy on a file size
}

fn decide_identity(ch: char) -> Identity {
    use self::Identity::*;
    match ch {
        '{' => OCurly,
        '}' => CCurly,
        '(' => OParantheses,
        ')' => CParantheses,
        '[' => OSquare,
        ']' => CSquare,
        ',' => Comma,
        ' ' => Space,
        '+' | '-' | '*' => Operator,
        '\n' => Newline,
        _   => Unknown
    }
}

pub fn lex(f: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    for (i, character) in f.chars().enumerate() {
        tokens.push(Token { // Creates the current 'Token' [full lexed struct of character]
            identity: decide_identity(character),
            ch: character, 
            index: i as u32
        });
    }
    return tokens;
}