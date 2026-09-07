use crate::ast::*;
use crate::codegen::context::VarType;
use std::collections::{HashMap, HashSet};

pub fn is_string_expr(expr: &Expr, vars: &HashMap<String, VarType>) -> bool {
    match expr {
        Expr::String(_) => true,
        Expr::InterpolatedString(_) => true,
        Expr::Call { name, .. } if name == "ask" || name == "str" => true,
        Expr::Identifier(name) => {
            if let Some(var_type) = vars.get(name) {
                matches!(var_type, VarType::StringLabel(_) | VarType::StringOffset(_))
            } else {
                false
            }
        }
        Expr::Binary {
            left,
            op: BinaryOp::Add,
            right,
        } => is_string_expr(left, vars) || is_string_expr(right, vars),
        _ => false,
    }
}

fn expr_is_definitely_string(expr: &Expr, known_strings: &HashSet<String>) -> bool {
    match expr {
        Expr::String(_) | Expr::InterpolatedString(_) => true,
        Expr::Call { name, .. } if name == "ask" || name == "str" => true,
        Expr::Identifier(name) => known_strings.contains(name),
        Expr::Binary {
            left,
            op: BinaryOp::Add,
            right,
        } => {
            expr_is_definitely_string(left, known_strings)
                || expr_is_definitely_string(right, known_strings)
        }
        _ => false,
    }
}

fn collect_string_vars_from_stmts(stmts: &[Stmt], known_strings: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. }
                if expr_is_definitely_string(value, known_strings) =>
            {
                known_strings.insert(name.clone());
            }
            Stmt::TryCatch {
                try_block,
                catch_var,
                catch_block,
            } => {
                if let Some(err_var) = catch_var {
                    known_strings.insert(err_var.clone());
                }
                collect_string_vars_from_stmts(try_block, known_strings);
                collect_string_vars_from_stmts(catch_block, known_strings);
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                collect_string_vars_from_stmts(then_block, known_strings);
                if let Some(else_stmts) = else_block {
                    collect_string_vars_from_stmts(else_stmts, known_strings);
                }
            }
            Stmt::While { body, .. } | Stmt::Repeat { body } | Stmt::For { body, .. } => {
                collect_string_vars_from_stmts(body, known_strings);
            }
            Stmt::Function { body, .. } => {
                collect_string_vars_from_stmts(body, known_strings);
            }
            _ => {}
        }
    }
}

pub fn collect_known_string_vars(program: &Program) -> HashSet<String> {
    let mut known_strings = HashSet::new();
    for _ in 0..4 {
        let prev_len = known_strings.len();
        collect_string_vars_from_stmts(&program.statements, &mut known_strings);
        if known_strings.len() == prev_len {
            break;
        }
    }
    known_strings
}

pub fn infer_param_is_string(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_strings = collect_known_string_vars(program);
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            expr_is_definitely_string(arg, &known_strings)
        } else {
            false
        }
    })
}

pub fn find_call_arg<'a>(stmt: &'a Stmt, func_name: &str, param_idx: usize) -> Option<&'a Expr> {
    match stmt {
        Stmt::Expr(expr) | Stmt::Say(expr) => find_call_arg_in_expr(expr, func_name, param_idx),
        Stmt::Let { value, .. } | Stmt::Assign { value, .. } => {
            find_call_arg_in_expr(value, func_name, param_idx)
        }
        Stmt::If {
            condition,
            then_block,
            else_block,
        } => {
            if let Some(arg) = find_call_arg_in_expr(condition, func_name, param_idx) {
                return Some(arg);
            }
            for s in then_block {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            if let Some(else_stmts) = else_block {
                for s in else_stmts {
                    if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                        return Some(arg);
                    }
                }
            }
            None
        }
        Stmt::While { condition, body } => {
            if let Some(arg) = find_call_arg_in_expr(condition, func_name, param_idx) {
                return Some(arg);
            }
            for s in body {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        Stmt::Repeat { body } | Stmt::For { body, .. } => {
            for s in body {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            ..
        } => {
            for s in try_block {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            for s in catch_block {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        _ => None,
    }
}

fn find_call_arg_in_expr<'a>(
    expr: &'a Expr,
    func_name: &str,
    param_idx: usize,
) -> Option<&'a Expr> {
    match expr {
        Expr::Call { name, args } if name == func_name => args.get(param_idx),
        Expr::Binary { left, right, .. } => find_call_arg_in_expr(left, func_name, param_idx)
            .or_else(|| find_call_arg_in_expr(right, func_name, param_idx)),
        Expr::Unary { expr, .. } => find_call_arg_in_expr(expr, func_name, param_idx),
        _ => None,
    }
}

pub fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}
