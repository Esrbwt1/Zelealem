use crate::token::Token;
use std::collections::HashMap;

pub struct Lexer {
    input: Vec<char>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("fn".to_string(), Token::Function);
        keywords.insert("let".to_string(), Token::Let);

        let mut l = Self {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            keywords,
        };
        l.read_char();
        l
    }

    // Reads the next character and advances our position.
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; // NUL character signifies "end of file"
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Skips over any whitespace.
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    // The main function of the lexer. It reads the current character
    // and returns the corresponding token.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => Token::Bang,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\0' => Token::Eof,
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let ident = self.read_identifier();
                    return self.lookup_ident(&ident);
                } else if self.ch.is_digit(10) {
                    return Token::Int(self.read_number());
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
    }
    
    // Reads a sequence of letters to form an identifier or keyword.
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[start_pos..self.position].iter().collect()
    }

    // Looks up an identifier in the keywords table to see if it's a keyword.
    fn lookup_ident(&self, ident: &str) -> Token {
        if let Some(tok) = self.keywords.get(ident) {
            tok.clone()
        } else {
            Token::Ident(ident.to_string())
        }
    }

    // Reads a sequence of digits to form a number.
    fn read_number(&mut self) -> i64 {
        let start_pos = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        let s: String = self.input[start_pos..self.position].iter().collect();
        s.parse().unwrap()
    }
}