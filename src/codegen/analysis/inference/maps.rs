use super::common::collect_function_defs;
use crate::ast::*;
use crate::codegen::analysis::traversal::find_call_arg;
use std::collections::HashSet;

fn expr_is_definitely_map(
    expr: &Expr,
    fn_scope: Option<&str>,
    known_maps: &HashSet<String>,
) -> bool {
    match expr {
        Expr::Call { name, .. } => {
            let bare = name.rsplit("::").next().unwrap_or(name.as_str());
            let bare = bare.rsplit("__").next().unwrap_or(bare);
            matches!(
                bare,
                "map"
                    | "set_new"
                    | "set_from_array"
                    | "set_union"
                    | "set_intersection"
                    | "set_difference"
                    | "map_clone"
                    | "map_merge"
                    | "map_from_entries"
            ) || known_maps.contains(&format!("fn_ret_map:{}", name))
                || known_maps.contains(&format!("fn_ret_map:{}", bare))
        }
        Expr::Map(_) => true,
        Expr::Identifier(name) => {
            if let Some(scope) = fn_scope {
                known_maps.contains(&format!("{}:{}", scope, name))
            } else {
                known_maps.contains(name)
            }
        }
        Expr::Index { array, index } => {
            if let Expr::String(field) = &**index {
                if known_maps.contains(&format!("map_field_map:{}", field)) {
                    return true;
                }
            }
            if let (Expr::Identifier(map_name), Expr::String(field)) = (&**array, &**index) {
                if let Some(scope) = fn_scope {
                    if known_maps.contains(&format!("{}:{}.{}", scope, map_name, field)) {
                        return true;
                    }
                }
                if known_maps.contains(&format!("map_map:{}.{}", map_name, field)) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn collect_return_exprs<'a>(stmts: &'a [Stmt], returns: &mut Vec<&'a Expr>) {
    for s in stmts {
        match s {
            Stmt::Return(Some(expr)) => returns.push(expr),
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                collect_return_exprs(then_block, returns);
                if let Some(eb) = else_block {
                    collect_return_exprs(eb, returns);
                }
            }
            Stmt::While { body, .. }
            | Stmt::Repeat { body }
            | Stmt::For { body, .. }
            | Stmt::ForEach { body, .. } => collect_return_exprs(body, returns),
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_return_exprs(try_block, returns);
                collect_return_exprs(catch_block, returns);
                if let Some(fb) = finally_block {
                    collect_return_exprs(fb, returns);
                }
            }
            _ => {}
        }
    }
}

fn stmts_return_map(stmts: &[Stmt], fn_scope: Option<&str>, known_maps: &HashSet<String>) -> bool {
    let mut returns = Vec::new();
    collect_return_exprs(stmts, &mut returns);
    let non_null: Vec<_> = returns
        .into_iter()
        .filter(|e| !matches!(e, Expr::Null))
        .collect();
    !non_null.is_empty()
        && non_null
            .iter()
            .all(|e| expr_is_definitely_map(e, fn_scope, known_maps))
}

fn collect_map_vars_from_stmts(
    stmts: &[Stmt],
    fn_scope: Option<&str>,
    known_maps: &mut HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. } => {
                if expr_is_definitely_map(value, fn_scope, known_maps) {
                    if let Some(scope) = fn_scope {
                        known_maps.insert(format!("{}:{}", scope, name));
                    } else {
                        known_maps.insert(name.clone());
                    }
                }
                if let Expr::Map(entries) = value {
                    for (k, v) in entries {
                        if expr_is_definitely_map(v, fn_scope, known_maps) {
                            if let Expr::String(field) = k {
                                known_maps.insert(format!("map_field_map:{}", field));
                                if let Some(scope) = fn_scope {
                                    known_maps.insert(format!("{}:{}.{}", scope, name, field));
                                }
                                known_maps.insert(format!("map_map:{}.{}", name, field));
                            }
                        }
                    }
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_map_vars_from_stmts(try_block, fn_scope, known_maps);
                collect_map_vars_from_stmts(catch_block, fn_scope, known_maps);
                if let Some(finally_block) = finally_block {
                    collect_map_vars_from_stmts(finally_block, fn_scope, known_maps);
                }
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                collect_map_vars_from_stmts(then_block, fn_scope, known_maps);
                if let Some(else_stmts) = else_block {
                    collect_map_vars_from_stmts(else_stmts, fn_scope, known_maps);
                }
            }
            Stmt::While { body, .. }
            | Stmt::Repeat { body }
            | Stmt::For { body, .. }
            | Stmt::ForEach { body, .. } => {
                collect_map_vars_from_stmts(body, fn_scope, known_maps);
            }
            Stmt::Function { name, body, .. } => {
                let bare = name.rsplit("::").next().unwrap_or(name);
                let bare = bare.rsplit("__").next().unwrap_or(bare);
                collect_map_vars_from_stmts(body, Some(name), known_maps);
                if bare != name {
                    collect_map_vars_from_stmts(body, Some(bare), known_maps);
                }
            }
            _ => {}
        }
    }
}

pub fn collect_known_map_vars(program: &Program) -> HashSet<String> {
    let mut known_maps = HashSet::new();
    let mut funcs = Vec::new();
    collect_function_defs(&program.statements, &mut funcs);
    for _ in 0..5 {
        let prev_len = known_maps.len();
        collect_map_vars_from_stmts(&program.statements, None, &mut known_maps);
        for (name, params, body) in &funcs {
            let bare = name.rsplit("::").next().unwrap_or(name);
            let bare = bare.rsplit("__").next().unwrap_or(bare);
            if stmts_return_map(body, Some(name), &known_maps)
                || (bare != *name && stmts_return_map(body, Some(bare), &known_maps))
            {
                known_maps.insert(format!("fn_ret_map:{}", name));
                known_maps.insert(format!("fn_ret_map:{}", bare));
            }
            for (idx, _param) in params.iter().enumerate() {
                if known_maps.contains(&format!("fn_param_map:{}:{}", name, idx))
                    || known_maps.contains(&format!("fn_param_map:{}:{}", bare, idx))
                {
                    continue;
                }
                let mut found_call = false;
                let all_calls_map = program.statements.iter().all(|s| {
                    if let Some(arg) = find_call_arg(s, name, idx) {
                        found_call = true;
                        expr_is_definitely_map(arg, None, &known_maps)
                    } else if let Some(arg) = find_call_arg(s, bare, idx) {
                        found_call = true;
                        expr_is_definitely_map(arg, None, &known_maps)
                    } else {
                        true
                    }
                });
                if found_call && all_calls_map {
                    known_maps.insert(format!("fn_param_map:{}:{}", name, idx));
                    known_maps.insert(format!("fn_param_map:{}:{}", bare, idx));
                }
            }
        }
        if known_maps.len() == prev_len {
            break;
        }
    }
    known_maps
}

pub fn infer_param_is_map_with(
    func_name: &str,
    param_idx: usize,
    _program: &Program,
    known_maps: &HashSet<String>,
) -> bool {
    let bare = func_name.rsplit("::").next().unwrap_or(func_name);
    let bare = bare.rsplit("__").next().unwrap_or(bare);
    known_maps.contains(&format!("fn_param_map:{}:{}", func_name, param_idx))
        || known_maps.contains(&format!("fn_param_map:{}:{}", bare, param_idx))
}

pub fn infer_param_is_map(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_maps = collect_known_map_vars(program);
    infer_param_is_map_with(func_name, param_idx, program, &known_maps)
}
