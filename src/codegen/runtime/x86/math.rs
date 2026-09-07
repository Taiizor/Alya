use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

    // fn_abs
    out.push_str("fn_abs:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jns .L_x86_abs_end\n");
    out.push_str("    neg %eax\n");
    out.push_str(".L_x86_abs_end:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_min
    out.push_str("fn_min:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    mov 12(%ebp), %edx\n");
    out.push_str("    cmp %edx, %eax\n");
    out.push_str("    jle .L_x86_min_end\n");
    out.push_str("    mov %edx, %eax\n");
    out.push_str(".L_x86_min_end:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_max
    out.push_str("fn_max:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    mov 12(%ebp), %edx\n");
    out.push_str("    cmp %edx, %eax\n");
    out.push_str("    jge .L_x86_max_end\n");
    out.push_str("    mov %edx, %eax\n");
    out.push_str(".L_x86_max_end:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_sqrt
    out.push_str("fn_sqrt:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %ecx\n");
    out.push_str("    test %ecx, %ecx\n");
    out.push_str("    jle .L_x86_sqrt_zero\n");
    out.push_str("    cmp $4, %ecx\n");
    out.push_str("    jl .L_x86_sqrt_one\n");
    out.push_str("    push %ebx\n");
    out.push_str("    mov %ecx, %ebx\n");
    out.push_str("    shr $1, %ebx\n");
    out.push_str(".L_x86_sqrt_loop:\n");
    out.push_str("    mov %ecx, %eax\n");
    out.push_str("    xor %edx, %edx\n");
    out.push_str("    div %ebx\n");
    out.push_str("    add %ebx, %eax\n");
    out.push_str("    shr $1, %eax\n");
    out.push_str("    cmp %ebx, %eax\n");
    out.push_str("    jge .L_x86_sqrt_done\n");
    out.push_str("    mov %eax, %ebx\n");
    out.push_str("    jmp .L_x86_sqrt_loop\n");
    out.push_str(".L_x86_sqrt_done:\n");
    out.push_str("    mov %ebx, %eax\n");
    out.push_str("    pop %ebx\n");
    out.push_str("    jmp .L_x86_sqrt_end\n");
    out.push_str(".L_x86_sqrt_one:\n");
    out.push_str("    mov $1, %eax\n");
    out.push_str("    jmp .L_x86_sqrt_end\n");
    out.push_str(".L_x86_sqrt_zero:\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str(".L_x86_sqrt_end:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_pow
    out.push_str("fn_pow:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %ecx\n");
    out.push_str("    mov 12(%ebp), %edx\n");
    out.push_str("    test %edx, %edx\n");
    out.push_str("    js .L_x86_pow_zero\n");
    out.push_str("    mov $1, %eax\n");
    out.push_str(".L_x86_pow_loop:\n");
    out.push_str("    test %edx, %edx\n");
    out.push_str("    jle .L_x86_pow_end\n");
    out.push_str("    test $1, %edx\n");
    out.push_str("    jz .L_x86_pow_even\n");
    out.push_str("    imul %ecx, %eax\n");
    out.push_str(".L_x86_pow_even:\n");
    out.push_str("    imul %ecx, %ecx\n");
    out.push_str("    shr $1, %edx\n");
    out.push_str("    jmp .L_x86_pow_loop\n");
    out.push_str(".L_x86_pow_zero:\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str(".L_x86_pow_end:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // Bitwise operations
    out.push_str(".global fn_bit_and\n");
    out.push_str("fn_bit_and:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    and 12(%ebp), %eax\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_or\n");
    out.push_str("fn_bit_or:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    or 12(%ebp), %eax\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_xor\n");
    out.push_str("fn_bit_xor:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    xor 12(%ebp), %eax\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_not\n");
    out.push_str("fn_bit_not:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    not %eax\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_shl\n");
    out.push_str("fn_bit_shl:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %ecx\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    mov 12(%ebp), %ecx\n");
    out.push_str("    shll %cl, %eax\n");
    out.push_str("    pop %ecx\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_bit_shr\n");
    out.push_str("fn_bit_shr:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %ecx\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    mov 12(%ebp), %ecx\n");
    out.push_str("    shrl %cl, %eax\n");
    out.push_str("    pop %ecx\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // PRNG
    out.push_str(".global fn_rand\n");
    out.push_str("fn_rand:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov alya_rand_state, %eax\n");
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jnz .L_x86_rand_ok\n");
    out.push_str("    mov $123456789, %eax\n");
    out.push_str(".L_x86_rand_ok:\n");
    out.push_str("    imul $1103515245, %eax\n");
    out.push_str("    add $12345, %eax\n");
    out.push_str("    and $0x7fffffff, %eax\n");
    out.push_str("    mov %eax, alya_rand_state\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    out.push_str(".global fn_rand_seed\n");
    out.push_str("fn_rand_seed:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    mov %eax, alya_rand_state\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_time
    out.push_str(".global fn_time\n");
    out.push_str("fn_time:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push $0\n");
    out.push_str("    call time\n");
    out.push_str("    add $4, %esp\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

}
