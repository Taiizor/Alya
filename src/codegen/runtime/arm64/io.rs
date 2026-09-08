use crate::codegen::target::OperatingSystem;
use super::emit_adrp_add;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

    // fn_ask
    out.push_str("fn_ask:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #-16]!\n");
    out.push_str("    stp x21, x22, [sp, #-16]!\n");
    out.push_str("    cbz x0, .L_arm_ask_read\n");
    out.push_str("    mov x1, x0\n");
    emit_adrp_add(out, "x0", "alya_fmt_prompt", os);
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str("    sub sp, sp, #16\n");
        out.push_str("    str x1, [sp]\n");
        out.push_str(&format!("    bl {}printf\n", p));
        out.push_str("    add sp, sp, #16\n");
    } else {
        out.push_str(&format!("    bl {}printf\n", p));
    }
    out.push_str("    mov x0, #0\n");
    out.push_str(&format!("    bl {}fflush\n", p));
    out.push_str(".L_arm_ask_read:\n");
    emit_adrp_add(out, "x19", "alya_str_buf", os);
    emit_adrp_add(out, "x20", "alya_str_idx", os);
    out.push_str("    ldr x2, [x20]\n");
    out.push_str("    mov x9, #48000\n");
    out.push_str("    cmp x2, x9\n");
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

    // fn_get_env
    out.push_str(".align 2\n");
    out.push_str(".global fn_get_env\n");
    out.push_str("fn_get_env:\n");
    out.push_str("    stp x29, x30, [sp, #-48]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    stp x21, x22, [sp, #32]\n");
    out.push_str("    cbz x0, .L_arm64_getenv_empty\n");
    out.push_str(&format!("    bl {}getenv\n", p));
    out.push_str("    cbz x0, .L_arm64_getenv_empty\n");
    out.push_str("    mov x19, x0\n");
    emit_adrp_add(out, "x2", "alya_str_buf", os);
    emit_adrp_add(out, "x3", "alya_str_idx", os);
    out.push_str("    ldr x4, [x3]\n");
    out.push_str("    mov x5, #48000\n");
    out.push_str("    cmp x4, x5\n");
    out.push_str("    b.lt .L_arm64_getenv_buf_ok\n");
    out.push_str("    mov x4, #0\n");
    out.push_str(".L_arm64_getenv_buf_ok:\n");
    out.push_str("    add x20, x2, x4\n");
    out.push_str("    mov x21, x20\n");
    out.push_str(".L_arm64_getenv_copy:\n");
    out.push_str("    ldrb w6, [x19], #1\n");
    out.push_str("    strb w6, [x21], #1\n");
    out.push_str("    cbnz w6, .L_arm64_getenv_copy\n");
    out.push_str("    sub x7, x21, x2\n");
    out.push_str("    add x7, x7, #7\n");
    out.push_str("    and x7, x7, #-8\n");
    out.push_str("    str x7, [x3]\n");
    out.push_str("    mov x0, x20\n");
    out.push_str("    b .L_arm64_getenv_ret\n");
    out.push_str(".L_arm64_getenv_empty:\n");
    emit_adrp_add(out, "x0", "alya_str_empty", os);
    out.push_str(".L_arm64_getenv_ret:\n");
    out.push_str("    ldp x21, x22, [sp, #32]\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldp x29, x30, [sp], #48\n");
    out.push_str("    ret\n\n");

    // fn_target_os
    out.push_str(".align 2\n");
    out.push_str(".global fn_target_os\n");
    out.push_str("fn_target_os:\n");
    emit_adrp_add(out, "x0", "alya_str_target_os", os);
    out.push_str("    ret\n\n");

    // fn_target_arch
    out.push_str(".align 2\n");
    out.push_str(".global fn_target_arch\n");
    out.push_str("fn_target_arch:\n");
    emit_adrp_add(out, "x0", "alya_str_target_arch", os);
    out.push_str("    ret\n\n");

    // fn_set_console_output_cp
    out.push_str(".align 2\n");
    out.push_str(".global fn_set_console_output_cp\n");
    out.push_str("fn_set_console_output_cp:\n");
    out.push_str("    mov x0, #1\n");
    out.push_str("    ret\n\n");

    // fn_set_console_input_cp
    out.push_str(".align 2\n");
    out.push_str(".global fn_set_console_input_cp\n");
    out.push_str("fn_set_console_input_cp:\n");
    out.push_str("    mov x0, #1\n");
    out.push_str("    ret\n\n");

    // fn_get_console_output_cp
    out.push_str(".align 2\n");
    out.push_str(".global fn_get_console_output_cp\n");
    out.push_str("fn_get_console_output_cp:\n");
    out.push_str("    mov x0, #65001\n");
    out.push_str("    ret\n\n");

    // fn_get_console_input_cp
    out.push_str(".align 2\n");
    out.push_str(".global fn_get_console_input_cp\n");
    out.push_str("fn_get_console_input_cp:\n");
    out.push_str("    mov x0, #65001\n");
    out.push_str("    ret\n\n");

    // fn_enable_virtual_terminal
    out.push_str(".align 2\n");
    out.push_str(".global fn_enable_virtual_terminal\n");
    out.push_str("fn_enable_virtual_terminal:\n");
    out.push_str("    mov x0, #1\n");
    out.push_str("    ret\n\n");
}

