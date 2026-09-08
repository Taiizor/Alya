use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockKind {
    Function,
    If,
    While,
    For,
    Repeat,
    Try,
    Struct,
    When,
    WhenArm,
}

fn strip_line_comment(line: &str) -> &str {
    let mut in_str = false;
    let mut quote = '"';
    let mut escaped = false;
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == quote as u8 {
                in_str = false;
            }
            i += 1;
            continue;
        }

        if b == b'"' || b == b'`' {
            in_str = true;
            quote = b as char;
            i += 1;
            continue;
        }

        // Single-line comments starting with '#' or '//'
        if b == b'#' {
            return line[..i].trim();
        }
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            return line[..i].trim();
        }

        i += 1;
    }
    line.trim()
}

fn find_word_outside_quotes(s: &str, word: &str) -> Option<usize> {
    let mut in_str = false;
    let mut quote = '"';
    let mut escaped = false;
    let bytes = s.as_bytes();
    let word_bytes = word.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == quote as u8 {
                in_str = false;
            }
            i += 1;
            continue;
        }

        if b == b'"' || b == b'`' {
            in_str = true;
            quote = b as char;
            i += 1;
            continue;
        }

        if i + word_bytes.len() <= bytes.len() && &bytes[i..i + word_bytes.len()] == word_bytes {
            let before_ok = i == 0
                || bytes[i - 1].is_ascii_whitespace()
                || bytes[i - 1] == b'('
                || bytes[i - 1] == b')';
            let after_idx = i + word_bytes.len();
            let after_ok = after_idx == bytes.len()
                || bytes[after_idx].is_ascii_whitespace()
                || bytes[after_idx] == b'('
                || bytes[after_idx] == b')';
            if before_ok && after_ok {
                return Some(i);
            }
        }

        i += 1;
    }
    None
}

fn has_word_outside_quotes(s: &str, word: &str) -> bool {
    find_word_outside_quotes(s, word).is_some()
}

fn has_inline_if(code: &str) -> bool {
    has_word_outside_quotes(code, "then") && has_word_outside_quotes(code, "else")
}

fn is_when_arm_inline(code: &str) -> bool {
    if let Some(pos) = find_word_outside_quotes(code, "then") {
        let after = code[pos + "then".len()..].trim();
        !after.is_empty()
    } else {
        false
    }
}

fn is_when_else_inline(code: &str) -> bool {
    let trimmed = code.trim();
    if trimmed == "else" {
        return false;
    }
    if let Some(pos) = find_word_outside_quotes(code, "else") {
        let rest = code[pos + "else".len()..].trim();
        if rest.is_empty() {
            return false;
        }
        if rest == "then" {
            return false;
        }
        if let Some(then_pos) = find_word_outside_quotes(rest, "then") {
            let after_then = rest[then_pos + "then".len()..].trim();
            !after_then.is_empty()
        } else {
            !rest.is_empty()
        }
    } else {
        false
    }
}

fn ends_with_word_outside_quotes(s: &str, word: &str) -> bool {
    let bytes = s.as_bytes();
    let word_bytes = word.as_bytes();
    let mut i = bytes.len();

    // Skip trailing whitespace from the end
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }

    if i < word_bytes.len() {
        return false;
    }

    let start = i - word_bytes.len();
    if &bytes[start..i] != word_bytes {
        return false;
    }

    // Check boundary before the word (it must be whitespace, ')', ';', etc.)
    if start > 0 {
        let b_before = bytes[start - 1];
        if !b_before.is_ascii_whitespace() && b_before != b')' && b_before != b';' {
            return false;
        }
    }

    // Now verify that `start..i` is outside quotes
    let mut in_str_scan = false;
    let mut q_scan = '"';
    let mut esc_scan = false;
    let mut j = 0;
    while j < start {
        let b = bytes[j];
        if in_str_scan {
            if esc_scan {
                esc_scan = false;
            } else if b == b'\\' {
                esc_scan = true;
            } else if b == q_scan as u8 {
                in_str_scan = false;
            }
            j += 1;
            continue;
        }

        if b == b'"' || b == b'`' {
            in_str_scan = true;
            q_scan = b as char;
            j += 1;
            continue;
        }

        j += 1;
    }

    !in_str_scan
}

