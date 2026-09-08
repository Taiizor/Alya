pub mod expr;
pub mod stmt;
#[cfg(test)]
mod tests;

use crate::ast::{Expr, Program, Stmt};
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
            statements.extend(self.parse_statement()?);
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

    validate_unique_functions(&resolved_stmts)?;

    program.statements = resolved_stmts;
    Ok(())
}

pub fn validate_unique_functions(stmts: &[Stmt]) -> Result<(), String> {
    let mut seen_functions: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for stmt in stmts {
        if let Stmt::Function { name, .. } = stmt {
            let count = seen_functions.entry(name.clone()).or_insert(0);
            *count += 1;
            if *count > 1 {
                return Err(format!(
                    "Duplicate function definition '{}'. If importing multiple modules containing '{}', use 'import \"...\" as <alias>' to assign distinct namespaces.",
                    name, name
                ));
            }
        }
    }
    Ok(())
}

fn apply_module_alias(
    stmts: &mut [Stmt],
    alias: &str,
    local_fns: &std::collections::HashSet<String>,
) {
    for stmt in stmts {
        prefix_stmt(stmt, alias, local_fns);
    }
}

fn prefix_stmt(stmt: &mut Stmt, alias: &str, local_fns: &std::collections::HashSet<String>) {
    match stmt {
        Stmt::Function { name, body, .. } => {
            if local_fns.contains(name) {
                *name = format!("{}::{}", alias, name);
            }
            for s in body {
                prefix_stmt(s, alias, local_fns);
            }
        }
        Stmt::Say(expr) => prefix_expr(expr, alias, local_fns),
        Stmt::Expr(expr) => prefix_expr(expr, alias, local_fns),
        Stmt::Let { value, .. } => prefix_expr(value, alias, local_fns),
        Stmt::Assign { value, .. } => prefix_expr(value, alias, local_fns),
        Stmt::If {
            condition,
            then_block,
            else_block,
        } => {
            prefix_expr(condition, alias, local_fns);
            for s in then_block {
                prefix_stmt(s, alias, local_fns);
            }
            if let Some(eb) = else_block {
                for s in eb {
                    prefix_stmt(s, alias, local_fns);
                }
            }
        }
        Stmt::While { condition, body } => {
            prefix_expr(condition, alias, local_fns);
            for s in body {
                prefix_stmt(s, alias, local_fns);
            }
        }
        Stmt::For {
            start, end, body, ..
        } => {
            prefix_expr(start, alias, local_fns);
            prefix_expr(end, alias, local_fns);
            for s in body {
                prefix_stmt(s, alias, local_fns);
            }
        }
        Stmt::ForEach { iterable, body, .. } => {
            prefix_expr(iterable, alias, local_fns);
            for s in body {
                prefix_stmt(s, alias, local_fns);
            }
        }
        Stmt::Repeat { body } => {
            for s in body {
                prefix_stmt(s, alias, local_fns);
            }
        }
        Stmt::Return(Some(e)) => {
            prefix_expr(e, alias, local_fns);
        }
        Stmt::IndexAssign {
            array,
            index,
            value,
        } => {
            prefix_expr(array, alias, local_fns);
            prefix_expr(index, alias, local_fns);
            prefix_expr(value, alias, local_fns);
        }
        Stmt::FieldAssign { object, value, .. } => {
            prefix_expr(object, alias, local_fns);
            prefix_expr(value, alias, local_fns);
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            for s in try_block {
                prefix_stmt(s, alias, local_fns);
            }
            for s in catch_block {
                prefix_stmt(s, alias, local_fns);
            }
            if let Some(fb) = finally_block {
                for s in fb {
                    prefix_stmt(s, alias, local_fns);
                }
            }
        }
        Stmt::Throw(Some(e)) => {
            prefix_expr(e, alias, local_fns);
        }
        _ => {}
    }
}

fn prefix_expr(expr: &mut Expr, alias: &str, local_fns: &std::collections::HashSet<String>) {
    match expr {
        Expr::Call { name, args } => {
            if local_fns.contains(name) {
                *name = format!("{}::{}", alias, name);
            }
            for arg in args {
                prefix_expr(arg, alias, local_fns);
            }
        }
        Expr::Binary { left, right, .. } => {
            prefix_expr(left, alias, local_fns);
            prefix_expr(right, alias, local_fns);
        }
        Expr::Unary { expr, .. } => {
            prefix_expr(expr, alias, local_fns);
        }
        Expr::Array(items) => {
            for item in items {
                prefix_expr(item, alias, local_fns);
            }
        }
        Expr::Index { array, index } => {
            prefix_expr(array, alias, local_fns);
            prefix_expr(index, alias, local_fns);
        }
        Expr::FieldAccess { object, .. } => {
            prefix_expr(object, alias, local_fns);
        }
        Expr::StructInit { fields, .. } => {
            for (_, field_expr) in fields {
                prefix_expr(field_expr, alias, local_fns);
            }
        }
        Expr::Map(entries) => {
            for (k, v) in entries {
                prefix_expr(k, alias, local_fns);
                prefix_expr(v, alias, local_fns);
            }
        }
        Expr::InterpolatedString(parts) => {
            for part in parts {
                prefix_expr(part, alias, local_fns);
            }
        }
        _ => {}
    }
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
        "bench" => Some(include_str!("../../stdlib/bench.alya")),
        _ => None,
    }
}

fn resolve_stmt_imports(
    stmt: Stmt,
    current_dir: &std::path::Path,
    visited: &mut std::collections::HashSet<(std::path::PathBuf, Option<String>)>,
    out: &mut Vec<Stmt>,
) -> Result<(), String> {
    match stmt {
        Stmt::Import {
            path: import_path_str,
            alias,
        } => {
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
                if visited.contains(&(canon.clone(), alias.clone())) {
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
                    if visited.contains(&(synthetic.clone(), alias.clone())) {
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

            visited.insert((canonical.clone(), alias.clone()));

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

            let local_fns: std::collections::HashSet<String> = sub_program
                .statements
                .iter()
                .filter_map(|s| match s {
                    Stmt::Function { name, .. } => Some(name.clone()),
                    _ => None,
                })
                .collect();

            let sub_dir = canonical.parent().unwrap_or(current_dir);
            let mut sub_resolved = Vec::new();
            for sub_stmt in sub_program.statements {
                resolve_stmt_imports(sub_stmt, sub_dir, visited, &mut sub_resolved)?;
            }

            if let Some(ref alias_str) = alias {
                apply_module_alias(&mut sub_resolved, alias_str, &local_fns);
            }

            out.extend(sub_resolved);
        }
        other => {
            out.push(other);
        }
    }
    Ok(())
}
