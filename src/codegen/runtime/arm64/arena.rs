use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

    // fn_arena_create
    out.push_str(".align 2\n");
    out.push_str(".global fn_arena_create\n");
    out.push_str("fn_arena_create:\n");
    out.push_str("    stp x29, x30, [sp, #-32]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    mov x19, x0\n");
    out.push_str("    cmp x19, #0\n");
    out.push_str("    b.gt .L_arm64_ac_size_ok\n");
    out.push_str("    mov x19, #65536\n");
    out.push_str(".L_arm64_ac_size_ok:\n");
    out.push_str("    mov x0, #1\n");
    out.push_str("    mov x1, #24\n");
    out.push_str(&format!("    bl {}calloc\n", p));
    out.push_str("    mov x20, x0\n");
    out.push_str("    add x0, x19, #24\n");
    out.push_str(&format!("    bl {}malloc\n", p));
    out.push_str("    str xzr, [x0]\n");
    out.push_str("    str x19, [x0, #8]\n");
    out.push_str("    str xzr, [x0, #16]\n");
    out.push_str("    str x0, [x20]\n");
    out.push_str("    str x19, [x20, #8]\n");
    out.push_str("    str xzr, [x20, #16]\n");
    out.push_str("    mov x0, x20\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldp x29, x30, [sp], #32\n");
    out.push_str("    ret\n\n");

    // fn_arena_alloc
    out.push_str(".align 2\n");
    out.push_str(".global fn_arena_alloc\n");
    out.push_str("fn_arena_alloc:\n");
    out.push_str("    stp x29, x30, [sp, #-48]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    stp x21, x22, [sp, #32]\n");
    out.push_str("    mov x19, x0\n");
    out.push_str("    mov x20, x1\n");
    out.push_str("    cbz x19, .L_arm64_aa_fail\n");
    out.push_str("    cmp x20, #0\n");
    out.push_str("    b.le .L_arm64_aa_fail\n");
    out.push_str("    add x20, x20, #7\n");
    out.push_str("    and x20, x20, #-8\n");
    out.push_str("    ldr x21, [x19]\n");
    out.push_str("    cbz x21, .L_arm64_aa_new_chunk\n");
    out.push_str("    ldr x2, [x21, #16]\n");
    out.push_str("    ldr x3, [x21, #8]\n");
    out.push_str("    add x4, x2, x20\n");
    out.push_str("    cmp x4, x3\n");
    out.push_str("    b.gt .L_arm64_aa_new_chunk\n");
    out.push_str("    add x0, x21, #24\n");
    out.push_str("    add x0, x0, x2\n");
    out.push_str("    str x4, [x21, #16]\n");
    out.push_str("    ldr x5, [x19, #16]\n");
    out.push_str("    add x5, x5, x20\n");
    out.push_str("    str x5, [x19, #16]\n");
    out.push_str("    b .L_arm64_aa_ret\n");
    out.push_str(".L_arm64_aa_new_chunk:\n");
    out.push_str("    ldr x22, [x19, #8]\n");
    out.push_str("    cmp x20, x22\n");
    out.push_str("    csel x22, x20, x22, gt\n");
    out.push_str("    add x0, x22, #24\n");
    out.push_str(&format!("    bl {}malloc\n", p));
    out.push_str("    ldr x1, [x19]\n");
    out.push_str("    str x1, [x0]\n");
    out.push_str("    str x22, [x0, #8]\n");
    out.push_str("    str x20, [x0, #16]\n");
    out.push_str("    str x0, [x19]\n");
    out.push_str("    ldr x5, [x19, #16]\n");
    out.push_str("    add x5, x5, x20\n");
    out.push_str("    str x5, [x19, #16]\n");
    out.push_str("    add x0, x0, #24\n");
    out.push_str("    b .L_arm64_aa_ret\n");
    out.push_str(".L_arm64_aa_fail:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str(".L_arm64_aa_ret:\n");
    out.push_str("    ldp x21, x22, [sp, #32]\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldp x29, x30, [sp], #48\n");
    out.push_str("    ret\n\n");

    // fn_arena_reset
    out.push_str(".align 2\n");
    out.push_str(".global fn_arena_reset\n");
    out.push_str("fn_arena_reset:\n");
    out.push_str("    cbz x0, .L_arm64_ar_ret\n");
    out.push_str("    ldr x1, [x0]\n");
    out.push_str(".L_arm64_ar_loop:\n");
    out.push_str("    cbz x1, .L_arm64_ar_fin\n");
    out.push_str("    str xzr, [x1, #16]\n");
    out.push_str("    ldr x1, [x1]\n");
    out.push_str("    b .L_arm64_ar_loop\n");
    out.push_str(".L_arm64_ar_fin:\n");
    out.push_str("    str xzr, [x0, #16]\n");
    out.push_str(".L_arm64_ar_ret:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ret\n\n");

    // fn_arena_destroy
    out.push_str(".align 2\n");
    out.push_str(".global fn_arena_destroy\n");
    out.push_str("fn_arena_destroy:\n");
    out.push_str("    stp x29, x30, [sp, #-48]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    stp x21, x22, [sp, #32]\n");
    out.push_str("    mov x19, x0\n");
    out.push_str("    cbz x19, .L_arm64_ad_ret\n");
    out.push_str("    ldr x20, [x19]\n");
    out.push_str(".L_arm64_ad_loop:\n");
    out.push_str("    cbz x20, .L_arm64_ad_free_arena\n");
    out.push_str("    ldr x21, [x20]\n");
    out.push_str("    mov x0, x20\n");
    out.push_str(&format!("    bl {}free\n", p));
    out.push_str("    mov x20, x21\n");
    out.push_str("    b .L_arm64_ad_loop\n");
    out.push_str(".L_arm64_ad_free_arena:\n");
    out.push_str("    mov x0, x19\n");
    out.push_str(&format!("    bl {}free\n", p));
    out.push_str(".L_arm64_ad_ret:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ldp x21, x22, [sp, #32]\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldp x29, x30, [sp], #48\n");
    out.push_str("    ret\n\n");

    // fn_arena_allocated
    out.push_str(".align 2\n");
    out.push_str(".global fn_arena_allocated\n");
    out.push_str("fn_arena_allocated:\n");
    out.push_str("    cbz x0, .L_arm64_aal_zero\n");
    out.push_str("    ldr x0, [x0, #16]\n");
    out.push_str("    ret\n");
    out.push_str(".L_arm64_aal_zero:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ret\n\n");
}
