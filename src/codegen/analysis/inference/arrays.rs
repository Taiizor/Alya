use super::common::collect_function_defs;
use crate::ast::*;
use crate::codegen::analysis::traversal::find_call_arg;
use std::collections::HashSet;

fn expr_is_definitely_array(expr: &Expr, known_arrays: &HashSet<String>) -> bool {
    match expr {
        Expr::Array(_) => true,
        Expr::Identifier(name) => known_arrays.contains(name),
        Expr::Call { name, .. }
            if matches!(
                name.as_str(),
                "split"
                    | "args"
                    | "keys"
                    | "values"
                    | "lines"
                    | "set_to_array"
                    | "stack_new"
                    | "queue_new"
            ) =>
        {
            true
        }
        _ => false,
    }
}

fn collect_array_vars_from_stmts(stmts: &[Stmt], known_arrays: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. }
                if expr_is_definitely_array(value, known_arrays) =>
            {
                known_arrays.insert(name.clone());
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                ..
            } => {
                collect_array_vars_from_stmts(try_block, known_arrays);
                collect_array_vars_from_stmts(catch_block, known_arrays);
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                collect_array_vars_from_stmts(then_block, known_arrays);
                if let Some(else_stmts) = else_block {
                    collect_array_vars_from_stmts(else_stmts, known_arrays);
                }
            }
            Stmt::While { body, .. }
            | Stmt::Repeat { body }
            | Stmt::For { body, .. }
            | Stmt::ForEach { body, .. } => {
                collect_array_vars_from_stmts(body, known_arrays);
            }
            Stmt::Function { .. } => {}
            _ => {}
        }
    }
}

pub fn collect_known_array_vars(program: &Program) -> HashSet<String> {
    let mut known_arrays = HashSet::new();
    let mut funcs = Vec::new();
    collect_function_defs(&program.statements, &mut funcs);
    for _ in 0..8 {
        let prev_len = known_arrays.len();
        collect_array_vars_from_stmts(&program.statements, &mut known_arrays);
        for (name, params, _) in &funcs {
            for (idx, param) in params.iter().enumerate() {
                if known_arrays.contains(param) {
                    continue;
                }
                let is_arr_arg = program.statements.iter().any(|s| {
                    if let Some(arg) = find_call_arg(s, name, idx) {
                        expr_is_definitely_array(arg, &known_arrays)
                    } else {
                        false
                    }
                });
                if is_arr_arg {
                    known_arrays.insert(param.clone());
                }
            }
        }
        if known_arrays.len() == prev_len {
            break;
        }
    }
    known_arrays
}

pub fn infer_param_is_array(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_arrays = collect_known_array_vars(program);
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            expr_is_definitely_array(arg, &known_arrays)
        } else {
            false
        }
    })
}
