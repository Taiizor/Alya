use super::{arm64, x64, x86};
use crate::codegen::target::{Architecture, OperatingSystem};

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

