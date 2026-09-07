pub mod arm64;
pub mod x64;
pub mod x86;

use crate::ast::{BinaryOp, UnaryOp};
use crate::codegen::target::{Architecture, OperatingSystem};

pub fn emit_header(out: &mut String, arch: Architecture, os: OperatingSystem) {
    match arch {
        Architecture::ARM64 => arm64::emit_header(out, os),
        Architecture::X64 => x64::emit_header(out, os),
        Architecture::X86 => x86::emit_header(out),
    }
}

pub fn emit_footer(out: &mut String, arch: Architecture) {
    match arch {
        Architecture::ARM64 => arm64::emit_footer(out),
        Architecture::X64 => x64::emit_footer(out),
        Architecture::X86 => x86::emit_footer(out),
    }
}

pub fn emit_load_num(out: &mut String, arch: Architecture, val: i64) {
    match arch {
        Architecture::ARM64 => arm64::emit_load_num(out, val),
        Architecture::X64 => x64::emit_load_num(out, val),
        Architecture::X86 => x86::emit_load_num(out, val),
    }
}

pub fn emit_load_str_label(out: &mut String, arch: Architecture, label: &str, os: OperatingSystem) {
    match arch {
        Architecture::ARM64 => arm64::emit_load_str_label(out, label, os),
        Architecture::X64 => x64::emit_load_str_label(out, label),
        Architecture::X86 => x86::emit_load_str_label(out, label),
    }
}

pub fn emit_load_var(out: &mut String, arch: Architecture, offset: i32, stack_offset: i32) {
    match arch {
        Architecture::ARM64 => arm64::emit_load_var(out, offset, stack_offset),
        Architecture::X64 => x64::emit_load_var(out, offset),
        Architecture::X86 => x86::emit_load_var(out, offset),
    }
}

pub fn emit_store_var(out: &mut String, arch: Architecture, offset: i32, stack_offset: i32) {
    match arch {
        Architecture::ARM64 => arm64::emit_store_var(out, offset, stack_offset),
        Architecture::X64 => x64::emit_store_var(out, offset),
        Architecture::X86 => x86::emit_store_var(out, offset),
    }
}

pub fn emit_allocate_var(out: &mut String, arch: Architecture, stack_offset: &mut i32) {
    match arch {
        Architecture::ARM64 => arm64::emit_allocate_var(out, stack_offset),
        Architecture::X64 => x64::emit_allocate_var(out, stack_offset),
        Architecture::X86 => x86::emit_allocate_var(out, stack_offset),
    }
}

pub fn emit_push_temp(out: &mut String, arch: Architecture) {
    match arch {
        Architecture::ARM64 => arm64::emit_push_temp(out),
        Architecture::X64 => x64::emit_push_temp(out),
        Architecture::X86 => x86::emit_push_temp(out),
    }
}

pub fn emit_binary_op(out: &mut String, arch: Architecture, op: BinaryOp) {
    match arch {
        Architecture::ARM64 => arm64::emit_binary_op(out, op),
        Architecture::X64 => x64::emit_binary_op(out, op),
        Architecture::X86 => x86::emit_binary_op(out, op),
    }
}

pub fn emit_unary_op(out: &mut String, arch: Architecture, op: UnaryOp) {
    match arch {
        Architecture::ARM64 => arm64::emit_unary_op(out, op),
        Architecture::X64 => x64::emit_unary_op(out, op),
        Architecture::X86 => x86::emit_unary_op(out, op),
    }
}

pub fn emit_jump_if_zero(out: &mut String, arch: Architecture, label: &str) {
    match arch {
        Architecture::ARM64 => arm64::emit_jump_if_zero(out, label),
        Architecture::X64 => x64::emit_jump_if_zero(out, label),
        Architecture::X86 => x86::emit_jump_if_zero(out, label),
    }
}

pub fn emit_jump(out: &mut String, arch: Architecture, label: &str) {
    match arch {
        Architecture::ARM64 => arm64::emit_jump(out, label),
        Architecture::X64 => x64::emit_jump(out, label),
        Architecture::X86 => x86::emit_jump(out, label),
    }
}

