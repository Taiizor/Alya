use super::loads::{emit_arm64_load_x29_offset, emit_arm64_store_x29_offset};
use crate::codegen::target::OperatingSystem;

pub fn emit_jump_if_zero(out: &mut String, label: &str) {
    out.push_str(&format!("    cbz x0, {}\n", label));
}

pub fn emit_jump(out: &mut String, label: &str) {
    out.push_str(&format!("    b {}\n", label));
}

pub fn emit_compare_and_jump_if_greater(out: &mut String, label: &str) {
    out.push_str("    ldr x1, [sp], #16\n");
    out.push_str("    cmp x1, x0\n");
    out.push_str(&format!("    b.gt {}\n", label));
}

pub fn emit_increment_var(
    out: &mut String,
    var_offset: i32,
    _stack_offset: i32,
    start_label: &str,
) {
    emit_arm64_load_x29_offset(out, "x0", var_offset, "x9");
    out.push_str("    add x0, x0, #1\n");
    emit_arm64_store_x29_offset(out, "x0", var_offset, "x9");
    out.push_str(&format!("    b {}\n", start_label));
}

pub fn emit_function_prologue(out: &mut String, name: &str) {
    out.push_str(&format!("\n.globl fn_{}\n", name));
    out.push_str(&format!("fn_{}:\n", name));
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n\n");
}

pub fn emit_function_param_push(out: &mut String, param_idx: usize, stack_offset: &mut i32) {
    *stack_offset += 16;
    let reg = match param_idx {
        0 => "x0",
        1 => "x1",
        2 => "x2",
        3 => "x3",
        4 => "x4",
        5 => "x5",
        6 => "x6",
        7 => "x7",
        _ => "x0",
    };
    out.push_str(&format!("    str {}, [sp, #-16]!\n", reg));
}

pub fn emit_function_epilogue(out: &mut String) {
    out.push_str("    mov sp, x29\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");
}

pub fn emit_function_call(out: &mut String, name: &str, args_count: usize) {
    for i in (0..args_count).rev() {
        let reg = match i {
            0 => "x0",
            1 => "x1",
            2 => "x2",
            3 => "x3",
            4 => "x4",
            5 => "x5",
            6 => "x6",
            7 => "x7",
            _ => "x0",
        };
        out.push_str(&format!("    ldr {}, [sp], #16\n", reg));
    }
    out.push_str(&format!("    bl fn_{}\n", name));
}

pub fn emit_c_function_call(out: &mut String, name: &str, args_count: usize, os: OperatingSystem) {
    for i in (0..args_count).rev() {
        let reg = match i {
            0 => "x0",
            1 => "x1",
            2 => "x2",
            3 => "x3",
            4 => "x4",
            5 => "x5",
            6 => "x6",
            7 => "x7",
            _ => "x0",
        };
        out.push_str(&format!("    ldr {}, [sp], #16\n", reg));
    }
    let target = if matches!(os, OperatingSystem::MacOS) {
        format!("_{}", name)
    } else {
        name.to_string()
    };
    out.push_str(&format!("    bl {}\n", target));
}

pub fn emit_stack_restore(out: &mut String, delta: i32) {
    out.push_str(&format!("    add sp, sp, #{}\n", delta));
}
