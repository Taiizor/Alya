use super::common::collect_function_defs;
use crate::ast::*;
use crate::codegen::analysis::traversal::find_call_arg;
use std::collections::HashSet;

fn expr_is_definitely_string(expr: &Expr, known_strings: &HashSet<String>) -> bool {
    match expr {
        Expr::String(_) | Expr::InterpolatedString(_) => true,
        Expr::Call { name, .. } => {
            if matches!(
                name.as_str(),
                "ask"
                    | "str"
                    | "trim"
                    | "upper"
                    | "lower"
                    | "substring"
                    | "substr"
                    | "join"
                    | "char_at"
                    | "chr"
                    | "char_from_code"
                    | "read_file"
                    | "get_env"
                    | "env"
                    | "env_or"
                    | "target_os"
                    | "target_arch"
                    | "os_name"
                    | "arch_name"
                    | "arch"
                    | "platform"
                    | "temp_dir"
                    | "home_dir"
                    | "user_name"
                    | "hostname"
                    | "null_device"
                    | "path_list_separator"
                    | "arg_at"
                    | "str_from_ptr"
            ) {
                return true;
            }
            known_strings.contains(&format!("fn_ret_str:{}", name))
        }
        Expr::Identifier(name) => known_strings.contains(name),
        Expr::Binary {
            left,
            op: BinaryOp::Add,
            right,
        } => {
            expr_is_definitely_string(left, known_strings)
                || expr_is_definitely_string(right, known_strings)
        }
        Expr::Index { array, index } => {
            if let Expr::String(field) = &**index {
                if known_strings.contains(&format!("map_field_str:{}", field)) {
                    return true;
                }
            }
            if let (Expr::Identifier(map_name), Expr::String(field)) = (&**array, &**index) {
                if known_strings.contains(&format!("map_str:{}.{}", map_name, field)) {
                    return true;
                }
            }
            match &**array {
                Expr::Identifier(arr_name) => {
                    known_strings.contains(&format!("arr_is_str:{}", arr_name))
                        || known_strings.contains(arr_name)
                }
                Expr::Call { name, .. }
                    if name == "split" || name == "args" || name == "cli_args" =>
                {
                    true
                }
                _ => expr_is_definitely_string(array, known_strings),
            }
        }
        _ => false,
    }
}

fn expr_is_string_array(expr: &Expr, known_strings: &HashSet<String>) -> bool {
    match expr {
        Expr::Array(elems) => elems
            .first()
            .is_some_and(|e| expr_is_definitely_string(e, known_strings)),
        Expr::Identifier(name) => known_strings.contains(&format!("arr_is_str:{}", name)),
        Expr::Call { name, .. } if name == "split" || name == "args" || name == "cli_args" => true,
        _ => false,
    }
}

fn stmts_return_string(stmts: &[Stmt], known_strings: &HashSet<String>) -> bool {
    stmts.iter().any(|s| match s {
        Stmt::Return(Some(expr)) => expr_is_definitely_string(expr, known_strings),
        Stmt::If {
            then_block,
            else_block,
            ..
        } => {
            stmts_return_string(then_block, known_strings)
                || else_block
                    .as_ref()
                    .is_some_and(|eb| stmts_return_string(eb, known_strings))
        }
        Stmt::While { body, .. }
        | Stmt::Repeat { body }
        | Stmt::For { body, .. }
        | Stmt::ForEach { body, .. } => stmts_return_string(body, known_strings),
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            stmts_return_string(try_block, known_strings)
                || stmts_return_string(catch_block, known_strings)
                || finally_block
                    .as_ref()
                    .is_some_and(|fb| stmts_return_string(fb, known_strings))
        }
        _ => false,
    })
}

fn collect_string_vars_from_stmts(stmts: &[Stmt], known_strings: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. } => {
                if expr_is_definitely_string(value, known_strings) {
                    known_strings.insert(name.clone());
                }
                if expr_is_string_array(value, known_strings) {
                    known_strings.insert(format!("arr_is_str:{}", name));
                }
                if let Expr::Map(entries) = value {
                    for (k, v) in entries {
                        if expr_is_definitely_string(v, known_strings) {
                            if let Expr::String(field) = k {
                                known_strings.insert(format!("map_field_str:{}", field));
                                known_strings.insert(format!("map_str:{}.{}", name, field));
                            }
                        }
                    }
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_var,
                catch_block,
                finally_block,
            } => {
                if let Some(err_var) = catch_var {
                    known_strings.insert(err_var.clone());
                }
                collect_string_vars_from_stmts(try_block, known_strings);
                collect_string_vars_from_stmts(catch_block, known_strings);
                if let Some(finally_block) = finally_block {
                    collect_string_vars_from_stmts(finally_block, known_strings);
                }
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
            Stmt::ForEach {
                var,
                iterable,
                body,
            } => {
                if expr_is_string_array(iterable, known_strings) {
                    known_strings.insert(var.clone());
                }
                collect_string_vars_from_stmts(body, known_strings);
            }
            Stmt::Function { name, body, .. } => {
                let mut fn_locals = known_strings.clone();
                collect_string_vars_from_stmts(body, &mut fn_locals);
                if stmts_return_string(body, &fn_locals) {
                    known_strings.insert(format!("fn_ret_str:{}", name));
                }
                for item in fn_locals {
                    if item.starts_with("fn_ret_str:")
                        || item.starts_with("map_field_str:")
                        || item.starts_with("map_str:")
                    {
                        known_strings.insert(item);
                    }
                }
            }
            Stmt::IndexAssign {
                array,
                index,
                value,
            } if expr_is_definitely_string(value, known_strings) => {
                if let Expr::String(field) = index {
                    known_strings.insert(format!("map_field_str:{}", field));
                }
                if let (Expr::Identifier(map_name), Expr::String(field)) = (array, index) {
                    known_strings.insert(format!("map_str:{}.{}", map_name, field));
                }
            }
            _ => {}
        }
    }
}

pub fn collect_known_string_vars(program: &Program) -> HashSet<String> {
    let mut known_strings = HashSet::new();
    let mut funcs = Vec::new();
    collect_function_defs(&program.statements, &mut funcs);
    for _ in 0..8 {
        let prev_len = known_strings.len();
        collect_string_vars_from_stmts(&program.statements, &mut known_strings);
        for (name, params, _) in &funcs {
            for (idx, param) in params.iter().enumerate() {
                if !known_strings.contains(&format!("arr_is_str:{}", param)) {
                    let is_str_arr = program.statements.iter().any(|s| {
                        if let Some(arg) = find_call_arg(s, name, idx) {
                            expr_is_string_array(arg, &known_strings)
                        } else {
                            false
                        }
                    });
                    if is_str_arr {
                        known_strings.insert(format!("arr_is_str:{}", param));
                    }
                }
                if known_strings.contains(param) {
                    continue;
                }
                let is_str_arg = program.statements.iter().any(|s| {
                    if let Some(arg) = find_call_arg(s, name, idx) {
                        expr_is_definitely_string(arg, &known_strings)
                    } else {
                        false
                    }
                });
                if is_str_arg {
                    known_strings.insert(param.clone());
                }
            }
        }
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

pub fn infer_param_is_string_array(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_strings = collect_known_string_vars(program);
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            expr_is_string_array(arg, &known_strings)
        } else {
            false
        }
    })
}
