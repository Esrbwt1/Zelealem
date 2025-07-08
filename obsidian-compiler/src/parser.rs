use crate::ast::{Identifier, LetStatement, Program, Statement, Expression}; // Updated imports
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>, // A list of parsing errors
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
        }
    }
    
    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement().map(Statement::Let),
            _ => None,
        }
    }
    
    // This is the new, complete implementation.
    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let let_token = self.cur_token.clone(); // `let` token

        // We expect an identifier next, e.g., `let x ...`
        if !self.expect_peek(Token::Ident("".to_string())) { // The content doesn't matter, just the type
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(), // The identifier token
            value: match &self.cur_token {
                Token::Ident(s) => s.clone(),
                _ => return None, // Should not happen due to expect_peek
            },
        };

        // After the identifier, we expect an equals sign, e.g., `let x = ...`
        if !self.expect_peek(Token::Assign) {
            return None;
        }
        
        // TODO: For now, we are skipping the expression until we implement expression parsing.
        while self.cur_token != Token::Semicolon {
            self.next_token();
        }
        
        // Placeholder expression
        let value = Expression::Identifier(Identifier {
            token: Token::Ident("DUMMY".to_string()),
            value: "DUMMY".to_string(),
        });

        Some(LetStatement {
            token: let_token,
            name,
            value,
        })
    }

    // Helper function to check if the next token is what we expect.
    fn cur_token_is(&self, t: &Token) -> bool {
        std::mem::discriminant(&self.cur_token) == std::mem::discriminant(t)
    }

    // Helper function to check if the "peek" token is what we expect.
    fn peek_token_is(&self, t: &Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(t)
    }
    
    // An assertion function that advances the tokens only if the peek token
    // is of the correct type. This is a core pattern in parsing.
    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }
    
    // Adds an error to our list when we encounter an unexpected token.
    fn peek_error(&mut self, t: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token
        );
        self.errors.push(msg);
    }
}