use crate::ast::{BinaryOp, UnaryOp};
use crate::codegen::target::OperatingSystem;

pub fn emit_adrp_add(out: &mut String, reg: &str, label: &str, os: OperatingSystem) {
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str(&format!("    adrp {}, {}@PAGE\n", reg, label));
        out.push_str(&format!("    add {}, {}, {}@PAGEOFF\n", reg, reg, label));
    } else {
        out.push_str(&format!("    adrp {}, {}\n", reg, label));
        out.push_str(&format!("    add {}, {}, :lo12:{}\n", reg, reg, label));
    }
}

pub fn emit_header(out: &mut String, os: OperatingSystem) {
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str(".globl _main\n");
        out.push_str(".extern _printf\n");
        out.push_str(".extern _exit\n");
        out.push_str(".extern _getchar\n");
        out.push_str(".extern _fflush\n\n");
        out.push_str(".text\n");
        out.push_str(".align 2\n");
        out.push_str("_main:\n");
        out.push_str("    stp x29, x30, [sp, #-16]!\n");
        out.push_str("    mov x29, sp\n\n");
    } else {
        out.push_str(".global main\n");
        out.push_str(".extern printf\n");
        out.push_str(".extern exit\n");
        out.push_str(".extern getchar\n");
        out.push_str(".extern fflush\n\n");
        out.push_str(".text\n");
        out.push_str(".align 2\n");
        out.push_str("main:\n");
        out.push_str("    stp x29, x30, [sp, #-16]!\n");
        out.push_str("    mov x29, sp\n\n");
    }
}

pub fn emit_footer(out: &mut String) {
    out.push_str("    mov w0, #0\n");
    out.push_str("    mov sp, x29\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n");
}

pub fn emit_call_printf(out: &mut String, os: OperatingSystem) {
    let p = if matches!(os, OperatingSystem::MacOS) {
        "_"
    } else {
        ""
    };
    out.push_str(&format!("    bl {}printf\n", p));
}

pub fn emit_load_num(out: &mut String, val: i64) {
    out.push_str(&format!("    mov x0, #{}\n", val));
}

pub fn emit_load_str_label(out: &mut String, label: &str, os: OperatingSystem) {
    emit_adrp_add(out, "x0", label, os);
}

pub fn emit_load_var(out: &mut String, offset: i32, _stack_offset: i32) {
    out.push_str(&format!("    ldr x0, [x29, #-{}]\n", offset));
}

pub fn emit_store_var(out: &mut String, offset: i32, _stack_offset: i32) {
    out.push_str(&format!("    str x0, [x29, #-{}]\n", offset));
}

pub fn emit_allocate_var(out: &mut String, stack_offset: &mut i32) {
    *stack_offset += 16;
    out.push_str("    str x0, [sp, #-16]!\n");
}

pub fn emit_push_temp(out: &mut String) {
    out.push_str("    str x0, [sp, #-16]!\n");
}

pub fn emit_binary_op(out: &mut String, op: BinaryOp) {
    out.push_str("    ldr x1, [sp], #16\n");
    match op {
        BinaryOp::Add => out.push_str("    add x0, x1, x0\n"),
        BinaryOp::Subtract => out.push_str("    sub x0, x1, x0\n"),
        BinaryOp::Multiply => out.push_str("    mul x0, x1, x0\n"),
        BinaryOp::Divide => {
            out.push_str("    cbz x0, alya_error_div_zero\n");
            out.push_str("    sdiv x0, x1, x0\n");
        }
        BinaryOp::Modulo => {
            out.push_str("    cbz x0, alya_error_div_zero\n");
            out.push_str("    sdiv x2, x1, x0\n");
            out.push_str("    msub x0, x2, x0, x1\n");
        }
        BinaryOp::Equal => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, eq\n");
        }
        BinaryOp::NotEqual => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, ne\n");
        }
        BinaryOp::Less => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, lt\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, le\n");
        }
        BinaryOp::Greater => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, gt\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, ge\n");
        }
        BinaryOp::And => {
            out.push_str("    and x0, x1, x0\n");
        }
        BinaryOp::Or => {
            out.push_str("    orr x0, x1, x0\n");
        }
    }
}