pub fn emit_compare_and_jump_if_greater(out: &mut String, arch: Architecture, label: &str) {
    match arch {
        Architecture::ARM64 => arm64::emit_compare_and_jump_if_greater(out, label),
        Architecture::X64 => x64::emit_compare_and_jump_if_greater(out, label),
        Architecture::X86 => x86::emit_compare_and_jump_if_greater(out, label),
    }
}

pub fn emit_increment_var(
    out: &mut String,
    arch: Architecture,
    var_offset: i32,
    stack_offset: i32,
    start_label: &str,
) {
    match arch {
        Architecture::ARM64 => {
            arm64::emit_increment_var(out, var_offset, stack_offset, start_label)
        }
        Architecture::X64 => x64::emit_increment_var(out, var_offset, start_label),
        Architecture::X86 => x86::emit_increment_var(out, var_offset, start_label),
    }
}

pub fn emit_function_prologue(out: &mut String, arch: Architecture, name: &str) {
    match arch {
        Architecture::ARM64 => arm64::emit_function_prologue(out, name),
        Architecture::X64 => x64::emit_function_prologue(out, name),
        Architecture::X86 => x86::emit_function_prologue(out, name),
    }
}

pub fn emit_function_epilogue(out: &mut String, arch: Architecture) {
    match arch {
        Architecture::ARM64 => arm64::emit_function_epilogue(out),
        Architecture::X64 => x64::emit_function_epilogue(out),
        Architecture::X86 => x86::emit_function_epilogue(out),
    }
}

pub fn emit_function_param_push(
    out: &mut String,
    arch: Architecture,
    param_idx: usize,
    stack_offset: &mut i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_function_param_push(out, param_idx, stack_offset),
        Architecture::X64 => x64::emit_function_param_push(out, param_idx, stack_offset, os),
        Architecture::X86 => x86::emit_function_param_push(out, param_idx, stack_offset),
    }
}

pub fn emit_function_call(
    out: &mut String,
    arch: Architecture,
    name: &str,
    args_count: usize,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_function_call(out, name, args_count),
        Architecture::X64 => x64::emit_function_call(out, name, args_count, stack_offset, os),
        Architecture::X86 => x86::emit_function_call(out, name, args_count),
    }
}

pub fn emit_say_str(
    out: &mut String,
    arch: Architecture,
    label: &str,
    fmt_label: &str,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_say_str(out, label, fmt_label, os),
        Architecture::X64 => x64::emit_say_str(out, label, fmt_label, stack_offset, os),
        Architecture::X86 => x86::emit_say_str(out, label, fmt_label),
    }
}

pub fn emit_say_str_lit(
    out: &mut String,
    arch: Architecture,
    label: &str,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_say_str_lit(out, label, os),
        Architecture::X64 => x64::emit_say_str_lit(out, label, stack_offset, os),
        Architecture::X86 => x86::emit_say_str_lit(out, label),
    }
}

pub fn emit_say_offset(
    out: &mut String,
    arch: Architecture,
    offset: i32,
    stack_offset: i32,
    fmt_label: &str,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_say_offset(out, offset, stack_offset, fmt_label, os),
        Architecture::X64 => x64::emit_say_offset(out, offset, fmt_label, stack_offset, os),
        Architecture::X86 => x86::emit_say_offset(out, offset, fmt_label),
    }
}

pub fn emit_say_num_const(
    out: &mut String,
    arch: Architecture,
    val: i64,
    fmt_label: &str,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_say_num_const(out, val, fmt_label, os),
        Architecture::X64 => x64::emit_say_num_const(out, val, fmt_label, stack_offset, os),
        Architecture::X86 => x86::emit_say_num_const(out, val, fmt_label),
    }
}

pub fn emit_say_acc(
    out: &mut String,
    arch: Architecture,
    fmt_label: &str,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_say_acc(out, fmt_label, os),
        Architecture::X64 => x64::emit_say_acc(out, fmt_label, stack_offset, os),
        Architecture::X86 => x86::emit_say_acc(out, fmt_label),
    }
}

pub fn emit_say_interpolated(
    out: &mut String,
    arch: Architecture,
    fmt_label: &str,
    count: usize,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_say_interpolated_pop_and_call(out, fmt_label, count, os),
        Architecture::X64 => {
            x64::emit_say_interpolated_pop_and_call(out, fmt_label, count, stack_offset, os)
        }
        Architecture::X86 => x86::emit_say_interpolated_call(out, fmt_label, count),
    }
}

