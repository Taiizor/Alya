pub fn emit_arm64_runtime(out: &mut String) {
    // alya_concat
    out.push_str(".align 2\n");
    out.push_str("alya_concat:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #-16]!\n");
    out.push_str("    stp x21, x22, [sp, #-16]!\n");
    out.push_str("    adrp x19, alya_str_buf\n");
    out.push_str("    add x19, x19, :lo12:alya_str_buf\n");
    out.push_str("    adrp x20, alya_str_idx\n");
    out.push_str("    add x20, x20, :lo12:alya_str_idx\n");
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
    out.push_str("    b exit\n\n");

    // fn_ask
    out.push_str("fn_ask:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #-16]!\n");
    out.push_str("    stp x21, x22, [sp, #-16]!\n");
    out.push_str("    cbz x0, .L_arm_ask_read\n");
    out.push_str("    mov x1, x0\n");
    out.push_str("    adrp x0, alya_fmt_prompt\n");
    out.push_str("    add x0, x0, :lo12:alya_fmt_prompt\n");
    out.push_str("    bl printf\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    bl fflush\n");
    out.push_str(".L_arm_ask_read:\n");
    out.push_str("    adrp x19, alya_str_buf\n");
    out.push_str("    add x19, x19, :lo12:alya_str_buf\n");
    out.push_str("    adrp x20, alya_str_idx\n");
    out.push_str("    add x20, x20, :lo12:alya_str_idx\n");
    out.push_str("    ldr x2, [x20]\n");
    out.push_str("    cmp x2, #48000\n");
    out.push_str("    b.lt .L_arm_ask_buf_ok\n");
    out.push_str("    mov x2, #0\n");
    out.push_str(".L_arm_ask_buf_ok:\n");
    out.push_str("    add x19, x19, x2\n");
    out.push_str("    mov x21, x19\n");
    out.push_str(".L_arm_ask_loop:\n");
    out.push_str("    bl getchar\n");
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
    out.push_str("    adrp x1, alya_str_buf\n");
    out.push_str("    add x1, x1, :lo12:alya_str_buf\n");
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
    out.push_str("    adrp x9, alya_catch_idx\n");
    out.push_str("    add x9, x9, :lo12:alya_catch_idx\n");
    out.push_str("    ldr x10, [x9]\n");
    out.push_str("    cbz x10, .L_arm_fatal_div_zero\n");
    out.push_str("    sub x10, x10, #1\n");
    out.push_str("    str x10, [x9]\n");
    out.push_str("    adrp x11, alya_str_div_zero\n");
    out.push_str("    add x11, x11, :lo12:alya_str_div_zero\n");
    out.push_str("    adrp x12, alya_err_msg\n");
    out.push_str("    add x12, x12, :lo12:alya_err_msg\n");
    out.push_str("    str x11, [x12]\n");
    out.push_str("    adrp x11, alya_catch_stack_sp\n");
    out.push_str("    add x11, x11, :lo12:alya_catch_stack_sp\n");
    out.push_str("    ldr x13, [x11, x10, lsl #3]\n");
    out.push_str("    mov sp, x13\n");
    out.push_str("    adrp x11, alya_catch_stack_bp\n");
    out.push_str("    add x11, x11, :lo12:alya_catch_stack_bp\n");
    out.push_str("    ldr x29, [x11, x10, lsl #3]\n");
    out.push_str("    adrp x11, alya_catch_stack_handler\n");
    out.push_str("    add x11, x11, :lo12:alya_catch_stack_handler\n");
    out.push_str("    ldr x14, [x11, x10, lsl #3]\n");
    out.push_str("    br x14\n");
    out.push_str(".L_arm_fatal_div_zero:\n");
    out.push_str("    mov x19, sp\n");
    out.push_str("    and x19, x19, #~15\n");
    out.push_str("    mov sp, x19\n");
    out.push_str("    adrp x0, alya_fmt_div_zero\n");
    out.push_str("    add x0, x0, :lo12:alya_fmt_div_zero\n");
    out.push_str("    bl printf\n");
    out.push_str("    mov w0, #1\n");
    out.push_str("    bl exit\n\n");
}