pub fn emit_unary_op(out: &mut String, op: UnaryOp) {
    match op {
        UnaryOp::Negate => out.push_str("    neg x0, x0\n"),
        UnaryOp::Not => {
            out.push_str("    cmp x0, #0\n");
            out.push_str("    cset x0, eq\n");
        }
    }
}

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
    out.push_str(&format!("    ldr x0, [x29, #-{}]\n", var_offset));
    out.push_str("    add x0, x0, #1\n");
    out.push_str(&format!("    str x0, [x29, #-{}]\n", var_offset));
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

pub fn emit_say_str(out: &mut String, label: &str, fmt_label: &str, os: OperatingSystem) {
    emit_adrp_add(out, "x1", label, os);
    emit_adrp_add(out, "x0", fmt_label, os);
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str("    sub sp, sp, #16\n");
        out.push_str("    str x1, [sp]\n");
        emit_call_printf(out, os);
        out.push_str("    add sp, sp, #16\n");
    } else {
        emit_call_printf(out, os);
    }
}

pub fn emit_say_str_lit(out: &mut String, label: &str, os: OperatingSystem) {
    emit_adrp_add(out, "x0", label, os);
    emit_call_printf(out, os);
}

pub fn emit_say_offset(
    out: &mut String,
    offset: i32,
    _stack_offset: i32,
    fmt_label: &str,
    os: OperatingSystem,
) {
    out.push_str(&format!("    ldr x1, [x29, #-{}]\n", offset));
    emit_adrp_add(out, "x0", fmt_label, os);
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str("    sub sp, sp, #16\n");
        out.push_str("    str x1, [sp]\n");
        emit_call_printf(out, os);
        out.push_str("    add sp, sp, #16\n");
    } else {
        emit_call_printf(out, os);
    }
}

pub fn emit_say_num_const(out: &mut String, val: i64, fmt_label: &str, os: OperatingSystem) {
    out.push_str(&format!("    mov x1, #{}\n", val));
    emit_adrp_add(out, "x0", fmt_label, os);
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str("    sub sp, sp, #16\n");
        out.push_str("    str x1, [sp]\n");
        emit_call_printf(out, os);
        out.push_str("    add sp, sp, #16\n");
    } else {
        emit_call_printf(out, os);
    }
}

pub fn emit_say_acc(out: &mut String, fmt_label: &str, os: OperatingSystem) {
    out.push_str("    mov x1, x0\n");
    emit_adrp_add(out, "x0", fmt_label, os);
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str("    sub sp, sp, #16\n");
        out.push_str("    str x1, [sp]\n");
        emit_call_printf(out, os);
        out.push_str("    add sp, sp, #16\n");
    } else {
        emit_call_printf(out, os);
    }
}

pub fn emit_say_interpolated_pop_and_call(
    out: &mut String,
    fmt_label: &str,
    count: usize,
    os: OperatingSystem,
) {
    for i in (0..count).rev() {
        out.push_str(&format!("    ldr x{}, [sp], #16\n", i + 1));
    }
    emit_adrp_add(out, "x0", fmt_label, os);
    if matches!(os, OperatingSystem::MacOS) {
        let stack_space = (count * 8).div_ceil(16) * 16;
        out.push_str(&format!("    sub sp, sp, #{}\n", stack_space));
        for i in 0..count {
            out.push_str(&format!("    str x{}, [sp, #{}]\n", i + 1, i * 8));
        }
        emit_call_printf(out, os);
        out.push_str(&format!("    add sp, sp, #{}\n", stack_space));
    } else {
        emit_call_printf(out, os);
    }
}

pub fn emit_string_concat_call(out: &mut String) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    bl alya_concat\n");
}

