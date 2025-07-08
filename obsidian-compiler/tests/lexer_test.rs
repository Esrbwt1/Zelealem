// We need to tell Rust that we are using the obsidian_compiler library.
use obsidian_compiler::{
    lexer::Lexer,
    token::Token,
};

#[test]
fn test_next_token() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!*-/
";

    // This is the expected sequence of tokens our lexer should produce.
    let expected_tokens = vec![
        Token::Let,
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::RParen,
        Token::LBrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::LParen,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::RParen,
        Token::Semicolon,
        Token::Bang,
        Token::Asterisk,
        Token::Minus,
        Token::Slash,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(input);

    for expected_token in expected_tokens {
        let received_token = lexer.next_token();
        println!("Expected: {:?}, Received: {:?}", expected_token, received_token);
        assert_eq!(expected_token, received_token);
    }
}