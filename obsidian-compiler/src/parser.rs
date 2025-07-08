use crate::ast::{Identifier, IntegerLiteral, LetStatement, Program, Statement, Expression};
use crate::lexer::Lexer;
use crate::token::Token;

// A type alias for our parsing functions for clarity.
type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
// We will add infix functions (like for `+`) later.

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
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
        // ... this function is unchanged ...
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
            _ => None, // Will be replaced by ExpressionStatement later
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let let_token = self.cur_token.clone();

        if !self.expect_peek(Token::Ident("".to_string())) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: match &self.cur_token {
                Token::Ident(s) => s.clone(),
                _ => return None,
            },
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }
        
        self.next_token(); // Move to the expression token

        // THIS IS THE UPGRADED PART
        // We now call a generic expression parser instead of skipping.
        let value = self.parse_expression().unwrap(); // For now, we unwrap

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(LetStatement {
            token: let_token,
            name,
            value,
        })
    }
    
    // --- NEW EXPRESSION PARSING LOGIC ---
    
    fn parse_expression(&mut self) -> Option<Expression> {
        // This is the core of our expression parser. It looks at the current
        // token and calls the appropriate parsing function.
        let prefix_fn = self.get_prefix_fn(&self.cur_token);

        prefix_fn.map(|f| f(self)).flatten()
    }
    
    // Returns the function needed to parse an expression based on the token type.
    fn get_prefix_fn(&self, token: &Token) -> Option<PrefixParseFn> {
        match token {
            Token::Ident(_) => Some(Self::parse_identifier),
            Token::Int(_) => Some(Self::parse_integer_literal),
            _ => None,
        }
    }
    
    // The parsing function for identifiers.
    fn parse_identifier(parser: &mut Parser) -> Option<Expression> {
        let token = parser.cur_token.clone();
        let value = match &token {
            Token::Ident(s) => s.clone(),
            _ => return None,
        };
        Some(Expression::Identifier(Identifier { token, value }))
    }

    // The parsing function for integer literals.
    fn parse_integer_literal(parser: &mut Parser) -> Option<Expression> {
        let token = parser.cur_token.clone();
        let value = match token {
            Token::Int(v) => v,
            _ => {
                parser.errors.push(format!("could not parse {:?} as integer", token));
                return None;
            }
        };
        Some(Expression::IntegerLiteral(IntegerLiteral { token, value }))
    }
    
    // --- HELPER FUNCTIONS (mostly unchanged) ---
    
    fn cur_token_is(&self, t: &Token) -> bool {
        std::mem::discriminant(&self.cur_token) == std::mem::discriminant(t)
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(t)
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token
        );
        self.errors.push(msg);
    }
}