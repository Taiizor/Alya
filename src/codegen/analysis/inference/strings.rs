use super::common::collect_function_defs;
use crate::ast::*;
use crate::codegen::analysis::traversal::find_call_arg;
use std::collections::{HashMap, HashSet};

fn expr_is_definitely_string(expr: &Expr, known_strings: &HashSet<String>) -> bool {
    match expr {
        Expr::String(_) | Expr::InterpolatedString(_) => true,
        Expr::Call { name, .. } => {
            let bare = name.rsplit("::").next().unwrap_or(name.as_str());
            let bare = bare.rsplit("__").next().unwrap_or(bare);
            if matches!(
                bare,
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
                    | "replace"
                    | "str_repeat"
                    | "pad_left"
                    | "pad_right"
                    | "center"
                    | "trim_start"
                    | "ltrim"
                    | "trim_end"
                    | "rtrim"
                    | "trim_char"
                    | "capitalize"
                    | "title_case"
                    | "reverse_str"
                    | "truncate"
                    | "slugify"
                    | "path_separator"
                    | "path_join"
                    | "file_name"
                    | "file_ext"
                    | "parent_dir"
                    | "file_stem"
                    | "base64_encode"
                    | "base64_decode"
                    | "to_base64"
                    | "from_base64"
                    | "hex_encode"
                    | "hex_decode"
                    | "to_hex"
                    | "from_hex"
                    | "json_object"
                    | "json_map"
                    | "json_string_map"
                    | "json_string"
                    | "json_escape"
                    | "json_null"
                    | "json_int"
                    | "json_float"
                    | "json_kv"
                    | "json_pretty"
                    | "json_get_string"
                    | "read_file_or"
                    | "glob_escape"
                    | "uuid_v4"
                    | "uuid_v4_simple"
                    | "uuid_v7"
                    | "uuid_v7_at"
                    | "uuid_v7_simple"
                    | "uuid_v7_simple_at"
                    | "ulid_generate"
                    | "ulid_at"
                    | "str_clone"
                    | "string_clone"
                    | "tcp_recv"
                    | "net_recv"
                    | "http_recv"
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
        Expr::FieldAccess { object, field } => {
            if let Expr::Identifier(obj_name) = &**object {
                if known_strings.contains(&format!("{}.{}", obj_name, field)) {
                    return true;
                }
            }
            known_strings.contains(&format!("struct_field_str:{}", field))
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
                Expr::Call { name, .. } => {
                    let bare = name.rsplit("::").next().unwrap_or(name.as_str());
                    let bare = bare.rsplit("__").next().unwrap_or(bare);
                    matches!(
                        bare,
                        "split" | "args" | "cli_args" | "lines" | "read_lines" | "keys"
                    )
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
        Expr::Call { name, .. } => {
            let bare = name.rsplit("::").next().unwrap_or(name.as_str());
            let bare = bare.rsplit("__").next().unwrap_or(bare);
            matches!(
                bare,
                "split" | "args" | "cli_args" | "lines" | "read_lines" | "keys"
            )
        }
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

fn collect_struct_defs(stmts: &[Stmt], map: &mut HashMap<String, Vec<String>>) {
    for s in stmts {
        match s {
            Stmt::StructDef { name, fields } => {
                map.insert(name.clone(), fields.clone());
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                collect_struct_defs(then_block, map);
                if let Some(eb) = else_block {
                    collect_struct_defs(eb, map);
                }
            }
            Stmt::While { body, .. }
            | Stmt::Repeat { body }
            | Stmt::For { body, .. }
            | Stmt::ForEach { body, .. }
            | Stmt::Function { body, .. } => {
                collect_struct_defs(body, map);
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_struct_defs(try_block, map);
                collect_struct_defs(catch_block, map);
                if let Some(fb) = finally_block {
                    collect_struct_defs(fb, map);
                }
            }
            _ => {}
        }
    }
}

fn scan_expr_for_strings(
    expr: &Expr,
    struct_defs: &HashMap<String, Vec<String>>,
    known_strings: &mut HashSet<String>,
) {
    match expr {
        Expr::Call { name, args } => {
            if let Some(fields) = struct_defs.get(name) {
                for (i, arg) in args.iter().enumerate() {
                    if expr_is_definitely_string(arg, known_strings) {
                        if let Some(fname) = fields.get(i) {
                            known_strings.insert(format!("struct_field_str:{}.{}", name, fname));
                            known_strings.insert(format!("struct_field_str:{}", fname));
                        }
                    }
                }
            }
            for arg in args {
                scan_expr_for_strings(arg, struct_defs, known_strings);
            }
        }
        Expr::StructInit { name, fields } => {
            for (fname, fval) in fields {
                if expr_is_definitely_string(fval, known_strings) {
                    known_strings.insert(format!("struct_field_str:{}.{}", name, fname));
                    known_strings.insert(format!("struct_field_str:{}", fname));
                }
                scan_expr_for_strings(fval, struct_defs, known_strings);
            }
        }
        Expr::Binary { left, right, .. } => {
            scan_expr_for_strings(left, struct_defs, known_strings);
            scan_expr_for_strings(right, struct_defs, known_strings);
        }
        Expr::Unary { expr, .. } => {
            scan_expr_for_strings(expr, struct_defs, known_strings);
        }
        Expr::Array(elems) => {
            for elem in elems {
                scan_expr_for_strings(elem, struct_defs, known_strings);
            }
        }
        Expr::Index { array, index } => {
            scan_expr_for_strings(array, struct_defs, known_strings);
            scan_expr_for_strings(index, struct_defs, known_strings);
        }
        Expr::FieldAccess { object, .. } => {
            scan_expr_for_strings(object, struct_defs, known_strings);
        }
        Expr::Map(entries) => {
            for (k, v) in entries {
                scan_expr_for_strings(k, struct_defs, known_strings);
                scan_expr_for_strings(v, struct_defs, known_strings);
            }
        }
        Expr::InterpolatedString(parts) => {
            for part in parts {
                scan_expr_for_strings(part, struct_defs, known_strings);
            }
        }
        Expr::Ternary {
            condition,
            then_branch,
            else_branch,
        } => {
            scan_expr_for_strings(condition, struct_defs, known_strings);
            scan_expr_for_strings(then_branch, struct_defs, known_strings);
            scan_expr_for_strings(else_branch, struct_defs, known_strings);
        }
        Expr::NullCoalesce { value, default } => {
            scan_expr_for_strings(value, struct_defs, known_strings);
            scan_expr_for_strings(default, struct_defs, known_strings);
        }
        _ => {}
    }
}

fn collect_string_vars_from_stmts(
    stmts: &[Stmt],
    struct_defs: &HashMap<String, Vec<String>>,
    known_strings: &mut HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. } => {
                scan_expr_for_strings(value, struct_defs, known_strings);
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
                if let Expr::StructInit {
                    name: sname,
                    fields,
                } = value
                {
                    for (fname, fval) in fields {
                        if expr_is_definitely_string(fval, known_strings) {
                            known_strings.insert(format!("struct_field_str:{}.{}", sname, fname));
                            known_strings.insert(format!("struct_field_str:{}", fname));
                            known_strings.insert(format!("{}.{}", name, fname));
                        }
                    }
                }
                if let Expr::Call { name: cname, args } = value {
                    if let Some(fnames) = struct_defs.get(cname) {
                        for (i, arg) in args.iter().enumerate() {
                            if expr_is_definitely_string(arg, known_strings) {
                                if let Some(fname) = fnames.get(i) {
                                    known_strings
                                        .insert(format!("struct_field_str:{}.{}", cname, fname));
                                    known_strings.insert(format!("struct_field_str:{}", fname));
                                    known_strings.insert(format!("{}.{}", name, fname));
                                }
                            }
                        }
                    }
                }
            }
            Stmt::Say(expr) | Stmt::Expr(expr) => {
                scan_expr_for_strings(expr, struct_defs, known_strings);
            }
            Stmt::Return(Some(expr)) | Stmt::Throw(Some(expr)) => {
                scan_expr_for_strings(expr, struct_defs, known_strings);
            }
            Stmt::FieldAssign {
                object,
                field,
                value,
            } => {
                scan_expr_for_strings(value, struct_defs, known_strings);
                if expr_is_definitely_string(value, known_strings) {
                    known_strings.insert(format!("struct_field_str:{}", field));
                    if let Expr::Identifier(obj_name) = object {
                        known_strings.insert(format!("{}.{}", obj_name, field));
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
                collect_string_vars_from_stmts(try_block, struct_defs, known_strings);
                collect_string_vars_from_stmts(catch_block, struct_defs, known_strings);
                if let Some(finally_block) = finally_block {
                    collect_string_vars_from_stmts(finally_block, struct_defs, known_strings);
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                scan_expr_for_strings(condition, struct_defs, known_strings);
                collect_string_vars_from_stmts(then_block, struct_defs, known_strings);
                if let Some(else_stmts) = else_block {
                    collect_string_vars_from_stmts(else_stmts, struct_defs, known_strings);
                }
            }
            Stmt::While { condition, body } => {
                scan_expr_for_strings(condition, struct_defs, known_strings);
                collect_string_vars_from_stmts(body, struct_defs, known_strings);
            }
            Stmt::Repeat { body } => {
                collect_string_vars_from_stmts(body, struct_defs, known_strings);
            }
            Stmt::For {
                start, end, body, ..
            } => {
                scan_expr_for_strings(start, struct_defs, known_strings);
                scan_expr_for_strings(end, struct_defs, known_strings);
                collect_string_vars_from_stmts(body, struct_defs, known_strings);
            }
            Stmt::ForEach {
                var,
                iterable,
                body,
            } => {
                scan_expr_for_strings(iterable, struct_defs, known_strings);
                if expr_is_string_array(iterable, known_strings) {
                    known_strings.insert(var.clone());
                }
                collect_string_vars_from_stmts(body, struct_defs, known_strings);
            }
            Stmt::Function { name, body, .. } => {
                let mut fn_locals = known_strings.clone();
                collect_string_vars_from_stmts(body, struct_defs, &mut fn_locals);
                if stmts_return_string(body, &fn_locals) {
                    known_strings.insert(format!("fn_ret_str:{}", name));
                }
                for item in fn_locals {
                    if item.starts_with("fn_ret_str:")
                        || item.starts_with("map_field_str:")
                        || item.starts_with("map_str:")
                        || item.starts_with("struct_field_str:")
                    {
                        known_strings.insert(item);
                    }
                }
            }
            Stmt::IndexAssign {
                array,
                index,
                value,
            } => {
                scan_expr_for_strings(index, struct_defs, known_strings);
                scan_expr_for_strings(value, struct_defs, known_strings);
                if expr_is_definitely_string(value, known_strings) {
                    if let Expr::String(field) = index {
                        known_strings.insert(format!("map_field_str:{}", field));
                    }
                    if let (Expr::Identifier(map_name), Expr::String(field)) = (array, index) {
                        known_strings.insert(format!("map_str:{}.{}", map_name, field));
                    }
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
    let mut struct_defs = HashMap::new();
    collect_struct_defs(&program.statements, &mut struct_defs);
    for _ in 0..5 {
        let prev_len = known_strings.len();
        collect_string_vars_from_stmts(&program.statements, &struct_defs, &mut known_strings);
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

pub fn infer_param_is_string_with(
    func_name: &str,
    param_idx: usize,
    program: &Program,
    known_strings: &HashSet<String>,
) -> bool {
    let bare = func_name.rsplit("::").next().unwrap_or(func_name);
    let bare = bare.rsplit("__").next().unwrap_or(bare);
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            expr_is_definitely_string(arg, known_strings)
        } else if let Some(arg) = find_call_arg(s, bare, param_idx) {
            expr_is_definitely_string(arg, known_strings)
        } else {
            false
        }
    })
}

pub fn infer_param_is_string_array_with(
    func_name: &str,
    param_idx: usize,
    program: &Program,
    known_strings: &HashSet<String>,
) -> bool {
    let bare = func_name.rsplit("::").next().unwrap_or(func_name);
    let bare = bare.rsplit("__").next().unwrap_or(bare);
    program.statements.iter().any(|s| {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            expr_is_string_array(arg, known_strings)
        } else if let Some(arg) = find_call_arg(s, bare, param_idx) {
            expr_is_string_array(arg, known_strings)
        } else {
            false
        }
    })
}

pub fn infer_param_is_string(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_strings = collect_known_string_vars(program);
    infer_param_is_string_with(func_name, param_idx, program, &known_strings)
}

pub fn infer_param_is_string_array(func_name: &str, param_idx: usize, program: &Program) -> bool {
    let known_strings = collect_known_string_vars(program);
    infer_param_is_string_array_with(func_name, param_idx, program, &known_strings)
}
