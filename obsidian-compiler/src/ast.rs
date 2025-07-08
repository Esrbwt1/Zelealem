use crate::token::Token;

// The AST will be a tree of nodes. Every part of our AST, whether it's
// a statement or an expression, will be a Node.
pub trait Node {
    // Returns the literal value of the token associated with the node.
    // Used for debugging and testing.
    fn token_literal(&self) -> String;
}

// A Statement is a block of code that does not produce a value.
// e.g., `let x = 5;` is a statement.
#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token.token_literal(),
        }
    }
}

// An Expression is a block of code that results in a value.
// e.g., `5`, `x + 10`, `add(5, 10)` are all expressions.
#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token.token_literal(),
            Expression::IntegerLiteral(il) => il.token.token_literal(), 
        }
    }
}

// The root of every AST our parser produces will be a Program node.
// It's just a collection of statements.
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            "".to_string()
        } else {
            self.statements[0].token_literal()
        }
    }
}


// --- Statement Nodes ---

// Represents a `let` statement, e.g., `let my_var = my_expression;`
#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub token: Token, // The `Token::Let` token
    pub name: Identifier,
    pub value: Expression,
}


// --- Expression Nodes ---

// Represents an identifier, e.g., the `x` in `let x = 5;`
#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token, // The `Token::Ident` token
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub token: Token, // The Token::Int token
    pub value: i64,
}