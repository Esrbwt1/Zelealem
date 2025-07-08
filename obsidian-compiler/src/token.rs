// The set of all possible tokens in the Obsidian language.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Special Tokens
    Illegal, // Represents a token we don't recognize
    Eof,     // End of File

    // Identifiers + Literals
    Ident(String), // my_variable, function_name
    Int(i64),      // 12345

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /

    // Delimiters
    Comma,     // ,
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }

    // Keywords
    Function,
    Let,
}

impl Token {
    pub fn token_literal(&self) -> String {
        match self {
            Token::Ident(s) => s.clone(),
            _ => format!("{:?}", self), // Default representation for others
        }
    }
}