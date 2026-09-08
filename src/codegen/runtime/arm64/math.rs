use crate::codegen::target::OperatingSystem;
use super::emit_adrp_add;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

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

    // fn_sqrt
    out.push_str("fn_sqrt:\n");
    out.push_str("    cmp x0, #0\n");
    out.push_str("    b.le .L_arm64_sqrt_zero\n");
    out.push_str("    cmp x0, #4\n");
    out.push_str("    b.lt .L_arm64_sqrt_one\n");
    out.push_str("    mov x9, x0\n");
    out.push_str("    lsr x10, x9, #1\n");
    out.push_str(".L_arm64_sqrt_loop:\n");
    out.push_str("    sdiv x11, x9, x10\n");
    out.push_str("    add x11, x10, x11\n");
    out.push_str("    lsr x11, x11, #1\n");
    out.push_str("    cmp x11, x10\n");
    out.push_str("    b.ge .L_arm64_sqrt_done\n");
    out.push_str("    mov x10, x11\n");
    out.push_str("    b .L_arm64_sqrt_loop\n");
    out.push_str(".L_arm64_sqrt_done:\n");
    out.push_str("    mov x0, x10\n");
    out.push_str("    ret\n");
    out.push_str(".L_arm64_sqrt_one:\n");
    out.push_str("    mov x0, #1\n");
    out.push_str("    ret\n");
    out.push_str(".L_arm64_sqrt_zero:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ret\n\n");

    // fn_pow
    out.push_str("fn_pow:\n");
    out.push_str("    cmp x1, #0\n");
    out.push_str("    b.lt .L_arm64_pow_zero\n");
    out.push_str("    mov x9, x0\n");
    out.push_str("    mov x10, x1\n");
    out.push_str("    mov x0, #1\n");
    out.push_str(".L_arm64_pow_loop:\n");
    out.push_str("    cmp x10, #0\n");
    out.push_str("    b.le .L_arm64_pow_end\n");
    out.push_str("    tst x10, #1\n");
    out.push_str("    b.eq .L_arm64_pow_even\n");
    out.push_str("    mul x0, x0, x9\n");
    out.push_str(".L_arm64_pow_even:\n");
    out.push_str("    mul x9, x9, x9\n");
    out.push_str("    lsr x10, x10, #1\n");
    out.push_str("    b .L_arm64_pow_loop\n");
    out.push_str(".L_arm64_pow_zero:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str(".L_arm64_pow_end:\n");
    out.push_str("    ret\n\n");

    // Bitwise operations
    out.push_str(".align 2\n");
    out.push_str(".global fn_bit_and\n");
    out.push_str("fn_bit_and:\n");
    out.push_str("    and x0, x0, x1\n");
    out.push_str("    ret\n\n");

    out.push_str(".align 2\n");
    out.push_str(".global fn_bit_or\n");
    out.push_str("fn_bit_or:\n");
    out.push_str("    orr x0, x0, x1\n");
    out.push_str("    ret\n\n");

    out.push_str(".align 2\n");
    out.push_str(".global fn_bit_xor\n");
    out.push_str("fn_bit_xor:\n");
    out.push_str("    eor x0, x0, x1\n");
    out.push_str("    ret\n\n");

    out.push_str(".align 2\n");
    out.push_str(".global fn_bit_not\n");
    out.push_str("fn_bit_not:\n");
    out.push_str("    mvn x0, x0\n");
    out.push_str("    ret\n\n");

    out.push_str(".align 2\n");
    out.push_str(".global fn_bit_shl\n");
    out.push_str("fn_bit_shl:\n");
    out.push_str("    lsl x0, x0, x1\n");
    out.push_str("    ret\n\n");

    out.push_str(".align 2\n");
    out.push_str(".global fn_bit_shr\n");
    out.push_str("fn_bit_shr:\n");
    out.push_str("    lsr x0, x0, x1\n");
    out.push_str("    ret\n\n");

    // PRNG
    out.push_str(".align 2\n");
    out.push_str(".global fn_rand\n");
    out.push_str("fn_rand:\n");
    emit_adrp_add(out, "x9", "alya_rand_state", os);
    out.push_str("    ldr x0, [x9]\n");
    out.push_str("    cbnz x0, .L_arm64_rand_ok\n");
    out.push_str("    mrs x0, cntvct_el0\n");
    out.push_str("    cbnz x0, .L_arm64_rand_ok\n");
    out.push_str("    movz x0, #0xcd15\n");
    out.push_str("    movk x0, #0x075b, lsl #16\n");
    out.push_str(".L_arm64_rand_ok:\n");
    out.push_str("    movz x10, #0x4e6d\n");
    out.push_str("    movk x10, #0x41c6, lsl #16\n");
    out.push_str("    mul x0, x0, x10\n");
    out.push_str("    mov x10, #12345\n");
    out.push_str("    add x0, x0, x10\n");
    out.push_str("    and x0, x0, #0x7fffffff\n");
    out.push_str("    str x0, [x9]\n");
    out.push_str("    ret\n\n");

    out.push_str(".align 2\n");
    out.push_str(".global fn_rand_seed\n");
    out.push_str("fn_rand_seed:\n");
    emit_adrp_add(out, "x9", "alya_rand_state", os);
    out.push_str("    str x0, [x9]\n");
    out.push_str("    mov x0, #0\n");
    out.push_str("    ret\n\n");

    // fn_time
    out.push_str(".align 2\n");
    out.push_str(".global fn_time\n");
    out.push_str("fn_time:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    mov x0, #0\n");
    out.push_str(&format!("    bl {}time\n", p));
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // fn_clock
    out.push_str(".align 2\n");
    out.push_str(".global fn_clock\n");
    out.push_str("fn_clock:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str(&format!("    bl {}clock\n", p));
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // fn_clock_ms
    out.push_str(".align 2\n");
    out.push_str(".global fn_clock_ms\n");
    out.push_str("fn_clock_ms:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str(&format!("    bl {}clock\n", p));
    out.push_str("    ldp x29, x30, [sp], #16\n");
    if !matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov x1, #1000\n");
        out.push_str("    udiv x0, x0, x1\n");
    }
    out.push_str("    ret\n\n");
}
