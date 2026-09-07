use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

    // fn_abs
    out.push_str("fn_abs:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jns .L_x64_abs_end\n");
    out.push_str("    neg %rax\n");
    out.push_str(".L_x64_abs_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_min
    out.push_str("fn_min:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
        out.push_str("    cmp %rdx, %rax\n");
        out.push_str("    jle .L_x64_min_end\n");
        out.push_str("    mov %rdx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
        out.push_str("    cmp %rsi, %rax\n");
        out.push_str("    jle .L_x64_min_end\n");
        out.push_str("    mov %rsi, %rax\n");
    }
    out.push_str(".L_x64_min_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_max
    out.push_str("fn_max:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
        out.push_str("    cmp %rdx, %rax\n");
        out.push_str("    jge .L_x64_max_end\n");
        out.push_str("    mov %rdx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
        out.push_str("    cmp %rsi, %rax\n");
        out.push_str("    jge .L_x64_max_end\n");
        out.push_str("    mov %rsi, %rax\n");
    }
    out.push_str(".L_x64_max_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_sqrt
    out.push_str("fn_sqrt:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r8\n");
    } else {
        out.push_str("    mov %rdi, %r8\n");
    }
    out.push_str("    test %r8, %r8\n");
    out.push_str("    jle .L_x64_sqrt_zero\n");
    out.push_str("    cmp $4, %r8\n");
    out.push_str("    jl .L_x64_sqrt_one\n");
    out.push_str("    mov %r8, %r9\n");
    out.push_str("    shr $1, %r9\n");
    out.push_str(".L_x64_sqrt_loop:\n");
    out.push_str("    mov %r8, %rax\n");
    out.push_str("    xor %rdx, %rdx\n");
    out.push_str("    div %r9\n");
    out.push_str("    add %r9, %rax\n");
    out.push_str("    shr $1, %rax\n");
    out.push_str("    cmp %r9, %rax\n");
    out.push_str("    jge .L_x64_sqrt_done\n");
    out.push_str("    mov %rax, %r9\n");
    out.push_str("    jmp .L_x64_sqrt_loop\n");
    out.push_str(".L_x64_sqrt_done:\n");
    out.push_str("    mov %r9, %rax\n");
    out.push_str("    jmp .L_x64_sqrt_end\n");
    out.push_str(".L_x64_sqrt_one:\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str("    jmp .L_x64_sqrt_end\n");
    out.push_str(".L_x64_sqrt_zero:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_sqrt_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_pow
    out.push_str("fn_pow:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r8\n");
        out.push_str("    mov %rdx, %r9\n");
    } else {
        out.push_str("    mov %rdi, %r8\n");
        out.push_str("    mov %rsi, %r9\n");
    }
    out.push_str("    test %r9, %r9\n");
    out.push_str("    js .L_x64_pow_zero\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str(".L_x64_pow_loop:\n");
    out.push_str("    test %r9, %r9\n");
    out.push_str("    jle .L_x64_pow_end\n");
    out.push_str("    test $1, %r9\n");
    out.push_str("    jz .L_x64_pow_even\n");
    out.push_str("    imul %r8, %rax\n");
    out.push_str(".L_x64_pow_even:\n");
    out.push_str("    imul %r8, %r8\n");
    out.push_str("    shr $1, %r9\n");
    out.push_str("    jmp .L_x64_pow_loop\n");
    out.push_str(".L_x64_pow_zero:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_pow_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");


    // Bitwise operations
    out.push_str(".global fn_bit_and\n");
    out.push_str("fn_bit_and:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
        out.push_str("    and %rdx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
        out.push_str("    and %rsi, %rax\n");
    }
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_or\n");
    out.push_str("fn_bit_or:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
        out.push_str("    or %rdx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
        out.push_str("    or %rsi, %rax\n");
    }
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_xor\n");
    out.push_str("fn_bit_xor:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
        out.push_str("    xor %rdx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
        out.push_str("    xor %rsi, %rax\n");
    }
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_not\n");
    out.push_str("fn_bit_not:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
    }
    out.push_str("    not %rax\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_shl\n");
    out.push_str("fn_bit_shl:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
        out.push_str("    mov %rdx, %rcx\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
        out.push_str("    mov %rsi, %rcx\n");
    }
    out.push_str("    shl %cl, %rax\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_shr\n");
    out.push_str("fn_bit_shr:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
        out.push_str("    mov %rdx, %rcx\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
        out.push_str("    mov %rsi, %rcx\n");
    }
    out.push_str("    shr %cl, %rax\n");
    out.push_str("    ret\n\n");

    // PRNG
    out.push_str(".global fn_rand\n");
    out.push_str("fn_rand:\n");
    out.push_str("    mov alya_rand_state(%rip), %rax\n");
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jnz .L_x64_rand_ok\n");
    out.push_str("    mov $123456789, %rax\n");
    out.push_str(".L_x64_rand_ok:\n");
    out.push_str("    mov $1103515245, %rdx\n");
    out.push_str("    imul %rdx, %rax\n");
    out.push_str("    add $12345, %rax\n");
    out.push_str("    and $0x7fffffff, %rax\n");
    out.push_str("    mov %rax, alya_rand_state(%rip)\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_rand_seed\n");
    out.push_str("fn_rand_seed:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, alya_rand_state(%rip)\n");
    } else {
        out.push_str("    mov %rdi, alya_rand_state(%rip)\n");
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    ret\n\n");

    // fn_time
    out.push_str(".global fn_time\n");
    out.push_str("fn_time:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    sub $32, %rsp\n");
    out.push_str("    xor %rcx, %rcx\n");
    out.push_str("    xor %rdi, %rdi\n");
    out.push_str(&format!("    call {}time\n", p));
    out.push_str("    add $32, %rsp\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

}
