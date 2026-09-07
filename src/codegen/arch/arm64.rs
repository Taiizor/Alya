use crate::ast::{BinaryOp, UnaryOp};

pub fn emit_header(out: &mut String) {
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

pub fn emit_footer(out: &mut String) {
    out.push_str("    mov w0, #0\n");
    out.push_str("    mov sp, x29\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n");
}

pub fn emit_call_printf(out: &mut String) {
    out.push_str("    bl printf\n");
}

pub fn emit_load_num(out: &mut String, val: i64) {
    out.push_str(&format!("    mov x0, #{}\n", val));
}

pub fn emit_load_str_label(out: &mut String, label: &str) {
    out.push_str(&format!("    adrp x0, {}@PAGE\n", label));
    out.push_str(&format!("    add x0, x0, {}@PAGEOFF\n", label));
}

pub fn emit_load_var(out: &mut String, offset: i32, stack_offset: i32) {
    out.push_str(&format!("    ldr x0, [sp, #{}]\n", stack_offset - offset));
}

pub fn emit_store_var(out: &mut String, offset: i32, stack_offset: i32) {
    out.push_str(&format!("    str x0, [sp, #{}]\n", stack_offset - offset));
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
        BinaryOp::Greater => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, gt\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, le\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, ge\n");
        }
        BinaryOp::And => out.push_str("    and x0, x1, x0\n"),
        BinaryOp::Or => out.push_str("    orr x0, x1, x0\n"),
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

pub fn emit_increment_var(out: &mut String, var_offset: i32, stack_offset: i32, start_label: &str) {
    out.push_str(&format!(
        "    ldr x0, [sp, #{}]\n",
        stack_offset - var_offset
    ));
    out.push_str("    add x0, x0, #1\n");
    out.push_str(&format!(
        "    str x0, [sp, #{}]\n",
        stack_offset - var_offset
    ));
    out.push_str(&format!("    b {}\n", start_label));
}

pub fn emit_function_prologue(out: &mut String, name: &str) {
    out.push_str(&format!("\n.global fn_{}\n", name));
    out.push_str(&format!("fn_{}:\n", name));
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
}

pub fn emit_function_epilogue(out: &mut String) {
    out.push_str("    mov sp, x29\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n");
}

pub fn emit_function_param_push(out: &mut String, param_idx: usize, stack_offset: &mut i32) {
    *stack_offset += 16;
    out.push_str(&format!("    str x{}, [sp, #-16]!\n", param_idx));
}

pub fn emit_function_call(out: &mut String, name: &str, args_count: usize) {
    for i in (0..args_count).rev() {
        out.push_str(&format!("    ldr x{}, [sp], #16\n", i));
    }
    out.push_str(&format!("    bl fn_{}\n", name));
}

pub fn emit_say_str(out: &mut String, label: &str, fmt_label: &str) {
    out.push_str(&format!("    adrp x1, {}@PAGE\n", label));
    out.push_str(&format!("    add x1, x1, {}@PAGEOFF\n", label));
    out.push_str(&format!("    adrp x0, {}@PAGE\n", fmt_label));
    out.push_str(&format!("    add x0, x0, {}@PAGEOFF\n", fmt_label));
    emit_call_printf(out);
}

pub fn emit_say_str_lit(out: &mut String, label: &str) {
    out.push_str(&format!("    adrp x0, {}@PAGE\n", label));
    out.push_str(&format!("    add x0, x0, {}@PAGEOFF\n", label));
    emit_call_printf(out);
}

pub fn emit_say_offset(out: &mut String, offset: i32, stack_offset: i32, fmt_label: &str) {
    out.push_str(&format!("    ldr x1, [sp, #{}]\n", stack_offset - offset));
    out.push_str(&format!("    adrp x0, {}@PAGE\n", fmt_label));
    out.push_str(&format!("    add x0, x0, {}@PAGEOFF\n", fmt_label));
    emit_call_printf(out);
}

pub fn emit_say_num_const(out: &mut String, val: i64, fmt_label: &str) {
    out.push_str(&format!("    mov x1, #{}\n", val));
    out.push_str(&format!("    adrp x0, {}@PAGE\n", fmt_label));
    out.push_str(&format!("    add x0, x0, {}@PAGEOFF\n", fmt_label));
    emit_call_printf(out);
}

pub fn emit_say_acc(out: &mut String, fmt_label: &str) {
    out.push_str("    mov x1, x0\n");
    out.push_str(&format!("    adrp x0, {}@PAGE\n", fmt_label));
    out.push_str(&format!("    add x0, x0, {}@PAGEOFF\n", fmt_label));
    emit_call_printf(out);
}

pub fn emit_say_interpolated_pop_and_call(out: &mut String, fmt_label: &str, count: usize) {
    for i in (0..count).rev() {
        out.push_str(&format!("    ldr x{}, [sp], #16\n", i + 1));
    }
    out.push_str(&format!("    adrp x0, {}@PAGE\n", fmt_label));
    out.push_str(&format!("    add x0, x0, {}@PAGEOFF\n", fmt_label));
    emit_call_printf(out);
}

pub fn emit_string_concat_call(out: &mut String) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    bl alya_concat\n");
}

pub fn emit_try_begin(out: &mut String, catch_label: &str) {
    out.push_str("    adrp x9, alya_catch_idx\n");
    out.push_str("    add x9, x9, :lo12:alya_catch_idx\n");
    out.push_str("    ldr x10, [x9]\n");
    out.push_str(&format!("    adrp x11, {}\n", catch_label));
    out.push_str(&format!("    add x11, x11, :lo12:{}\n", catch_label));
    out.push_str("    adrp x12, alya_catch_stack_handler\n");
    out.push_str("    add x12, x12, :lo12:alya_catch_stack_handler\n");
    out.push_str("    str x11, [x12, x10, lsl #3]\n");
    out.push_str("    mov x13, sp\n");
    out.push_str("    adrp x12, alya_catch_stack_sp\n");
    out.push_str("    add x12, x12, :lo12:alya_catch_stack_sp\n");
    out.push_str("    str x13, [x12, x10, lsl #3]\n");
    out.push_str("    adrp x12, alya_catch_stack_bp\n");
    out.push_str("    add x12, x12, :lo12:alya_catch_stack_bp\n");
    out.push_str("    str x29, [x12, x10, lsl #3]\n");
    out.push_str("    add x10, x10, #1\n");
    out.push_str("    str x10, [x9]\n");
}

pub fn emit_try_end(out: &mut String, end_label: &str, stack_delta: i32) {
    out.push_str("    adrp x9, alya_catch_idx\n");
    out.push_str("    add x9, x9, :lo12:alya_catch_idx\n");
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

pub fn emit_catch_load_err(out: &mut String) {
    out.push_str("    adrp x9, alya_err_msg\n");
    out.push_str("    add x9, x9, :lo12:alya_err_msg\n");
    out.push_str("    ldr x0, [x9]\n");
}

pub fn emit_catch_end(out: &mut String, stack_delta: i32) {
    if stack_delta > 0 {
        out.push_str(&format!("    add sp, sp, #{}\n", stack_delta));
    }
}
