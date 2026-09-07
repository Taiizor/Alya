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
        out.push_str(".extern _fflush\n");
        out.push_str(".extern _calloc\n");
        out.push_str(".extern _realloc\n\n");
        out.push_str(".text\n");
        out.push_str(".align 2\n");
        out.push_str("_main:\n");
        out.push_str("    stp x29, x30, [sp, #-16]!\n");
        out.push_str("    mov x29, sp\n");
        emit_adrp_add(out, "x2", "alya_argc", os);
        out.push_str("    str x0, [x2]\n");
        emit_adrp_add(out, "x2", "alya_argv", os);
        out.push_str("    str x1, [x2]\n\n");
    } else {
        out.push_str(".global main\n");
        out.push_str(".extern printf\n");
        out.push_str(".extern exit\n");
        out.push_str(".extern getchar\n");
        out.push_str(".extern fflush\n");
        out.push_str(".extern calloc\n");
        out.push_str(".extern realloc\n\n");
        out.push_str(".text\n");
        out.push_str(".align 2\n");
        out.push_str("main:\n");
        out.push_str("    stp x29, x30, [sp, #-16]!\n");
        out.push_str("    mov x29, sp\n");
        emit_adrp_add(out, "x2", "alya_argc", os);
        out.push_str("    str x0, [x2]\n");
        emit_adrp_add(out, "x2", "alya_argv", os);
        out.push_str("    str x1, [x2]\n\n");
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

pub fn emit_load_float(out: &mut String, val: f64) {
    let bits = val.to_bits();
    out.push_str(&format!("    movz x0, #{}\n", bits & 0xFFFF));
    if (bits >> 16) & 0xFFFF != 0 {
        out.push_str(&format!(
            "    movk x0, #{}, lsl #16\n",
            (bits >> 16) & 0xFFFF
        ));
    }
    if (bits >> 32) & 0xFFFF != 0 {
        out.push_str(&format!(
            "    movk x0, #{}, lsl #32\n",
            (bits >> 32) & 0xFFFF
        ));
    }
    if (bits >> 48) & 0xFFFF != 0 {
        out.push_str(&format!(
            "    movk x0, #{}, lsl #48\n",
            (bits >> 48) & 0xFFFF
        ));
    }
    out.push_str("    fmov d0, x0\n");
}

pub fn emit_int_to_float(out: &mut String) {
    out.push_str("    scvtf d0, x0\n");
    out.push_str("    fmov x0, d0\n");
}

pub fn emit_float_to_int(out: &mut String) {
    out.push_str("    fmov d0, x0\n");
    out.push_str("    fcvtzs x0, d0\n");
}

pub fn emit_load_str_label(out: &mut String, label: &str, os: OperatingSystem) {
    emit_adrp_add(out, "x0", label, os);
}

pub fn emit_arm64_load_x29_offset(
    out: &mut String,
    dest_reg: &str,
    offset: i32,
    scratch_reg: &str,
) {
    if (0..=256).contains(&offset) {
        out.push_str(&format!("    ldr {}, [x29, #-{}]\n", dest_reg, offset));
    } else if (0..=4095).contains(&offset) {
        out.push_str(&format!("    sub {}, x29, #{}\n", scratch_reg, offset));
        out.push_str(&format!("    ldr {}, [{}]\n", dest_reg, scratch_reg));
    } else {
        out.push_str(&format!("    mov {}, #{}\n", scratch_reg, offset));
        out.push_str(&format!("    sub {}, x29, {}\n", scratch_reg, scratch_reg));
        out.push_str(&format!("    ldr {}, [{}]\n", dest_reg, scratch_reg));
    }
}

pub fn emit_arm64_store_x29_offset(
    out: &mut String,
    src_reg: &str,
    offset: i32,
    scratch_reg: &str,
) {
    if (0..=256).contains(&offset) {
        out.push_str(&format!("    str {}, [x29, #-{}]\n", src_reg, offset));
    } else if (0..=4095).contains(&offset) {
        out.push_str(&format!("    sub {}, x29, #{}\n", scratch_reg, offset));
        out.push_str(&format!("    str {}, [{}]\n", src_reg, scratch_reg));
    } else {
        out.push_str(&format!("    mov {}, #{}\n", scratch_reg, offset));
        out.push_str(&format!("    sub {}, x29, {}\n", scratch_reg, scratch_reg));
        out.push_str(&format!("    str {}, [{}]\n", src_reg, scratch_reg));
    }
}

pub fn emit_load_var(out: &mut String, offset: i32, _stack_offset: i32) {
    emit_arm64_load_x29_offset(out, "x0", offset, "x9");
}

pub fn emit_store_var(out: &mut String, offset: i32, _stack_offset: i32) {
    emit_arm64_store_x29_offset(out, "x0", offset, "x9");
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

pub fn emit_float_binary_op(out: &mut String, op: BinaryOp) {
    out.push_str("    fmov d1, x0\n");
    out.push_str("    ldr x1, [sp], #16\n");
    out.push_str("    fmov d0, x1\n");
    match op {
        BinaryOp::Add => {
            out.push_str("    fadd d0, d0, d1\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Subtract => {
            out.push_str("    fsub d0, d0, d1\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Multiply => {
            out.push_str("    fmul d0, d0, d1\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Divide => {
            out.push_str("    fdiv d0, d0, d1\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Modulo => {
            out.push_str("    fdiv d2, d0, d1\n");
            out.push_str("    fcvtzs x2, d2\n");
            out.push_str("    scvtf d2, x2\n");
            out.push_str("    fmul d2, d2, d1\n");
            out.push_str("    fsub d0, d0, d2\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Equal => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, eq\n");
        }
        BinaryOp::NotEqual => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, ne\n");
        }
        BinaryOp::Less => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, mi\n");
        }
        BinaryOp::Greater => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, gt\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, le\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    fcmp d0, d1\n");
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

pub fn emit_float_unary_op(out: &mut String, op: UnaryOp) {
    match op {
        UnaryOp::Negate => {
            out.push_str("    fmov d0, x0\n");
            out.push_str("    fneg d0, d0\n");
            out.push_str("    fmov x0, d0\n");
        }
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
    emit_arm64_load_x29_offset(out, "x1", offset, "x9");
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

pub fn emit_say_float(out: &mut String, fmt_label: &str, os: OperatingSystem) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    fmov d0, x0\n");
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
    is_floats: &[bool],
    os: OperatingSystem,
) {
    let count = is_floats.len();
    for i in (0..count).rev() {
        out.push_str(&format!("    ldr x{}, [sp], #16\n", i + 1));
    }
    if !matches!(os, OperatingSystem::MacOS) {
        let mut d_idx = 0;
        for (i, &is_flt) in is_floats.iter().enumerate() {
            if is_flt && d_idx < 8 {
                out.push_str(&format!("    fmov d{}, x{}\n", d_idx, i + 1));
                d_idx += 1;
            }
        }
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
    out.push_str("    ldr x1, [x1, #16]\n");
    out.push_str(&format!("    mov x2, #{}\n", index * 8));
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
    out.push_str("    ldr x0, [x0, #16]\n");
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
    out.push_str("    ldr x0, [x0, #16]\n");
    out.push_str("    str x2, [x0, x1, lsl #3]\n");
}

pub fn emit_array_push(out: &mut String) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    bl alya_array_push\n");
}

pub fn emit_array_pop(out: &mut String) {
    out.push_str("    bl alya_array_pop\n");
}

pub fn emit_array_len(out: &mut String) {
    out.push_str("    cbz x0, 1f\n");
    out.push_str("    ldr x0, [x0]\n");
    out.push_str("1:\n");
}

pub fn emit_print_array(out: &mut String) {
    out.push_str("    bl alya_print_array\n");
}

pub fn emit_print_map(out: &mut String) {
    out.push_str("    bl alya_print_map\n");
}

pub fn emit_struct_new(
    out: &mut String,
    desc_label: &str,
    field_count: usize,
    os: OperatingSystem,
) {
    emit_adrp_add(out, "x0", desc_label, os);
    out.push_str(&format!("    mov x1, #{}\n", field_count));
    out.push_str("    bl alya_struct_new\n");
}

pub fn emit_struct_field_get(out: &mut String, field_idx: usize) {
    out.push_str(&format!("    ldr x0, [x0, #{}]\n", (field_idx + 1) * 8));
}

pub fn emit_struct_field_set_imm(out: &mut String, field_idx: usize) {
    out.push_str("    ldr x1, [sp]\n");
    out.push_str(&format!("    str x0, [x1, #{}]\n", (field_idx + 1) * 8));
}

pub fn emit_struct_field_set(out: &mut String, field_idx: usize) {
    out.push_str("    ldr x1, [sp], #16\n");
    out.push_str(&format!("    str x0, [x1, #{}]\n", (field_idx + 1) * 8));
}

pub fn emit_print_struct(out: &mut String) {
    out.push_str("    bl alya_print_struct\n");
}

pub fn emit_for_each_load_element(
    out: &mut String,
    arr_offset: i32,
    idx_offset: i32,
    var_offset: i32,
    end_label: &str,
) {
    emit_arm64_load_x29_offset(out, "x0", arr_offset, "x9");
    out.push_str(&format!("    cbz x0, {}\n", end_label));
    out.push_str("    ldr x1, [x0]\n");
    emit_arm64_load_x29_offset(out, "x2", idx_offset, "x9");
    out.push_str("    cmp x2, x1\n");
    out.push_str(&format!("    b.ge {}\n", end_label));
    out.push_str("    ldr x3, [x0, #16]\n");
    out.push_str("    ldr x0, [x3, x2, lsl #3]\n");
    emit_arm64_store_x29_offset(out, "x0", var_offset, "x9");
}

pub fn emit_string_equality_call(out: &mut String, op: BinaryOp) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    bl fn_streq\n");
    if matches!(op, BinaryOp::NotEqual) {
        out.push_str("    eor x0, x0, #1\n");
    }
}
