use crate::ast::*;
use crate::codegen::context::VarType;
use std::collections::HashMap;

pub fn is_string_expr(expr: &Expr, vars: &HashMap<String, VarType>) -> bool {
    match expr {
        Expr::String(_) => true,
        Expr::InterpolatedString(_) => true,
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
            ) {
                return true;
            }
            vars.contains_key(&format!("fn_ret_str:{}", name))
        }
        Expr::Identifier(name) => {
            if let Some(var_type) = vars.get(name) {
                matches!(var_type, VarType::StringLabel(_) | VarType::StringOffset(_))
            } else {
                false
            }
        }
        Expr::Index { array, index } => {
            if let Expr::String(field) = &**index {
                let key = format!("map_field_str:{}", field);
                if vars.contains_key(&key) {
                    return true;
                }
            }
            if let (Expr::Identifier(obj_name), Expr::String(field)) = (&**array, &**index) {
                let key = format!("map_str:{}.{}", obj_name, field);
                if vars.contains_key(&key) {
                    return true;
                }
            }
            is_string_array(array, vars) || is_string_expr(array, vars)
        }
        Expr::FieldAccess { object, field } => {
            if let Expr::Identifier(obj_name) = &**object {
                let key = format!("{}.{}", obj_name, field);
                if let Some(var_type) = vars.get(&key) {
                    if matches!(var_type, VarType::StringLabel(_) | VarType::StringOffset(_)) {
                        return true;
                    }
                }
                if let Some(VarType::Struct { struct_name, .. }) = vars.get(obj_name) {
                    let field_key = format!("struct_field_str:{}.{}", struct_name, field);
                    if vars.contains_key(&field_key) {
                        return true;
                    }
                }
            }
            let global_field_key = format!("struct_field_str:{}", field);
            if vars.contains_key(&global_field_key) {
                return true;
            }
            false
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
                    | "csv_parse"
                    | "csv_parse_with_delimiter"
                    | "csv_parse_tsv"
                    | "tsv_parse"
                    | "csv_parse_records"
                    | "csv_parse_records_with_delimiter"
                    | "tsv_parse_records"
                    | "csv_read_file"
                    | "csv_read_file_with_delimiter"
                    | "csv_read_records"
                    | "csv_read_records_with_delimiter"
                    | "tsv_read_file"
                    | "tsv_read_records"
                    | "url_path_segments"
            )
        }
        Expr::FieldAccess { object, field } => {
            if let Expr::Identifier(obj_name) = &**object {
                let key = format!("{}.{}", obj_name, field);
                if let Some(var_type) = vars.get(&key) {
                    if matches!(var_type, VarType::Array(_)) {
                        return true;
                    }
                }
                if let Some(VarType::Struct { struct_name, .. }) = vars.get(obj_name) {
                    let field_key = format!("struct_field_arr:{}.{}", struct_name, field);
                    if vars.contains_key(&field_key) {
                        return true;
                    }
                }
            }
            let global_field_key = format!("struct_field_arr:{}", field);
            if vars.contains_key(&global_field_key) {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn is_map_expr(expr: &Expr, vars: &HashMap<String, VarType>) -> bool {
    match expr {
        Expr::Identifier(name) => {
            matches!(vars.get(name), Some(VarType::Map(_)))
        }
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
                    | "url_parse_query"
            )
        }
        Expr::Map(_) => true,
        Expr::Index { array, index } => {
            if let Expr::String(field) = &**index {
                let key = format!("map_field_map:{}", field);
                if vars.contains_key(&key) {
                    return true;
                }
            }
            if let (Expr::Identifier(obj_name), Expr::String(field)) = (&**array, &**index) {
                let key = format!("map_map:{}.{}", obj_name, field);
                if vars.contains_key(&key) {
                    return true;
                }
            }
            false
        }
        Expr::FieldAccess { object, field } => {
            if let Expr::Identifier(obj_name) = &**object {
                let key = format!("{}.{}", obj_name, field);
                if let Some(var_type) = vars.get(&key) {
                    if matches!(var_type, VarType::Map(_)) {
                        return true;
                    }
                }
                if let Some(VarType::Struct { struct_name, .. }) = vars.get(obj_name) {
                    let field_key = format!("struct_field_map:{}.{}", struct_name, field);
                    if vars.contains_key(&field_key) {
                        return true;
                    }
                }
            }
            let global_field_key = format!("struct_field_map:{}", field);
            if vars.contains_key(&global_field_key) {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn is_string_array(expr: &Expr, vars: &HashMap<String, VarType>) -> bool {
    match expr {
        Expr::Array(elems) => elems.first().is_some_and(|e| is_string_expr(e, vars)),
        Expr::Identifier(name) => vars.contains_key(&format!("arr_is_str:{}", name)),
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

pub fn is_float_array(expr: &Expr, vars: &HashMap<String, VarType>) -> bool {
    match expr {
        Expr::Array(elems) => elems.first().is_some_and(|e| is_float_expr(e, vars)),
        Expr::Identifier(name) => vars.contains_key(&format!("arr_is_flt:{}", name)),
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
                if matches!(vars.get(&key), Some(VarType::Float(_))) {
                    return true;
                }
                if let Some(VarType::Struct { struct_name, .. }) = vars.get(obj_name) {
                    let field_key = format!("struct_field_flt:{}.{}", struct_name, field);
                    if vars.contains_key(&field_key) {
                        return true;
                    }
                }
            }
            let global_field_key = format!("struct_field_flt:{}", field);
            if vars.contains_key(&global_field_key) {
                return true;
            }
            false
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
        Expr::Index { array, .. } => is_float_array(array, vars),
        Expr::Call { name, .. } => {
            let bare = name.rsplit("::").next().unwrap_or(name.as_str());
            let bare = bare.rsplit("__").next().unwrap_or(bare);
            matches!(
                bare,
                "float"
                    | "to_float"
                    | "parse_float"
                    | "sin"
                    | "cos"
                    | "tan"
                    | "mean"
                    | "deg_to_rad"
                    | "rad_to_deg"
                    | "radians"
                    | "degrees"
                    | "lerp"
                    | "norm"
                    | "smoothstep"
                    | "variance"
                    | "rand_float"
                    | "rand_float_range"
                    | "rand_rng_float"
            ) || vars.contains_key(&format!("fn_ret_flt:{}", name))
                || vars.contains_key(&format!("fn_ret_flt:{}", bare))
        }
        _ => false,
    }
}