fn get_block_starter(code: &str) -> Option<BlockKind> {
    // If the line ends with 'end' outside quotes, whatever block it opened is immediately closed on the same line
    if ends_with_word_outside_quotes(code, "end") {
        return None;
    }

    let first_word = code.split_whitespace().next().unwrap_or("");
    if first_word == "function"
        || first_word == "fn"
        || code.starts_with("function(")
        || code.starts_with("fn(")
    {
        return Some(BlockKind::Function);
    }
    if first_word == "if" || code.starts_with("if(") {
        if has_inline_if(code) {
            return None;
        }
        return Some(BlockKind::If);
    }
    if first_word == "while" || code.starts_with("while(") {
        return Some(BlockKind::While);
    }
    if first_word == "for" {
        return Some(BlockKind::For);
    }
    if first_word == "repeat" {
        return Some(BlockKind::Repeat);
    }
    if first_word == "try" {
        return Some(BlockKind::Try);
    }
    if first_word == "when" || code.starts_with("when(") {
        return Some(BlockKind::When);
    }
    if first_word == "struct" {
        return Some(BlockKind::Struct);
    }
    None
}

/// Formats the given Alya source code string.
pub fn format_source(source: &str) -> Result<String, String> {
    let lines: Vec<&str> = source.lines().collect();
    let mut formatted_lines: Vec<String> = Vec::new();
    let mut block_stack: Vec<BlockKind> = Vec::new();
    let mut in_multiline_str = false;
    let mut in_raw_str = false;
    let mut in_multiline_comment = false;
    let mut prev_was_empty = false;

    for line in lines {
        let trimmed = line.trim();

        // 1. Multiline string handling
        if in_multiline_str {
            formatted_lines.push(line.to_string());
            if trimmed.contains("\"\"\"") {
                in_multiline_str = false;
            }
            prev_was_empty = false;
            continue;
        }

        // 2. Raw string handling
        if in_raw_str {
            formatted_lines.push(line.to_string());
            if trimmed.contains('`') {
                in_raw_str = false;
            }
            prev_was_empty = false;
            continue;
        }

        // 3. Multiline comment handling: preserve lines verbatim
        if in_multiline_comment {
            formatted_lines.push(line.to_string());
            if trimmed.contains("*/") {
                in_multiline_comment = false;
            }
            prev_was_empty = false;
            continue;
        }

        // Check opening of multiline tokens
        let count_triple = trimmed.matches("\"\"\"").count();
        if count_triple % 2 != 0 {
            in_multiline_str = true;
        }
        let count_ticks = trimmed.matches('`').count();
        if count_ticks % 2 != 0 {
            in_raw_str = true;
        }
        if trimmed.starts_with("/*") && !trimmed.contains("*/") {
            in_multiline_comment = true;
        }

        // Empty line handling
        if trimmed.is_empty() {
            if !prev_was_empty && !formatted_lines.is_empty() {
                formatted_lines.push(String::new());
                prev_was_empty = true;
            }
            continue;
        }
        prev_was_empty = false;

        let code = strip_line_comment(trimmed);

        // Pure comment line: preserve comment indentation with current block level
        if code.is_empty() {
            let indent = " ".repeat(block_stack.len() * 4);
            formatted_lines.push(format!("{}{}", indent, trimmed));
            continue;
        }

        let first_word = code.split_whitespace().next().unwrap_or("");
        let is_end = first_word == "end" || code.starts_with("end(");
        let is_elif = first_word == "elif" || code.starts_with("elif(");
        let is_else = first_word == "else";
        let is_is = first_word == "is" || code.starts_with("is(");
        let is_catch = first_word == "catch" || code.starts_with("catch(");
        let is_finally = first_word == "finally" || code.starts_with("finally(");

        let line_indent: usize;

        if is_end {
            if block_stack.last() == Some(&BlockKind::WhenArm) {
                block_stack.pop();
            }
            if !block_stack.is_empty() {
                block_stack.pop();
            }
            line_indent = block_stack.len();
        } else if is_elif {
            line_indent = block_stack.len().saturating_sub(1);
        } else if is_else {
            if block_stack.last() == Some(&BlockKind::WhenArm) {
                block_stack.pop();
            }
            if block_stack.last() == Some(&BlockKind::When) {
                line_indent = block_stack.len();
                let is_inline = is_when_else_inline(code);
                if !is_inline {
                    block_stack.push(BlockKind::WhenArm);
                }
            } else {
                line_indent = block_stack.len().saturating_sub(1);
            }
        } else if is_is {
            if block_stack.last() == Some(&BlockKind::WhenArm) {
                block_stack.pop();
            }
            line_indent = block_stack.len();
            let is_inline = is_when_arm_inline(code);
            if !is_inline {
                block_stack.push(BlockKind::WhenArm);
            }
        } else if is_catch || is_finally {
            line_indent = block_stack.len().saturating_sub(1);
        } else {
            line_indent = block_stack.len();
        }

        // Format code on the line
        let clean_content = format_line_content(trimmed);
        let indent_spaces = " ".repeat(line_indent * 4);
        formatted_lines.push(format!("{}{}", indent_spaces, clean_content));

        // Indent increase triggers (opens a new block for following lines)
        if !is_end && !is_is && !is_else && !is_elif && !is_catch && !is_finally {
            if let Some(new_block) = get_block_starter(code) {
                block_stack.push(new_block);
            }
        }
    }

    // Trim trailing empty lines so that the file ends cleanly with no trailing blank lines
    while formatted_lines.last().is_some_and(|s| s.is_empty()) {
        formatted_lines.pop();
    }

    let eol = if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut result = formatted_lines.join(eol);
    if !result.is_empty() {
        result.push_str(eol);
    }

    // Safety Verification: Ensure formatted code parses successfully into valid AST
    let mut lexer = Lexer::new(&result);
    if let Ok(tokens) = lexer.tokenize() {
        let mut parser = Parser::new(tokens);
        if let Err(err) = parser.parse() {
            return Err(format!(
                "Formatter safety check failed: formatted code would produce parse error: {}",
                err
            ));
        }
    } else {
        return Err(
            "Formatter safety check failed: formatted code cannot be tokenized".to_string(),
        );
    }

    Ok(result)
}

