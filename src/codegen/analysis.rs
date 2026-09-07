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
        Expr::FieldAccess { object, field } => {
            if let Expr::Identifier(obj_name) = &**object {
                let key = format!("{}.{}", obj_name, field);
                if let Some(var_type) = vars.get(&key) {
                    matches!(var_type, VarType::StringLabel(_) | VarType::StringOffset(_))
                } else {
                    false
                }
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

pub fn is_array_expr(expr: &Expr, vars: &HashMap<String, VarType>) -> bool {
    match expr {
        Expr::Array(_) => true,
        Expr::Identifier(name) => {
            matches!(vars.get(name), Some(VarType::Array(_)))
        }
        _ => false,
    }
}

pub fn is_float_expr(expr: &Expr, vars: &HashMap<String, VarType>) -> bool {
    match expr {
        Expr::Float(_) => true,
        Expr::Number(n) => n.fract() != 0.0,
        Expr::Identifier(name) => {
            matches!(vars.get(name), Some(VarType::Float(_)))
        }
        Expr::FieldAccess { object, field } => {
            if let Expr::Identifier(obj_name) = &**object {
                let key = format!("{}.{}", obj_name, field);
                matches!(vars.get(&key), Some(VarType::Float(_)))
            } else {
                false
            }
        }
        Expr::Binary {
            left,
            op:
                BinaryOp::Add
                | BinaryOp::Subtract
                | BinaryOp::Multiply
                | BinaryOp::Divide
                | BinaryOp::Modulo,
            right,
        } => is_float_expr(left, vars) || is_float_expr(right, vars),
        Expr::Unary {
            op: UnaryOp::Negate,
            expr,
        } => is_float_expr(expr, vars),
        Expr::Call { name, .. } => name == "float",
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

fn expr_is_definitely_float(expr: &Expr, known_floats: &HashSet<String>) -> bool {
    match expr {
        Expr::Float(_) => true,
        Expr::Number(n) => n.fract() != 0.0,
        Expr::Identifier(name) => known_floats.contains(name),
        Expr::Binary {
            left,
            op:
                BinaryOp::Add
                | BinaryOp::Subtract
                | BinaryOp::Multiply
                | BinaryOp::Divide
                | BinaryOp::Modulo,
            right,
        } => {
            expr_is_definitely_float(left, known_floats)
                || expr_is_definitely_float(right, known_floats)
        }
        Expr::Unary {
            op: UnaryOp::Negate,
            expr,
        } => expr_is_definitely_float(expr, known_floats),
        Expr::Call { name, .. } => name == "float",
        _ => false,
    }
}

fn collect_float_vars_from_stmts(stmts: &[Stmt], known_floats: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. }
                if expr_is_definitely_float(value, known_floats) =>
            {
                known_floats.insert(name.clone());
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                ..
            } => {
                collect_float_vars_from_stmts(try_block, known_floats);
                collect_float_vars_from_stmts(catch_block, known_floats);
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                collect_float_vars_from_stmts(then_block, known_floats);
                if let Some(else_stmts) = else_block {
                    collect_float_vars_from_stmts(else_stmts, known_floats);
                }
            }
            Stmt::While { body, .. } | Stmt::Repeat { body } | Stmt::For { body, .. } => {
                collect_float_vars_from_stmts(body, known_floats);
            }
            Stmt::Function { body, .. } => {
                collect_float_vars_from_stmts(body, known_floats);
            }
            _ => {}
        }
    }
}

pub fn collect_known_float_vars(program: &Program) -> HashSet<String> {
    let mut known_floats = HashSet::new();
    for _ in 0..4 {
        let prev_len = known_floats.len();
        collect_float_vars_from_stmts(&program.statements, &mut known_floats);
        if known_floats.len() == prev_len {
            break;
        }
    }
    known_floats
}

pub fn infer_param_is_float(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_floats = collect_known_float_vars(program);
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            expr_is_definitely_float(arg, &known_floats)
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
        Expr::InterpolatedString(parts) => {
            for part in parts {
                if let Some(arg) = find_call_arg_in_expr(part, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        _ => None,
    }
}

pub fn infer_param_struct_type(
    func_name: &str,
    param_idx: usize,
    program: &Program,
) -> Option<String> {
    for s in &program.statements {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            if let Some(st) = infer_expr_struct_type(arg, program) {
                return Some(st);
            }
        }
    }
    None
}

fn infer_expr_struct_type(expr: &Expr, program: &Program) -> Option<String> {
    match expr {
        Expr::StructInit { name, .. } => Some(name.clone()),
        Expr::Call { name, .. } => {
            for s in &program.statements {
                if let Stmt::StructDef { name: sname, .. } = s {
                    if sname == name {
                        return Some(name.clone());
                    }
                }
            }
            None
        }
        Expr::Identifier(var_name) => {
            for s in &program.statements {
                if let Stmt::Let { name, value } = s {
                    if name == var_name {
                        return infer_expr_struct_type(value, program);
                    }
                }
            }
            None
        }
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