pub fn emit_try_begin(out: &mut String, catch_label: &str, os: OperatingSystem) {
    emit_adrp_add(out, "x9", "alya_catch_idx", os);
    out.push_str("    ldr x10, [x9]\n");
    emit_adrp_add(out, "x11", catch_label, os);
    emit_adrp_add(out, "x12", "alya_catch_stack_handler", os);
    out.push_str("    str x11, [x12, x10, lsl #3]\n");
    out.push_str("    mov x13, sp\n");
    emit_adrp_add(out, "x12", "alya_catch_stack_sp", os);
    out.push_str("    str x13, [x12, x10, lsl #3]\n");
    emit_adrp_add(out, "x12", "alya_catch_stack_bp", os);
    out.push_str("    str x29, [x12, x10, lsl #3]\n");
    out.push_str("    add x10, x10, #1\n");
    out.push_str("    str x10, [x9]\n");
}

pub fn emit_try_end(out: &mut String, end_label: &str, stack_delta: i32, os: OperatingSystem) {
    emit_adrp_add(out, "x9", "alya_catch_idx", os);
    out.push_str("    ldr x10, [x9]\n");
    out.push_str("    sub x10, x10, #1\n");
    out.push_str("    str x10, [x9]\n");
    if stack_delta > 0 {
        out.push_str(&format!("    add sp, sp, #{}\n", stack_delta));
    }
    out.push_str(&format!("    b {}\n", end_label));
}

pub fn emit_catch_begin(out: &mut String, catch_label: &str) {
    out.push_str(&format!("{}:\n", catch_label));
}

pub fn emit_catch_load_err(out: &mut String, os: OperatingSystem) {
    emit_adrp_add(out, "x9", "alya_err_msg", os);
    out.push_str("    ldr x0, [x9]\n");
}

pub fn emit_catch_end(out: &mut String, stack_delta: i32) {
    if stack_delta > 0 {
        out.push_str(&format!("    add sp, sp, #{}\n", stack_delta));
    }
}

pub fn emit_pop_temp(out: &mut String) {
    out.push_str("    ldr x0, [sp], #16\n");
}

pub fn emit_array_new(out: &mut String, count: usize) {
    out.push_str(&format!("    mov x0, #{}\n", count));
    out.push_str("    bl alya_array_new\n");
}

pub fn emit_array_set_imm(out: &mut String, index: usize) {
    out.push_str("    ldr x1, [sp]\n");
    out.push_str(&format!("    mov x2, #{}\n", (index + 1) * 8));
    out.push_str("    str x0, [x1, x2]\n");
}

pub fn emit_array_get(out: &mut String) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    cmp x1, #0\n");
    out.push_str("    b.lt alya_error_index_out_of_bounds\n");
    out.push_str("    ldr x2, [x0]\n");
    out.push_str("    cmp x1, x2\n");
    out.push_str("    b.ge alya_error_index_out_of_bounds\n");
    out.push_str("    add x1, x1, #1\n");
    out.push_str("    ldr x0, [x0, x1, lsl #3]\n");
}

pub fn emit_array_set(out: &mut String) {
    out.push_str("    mov x2, x0\n");
    out.push_str("    ldr x1, [sp], #16\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    cmp x1, #0\n");
    out.push_str("    b.lt alya_error_index_out_of_bounds\n");
    out.push_str("    ldr x3, [x0]\n");
    out.push_str("    cmp x1, x3\n");
    out.push_str("    b.ge alya_error_index_out_of_bounds\n");
    out.push_str("    add x1, x1, #1\n");
    out.push_str("    str x2, [x0, x1, lsl #3]\n");
}

pub fn emit_array_len(out: &mut String) {
    out.push_str("    cbz x0, 1f\n");
    out.push_str("    ldr x0, [x0]\n");
    out.push_str("1:\n");
}

pub fn emit_print_array(out: &mut String) {
    out.push_str("    bl alya_print_array\n");
}
