use super::Diagnostic;

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
