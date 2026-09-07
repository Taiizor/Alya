use super::common::collect_function_defs;
use crate::ast::*;
use crate::codegen::analysis::traversal::find_call_arg;
use std::collections::HashSet;

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
        Expr::Call { name, .. } => {
            matches!(
                name.as_str(),
                "float" | "sin" | "cos" | "tan" | "mean" | "deg_to_rad" | "rad_to_deg"
            ) || known_floats.contains(&format!("fn_ret_flt:{}", name))
        }
        Expr::Index { array, .. } => match &**array {
            Expr::Identifier(arr_name) => {
                known_floats.contains(&format!("arr_is_flt:{}", arr_name))
            }
            _ => false,
        },
        _ => false,
    }
}

fn expr_is_float_array(expr: &Expr, known_floats: &HashSet<String>) -> bool {
    match expr {
        Expr::Array(elems) => elems
            .first()
            .is_some_and(|e| expr_is_definitely_float(e, known_floats)),
        Expr::Identifier(name) => known_floats.contains(&format!("arr_is_flt:{}", name)),
        _ => false,
    }
}

fn stmts_return_float(stmts: &[Stmt], known_floats: &HashSet<String>) -> bool {
    stmts.iter().any(|s| match s {
        Stmt::Return(Some(expr)) => expr_is_definitely_float(expr, known_floats),
        Stmt::If {
            then_block,
            else_block,
            ..
        } => {
            stmts_return_float(then_block, known_floats)
                || else_block
                    .as_ref()
                    .is_some_and(|eb| stmts_return_float(eb, known_floats))
        }
        Stmt::While { body, .. }
        | Stmt::Repeat { body }
        | Stmt::For { body, .. }
        | Stmt::ForEach { body, .. } => stmts_return_float(body, known_floats),
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            stmts_return_float(try_block, known_floats)
                || stmts_return_float(catch_block, known_floats)
                || finally_block
                    .as_ref()
                    .is_some_and(|fb| stmts_return_float(fb, known_floats))
        }
        _ => false,
    })
}

fn collect_float_vars_from_stmts(stmts: &[Stmt], known_floats: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. } => {
                if expr_is_definitely_float(value, known_floats) {
                    known_floats.insert(name.clone());
                }
                if expr_is_float_array(value, known_floats) {
                    known_floats.insert(format!("arr_is_flt:{}", name));
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_float_vars_from_stmts(try_block, known_floats);
                collect_float_vars_from_stmts(catch_block, known_floats);
                if let Some(finally_block) = finally_block {
                    collect_float_vars_from_stmts(finally_block, known_floats);
                }
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
            Stmt::ForEach {
                var,
                iterable,
                body,
            } => {
                if expr_is_float_array(iterable, known_floats) {
                    known_floats.insert(var.clone());
                }
                collect_float_vars_from_stmts(body, known_floats);
            }
            Stmt::Function { name, body, .. } => {
                let mut fn_locals = known_floats.clone();
                collect_float_vars_from_stmts(body, &mut fn_locals);
                if stmts_return_float(body, &fn_locals) {
                    known_floats.insert(format!("fn_ret_flt:{}", name));
                }
                for item in fn_locals {
                    if item.starts_with("fn_ret_flt:") {
                        known_floats.insert(item);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn collect_known_float_vars(program: &Program) -> HashSet<String> {
    let mut known_floats = HashSet::new();
    let mut funcs = Vec::new();
    collect_function_defs(&program.statements, &mut funcs);
    for _ in 0..8 {
        let prev_len = known_floats.len();
        collect_float_vars_from_stmts(&program.statements, &mut known_floats);
        for (name, params, _) in &funcs {
            for (idx, param) in params.iter().enumerate() {
                if !known_floats.contains(&format!("arr_is_flt:{}", param)) {
                    let is_flt_arr = program.statements.iter().any(|s| {
                        if let Some(arg) = find_call_arg(s, name, idx) {
                            expr_is_float_array(arg, &known_floats)
                        } else {
                            false
                        }
                    });
                    if is_flt_arr {
                        known_floats.insert(format!("arr_is_flt:{}", param));
                    }
                }
                if known_floats.contains(param) {
                    continue;
                }
                let is_flt_arg = program.statements.iter().any(|s| {
                    if let Some(arg) = find_call_arg(s, name, idx) {
                        expr_is_definitely_float(arg, &known_floats)
                    } else {
                        false
                    }
                });
                if is_flt_arg {
                    known_floats.insert(param.clone());
                }
            }
        }
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

pub fn infer_param_is_float_array(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_floats = collect_known_float_vars(program);
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            expr_is_float_array(arg, &known_floats)
        } else {
            false
        }
    })
}