pub fn emit_string_concat_call(
    out: &mut String,
    arch: Architecture,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_string_concat_call(out),
        Architecture::X64 => x64::emit_string_concat_call(out, stack_offset, os),
        Architecture::X86 => x86::emit_string_concat_call(out),
    }
}

pub fn emit_try_begin(
    out: &mut String,
    arch: Architecture,
    catch_label: &str,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_try_begin(out, catch_label, os),
        Architecture::X64 => x64::emit_try_begin(out, catch_label),
        Architecture::X86 => x86::emit_try_begin(out, catch_label),
    }
}

pub fn emit_try_end(
    out: &mut String,
    arch: Architecture,
    end_label: &str,
    stack_delta: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_try_end(out, end_label, stack_delta, os),
        Architecture::X64 => x64::emit_try_end(out, end_label, stack_delta),
        Architecture::X86 => x86::emit_try_end(out, end_label, stack_delta),
    }
}

pub fn emit_catch_begin(out: &mut String, arch: Architecture, catch_label: &str) {
    match arch {
        Architecture::ARM64 => arm64::emit_catch_begin(out, catch_label),
        Architecture::X64 => x64::emit_catch_begin(out, catch_label),
        Architecture::X86 => x86::emit_catch_begin(out, catch_label),
    }
}

pub fn emit_catch_load_err(out: &mut String, arch: Architecture, os: OperatingSystem) {
    match arch {
        Architecture::ARM64 => arm64::emit_catch_load_err(out, os),
        Architecture::X64 => x64::emit_catch_load_err(out),
        Architecture::X86 => x86::emit_catch_load_err(out),
    }
}

pub fn emit_catch_end(out: &mut String, arch: Architecture, stack_delta: i32) {
    match arch {
        Architecture::ARM64 => arm64::emit_catch_end(out, stack_delta),
        Architecture::X64 => x64::emit_catch_end(out, stack_delta),
        Architecture::X86 => x86::emit_catch_end(out, stack_delta),
    }
}

pub fn emit_pop_temp(out: &mut String, arch: Architecture) {
    match arch {
        Architecture::ARM64 => arm64::emit_pop_temp(out),
        Architecture::X64 => x64::emit_pop_temp(out),
        Architecture::X86 => x86::emit_pop_temp(out),
    }
}

pub fn emit_array_new(
    out: &mut String,
    arch: Architecture,
    count: usize,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_array_new(out, count),
        Architecture::X64 => x64::emit_array_new(out, count, stack_offset, os),
        Architecture::X86 => x86::emit_array_new(out, count),
    }
}

pub fn emit_array_set_imm(out: &mut String, arch: Architecture, index: usize) {
    match arch {
        Architecture::ARM64 => arm64::emit_array_set_imm(out, index),
        Architecture::X64 => x64::emit_array_set_imm(out, index),
        Architecture::X86 => x86::emit_array_set_imm(out, index),
    }
}

pub fn emit_array_get(out: &mut String, arch: Architecture) {
    match arch {
        Architecture::ARM64 => arm64::emit_array_get(out),
        Architecture::X64 => x64::emit_array_get(out),
        Architecture::X86 => x86::emit_array_get(out),
    }
}

pub fn emit_array_set(out: &mut String, arch: Architecture) {
    match arch {
        Architecture::ARM64 => arm64::emit_array_set(out),
        Architecture::X64 => x64::emit_array_set(out),
        Architecture::X86 => x86::emit_array_set(out),
    }
}

pub fn emit_array_len(out: &mut String, arch: Architecture) {
    match arch {
        Architecture::ARM64 => arm64::emit_array_len(out),
        Architecture::X64 => x64::emit_array_len(out),
        Architecture::X86 => x86::emit_array_len(out),
    }
}

pub fn emit_print_array(
    out: &mut String,
    arch: Architecture,
    stack_offset: i32,
    os: OperatingSystem,
) {
    match arch {
        Architecture::ARM64 => arm64::emit_print_array(out),
        Architecture::X64 => x64::emit_print_array(out, stack_offset, os),
        Architecture::X86 => x86::emit_print_array(out),
    }
}