fn format_line_content(line: &str) -> String {
    // If the line contains raw string delimiters, triple quotes, or multiline comments,
    // do not touch it to avoid any corruption of literal contents.
    if line.contains("\"\"\"") || line.contains('`') || line.contains("/*") || line.contains("*/") {
        return line.to_string();
    }

    // Format spacing while strictly preserving string literals and comments
    let mut out = String::new();
    let mut in_str = false;
    let mut quote_char = '"';
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_str {
            out.push(ch);
            if ch == '\\' {
                if let Some(next_ch) = chars.next() {
                    out.push(next_ch);
                }
            } else if ch == quote_char {
                in_str = false;
            }
            continue;
        }

        if ch == '"' || ch == '`' {
            in_str = true;
            quote_char = ch;
            out.push(ch);
            continue;
        }

        // Comments: leave everything until end of line intact
        if ch == '#' || (ch == '/' && chars.peek() == Some(&'/')) {
            out.push(ch);
            for rest in chars {
                out.push(rest);
            }
            break;
        }

        // Clean spacing around commas outside quotes
        if ch == ',' {
            out.push(',');
            if chars.peek() != Some(&' ') && chars.peek().is_some() {
                out.push(' ');
            }
            continue;
        }

        out.push(ch);
    }

    out
}

/// Recursively discovers all .alya files in a given path.
pub fn find_alya_files(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if path.is_file() {
        if path.extension().and_then(|ext| ext.to_str()) == Some("alya") {
            files.push(path.to_path_buf());
        }
    } else if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                let file_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if file_name.starts_with('.') || file_name == "target" || file_name == "build" {
                    continue;
                }
                if p.is_dir() {
                    files.extend(find_alya_files(&p));
                } else if p.extension().and_then(|ext| ext.to_str()) == Some("alya") {
                    files.push(p);
                }
            }
        }
    }
    files.sort();
    files
}

