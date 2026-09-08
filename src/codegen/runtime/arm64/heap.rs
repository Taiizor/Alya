use crate::codegen::target::OperatingSystem;
use super::emit_adrp_add;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

    // fn_alloc
    out.push_str(".align 2\n");
    out.push_str(".global fn_alloc\n");
    out.push_str("fn_alloc:\n");
    out.push_str("    stp x29, x30, [sp, #-32]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    str x0, [sp, #16]\n");
    emit_adrp_add(out, "x1", "alya_allocated_bytes", os);
    out.push_str("    ldr x2, [x1]\n");
    out.push_str("    add x2, x2, x0\n");
    out.push_str("    str x2, [x1]\n");
    out.push_str("    ldr x0, [sp, #16]\n");
    out.push_str(&format!("    bl {}malloc\n", p));
    out.push_str("    ldp x29, x30, [sp], #32\n");
    out.push_str("    ret\n\n");

    // fn_free
    out.push_str(".align 2\n");
    out.push_str(".global fn_free\n");
    out.push_str("fn_free:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    cbz x0, .L_arm64_free_done\n");
    out.push_str(&format!("    bl {}free\n", p));
    out.push_str(".L_arm64_free_done:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // fn_realloc
    out.push_str(".align 2\n");
    out.push_str(".global fn_realloc\n");
    out.push_str("fn_realloc:\n");
    out.push_str("    stp x29, x30, [sp, #-32]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    str x0, [sp, #16]\n");
    out.push_str("    str x1, [sp, #24]\n");
    emit_adrp_add(out, "x2", "alya_allocated_bytes", os);
    out.push_str("    ldr x3, [x2]\n");
    out.push_str("    add x3, x3, x1\n");
    out.push_str("    str x3, [x2]\n");
    out.push_str("    ldr x0, [sp, #16]\n");
    out.push_str("    ldr x1, [sp, #24]\n");
    out.push_str(&format!("    bl {}realloc\n", p));
    out.push_str("    ldp x29, x30, [sp], #32\n");
    out.push_str("    ret\n\n");

    // fn_copy_mem
    out.push_str(".align 2\n");
    out.push_str(".global fn_copy_mem\n");
    out.push_str("fn_copy_mem:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str(&format!("    bl {}memcpy\n", p));
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // fn_zero_mem
    out.push_str(".align 2\n");
    out.push_str(".global fn_zero_mem\n");
    out.push_str("fn_zero_mem:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    mov x2, x1\n");
    out.push_str("    mov x1, #0\n");
    out.push_str(&format!("    bl {}memset\n", p));
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // fn_peek_byte
    out.push_str(".align 2\n");
    out.push_str(".global fn_peek_byte\n");
    out.push_str("fn_peek_byte:\n");
    out.push_str("    add x0, x0, x1\n");
    out.push_str("    ldrb w0, [x0]\n");
    out.push_str("    ret\n\n");

    // fn_poke_byte
    out.push_str(".align 2\n");
    out.push_str(".global fn_poke_byte\n");
    out.push_str("fn_poke_byte:\n");
    out.push_str("    add x0, x0, x1\n");
    out.push_str("    strb w2, [x0]\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ret\n\n");

    // fn_peek_int
    out.push_str(".align 2\n");
    out.push_str(".global fn_peek_int\n");
    out.push_str("fn_peek_int:\n");
    out.push_str("    add x0, x0, x1\n");
    out.push_str("    ldr x0, [x0]\n");
    out.push_str("    ret\n\n");

    // fn_poke_int
    out.push_str(".align 2\n");
    out.push_str(".global fn_poke_int\n");
    out.push_str("fn_poke_int:\n");
    out.push_str("    add x0, x0, x1\n");
    out.push_str("    str x2, [x0]\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ret\n\n");

    // fn_str_from_ptr
    out.push_str(".align 2\n");
    out.push_str(".global fn_str_from_ptr\n");
    out.push_str("fn_str_from_ptr:\n");
    out.push_str("    cbnz x0, .L_arm64_sfp_ret\n");
    emit_adrp_add(out, "x0", "alya_str_empty", os);
    out.push_str(".L_arm64_sfp_ret:\n");
    out.push_str("    ret\n\n");

    // fn_str_to_ptr
    out.push_str(".align 2\n");
    out.push_str(".global fn_str_to_ptr\n");
    out.push_str("fn_str_to_ptr:\n");
    out.push_str("    ret\n\n");

    // fn_mem_allocated
    out.push_str(".align 2\n");
    out.push_str(".global fn_mem_allocated\n");
    out.push_str("fn_mem_allocated:\n");
    emit_adrp_add(out, "x1", "alya_allocated_bytes", os);
    out.push_str("    ldr x0, [x1]\n");
    out.push_str("    ret\n\n");

    // fn_mem_reset_alloc
    out.push_str(".align 2\n");
    out.push_str(".global fn_mem_reset_alloc\n");
    out.push_str("fn_mem_reset_alloc:\n");
    emit_adrp_add(out, "x1", "alya_allocated_bytes", os);
    out.push_str("    str xzr, [x1]\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ret\n\n");

    // fn_str_clone
    out.push_str(".align 2\n");
    out.push_str(".global fn_str_clone\n");
    out.push_str("fn_str_clone:\n");
    out.push_str("    stp x29, x30, [sp, #-48]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    str x21, [sp, #32]\n");
    out.push_str("    mov x19, x0\n");
    out.push_str("    cbz x19, .L_arm64_sclone_empty\n");
    out.push_str("    mov x20, x19\n");
    out.push_str("    mov x21, #0\n");
    out.push_str(".L_arm64_sclone_len:\n");
    out.push_str("    ldrb w1, [x20]\n");
    out.push_str("    cbz w1, .L_arm64_sclone_alloc\n");
    out.push_str("    add x20, x20, #1\n");
    out.push_str("    add x21, x21, #1\n");
    out.push_str("    b .L_arm64_sclone_len\n");
    out.push_str(".L_arm64_sclone_alloc:\n");
    out.push_str("    add x21, x21, #1\n");
    out.push_str("    mov x0, x21\n");
    emit_adrp_add(out, "x1", "alya_allocated_bytes", os);
    out.push_str("    ldr x2, [x1]\n");
    out.push_str("    add x2, x2, x21\n");
    out.push_str("    str x2, [x1]\n");
    out.push_str(&format!("    bl {}malloc\n", p));
    out.push_str("    cbz x0, .L_arm64_sclone_empty\n");
    out.push_str("    mov x20, x0\n");
    out.push_str("    mov x1, x19\n");
    out.push_str("    mov x2, x21\n");
    out.push_str(&format!("    bl {}memcpy\n", p));
    out.push_str("    mov x0, x20\n");
    out.push_str("    b .L_arm64_sclone_done\n");
    out.push_str(".L_arm64_sclone_empty:\n");
    emit_adrp_add(out, "x0", "alya_str_empty", os);
    out.push_str(".L_arm64_sclone_done:\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldr x21, [sp, #32]\n");
    out.push_str("    ldp x29, x30, [sp], #48\n");
    out.push_str("    ret\n\n");

    // fn_str_free
    out.push_str(".align 2\n");
    out.push_str(".global fn_str_free\n");
    out.push_str("fn_str_free:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    cbz x0, .L_arm64_sfree_done\n");
    out.push_str(&format!("    bl {}free\n", p));
    out.push_str(".L_arm64_sfree_done:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");
}
