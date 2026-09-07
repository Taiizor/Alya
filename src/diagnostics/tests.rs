use super::*;

#[test]
fn test_parse_error_location() {
    let (msg, line, col) = parse_error_location("Expected identifier after 'let' at line 3, column 5");
    assert_eq!(msg, "Expected identifier after 'let'");
    assert_eq!(line, Some(3));
    assert_eq!(col, Some(5));

    let (msg, line, col) = parse_error_location("Unexpected character '&' at line 5, column 10. Did you mean '&&'?");
    assert_eq!(msg, "Unexpected character '&'. Did you mean '&&'?");
    assert_eq!(line, Some(5));
    assert_eq!(col, Some(10));

    let (msg, line, col) = parse_error_location("Unclosed comment starting at line 1, column 1");
    assert_eq!(msg, "Unclosed comment");
    assert_eq!(line, Some(1));
    assert_eq!(col, Some(1));

    let (msg, line, col) = parse_error_location("Expected 'end' at line 4");
    assert_eq!(msg, "Expected 'end'");
    assert_eq!(line, Some(4));
    assert_eq!(col, None);
}

#[test]
fn test_render_diagnostic() {
    let source = "let a = 10\nlet = 20\nsay a\n";
    let diag = Diagnostic::new("test.alya", "Expected identifier after 'let'")
        .with_location(2, 5);

    let rendered = diag.render(source);
    assert!(rendered.contains("error: Expected identifier after 'let'"));
    assert!(rendered.contains("--> test.alya:2:5"));
    assert!(rendered.contains("2 | let = 20"));
    assert!(rendered.contains("  |     ^"));
}

#[test]
fn test_render_error_end_to_end() {
    let source = "let x = 10\nsay @bad\n";
    let err_msg = "Unexpected character '@' at line 2, column 5";
    let rendered = render_error("main.alya", source, err_msg);

    assert!(rendered.contains("error: Unexpected character '@'"));
    assert!(rendered.contains("--> main.alya:2:5"));
    assert!(rendered.contains("2 | say @bad"));
    assert!(rendered.contains("  |     ^"));
}
