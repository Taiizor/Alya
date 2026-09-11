use super::common::collect_function_defs;
use crate::ast::*;
use crate::codegen::analysis::traversal::find_call_arg;
use std::collections::HashSet;

fn expr_is_definitely_array(expr: &Expr, known_arrays: &HashSet<String>) -> bool {
    match expr {
        Expr::Array(_) => true,
        Expr::Identifier(name) => known_arrays.contains(name),
        Expr::Call { name, .. } => {
            let bare = name.rsplit("::").next().unwrap_or(name.as_str());
            let bare = bare.rsplit("__").next().unwrap_or(bare);
            matches!(
                bare,
                "split"
                    | "args"
                    | "cli_args"
                    | "keys"
                    | "values"
                    | "lines"
                    | "read_lines"
                    | "set_to_array"
                    | "stack_new"
                    | "queue_new"
                    | "array_slice"
                    | "array_clone"
                    | "array_concat"
                    | "array_reverse"
                    | "array_reverse_in_place"
                    | "array_unique"
                    | "array_sort"
                    | "array_sort_in_place"
                    | "array_chunk"
                    | "array_fill"
                    | "map_entries"
                    | "rand_sample"
                    | "rand_shuffle"
                    | "rand_shuffled"
                    | "list_dir"
                    | "read_dir"
                    | "fs_list_dir"
                    | "fs_read_dir"
                    | "list_dir_recursive"
                    | "fs_list_dir_recursive"
                    | "glob"
                    | "glob_dir"
            )
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
                finally_block,
                ..
            } => {
                collect_array_vars_from_stmts(try_block, known_arrays);
                collect_array_vars_from_stmts(catch_block, known_arrays);
                if let Some(finally_block) = finally_block {
                    collect_array_vars_from_stmts(finally_block, known_arrays);
                }
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
            Stmt::Function { body, .. } => {
                collect_array_vars_from_stmts(body, known_arrays);
            }
            _ => {}
        }
    }
}

pub fn collect_known_array_vars(program: &Program) -> HashSet<String> {
    let mut known_arrays = HashSet::new();
    let mut funcs = Vec::new();
    collect_function_defs(&program.statements, &mut funcs);
    for _ in 0..5 {
        let prev_len = known_arrays.len();
        collect_array_vars_from_stmts(&program.statements, &mut known_arrays);
        for (name, params, _) in &funcs {
            let bare = name.rsplit("::").next().unwrap_or(name);
            let bare = bare.rsplit("__").next().unwrap_or(bare);
            for (idx, _param) in params.iter().enumerate() {
                if known_arrays.contains(&format!("fn_param_arr:{}:{}", name, idx))
                    || known_arrays.contains(&format!("fn_param_arr:{}:{}", bare, idx))
                {
                    continue;
                }
                let mut found_call = false;
                let all_calls_arr = program.statements.iter().all(|s| {
                    if let Some(arg) = find_call_arg(s, name, idx) {
                        found_call = true;
                        expr_is_definitely_array(arg, &known_arrays)
                    } else if let Some(arg) = find_call_arg(s, bare, idx) {
                        found_call = true;
                        expr_is_definitely_array(arg, &known_arrays)
                    } else {
                        true
                    }
                });
                if found_call && all_calls_arr {
                    known_arrays.insert(format!("fn_param_arr:{}:{}", name, idx));
                    known_arrays.insert(format!("fn_param_arr:{}:{}", bare, idx));
                }
            }
        }
        if known_arrays.len() == prev_len {
            break;
        }
    }
    known_arrays
}

pub fn infer_param_is_array_with(
    func_name: &str,
    param_idx: usize,
    _program: &Program,
    known_arrays: &HashSet<String>,
) -> bool {
    let bare = func_name.rsplit("::").next().unwrap_or(func_name);
    let bare = bare.rsplit("__").next().unwrap_or(bare);
    known_arrays.contains(&format!("fn_param_arr:{}:{}", func_name, param_idx))
        || known_arrays.contains(&format!("fn_param_arr:{}:{}", bare, param_idx))
}

pub fn infer_param_is_array(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_arrays = collect_known_array_vars(program);
    infer_param_is_array_with(func_name, param_idx, program, &known_arrays)
}
