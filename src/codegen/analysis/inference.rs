use super::traversal::find_call_arg;
use crate::ast::*;
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
                Expr::Call { name, .. } if name == "split" || name == "args" => true,
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
        Expr::Call { name, .. } if name == "split" || name == "args" => true,
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
            ..
        } => {
            stmts_return_string(try_block, known_strings)
                || stmts_return_string(catch_block, known_strings)
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

fn collect_function_defs<'a>(
    stmts: &'a [Stmt],
    defs: &mut Vec<(&'a str, &'a [String], &'a [Stmt])>,
) {
    for stmt in stmts {
        match stmt {
            Stmt::Function { name, params, body } => {
                defs.push((name.as_str(), params.as_slice(), body.as_slice()));
                collect_function_defs(body, defs);
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                collect_function_defs(then_block, defs);
                if let Some(eb) = else_block {
                    collect_function_defs(eb, defs);
                }
            }
            Stmt::While { body, .. }
            | Stmt::For { body, .. }
            | Stmt::ForEach { body, .. }
            | Stmt::Repeat { body } => {
                collect_function_defs(body, defs);
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                ..
            } => {
                collect_function_defs(try_block, defs);
                collect_function_defs(catch_block, defs);
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
            ..
        } => {
            stmts_return_float(try_block, known_floats)
                || stmts_return_float(catch_block, known_floats)
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

fn expr_is_definitely_map(expr: &Expr, known_maps: &HashSet<String>) -> bool {
    match expr {
        Expr::Call { name, .. } if name == "map" || name == "set_new" => true,
        Expr::Identifier(name) => known_maps.contains(name),
        _ => false,
    }
}

fn collect_map_vars_from_stmts(stmts: &[Stmt], known_maps: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } | Stmt::Assign { name, value, .. }
                if expr_is_definitely_map(value, known_maps) =>
            {
                known_maps.insert(name.clone());
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                ..
            } => {
                collect_map_vars_from_stmts(try_block, known_maps);
                collect_map_vars_from_stmts(catch_block, known_maps);
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
