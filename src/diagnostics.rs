//! Rich compiler diagnostics for Alya.
//!
//! Formats errors in the style of modern compilers (e.g. rustc, clang):
//!
//! ```text
//! error: Expected identifier after 'let'
//!   --> file.alya:3:9
//!    |
//!  3 |     let = 10
//!    |         ^
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub file: String,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub length: usize,
}

impl Diagnostic {
    pub fn new(file: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            message: message.into(),
            line: None,
            column: None,
            length: 1,
        }
    }

    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length.max(1);
        self
    }

    /// Render this diagnostic against the given source code.
    pub fn render(&self, source: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("error: {}\n", self.message));

        if let Some(line) = self.line {
            let col_str = self.column.map(|c| format!(":{}", c)).unwrap_or_default();
            out.push_str(&format!("  --> {}:{}{}\n", self.file, line, col_str));

            let lines: Vec<&str> = source.lines().collect();
            if line > 0 && line <= lines.len() {
                let line_str = lines[line - 1];
                let width = line.to_string().len().max(1);

                out.push_str(&format!("{:>width$} |\n", "", width = width));

                let display_line = line_str.replace('\t', "    ");
                out.push_str(&format!(" {:>width$} | {}\n", line, display_line, width = width));

                if let Some(column) = self.column {
                    let mut visual_col = 0;
                    for (idx, ch) in line_str.chars().enumerate() {
                        if idx + 1 == column {
                            break;
                        }
                        if ch == '\t' {
                            visual_col += 4;
                        } else {
                            visual_col += 1;
                        }
                    }

                    let remaining = if column <= line_str.len() {
                        &line_str[column - 1..]
                    } else {
                        ""
                    };

                    let token_len = remaining
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_' || "+-*/%=<>!&|".contains(*c))
                        .count()
                        .max(1);

                    let length = self.length.max(token_len);
                    let caret = "^".repeat(length);

                    out.push_str(&format!(
                        " {:>width$} | {}{}",
                        "",
                        " ".repeat(visual_col),
                        caret,
                        width = width
                    ));
                }
            }
        } else {
            out.push_str(&format!("  --> {}", self.file));
        }

        out
    }
}

/// Parses error message strings that embed line and column information.
///
/// Handles patterns like:
/// - `"Expected identifier after 'let' at line 3, column 5"`
/// - `"Unexpected character '&' at line 5, column 10. Did you mean '&&'?"`
/// - `"Unclosed multiline comment starting at line 1, column 1"`
/// - `"Expected 'end' at line 14"`
pub fn parse_error_location(msg: &str) -> (String, Option<usize>, Option<usize>) {
    let marker = if let Some(idx) = msg.find(" starting at line ") {
        Some((idx, " starting at line ".len()))
    } else {
        msg.find(" at line ").map(|idx| (idx, " at line ".len()))
    };

    if let Some((marker_start, marker_len)) = marker {
        let prefix = &msg[..marker_start];
        let rest = &msg[marker_start + marker_len..];

        let line_digits_len = rest.chars().take_while(|c| c.is_ascii_digit()).count();
        if line_digits_len > 0 {
            if let Ok(line_num) = rest[..line_digits_len].parse::<usize>() {
                let after_line = &rest[line_digits_len..];

                if let Some(col_rest) = after_line.strip_prefix(", column ") {
                    let col_digits_len = col_rest.chars().take_while(|c| c.is_ascii_digit()).count();
                    if col_digits_len > 0 {
                        if let Ok(col_num) = col_rest[..col_digits_len].parse::<usize>() {
                            let suffix = &col_rest[col_digits_len..];
                            let clean_msg = format!("{}{}", prefix, suffix);
                            return (clean_msg, Some(line_num), Some(col_num));
                        }
                    }
                }

                let clean_msg = format!("{}{}", prefix, after_line);
                return (clean_msg, Some(line_num), None);
            }
        }
    }

    (msg.to_string(), None, None)
}

/// Renders a diagnostic from an error message string and file/source context.
pub fn render_error(file: &str, source: &str, err_msg: &str) -> String {
    let (clean_msg, line, col) = parse_error_location(err_msg);
    let mut diag = Diagnostic::new(file, clean_msg);
    diag.line = line;
    diag.column = col;
    diag.render(source)
}

#[cfg(test)]
mod tests {
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
}

