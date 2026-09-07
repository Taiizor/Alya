use super::token::TokenType;
use super::Lexer;

#[test]
fn test_tokenize_numbers() {
    let source = "42 3.75 0 100";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    assert_eq!(tokens[0].token_type, TokenType::Number(42.0));
    assert_eq!(tokens[1].token_type, TokenType::Number(3.75));
    assert_eq!(tokens[2].token_type, TokenType::Number(0.0));
    assert_eq!(tokens[3].token_type, TokenType::Number(100.0));
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_keywords() {
    let source = "say let if else while for in function end return break continue";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    let expected = vec![
        TokenType::Say,
        TokenType::Let,
        TokenType::If,
        TokenType::Else,
        TokenType::While,
        TokenType::For,
        TokenType::In,
        TokenType::Function,
        TokenType::End,
        TokenType::Return,
        TokenType::Break,
        TokenType::Continue,
        TokenType::Eof,
    ];

    let actual: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_tokenize_identifiers() {
    let source = "my_var totalCount x1 _private";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    assert_eq!(tokens[0].token_type, TokenType::Identifier("my_var".into()));
    assert_eq!(
        tokens[1].token_type,
        TokenType::Identifier("totalCount".into())
    );
    assert_eq!(tokens[2].token_type, TokenType::Identifier("x1".into()));
    assert_eq!(
        tokens[3].token_type,
        TokenType::Identifier("_private".into())
    );
}

#[test]
fn test_tokenize_operators() {
    let source = "+ - * / % == != < <= > >= and or not =";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    let expected = vec![
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Multiply,
        TokenType::Divide,
        TokenType::Modulo,
        TokenType::Equal,
        TokenType::NotEqual,
        TokenType::Less,
        TokenType::LessEqual,
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::And,
        TokenType::Or,
        TokenType::Not,
        TokenType::Assign,
        TokenType::Eof,
    ];

    let actual: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_tokenize_strings_and_escapes() {
    let source = r#""Hello, World!" "Line1\nLine2\t\"quote\"""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    assert_eq!(
        tokens[0].token_type,
        TokenType::String("Hello, World!".into())
    );
    assert_eq!(
        tokens[1].token_type,
        TokenType::String("Line1\nLine2\t\"quote\"".into())
    );
}

#[test]
fn test_tokenize_comments() {
    let source = "# This is a comment\nsay 42 # inline comment\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    let types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
    assert_eq!(
        types,
        vec![
            TokenType::Newline,
            TokenType::Say,
            TokenType::Number(42.0),
            TokenType::Newline,
            TokenType::Eof,
        ]
    );
}

#[test]
fn test_tokenize_unexpected_character() {
    let source = "say @bad";
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character '@'"));
}

#[test]
fn test_tokenize_slash_and_multiline_comments() {
    let source = "// single line\nsay 10 /* inline multiline */ + 20\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    let types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
    assert_eq!(
        types,
        vec![
            TokenType::Newline,
            TokenType::Say,
            TokenType::Number(10.0),
            TokenType::Plus,
            TokenType::Number(20.0),
            TokenType::Newline,
            TokenType::Eof,
        ]
    );
}

#[test]
fn test_tokenize_compound_and_logical_operators() {
    let source = "+= -= *= /= && || ! elif";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    let types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
    assert_eq!(
        types,
        vec![
            TokenType::PlusAssign,
            TokenType::MinusAssign,
            TokenType::MultiplyAssign,
            TokenType::DivideAssign,
            TokenType::And,
            TokenType::Or,
            TokenType::Not,
            TokenType::Elif,
            TokenType::Eof,
        ]
    );
}

#[test]
fn test_tokenize_brackets() {
    let source = "[1, 2, 3] arr[0]";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization failed");

    let types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
    assert_eq!(
        types,
        vec![
            TokenType::LeftBracket,
            TokenType::Number(1.0),
            TokenType::Comma,
            TokenType::Number(2.0),
            TokenType::Comma,
            TokenType::Number(3.0),
            TokenType::RightBracket,
            TokenType::Identifier("arr".into()),
            TokenType::LeftBracket,
            TokenType::Number(0.0),
            TokenType::RightBracket,
            TokenType::Eof,
        ]
    );
}
