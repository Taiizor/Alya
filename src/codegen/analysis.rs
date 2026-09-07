use crate::ast::*;
use crate::codegen::context::VarType;
use std::collections::HashMap;

pub fn is_string_expr(expr: &Expr, vars: &HashMap<String, VarType>) -> bool {
    match expr {
        Expr::String(_) => true,
        Expr::InterpolatedString(_) => true,
        Expr::Identifier(name) => {
            if let Some(var_type) = vars.get(name) {
                matches!(var_type, VarType::StringLabel(_) | VarType::StringOffset(_))
            } else {
                false
            }
        }
        Expr::Binary { left, op: BinaryOp::Add, right } => {
            is_string_expr(left, vars) || is_string_expr(right, vars)
        }
        _ => false,
    }
}

pub fn infer_param_is_string(func_name: &str, param_idx: usize, program: &Program) -> bool {
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            matches!(arg, Expr::String(_) | Expr::InterpolatedString(_))
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
        Stmt::If { condition, then_block, else_block } => {
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
        Stmt::For { body, .. } => {
            for s in body {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        _ => None,
    }
}

fn find_call_arg_in_expr<'a>(expr: &'a Expr, func_name: &str, param_idx: usize) -> Option<&'a Expr> {
    match expr {
        Expr::Call { name, args } if name == func_name => args.get(param_idx),
        Expr::Binary { left, right, .. } => {
            find_call_arg_in_expr(left, func_name, param_idx)
                .or_else(|| find_call_arg_in_expr(right, func_name, param_idx))
        }
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