/// Formats a single file or directory. Returns Ok(number_of_changed_files).
pub fn run_fmt(path_str: &str, check_only: bool) -> Result<usize, String> {
    let root = Path::new(path_str);
    let files = find_alya_files(root);

    if files.is_empty() {
        println!("No .alya files found in '{}'.", path_str);
        return Ok(0);
    }

    let mut changed_count = 0;
    for file in &files {
        let display_path = file.display().to_string();
        let content = fs::read_to_string(file)
            .map_err(|e| format!("Error reading '{}': {}", display_path, e))?;

        let formatted = format_source(&content)
            .map_err(|e| format!("Error formatting '{}': {}", display_path, e))?;

        if formatted != content {
            changed_count += 1;
            if check_only {
                println!("  \x1b[1;33mneeds formatting\x1b[0m: {}", display_path);
            } else {
                fs::write(file, &formatted)
                    .map_err(|e| format!("Error writing formatted '{}': {}", display_path, e))?;
                println!("  \x1b[1;32mformatted\x1b[0m: {}", display_path);
            }
        } else if !check_only {
            println!("  \x1b[90malready formatted\x1b[0m: {}", display_path);
        }
    }

    if check_only {
        if changed_count > 0 {
            Err(format!(
                "Formatting check failed: {} file(s) need formatting.",
                changed_count
            ))
        } else {
            println!(
                "\x1b[1;32m✓ All {} file(s) are properly formatted.\x1b[0m",
                files.len()
            );
            Ok(0)
        }
    } else {
        println!(
            "\x1b[1;32m✓ Formatted {} of {} file(s).\x1b[0m",
            changed_count,
            files.len()
        );
        Ok(changed_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_try_catch_bare() {
        let input = "try\nlet a = 1\ncatch\nsay \"err\"\nend\n";
        let expected = "try\n    let a = 1\ncatch\n    say \"err\"\nend\n";
        assert_eq!(format_source(input).unwrap(), expected);
    }

    #[test]
    fn test_format_try_catch_with_err_and_finally() {
        let input = "try\nsay \"try\"\ncatch err\nsay err\nfinally\nsay \"done\"\nend\n";
        let expected =
            "try\n    say \"try\"\ncatch err\n    say err\nfinally\n    say \"done\"\nend\n";
        assert_eq!(format_source(input).unwrap(), expected);
    }

    #[test]
    fn test_format_when_multiline() {
        let input = r#"when status
is 200, 201 then
say "ok"
is 400..499
say "client error"
else
say "unknown"
end
"#;
        let expected = r#"when status
    is 200, 201 then
        say "ok"
    is 400..499
        say "client error"
    else
        say "unknown"
end
"#;
        assert_eq!(format_source(input).unwrap(), expected);
    }

    #[test]
    fn test_format_when_inline() {
        let input = r#"when priority
is 1 then say "Low"
is 2 then say "Medium"
else say "Custom"
end
"#;
        let expected = r#"when priority
    is 1 then say "Low"
    is 2 then say "Medium"
    else say "Custom"
end
"#;
        assert_eq!(format_source(input).unwrap(), expected);
    }

    #[test]
    fn test_format_multiline_comment_preservation() {
        let input = "/*\n   Showcasing:\n   - Item 1\n   - Item 2\n*/\nlet a = 1\n";
        let expected = "/*\n   Showcasing:\n   - Item 1\n   - Item 2\n*/\nlet a = 1\n";
        assert_eq!(format_source(input).unwrap(), expected);
    }

    #[test]
    fn test_format_nested_function_with_when() {
        let input = r#"function check(val)
when val
is 1 then
say "one"
else
say "other"
end
end
"#;
        let expected = r#"function check(val)
    when val
        is 1 then
            say "one"
        else
            say "other"
    end
end
"#;
        assert_eq!(format_source(input).unwrap(), expected);
    }

    #[test]
    fn test_format_trailing_blank_lines_trimmed() {
        let input = "let x = 1\n\n\n\n";
        let expected = "let x = 1\n";
        assert_eq!(format_source(input).unwrap(), expected);
    }

    #[test]
    fn test_format_single_line_functions() {
        let input =
            "function foo() return 1 end\nfunction bar() return 2 end\nlet x = foo() + bar()\n";
        let expected =
            "function foo() return 1 end\nfunction bar() return 2 end\nlet x = foo() + bar()\n";
        assert_eq!(format_source(input).unwrap(), expected);
    }
}
