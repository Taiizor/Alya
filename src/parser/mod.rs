pub mod expr;
pub mod stmt;
#[cfg(test)]
mod tests;

use crate::ast::{Program, Stmt};
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !matches!(self.current_token().token_type, TokenType::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        Ok(Program { statements })
    }

    pub(super) fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    pub(super) fn advance(&mut self) {
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
        }
    }

    pub(super) fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        if std::mem::discriminant(&self.current_token().token_type)
            != std::mem::discriminant(&expected)
        {
            return Err(format!(
                "Expected {}, found {} at line {}, column {}",
                expected,
                self.current_token().token_type,
                self.current_token().line,
                self.current_token().column
            ));
        }
        self.advance();
        Ok(())
    }

    pub(super) fn skip_newlines(&mut self) {
        while matches!(self.current_token().token_type, TokenType::Newline) {
            self.advance();
        }
    }
}

pub fn resolve_imports(program: &mut Program, base_dir: &std::path::Path) -> Result<(), String> {
    let mut visited = std::collections::HashSet::new();
    let mut resolved_stmts = Vec::new();

    for stmt in std::mem::take(&mut program.statements) {
        resolve_stmt_imports(stmt, base_dir, &mut visited, &mut resolved_stmts)?;
    }

    program.statements = resolved_stmts;
    Ok(())
}

fn get_embedded_stdlib(module: &str) -> Option<&'static str> {
    let clean = module
        .strip_prefix("std/")
        .or_else(|| module.strip_prefix("std::"))
        .unwrap_or(module);
    let clean = clean.strip_suffix(".alya").unwrap_or(clean);
    match clean {
        "math" => Some(include_str!("../../stdlib/math.alya")),
        "time" => Some(include_str!("../../stdlib/time.alya")),
        "os" => Some(include_str!("../../stdlib/os.alya")),
        "json" => Some(include_str!("../../stdlib/json.alya")),
        "mem" => Some(include_str!("../../stdlib/mem.alya")),
        "str" => Some(include_str!("../../stdlib/str.alya")),
        "path" => Some(include_str!("../../stdlib/path.alya")),
        "fs" => Some(include_str!("../../stdlib/fs.alya")),
        "hash" => Some(include_str!("../../stdlib/hash.alya")),
        "collections" => Some(include_str!("../../stdlib/collections.alya")),
        "test" => Some(include_str!("../../stdlib/test.alya")),
        _ => None,
    }
}

fn resolve_stmt_imports(
    stmt: Stmt,
    current_dir: &std::path::Path,
    visited: &mut std::collections::HashSet<std::path::PathBuf>,
    out: &mut Vec<Stmt>,
) -> Result<(), String> {
    match stmt {
        Stmt::Import(import_path_str) => {
            // Normalize path separators to '/' so Windows-style '\' works across Linux, macOS, and Windows
            let normalized_path = import_path_str.replace('\\', "/");
            let path = std::path::Path::new(&normalized_path);
            let target_path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                current_dir.join(path)
            };

            let candidate = if target_path.exists() {
                Some(target_path.clone())
            } else if target_path.with_extension("alya").exists() {
                Some(target_path.with_extension("alya"))
            } else if normalized_path.starts_with("std/") || normalized_path.starts_with("std::") {
                let clean = normalized_path
                    .strip_prefix("std/")
                    .or_else(|| normalized_path.strip_prefix("std::"))
                    .unwrap_or(&normalized_path);
                let std_dir = current_dir.join("stdlib").join(clean);
                let std_root = std::path::Path::new("stdlib").join(clean);
                if std_dir.exists() {
                    Some(std_dir)
                } else if std_dir.with_extension("alya").exists() {
                    Some(std_dir.with_extension("alya"))
                } else if std_root.exists() {
                    Some(std_root)
                } else if std_root.with_extension("alya").exists() {
                    Some(std_root.with_extension("alya"))
                } else {
                    None
                }
            } else {
                None
            };

            let (canonical, source) = if let Some(cand) = candidate {
                let canon = std::fs::canonicalize(&cand)
                    .map_err(|e| format!("Failed to resolve path '{}': {}", cand.display(), e))?;
                if visited.contains(&canon) {
                    return Ok(());
                }
                let src = std::fs::read_to_string(&canon).map_err(|e| {
                    format!(
                        "Failed to read imported module '{}': {}",
                        canon.display(),
                        e
                    )
                })?;
                (canon, src)
            } else if normalized_path.starts_with("std/") || normalized_path.starts_with("std::") {
                if let Some(src) = get_embedded_stdlib(&normalized_path) {
                    let synthetic =
                        std::path::PathBuf::from(format!("<embedded:{}>", normalized_path));
                    if visited.contains(&synthetic) {
                        return Ok(());
                    }
                    (synthetic, src.to_string())
                } else {
                    return Err(format!(
                        "Cannot find standard library module '{}'",
                        import_path_str
                    ));
                }
            } else {
                return Err(format!(
                    "Cannot find imported module '{}' (looked at '{}')",
                    import_path_str,
                    target_path.display()
                ));
            };

            visited.insert(canonical.clone());

            let mut lexer = crate::lexer::Lexer::new(&source);
            let tokens = lexer.tokenize().map_err(|e| {
                format!(
                    "Lexer error in imported module '{}': {}",
                    canonical.display(),
                    e
                )
            })?;

            let mut parser = Parser::new(tokens);
            let sub_program = parser.parse().map_err(|e| {
                format!(
                    "Parser error in imported module '{}': {}",
                    canonical.display(),
                    e
                )
            })?;

            let sub_dir = canonical.parent().unwrap_or(current_dir);
            for sub_stmt in sub_program.statements {
                resolve_stmt_imports(sub_stmt, sub_dir, visited, out)?;
            }
        }
        other => {
            out.push(other);
        }
    }
    Ok(())
}
