use obsidian_compiler::{
    ast::{Node, Statement},
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

    // Check for any parsing errors.
    check_parser_errors(&parser);

    // The program should have 3 statements.
    assert_eq!(program.statements.len(), 3, "program.statements does not contain 3 statements.");

    let expected_identifiers = vec!["x", "y", "foobar"];

    for (i, expected_ident) in expected_identifiers.iter().enumerate() {
        let stmt = &program.statements[i];
        assert_let_statement(stmt, expected_ident);
    }
}

// Helper function to assert that a statement is a valid LetStatement.
fn assert_let_statement(s: &Statement, name: &str) {
    assert_eq!(s.token_literal(), "Let", "s.token_literal not 'let'");

    if let Statement::Let(let_stmt) = s {
        assert_eq!(let_stmt.name.value, name, "let_stmt.name.value not '{}'", name);
        assert_eq!(let_stmt.name.token.token_literal(), name, "let_stmt.name.token.token_literal() not '{}'", name);
    } else {
        panic!("s not Statement::Let. got={:?}", s);
    }
}

// Helper function to check for and report any errors the parser collected.
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