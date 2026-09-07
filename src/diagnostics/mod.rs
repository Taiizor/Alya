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

pub mod parser;
#[cfg(test)]
mod tests;

pub use parser::{parse_error_location, render_error};

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
