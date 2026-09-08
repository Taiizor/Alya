use crate::ast::*;

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
        Stmt::Repeat { body } | Stmt::For { body, .. } | Stmt::ForEach { body, .. } => {
            for s in body {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        Stmt::Throw(opt_expr) => opt_expr
            .as_ref()
            .and_then(|expr| find_call_arg_in_expr(expr, func_name, param_idx)),
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
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
            if let Some(finally_block) = finally_block {
                for s in finally_block {
                    if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                        return Some(arg);
                    }
                }
            }
            None
        }
        Stmt::IndexAssign {
            array,
            index,
            value,
        } => find_call_arg_in_expr(array, func_name, param_idx)
            .or_else(|| find_call_arg_in_expr(index, func_name, param_idx))
            .or_else(|| find_call_arg_in_expr(value, func_name, param_idx)),
        Stmt::FieldAssign { object, value, .. } => {
            find_call_arg_in_expr(object, func_name, param_idx)
                .or_else(|| find_call_arg_in_expr(value, func_name, param_idx))
        }
        Stmt::Return(opt_expr) => {
            if let Some(expr) = opt_expr {
                find_call_arg_in_expr(expr, func_name, param_idx)
            } else {
                None
            }
        }
        Stmt::Function { body, .. } => {
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

pub fn find_call_arg_in_expr<'a>(
    expr: &'a Expr,
    func_name: &str,
    param_idx: usize,
) -> Option<&'a Expr> {
    match expr {
        Expr::Call { name, args } => {
            if name == func_name {
                if let Some(arg) = args.get(param_idx) {
                    return Some(arg);
                }
            }
            for arg in args {
                if let Some(res) = find_call_arg_in_expr(arg, func_name, param_idx) {
                    return Some(res);
                }
            }
            None
        }
        Expr::Binary { left, right, .. } => find_call_arg_in_expr(left, func_name, param_idx)
            .or_else(|| find_call_arg_in_expr(right, func_name, param_idx)),
        Expr::Unary { expr, .. } => find_call_arg_in_expr(expr, func_name, param_idx),
        Expr::Array(elements) => {
            for elem in elements {
                if let Some(arg) = find_call_arg_in_expr(elem, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        Expr::Index { array, index } => find_call_arg_in_expr(array, func_name, param_idx)
            .or_else(|| find_call_arg_in_expr(index, func_name, param_idx)),
        Expr::FieldAccess { object, .. } => find_call_arg_in_expr(object, func_name, param_idx),
        Expr::StructInit { fields, .. } => {
            for (_, val) in fields {
                if let Some(arg) = find_call_arg_in_expr(val, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        Expr::Map(entries) => {
            for (k, v) in entries {
                if let Some(arg) = find_call_arg_in_expr(k, func_name, param_idx) {
                    return Some(arg);
                }
                if let Some(arg) = find_call_arg_in_expr(v, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        Expr::InterpolatedString(parts) => {
            for part in parts {
                if let Some(arg) = find_call_arg_in_expr(part, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        Expr::Ternary {
            condition,
            then_branch,
            else_branch,
        } => find_call_arg_in_expr(condition, func_name, param_idx)
            .or_else(|| find_call_arg_in_expr(then_branch, func_name, param_idx))
            .or_else(|| find_call_arg_in_expr(else_branch, func_name, param_idx)),
        _ => None,
    }
}
