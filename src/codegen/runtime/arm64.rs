use crate::codegen::target::OperatingSystem;

fn emit_adrp_add(out: &mut String, reg: &str, label: &str, os: OperatingSystem) {
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str(&format!("    adrp {}, {}@PAGE\n", reg, label));
        out.push_str(&format!("    add {}, {}, {}@PAGEOFF\n", reg, reg, label));
    } else {
        out.push_str(&format!("    adrp {}, {}\n", reg, label));
        out.push_str(&format!("    add {}, {}, :lo12:{}\n", reg, reg, label));
    }
}

pub fn emit_arm64_runtime(out: &mut String, os: OperatingSystem) {
    let p = if matches!(os, OperatingSystem::MacOS) {
        "_"
    } else {
        ""
    };

    // alya_concat
    out.push_str(".align 2\n");
    out.push_str("alya_concat:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #-16]!\n");
    out.push_str("    stp x21, x22, [sp, #-16]!\n");
    emit_adrp_add(out, "x19", "alya_str_buf", os);
    emit_adrp_add(out, "x20", "alya_str_idx", os);
    out.push_str("    ldr x21, [x20]\n");
    out.push_str("    cmp x21, #48000\n");
    out.push_str("    b.lt .L_arm_concat_ok\n");
    out.push_str("    mov x21, #0\n");
    out.push_str(".L_arm_concat_ok:\n");
    out.push_str("    add x22, x19, x21\n");
    out.push_str(".L_arm_copy1:\n");
    out.push_str("    ldrb w2, [x0], #1\n");
    out.push_str("    cbz w2, .L_arm_copy2_start\n");
    out.push_str("    strb w2, [x22], #1\n");
    out.push_str("    b .L_arm_copy1\n");
    out.push_str(".L_arm_copy2_start:\n");
    out.push_str(".L_arm_copy2:\n");
    out.push_str("    ldrb w2, [x1], #1\n");
    out.push_str("    cbz w2, .L_arm_concat_end\n");
    out.push_str("    strb w2, [x22], #1\n");
    out.push_str("    b .L_arm_copy2\n");
    out.push_str(".L_arm_concat_end:\n");
    out.push_str("    strb wzr, [x22], #1\n");
    out.push_str("    sub x2, x22, x19\n");
    out.push_str("    add x2, x2, #7\n");
    out.push_str("    and x2, x2, #~7\n");
    out.push_str("    str x2, [x20]\n");
    out.push_str("    add x0, x19, x21\n");
    out.push_str("    ldp x21, x22, [sp], #16\n");
    out.push_str("    ldp x19, x20, [sp], #16\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // fn_len
    out.push_str("fn_len:\n");
    out.push_str("    mov x1, x0\n");
    out.push_str("    mov x0, #0\n");
    out.push_str(".L_arm_len_loop:\n");
    out.push_str("    ldrb w2, [x1, x0]\n");
    out.push_str("    cbz w2, .L_arm_len_end\n");
    out.push_str("    add x0, x0, #1\n");
    out.push_str("    b .L_arm_len_loop\n");
    out.push_str(".L_arm_len_end:\n");
    out.push_str("    ret\n\n");

    // fn_abs
    out.push_str("fn_abs:\n");
    out.push_str("    cmp x0, #0\n");
    out.push_str("    b.ge .L_arm_abs_end\n");
    out.push_str("    neg x0, x0\n");
    out.push_str(".L_arm_abs_end:\n");
    out.push_str("    ret\n\n");

    // fn_min
    out.push_str("fn_min:\n");
    out.push_str("    cmp x0, x1\n");
    out.push_str("    csel x0, x0, x1, le\n");
    out.push_str("    ret\n\n");

    // fn_max
    out.push_str("fn_max:\n");
    out.push_str("    cmp x0, x1\n");
    out.push_str("    csel x0, x0, x1, ge\n");
    out.push_str("    ret\n\n");

    // fn_exit
    out.push_str("fn_exit:\n");
    out.push_str(&format!("    b {}exit\n\n", p));

    // fn_ask
    out.push_str("fn_ask:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #-16]!\n");
    out.push_str("    stp x21, x22, [sp, #-16]!\n");
    out.push_str("    cbz x0, .L_arm_ask_read\n");
    out.push_str("    mov x1, x0\n");
    emit_adrp_add(out, "x0", "alya_fmt_prompt", os);
    out.push_str(&format!("    bl {}printf\n", p));
    out.push_str("    mov x0, #0\n");
    out.push_str(&format!("    bl {}fflush\n", p));
    out.push_str(".L_arm_ask_read:\n");
    emit_adrp_add(out, "x19", "alya_str_buf", os);
    emit_adrp_add(out, "x20", "alya_str_idx", os);
    out.push_str("    ldr x2, [x20]\n");
    out.push_str("    cmp x2, #48000\n");
    out.push_str("    b.lt .L_arm_ask_buf_ok\n");
    out.push_str("    mov x2, #0\n");
    out.push_str(".L_arm_ask_buf_ok:\n");
    out.push_str("    add x19, x19, x2\n");
    out.push_str("    mov x21, x19\n");
    out.push_str(".L_arm_ask_loop:\n");
    out.push_str(&format!("    bl {}getchar\n", p));
    out.push_str("    cmp w0, #-1\n");
    out.push_str("    b.eq .L_arm_ask_done\n");
    out.push_str("    cmp w0, #10\n");
    out.push_str("    b.eq .L_arm_ask_done\n");
    out.push_str("    cmp w0, #13\n");
    out.push_str("    b.eq .L_arm_ask_loop\n");
    out.push_str("    strb w0, [x19], #1\n");
    out.push_str("    b .L_arm_ask_loop\n");
    out.push_str(".L_arm_ask_done:\n");
    out.push_str("    strb wzr, [x19], #1\n");
    emit_adrp_add(out, "x1", "alya_str_buf", os);
    out.push_str("    sub x2, x19, x1\n");
    out.push_str("    add x2, x2, #7\n");
    out.push_str("    and x2, x2, #~7\n");
    out.push_str("    str x2, [x20]\n");
    out.push_str("    mov x0, x21\n");
    out.push_str("    ldp x21, x22, [sp], #16\n");
    out.push_str("    ldp x19, x20, [sp], #16\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // alya_error_div_zero
    out.push_str("alya_error_div_zero:\n");
    emit_adrp_add(out, "x9", "alya_catch_idx", os);
    out.push_str("    ldr x10, [x9]\n");
    out.push_str("    cbz x10, .L_arm_fatal_div_zero\n");
    out.push_str("    sub x10, x10, #1\n");
    out.push_str("    str x10, [x9]\n");
    emit_adrp_add(out, "x11", "alya_str_div_zero", os);
    emit_adrp_add(out, "x12", "alya_err_msg", os);
    out.push_str("    str x11, [x12]\n");
    emit_adrp_add(out, "x11", "alya_catch_stack_sp", os);
    out.push_str("    ldr x13, [x11, x10, lsl #3]\n");
    out.push_str("    mov sp, x13\n");
    emit_adrp_add(out, "x11", "alya_catch_stack_bp", os);
    out.push_str("    ldr x29, [x11, x10, lsl #3]\n");
    emit_adrp_add(out, "x11", "alya_catch_stack_handler", os);
    out.push_str("    ldr x14, [x11, x10, lsl #3]\n");
    out.push_str("    br x14\n");
    out.push_str(".L_arm_fatal_div_zero:\n");
    out.push_str("    mov x19, sp\n");
    out.push_str("    and x19, x19, #~15\n");
    out.push_str("    mov sp, x19\n");
    emit_adrp_add(out, "x0", "alya_fmt_div_zero", os);
    out.push_str(&format!("    bl {}printf\n", p));
    out.push_str("    mov w0, #1\n");
    out.push_str(&format!("    bl {}exit\n\n", p));
}
