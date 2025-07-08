use crate::ast::{Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            peek_token,
        }
    }

    // Advances the parser's tokens.
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // The main entry point for the parser. It iterates through the
    // token stream and parses statements until it hits the end.
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

    // The router for parsing different types of statements.
    // For now, it only knows about `let` statements.
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            _ => None, // We don't know how to parse anything else yet.
        }
    }

    // Parses a `let` statement.
    // A `let` statement must follow the pattern: `let <identifier> = <expression>;`
    fn parse_let_statement(&mut self) -> Option<Statement> {
        // The current token is `Let`. We expect the next token to be an identifier.
        // We will implement the logic to check this in the next step.

        // TODO: For now, we just skip to the semicolon to consume the statement.
        while self.cur_token != Token::Semicolon && self.cur_token != Token::Eof {
            self.next_token();
        }

        // We return None because we haven't actually built the AST node yet.
        None
    }
}