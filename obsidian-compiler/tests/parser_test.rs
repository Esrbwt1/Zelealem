use obsidian_compiler::{
    ast::{Expression, Node, Statement},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_let_statements() {
    let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 3, "program.statements does not contain 3 statements.");

    let expected_identifiers = vec!["x", "y", "foobar"];

    for (i, expected_ident) in expected_identifiers.iter().enumerate() {
        let stmt = &program.statements[i];
        assert_let_statement(stmt, expected_ident);
    }
}

fn assert_let_statement(s: &Statement, name: &str) {
    assert_eq!(s.token_literal(), "Let");

    if let Statement::Let(let_stmt) = s {
        assert_eq!(let_stmt.name.value, name);
        assert_eq!(let_stmt.name.token.token_literal(), name);
    } else {
        panic!("s not Statement::Let. got={:?}", s);
    }
}

fn check_parser_errors(parser: &Parser) {
    let errors = parser.errors();
    if errors.is_empty() {
        return;
    }

    println!("\nParser has {} errors", errors.len());
    for msg in errors {
        println!("Parser error: {}", msg);
    }
    panic!("Failing test due to parser errors.");
}

#[test]
fn test_let_statement_with_integer_literal() {
    let input = "let x = 5;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    if let Statement::Let(let_stmt) = &program.statements[0] {
        if let Expression::IntegerLiteral(literal) = &let_stmt.value {
            assert_eq!(literal.value, 5);
            assert_eq!(literal.token.token_literal(), "Int(5)");
        } else {
            panic!("expression not IntegerLiteral. got={:?}", let_stmt.value);
        }
    } else {
        panic!("statement not Statement::Let. got={:?}", program.statements[0]);
    }
}

#[test]
fn test_let_statement_with_identifier() {
    let input = "let my_var = another_var;";
    let lexer = Lexer::new(input); // CORRECTED
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    if let Statement::Let(let_stmt) = &program.statements[0] {
        assert_eq!(let_stmt.name.value, "my_var");
        if let Expression::Identifier(ident) = &let_stmt.value {
            assert_eq!(ident.value, "another_var");
            assert_eq!(ident.token.token_literal(), "another_var");
        } else {
            panic!("expression not Identifier. got={:?}", let_stmt.value);
        }
    } else {
        panic!("statement not Statement::Let. got={:?}", program.statements[0]);
    }
}