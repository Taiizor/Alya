use super::common::collect_function_defs;
use crate::ast::*;
use crate::codegen::analysis::traversal::find_call_arg;
use std::collections::HashSet;

fn expr_is_definitely_map(expr: &Expr, known_maps: &HashSet<String>) -> bool {
    match expr {
        Expr::Call { name, .. } if name == "map" || name == "set_new" => true,
        Expr::Map(_) => true,
        Expr::Identifier(name) => known_maps.contains(name),
        Expr::Index { array, index } => {
            if let Expr::String(field) = &**index {
                if known_maps.contains(&format!("map_field_map:{}", field)) {
                    return true;
                }
            }
            if let (Expr::Identifier(map_name), Expr::String(field)) = (&**array, &**index) {
                if known_maps.contains(&format!("map_map:{}.{}", map_name, field)) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn collect_map_vars_from_stmts(stmts: &[Stmt], known_maps: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. } => {
                if expr_is_definitely_map(value, known_maps) {
                    known_maps.insert(name.clone());
                }
                if let Expr::Map(entries) = value {
                    for (k, v) in entries {
                        if expr_is_definitely_map(v, known_maps) {
                            if let Expr::String(field) = k {
                                known_maps.insert(format!("map_field_map:{}", field));
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
                collect_map_vars_from_stmts(try_block, known_maps);
                collect_map_vars_from_stmts(catch_block, known_maps);
                if let Some(finally_block) = finally_block {
                    collect_map_vars_from_stmts(finally_block, known_maps);
                }
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                collect_map_vars_from_stmts(then_block, known_maps);
                if let Some(else_stmts) = else_block {
                    collect_map_vars_from_stmts(else_stmts, known_maps);
                }
            }
            Stmt::While { body, .. }
            | Stmt::Repeat { body }
            | Stmt::For { body, .. }
            | Stmt::ForEach { body, .. } => {
                collect_map_vars_from_stmts(body, known_maps);
            }
            Stmt::Function { .. } => {}
            _ => {}
        }
    }
}

pub fn collect_known_map_vars(program: &Program) -> HashSet<String> {
    let mut known_maps = HashSet::new();
    let mut funcs = Vec::new();
    collect_function_defs(&program.statements, &mut funcs);
    for _ in 0..8 {
        let prev_len = known_maps.len();
        collect_map_vars_from_stmts(&program.statements, &mut known_maps);
        for (name, params, _) in &funcs {
            for (idx, param) in params.iter().enumerate() {
                if known_maps.contains(param) {
                    continue;
                }
                let is_map_arg = program.statements.iter().any(|s| {
                    if let Some(arg) = find_call_arg(s, name, idx) {
                        expr_is_definitely_map(arg, &known_maps)
                    } else {
                        false
                    }
                });
                if is_map_arg {
                    known_maps.insert(param.clone());
                }
            }
        }
        if known_maps.len() == prev_len {
            break;
        }
    }
    known_maps
}

pub fn infer_param_is_map(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_maps = collect_known_map_vars(program);
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            expr_is_definitely_map(arg, &known_maps)
        } else {
            false
        }
    })
}
