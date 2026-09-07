use crate::codegen::target::OperatingSystem;

pub fn emit_x64_runtime(out: &mut String, os: OperatingSystem) {
    // alya_concat
    out.push_str("alya_concat:\n");
    out.push_str("    push %rsi\n");
    out.push_str("    push %rdi\n");
    out.push_str("    push %rbx\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rsi\n");
        out.push_str("    mov %rdx, %r10\n");
    } else {
        out.push_str("    mov %rsi, %r10\n");
        out.push_str("    mov %rdi, %rsi\n");
    }
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_concat_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_concat_ok:\n");
    out.push_str("    lea (%r8, %rbx), %rdi\n");
    out.push_str("    mov %rdi, %rax\n");
    out.push_str(".L_x64_copy1:\n");
    out.push_str("    movb (%rsi), %cl\n");
    out.push_str("    test %cl, %cl\n");
    out.push_str("    jz .L_x64_copy2_start\n");
    out.push_str("    movb %cl, (%rdi)\n");
    out.push_str("    inc %rsi\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    jmp .L_x64_copy1\n");
    out.push_str(".L_x64_copy2_start:\n");
    out.push_str("    mov %r10, %rsi\n");
    out.push_str(".L_x64_copy2:\n");
    out.push_str("    movb (%rsi), %cl\n");
    out.push_str("    test %cl, %cl\n");
    out.push_str("    jz .L_x64_concat_end\n");
    out.push_str("    movb %cl, (%rdi)\n");
    out.push_str("    inc %rsi\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    jmp .L_x64_copy2\n");
    out.push_str(".L_x64_concat_end:\n");
    out.push_str("    movb $0, (%rdi)\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    sub %r8, %rdi\n");
    out.push_str("    add $7, %rdi\n");
    out.push_str("    and $-8, %rdi\n");
    out.push_str("    mov %rdi, alya_str_idx(%rip)\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    pop %rdi\n");
    out.push_str("    pop %rsi\n");
    out.push_str("    ret\n\n");

    // fn_len
    out.push_str("fn_len:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdi\n");
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_len_loop:\n");
    out.push_str("    cmpb $0, (%rdi, %rax)\n");
    out.push_str("    je .L_x64_len_end\n");
    out.push_str("    inc %rax\n");
    out.push_str("    jmp .L_x64_len_loop\n");
    out.push_str(".L_x64_len_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

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

    let p = if matches!(os, OperatingSystem::MacOS) {
        "_"
    } else {
        ""
    };

    // fn_exit
    out.push_str("fn_exit:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call exit\n");
    } else {
        out.push_str("    sub $8, %rsp\n");
        out.push_str(&format!("    call {}exit\n", p));
    }

    // fn_ask
    out.push_str("fn_ask:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %rsi\n");
    out.push_str("    push %rdi\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    sub $56, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdi\n");
        out.push_str("    test %rdi, %rdi\n");
        out.push_str("    jz .L_x64_ask_read\n");
        out.push_str("    lea alya_fmt_prompt(%rip), %rcx\n");
        out.push_str("    mov %rdi, %rdx\n");
        out.push_str("    call printf\n");
        out.push_str("    xor %rcx, %rcx\n");
        out.push_str("    call fflush\n");
    } else {
        out.push_str("    test %rdi, %rdi\n");
        out.push_str("    jz .L_x64_ask_read\n");
        out.push_str("    mov %rdi, %rsi\n");
        out.push_str("    lea alya_fmt_prompt(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    xor %rdi, %rdi\n");
        out.push_str(&format!("    call {}fflush\n", p));
    }
    out.push_str(".L_x64_ask_read:\n");
    out.push_str("    lea alya_str_buf(%rip), %rbx\n");
    out.push_str("    mov alya_str_idx(%rip), %rsi\n");
    out.push_str("    cmp $48000, %rsi\n");
    out.push_str("    jl .L_x64_ask_buf_ok\n");
    out.push_str("    xor %rsi, %rsi\n");
    out.push_str(".L_x64_ask_buf_ok:\n");
    out.push_str("    lea (%rbx, %rsi), %r12\n");
    out.push_str("    mov %r12, %r13\n");
    out.push_str(".L_x64_ask_loop:\n");
    out.push_str(&format!("    call {}getchar\n", p));
    out.push_str("    cmp $-1, %rax\n");
    out.push_str("    je .L_x64_ask_done\n");
    out.push_str("    cmp $10, %rax\n");
    out.push_str("    je .L_x64_ask_done\n");
    out.push_str("    cmp $13, %rax\n");
    out.push_str("    je .L_x64_ask_loop\n");
    out.push_str("    movb %al, (%r13)\n");
    out.push_str("    inc %r13\n");
    out.push_str("    jmp .L_x64_ask_loop\n");
    out.push_str(".L_x64_ask_done:\n");
    out.push_str("    movb $0, (%r13)\n");
    out.push_str("    inc %r13\n");
    out.push_str("    lea alya_str_buf(%rip), %rbx\n");
    out.push_str("    sub %rbx, %r13\n");
    out.push_str("    add $7, %r13\n");
    out.push_str("    and $-8, %r13\n");
    out.push_str("    mov %r13, alya_str_idx(%rip)\n");
    out.push_str("    mov %r12, %rax\n");
    out.push_str("    add $56, %rsp\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rdi\n");
    out.push_str("    pop %rsi\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_upper
    out.push_str("fn_upper:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdi\n");
    }
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_upper_buf_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_upper_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %r12\n");
    out.push_str("    mov %r12, %rax\n");
    out.push_str("    mov %r12, %r13\n");
    out.push_str(".L_x64_upper_loop:\n");
    out.push_str("    movb (%rdi), %cl\n");
    out.push_str("    test %cl, %cl\n");
    out.push_str("    jz .L_x64_upper_done\n");
    out.push_str("    cmpb $'a', %cl\n");
    out.push_str("    jl .L_x64_upper_store\n");
    out.push_str("    cmpb $'z', %cl\n");
    out.push_str("    jg .L_x64_upper_store\n");
    out.push_str("    subb $32, %cl\n");
    out.push_str(".L_x64_upper_store:\n");
    out.push_str("    movb %cl, (%r13)\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    inc %r13\n");
    out.push_str("    jmp .L_x64_upper_loop\n");
    out.push_str(".L_x64_upper_done:\n");
    out.push_str("    movb $0, (%r13)\n");
    out.push_str("    inc %r13\n");
    out.push_str("    sub %r8, %r13\n");
    out.push_str("    add $7, %r13\n");
    out.push_str("    and $-8, %r13\n");
    out.push_str("    mov %r13, alya_str_idx(%rip)\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_lower
    out.push_str("fn_lower:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdi\n");
    }
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_lower_buf_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_lower_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %r12\n");
    out.push_str("    mov %r12, %rax\n");
    out.push_str("    mov %r12, %r13\n");
    out.push_str(".L_x64_lower_loop:\n");
    out.push_str("    movb (%rdi), %cl\n");
    out.push_str("    test %cl, %cl\n");
    out.push_str("    jz .L_x64_lower_done\n");
    out.push_str("    cmpb $'A', %cl\n");
    out.push_str("    jl .L_x64_lower_store\n");
    out.push_str("    cmpb $'Z', %cl\n");
    out.push_str("    jg .L_x64_lower_store\n");
    out.push_str("    addb $32, %cl\n");
    out.push_str(".L_x64_lower_store:\n");
    out.push_str("    movb %cl, (%r13)\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    inc %r13\n");
    out.push_str("    jmp .L_x64_lower_loop\n");
    out.push_str(".L_x64_lower_done:\n");
    out.push_str("    movb $0, (%r13)\n");
    out.push_str("    inc %r13\n");
    out.push_str("    sub %r8, %r13\n");
    out.push_str("    add $7, %r13\n");
    out.push_str("    and $-8, %r13\n");
    out.push_str("    mov %r13, alya_str_idx(%rip)\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_trim
    out.push_str("fn_trim:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdi\n");
    }
    out.push_str(".L_x64_trim_lws:\n");
    out.push_str("    movb (%rdi), %al\n");
    out.push_str("    cmpb $32, %al\n");
    out.push_str("    je .L_x64_trim_lws_inc\n");
    out.push_str("    cmpb $9, %al\n");
    out.push_str("    je .L_x64_trim_lws_inc\n");
    out.push_str("    cmpb $10, %al\n");
    out.push_str("    je .L_x64_trim_lws_inc\n");
    out.push_str("    cmpb $13, %al\n");
    out.push_str("    je .L_x64_trim_lws_inc\n");
    out.push_str("    jmp .L_x64_trim_lws_done\n");
    out.push_str(".L_x64_trim_lws_inc:\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    jmp .L_x64_trim_lws\n");
    out.push_str(".L_x64_trim_lws_done:\n");
    out.push_str("    mov %rdi, %r12\n");
    out.push_str(".L_x64_trim_end_loop:\n");
    out.push_str("    cmpb $0, (%r12)\n");
    out.push_str("    je .L_x64_trim_find_end_done\n");
    out.push_str("    inc %r12\n");
    out.push_str("    jmp .L_x64_trim_end_loop\n");
    out.push_str(".L_x64_trim_find_end_done:\n");
    out.push_str(".L_x64_trim_tws:\n");
    out.push_str("    cmp %rdi, %r12\n");
    out.push_str("    jle .L_x64_trim_copy_start\n");
    out.push_str("    movb -1(%r12), %al\n");
    out.push_str("    cmpb $32, %al\n");
    out.push_str("    je .L_x64_trim_tws_dec\n");
    out.push_str("    cmpb $9, %al\n");
    out.push_str("    je .L_x64_trim_tws_dec\n");
    out.push_str("    cmpb $10, %al\n");
    out.push_str("    je .L_x64_trim_tws_dec\n");
    out.push_str("    cmpb $13, %al\n");
    out.push_str("    je .L_x64_trim_tws_dec\n");
    out.push_str("    jmp .L_x64_trim_copy_start\n");
    out.push_str(".L_x64_trim_tws_dec:\n");
    out.push_str("    dec %r12\n");
    out.push_str("    jmp .L_x64_trim_tws\n");
    out.push_str(".L_x64_trim_copy_start:\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_trim_buf_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_trim_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %r13\n");
    out.push_str("    mov %r13, %rax\n");
    out.push_str("    mov %r13, %r14\n");
    out.push_str(".L_x64_trim_copy_loop:\n");
    out.push_str("    cmp %r12, %rdi\n");
    out.push_str("    jge .L_x64_trim_done\n");
    out.push_str("    movb (%rdi), %cl\n");
    out.push_str("    movb %cl, (%r14)\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    inc %r14\n");
    out.push_str("    jmp .L_x64_trim_copy_loop\n");
    out.push_str(".L_x64_trim_done:\n");
    out.push_str("    movb $0, (%r14)\n");
    out.push_str("    inc %r14\n");
    out.push_str("    sub %r8, %r14\n");
    out.push_str("    add $7, %r14\n");
    out.push_str("    and $-8, %r14\n");
    out.push_str("    mov %r14, alya_str_idx(%rip)\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_contains
    out.push_str("fn_contains:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdi\n");
        out.push_str("    mov %rdx, %rsi\n");
    }
    out.push_str("    cmpb $0, (%rsi)\n");
    out.push_str("    je .L_x64_contains_match\n");
    out.push_str(".L_x64_contains_outer:\n");
    out.push_str("    cmpb $0, (%rdi)\n");
    out.push_str("    je .L_x64_contains_nomatch\n");
    out.push_str("    mov %rdi, %rax\n");
    out.push_str("    mov %rsi, %rdx\n");
    out.push_str(".L_x64_contains_inner:\n");
    out.push_str("    movb (%rdx), %cl\n");
    out.push_str("    test %cl, %cl\n");
    out.push_str("    jz .L_x64_contains_match\n");
    out.push_str("    movb (%rax), %r8b\n");
    out.push_str("    cmp %r8b, %cl\n");
    out.push_str("    jne .L_x64_contains_next\n");
    out.push_str("    inc %rax\n");
    out.push_str("    inc %rdx\n");
    out.push_str("    jmp .L_x64_contains_inner\n");
    out.push_str(".L_x64_contains_next:\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    jmp .L_x64_contains_outer\n");
    out.push_str(".L_x64_contains_match:\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str("    jmp .L_x64_contains_end\n");
    out.push_str(".L_x64_contains_nomatch:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_contains_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_substring / fn_substr
    out.push_str("fn_substr:\n");
    out.push_str("fn_substring:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdi\n");
        out.push_str("    mov %rdx, %rsi\n");
        out.push_str("    mov %r8, %rdx\n");
    }
    out.push_str("    cmp $0, %rsi\n");
    out.push_str("    jge .L_x64_sub_adv\n");
    out.push_str("    xor %rsi, %rsi\n");
    out.push_str(".L_x64_sub_adv:\n");
    out.push_str("    test %rsi, %rsi\n");
    out.push_str("    jle .L_x64_sub_adv_done\n");
    out.push_str("    cmpb $0, (%rdi)\n");
    out.push_str("    je .L_x64_sub_adv_done\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    dec %rsi\n");
    out.push_str("    jmp .L_x64_sub_adv\n");
    out.push_str(".L_x64_sub_adv_done:\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_sub_buf_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_sub_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %r12\n");
    out.push_str("    mov %r12, %rax\n");
    out.push_str("    mov %r12, %r13\n");
    out.push_str(".L_x64_sub_copy:\n");
    out.push_str("    cmpb $0, (%rdi)\n");
    out.push_str("    je .L_x64_sub_done\n");
    out.push_str("    test %rdx, %rdx\n");
    out.push_str("    jz .L_x64_sub_done\n");
    out.push_str("    movb (%rdi), %cl\n");
    out.push_str("    movb %cl, (%r13)\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    inc %r13\n");
    out.push_str("    cmp $0, %rdx\n");
    out.push_str("    jl .L_x64_sub_copy\n");
    out.push_str("    dec %rdx\n");
    out.push_str("    jmp .L_x64_sub_copy\n");
    out.push_str(".L_x64_sub_done:\n");
    out.push_str("    movb $0, (%r13)\n");
    out.push_str("    inc %r13\n");
    out.push_str("    sub %r8, %r13\n");
    out.push_str("    add $7, %r13\n");
    out.push_str("    and $-8, %r13\n");
    out.push_str("    mov %r13, alya_str_idx(%rip)\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_char_at
    out.push_str("fn_char_at:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdi\n");
        out.push_str("    mov %rdx, %rsi\n");
    }
    out.push_str("    lea alya_str_empty(%rip), %rax\n");
    out.push_str("    test %rdi, %rdi\n");
    out.push_str("    jz .L_x64_char_at_end\n");
    out.push_str("    cmp $0, %rsi\n");
    out.push_str("    jl .L_x64_char_at_end\n");
    out.push_str(".L_x64_char_at_adv:\n");
    out.push_str("    test %rsi, %rsi\n");
    out.push_str("    jle .L_x64_char_at_fetch\n");
    out.push_str("    cmpb $0, (%rdi)\n");
    out.push_str("    je .L_x64_char_at_end\n");
    out.push_str("    inc %rdi\n");
    out.push_str("    dec %rsi\n");
    out.push_str("    jmp .L_x64_char_at_adv\n");
    out.push_str(".L_x64_char_at_fetch:\n");
    out.push_str("    movzbl (%rdi), %edx\n");
    out.push_str("    test %edx, %edx\n");
    out.push_str("    jz .L_x64_char_at_end\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_char_at_buf_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_char_at_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %r12\n");
    out.push_str("    mov %r12, %rax\n");
    out.push_str("    movb %dl, (%r12)\n");
    out.push_str("    movb $0, 1(%r12)\n");
    out.push_str("    add $2, %r12\n");
    out.push_str("    sub %r8, %r12\n");
    out.push_str("    add $7, %r12\n");
    out.push_str("    and $-8, %r12\n");
    out.push_str("    mov %r12, alya_str_idx(%rip)\n");
    out.push_str(".L_x64_char_at_end:\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_ord / fn_char_code
    out.push_str("fn_char_code:\n");
    out.push_str("fn_ord:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz .L_x64_ord_ret\n");
    out.push_str("    cmp $256, %rax\n");
    out.push_str("    jb .L_x64_ord_ret\n");
    out.push_str("    movzbl (%rax), %eax\n");
    out.push_str(".L_x64_ord_ret:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_chr / fn_char_from_code
    out.push_str("fn_char_from_code:\n");
    out.push_str("fn_chr:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdx\n");
    } else {
        out.push_str("    mov %rdi, %rdx\n");
    }
    out.push_str("    lea alya_str_empty(%rip), %rax\n");
    out.push_str("    test %rdx, %rdx\n");
    out.push_str("    jz .L_x64_chr_end\n");
    out.push_str("    cmp $256, %rdx\n");
    out.push_str("    jb .L_x64_chr_code\n");
    out.push_str("    movzbl (%rdx), %edx\n");
    out.push_str(".L_x64_chr_code:\n");
    out.push_str("    and $255, %edx\n");
    out.push_str("    jz .L_x64_chr_end\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_chr_buf_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_chr_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %r12\n");
    out.push_str("    mov %r12, %rax\n");
    out.push_str("    movb %dl, (%r12)\n");
    out.push_str("    movb $0, 1(%r12)\n");
    out.push_str("    add $2, %r12\n");
    out.push_str("    sub %r8, %r12\n");
    out.push_str("    add $7, %r12\n");
    out.push_str("    and $-8, %r12\n");
    out.push_str("    mov %r12, alya_str_idx(%rip)\n");
    out.push_str(".L_x64_chr_end:\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_is_digit
    out.push_str("fn_is_digit:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdx\n");
    } else {
        out.push_str("    mov %rdi, %rdx\n");
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    test %rdx, %rdx\n");
    out.push_str("    jz .L_x64_is_digit_end\n");
    out.push_str("    cmp $256, %rdx\n");
    out.push_str("    jb .L_x64_is_digit_cmp\n");
    out.push_str("    movzbl (%rdx), %edx\n");
    out.push_str(".L_x64_is_digit_cmp:\n");
    out.push_str("    cmp $'0', %edx\n");
    out.push_str("    jb .L_x64_is_digit_end\n");
    out.push_str("    cmp $'9', %edx\n");
    out.push_str("    ja .L_x64_is_digit_end\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str(".L_x64_is_digit_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_is_alpha
    out.push_str("fn_is_alpha:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdx\n");
    } else {
        out.push_str("    mov %rdi, %rdx\n");
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    test %rdx, %rdx\n");
    out.push_str("    jz .L_x64_is_alpha_end\n");
    out.push_str("    cmp $256, %rdx\n");
    out.push_str("    jb .L_x64_is_alpha_cmp\n");
    out.push_str("    movzbl (%rdx), %edx\n");
    out.push_str(".L_x64_is_alpha_cmp:\n");
    out.push_str("    cmp $'_', %edx\n");
    out.push_str("    je .L_x64_is_alpha_true\n");
    out.push_str("    cmp $'a', %edx\n");
    out.push_str("    jb .L_x64_is_alpha_upper\n");
    out.push_str("    cmp $'z', %edx\n");
    out.push_str("    jbe .L_x64_is_alpha_true\n");
    out.push_str(".L_x64_is_alpha_upper:\n");
    out.push_str("    cmp $'A', %edx\n");
    out.push_str("    jb .L_x64_is_alpha_end\n");
    out.push_str("    cmp $'Z', %edx\n");
    out.push_str("    jbe .L_x64_is_alpha_true\n");
    out.push_str("    jmp .L_x64_is_alpha_end\n");
    out.push_str(".L_x64_is_alpha_true:\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str(".L_x64_is_alpha_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_is_alnum
    out.push_str("fn_is_alnum:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rbx\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call fn_is_alpha\n");
        out.push_str("    add $32, %rsp\n");
        out.push_str("    test %rax, %rax\n");
        out.push_str("    jnz .L_x64_is_alnum_end\n");
        out.push_str("    mov %rbx, %rcx\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call fn_is_digit\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    mov %rdi, %rbx\n");
        out.push_str("    call fn_is_alpha\n");
        out.push_str("    test %rax, %rax\n");
        out.push_str("    jnz .L_x64_is_alnum_end\n");
        out.push_str("    mov %rbx, %rdi\n");
        out.push_str("    call fn_is_digit\n");
    }
    out.push_str(".L_x64_is_alnum_end:\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_is_space / fn_is_whitespace
    out.push_str("fn_is_whitespace:\n");
    out.push_str("fn_is_space:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdx\n");
    } else {
        out.push_str("    mov %rdi, %rdx\n");
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    test %rdx, %rdx\n");
    out.push_str("    jz .L_x64_is_space_end\n");
    out.push_str("    cmp $256, %rdx\n");
    out.push_str("    jb .L_x64_is_space_cmp\n");
    out.push_str("    movzbl (%rdx), %edx\n");
    out.push_str(".L_x64_is_space_cmp:\n");
    out.push_str("    cmp $' ', %edx\n");
    out.push_str("    je .L_x64_is_space_true\n");
    out.push_str("    cmp $'\\t', %edx\n");
    out.push_str("    je .L_x64_is_space_true\n");
    out.push_str("    cmp $'\\n', %edx\n");
    out.push_str("    je .L_x64_is_space_true\n");
    out.push_str("    cmp $'\\r', %edx\n");
    out.push_str("    je .L_x64_is_space_true\n");
    out.push_str("    jmp .L_x64_is_space_end\n");
    out.push_str(".L_x64_is_space_true:\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str(".L_x64_is_space_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_file_exists
    out.push_str("fn_file_exists:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $40, %rsp\n");
        out.push_str("    mov %rcx, %rbx\n");
    } else {
        out.push_str("    sub $8, %rsp\n");
        out.push_str("    mov %rdi, %rbx\n");
    }
    out.push_str("    test %rbx, %rbx\n");
    out.push_str("    jz .L_x64_fexists_no\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rbx, %rcx\n");
        out.push_str("    lea alya_str_mode_rb(%rip), %rdx\n");
        out.push_str("    call fopen\n");
    } else {
        out.push_str("    mov %rbx, %rdi\n");
        out.push_str("    lea alya_str_mode_rb(%rip), %rsi\n");
        out.push_str(&format!("    call {}fopen\n", p));
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz .L_x64_fexists_no\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rax, %rcx\n");
        out.push_str("    call fclose\n");
    } else {
        out.push_str("    mov %rax, %rdi\n");
        out.push_str(&format!("    call {}fclose\n", p));
    }
    out.push_str("    mov $1, %rax\n");
    out.push_str("    jmp .L_x64_fexists_end\n");
    out.push_str(".L_x64_fexists_no:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_fexists_end:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    add $40, %rsp\n");
    } else {
        out.push_str("    add $8, %rsp\n");
    }
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_write_file
    out.push_str("fn_write_file:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $40, %rsp\n");
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    mov %rdx, %r13\n");
    } else {
        out.push_str("    sub $8, %rsp\n");
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    mov %rsi, %r13\n");
    }
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_fwrite_fail\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r12, %rcx\n");
        out.push_str("    lea alya_str_mode_wb(%rip), %rdx\n");
        out.push_str("    call fopen\n");
    } else {
        out.push_str("    mov %r12, %rdi\n");
        out.push_str("    lea alya_str_mode_wb(%rip), %rsi\n");
        out.push_str(&format!("    call {}fopen\n", p));
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz .L_x64_fwrite_fail\n");
    out.push_str("    mov %rax, %r14\n"); // r14 = fp
    out.push_str("    xor %r15, %r15\n");
    out.push_str("    test %r13, %r13\n");
    out.push_str("    jz .L_x64_fwrite_do_write\n");
    out.push_str(".L_x64_fwrite_len_loop:\n");
    out.push_str("    cmpb $0, (%r13, %r15)\n");
    out.push_str("    je .L_x64_fwrite_do_write\n");
    out.push_str("    inc %r15\n");
    out.push_str("    jmp .L_x64_fwrite_len_loop\n");
    out.push_str(".L_x64_fwrite_do_write:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    mov $1, %rdx\n");
        out.push_str("    mov %r15, %r8\n");
        out.push_str("    mov %r14, %r9\n");
        out.push_str("    call fwrite\n");
        out.push_str("    mov %r14, %rcx\n");
        out.push_str("    call fclose\n");
    } else {
        out.push_str("    mov %r13, %rdi\n");
        out.push_str("    mov $1, %rsi\n");
        out.push_str("    mov %r15, %rdx\n");
        out.push_str("    mov %r14, %rcx\n");
        out.push_str(&format!("    call {}fwrite\n", p));
        out.push_str("    mov %r14, %rdi\n");
        out.push_str(&format!("    call {}fclose\n", p));
    }
    out.push_str("    mov $1, %rax\n");
    out.push_str("    jmp .L_x64_fwrite_end\n");
    out.push_str(".L_x64_fwrite_fail:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_fwrite_end:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    add $40, %rsp\n");
    } else {
        out.push_str("    add $8, %rsp\n");
    }
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_read_file
    out.push_str("fn_read_file:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $40, %rsp\n");
        out.push_str("    mov %rcx, %r12\n");
    } else {
        out.push_str("    sub $8, %rsp\n");
        out.push_str("    mov %rdi, %r12\n");
    }
    out.push_str("    lea alya_str_empty(%rip), %rbx\n");
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_fread_ret\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r12, %rcx\n");
        out.push_str("    lea alya_str_mode_rb(%rip), %rdx\n");
        out.push_str("    call fopen\n");
    } else {
        out.push_str("    mov %r12, %rdi\n");
        out.push_str("    lea alya_str_mode_rb(%rip), %rsi\n");
        out.push_str(&format!("    call {}fopen\n", p));
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz .L_x64_fread_ret\n");
    out.push_str("    mov %rax, %r12\n"); // r12 = fp
                                          // fseek(fp, 0, 2)
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r12, %rcx\n");
        out.push_str("    xor %rdx, %rdx\n");
        out.push_str("    mov $2, %r8\n");
        out.push_str("    call fseek\n");
        out.push_str("    mov %r12, %rcx\n");
        out.push_str("    call ftell\n");
    } else {
        out.push_str("    mov %r12, %rdi\n");
        out.push_str("    xor %rsi, %rsi\n");
        out.push_str("    mov $2, %rdx\n");
        out.push_str(&format!("    call {}fseek\n", p));
        out.push_str("    mov %r12, %rdi\n");
        out.push_str(&format!("    call {}ftell\n", p));
    }
    out.push_str("    cmp $0, %rax\n");
    out.push_str("    jl .L_x64_fread_close\n");
    out.push_str("    mov %rax, %r13\n"); // r13 = len
                                          // fseek(fp, 0, 0)
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r12, %rcx\n");
        out.push_str("    xor %rdx, %rdx\n");
        out.push_str("    xor %r8, %r8\n");
        out.push_str("    call fseek\n");
        out.push_str("    mov $1, %rcx\n");
        out.push_str("    lea 1(%r13), %rdx\n");
        out.push_str("    call calloc\n");
    } else {
        out.push_str("    mov %r12, %rdi\n");
        out.push_str("    xor %rsi, %rsi\n");
        out.push_str("    xor %rdx, %rdx\n");
        out.push_str(&format!("    call {}fseek\n", p));
        out.push_str("    mov $1, %rdi\n");
        out.push_str("    lea 1(%r13), %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
    }
    out.push_str("    mov %rax, %r14\n"); // r14 = buf
    out.push_str("    mov %rax, %rbx\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r14, %rcx\n");
        out.push_str("    mov $1, %rdx\n");
        out.push_str("    mov %r13, %r8\n");
        out.push_str("    mov %r12, %r9\n");
        out.push_str("    call fread\n");
    } else {
        out.push_str("    mov %r14, %rdi\n");
        out.push_str("    mov $1, %rsi\n");
        out.push_str("    mov %r13, %rdx\n");
        out.push_str("    mov %r12, %rcx\n");
        out.push_str(&format!("    call {}fread\n", p));
    }
    out.push_str("    movb $0, (%r14, %r13)\n");
    out.push_str(".L_x64_fread_close:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r12, %rcx\n");
        out.push_str("    call fclose\n");
    } else {
        out.push_str("    mov %r12, %rdi\n");
        out.push_str(&format!("    call {}fclose\n", p));
    }
    out.push_str(".L_x64_fread_ret:\n");
    out.push_str("    mov %rbx, %rax\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    add $40, %rsp\n");
    } else {
        out.push_str("    add $8, %rsp\n");
    }
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_error_div_zero
    out.push_str("alya_error_div_zero:\n");
    out.push_str("    mov alya_catch_idx(%rip), %r8\n");
    out.push_str("    test %r8, %r8\n");
    out.push_str("    jz .L_x64_fatal_div_zero\n");
    out.push_str("    dec %r8\n");
    out.push_str("    mov %r8, alya_catch_idx(%rip)\n");
    out.push_str("    lea alya_str_div_zero(%rip), %rax\n");
    out.push_str("    mov %rax, alya_err_msg(%rip)\n");
    out.push_str("    lea alya_catch_stack_sp(%rip), %r9\n");
    out.push_str("    mov (%r9, %r8, 8), %rsp\n");
    out.push_str("    lea alya_catch_stack_bp(%rip), %r9\n");
    out.push_str("    mov (%r9, %r8, 8), %rbp\n");
    out.push_str("    lea alya_catch_stack_handler(%rip), %r9\n");
    out.push_str("    mov (%r9, %r8, 8), %r10\n");
    out.push_str("    jmp *%r10\n");
    out.push_str(".L_x64_fatal_div_zero:\n");
    out.push_str("    and $-16, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    lea alya_fmt_div_zero(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    mov $1, %rcx\n");
        out.push_str("    call exit\n\n");
    } else {
        out.push_str("    lea alya_fmt_div_zero(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    mov $1, %rdi\n");
        out.push_str(&format!("    call {}exit\n\n", p));
    }

    // alya_array_new
    out.push_str("alya_array_new:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
    }
    out.push_str("    mov %r12, %r13\n");
    out.push_str("    cmp $8, %r13\n");
    out.push_str("    jge .L_x64_new_cap_ok\n");
    out.push_str("    mov $8, %r13\n");
    out.push_str("    jmp .L_x64_new_alloc\n");
    out.push_str(".L_x64_new_cap_ok:\n");
    out.push_str("    shl $1, %r13\n");
    out.push_str(".L_x64_new_alloc:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov $1, %rcx\n");
        out.push_str("    mov $24, %rdx\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call calloc\n");
        out.push_str("    mov %rax, %r14\n");
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    mov $8, %rdx\n");
        out.push_str("    call calloc\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    mov $1, %rdi\n");
        out.push_str("    mov $24, %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
        out.push_str("    mov %rax, %r14\n");
        out.push_str("    mov %r13, %rdi\n");
        out.push_str("    mov $8, %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
    }
    out.push_str("    mov %r12, (%r14)\n");
    out.push_str("    mov %r13, 8(%r14)\n");
    out.push_str("    mov %rax, 16(%r14)\n");
    out.push_str("    mov %r14, %rax\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_array_push
    out.push_str("alya_array_push:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    mov %rdx, %r13\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    mov %rsi, %r13\n");
    }
    out.push_str("    mov (%r12), %r14\n");
    out.push_str("    mov 8(%r12), %r15\n");
    out.push_str("    cmp %r15, %r14\n");
    out.push_str("    jl .L_x64_push_store\n");
    out.push_str("    test %r15, %r15\n");
    out.push_str("    jnz .L_x64_push_double\n");
    out.push_str("    mov $8, %r15\n");
    out.push_str("    jmp .L_x64_push_realloc\n");
    out.push_str(".L_x64_push_double:\n");
    out.push_str("    shl $1, %r15\n");
    out.push_str(".L_x64_push_realloc:\n");
    out.push_str("    mov %r15, 8(%r12)\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov 16(%r12), %rcx\n");
        out.push_str("    lea (, %r15, 8), %rdx\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call realloc\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    mov 16(%r12), %rdi\n");
        out.push_str("    lea (, %r15, 8), %rsi\n");
        out.push_str(&format!("    call {}realloc\n", p));
    }
    out.push_str("    mov %rax, 16(%r12)\n");
    out.push_str(".L_x64_push_store:\n");
    out.push_str("    mov 16(%r12), %rdx\n");
    out.push_str("    mov (%r12), %rax\n");
    out.push_str("    mov %r13, (%rdx, %rax, 8)\n");
    out.push_str("    inc %rax\n");
    out.push_str("    mov %rax, (%r12)\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_array_pop
    out.push_str("alya_array_pop:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
    }
    out.push_str("    mov (%rax), %rdx\n");
    out.push_str("    test %rdx, %rdx\n");
    out.push_str("    jle alya_error_index_out_of_bounds\n");
    out.push_str("    dec %rdx\n");
    out.push_str("    mov %rdx, (%rax)\n");
    out.push_str("    mov 16(%rax), %rcx\n");
    out.push_str("    mov (%rcx, %rdx, 8), %rax\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_print_array
    out.push_str("alya_print_array:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %rbx\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    test %r12, %r12\n");
        out.push_str("    jnz .L_x64_arr_not_null\n");
        out.push_str("    lea alya_fmt_arr_empty(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    jmp .L_x64_arr_exit\n");
        out.push_str(".L_x64_arr_not_null:\n");
        out.push_str("    mov (%r12), %r13\n");
        out.push_str("    lea alya_fmt_arr_open(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    xor %r14, %r14\n");
        out.push_str(".L_x64_arr_loop:\n");
        out.push_str("    cmp %r13, %r14\n");
        out.push_str("    jge .L_x64_arr_close_call\n");
        out.push_str("    test %r14, %r14\n");
        out.push_str("    jz .L_x64_arr_print_elem\n");
        out.push_str("    lea alya_fmt_arr_comma(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str(".L_x64_arr_print_elem:\n");
        out.push_str("    lea alya_fmt_arr_elem(%rip), %rcx\n");
        out.push_str("    mov 16(%r12), %rax\n");
        out.push_str("    mov (%rax, %r14, 8), %rdx\n");
        out.push_str("    call printf\n");
        out.push_str("    inc %r14\n");
        out.push_str("    jmp .L_x64_arr_loop\n");
        out.push_str(".L_x64_arr_close_call:\n");
        out.push_str("    lea alya_fmt_arr_close(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str(".L_x64_arr_exit:\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    test %r12, %r12\n");
        out.push_str("    jnz .L_x64_arr_not_null\n");
        out.push_str("    lea alya_fmt_arr_empty(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    jmp .L_x64_arr_exit\n");
        out.push_str(".L_x64_arr_not_null:\n");
        out.push_str("    mov (%r12), %r13\n");
        out.push_str("    lea alya_fmt_arr_open(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    xor %r14, %r14\n");
        out.push_str(".L_x64_arr_loop:\n");
        out.push_str("    cmp %r13, %r14\n");
        out.push_str("    jge .L_x64_arr_close_call\n");
        out.push_str("    test %r14, %r14\n");
        out.push_str("    jz .L_x64_arr_print_elem\n");
        out.push_str("    lea alya_fmt_arr_comma(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str(".L_x64_arr_print_elem:\n");
        out.push_str("    lea alya_fmt_arr_elem(%rip), %rdi\n");
        out.push_str("    mov 16(%r12), %rax\n");
        out.push_str("    mov (%rax, %r14, 8), %rsi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    inc %r14\n");
        out.push_str("    jmp .L_x64_arr_loop\n");
        out.push_str(".L_x64_arr_close_call:\n");
        out.push_str("    lea alya_fmt_arr_close(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str(".L_x64_arr_exit:\n");
    }
    out.push_str("    pop %rbx\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_struct_new
    out.push_str("alya_struct_new:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    mov %rdx, %r13\n");
        out.push_str("    lea 1(%r13), %rcx\n");
        out.push_str("    mov $8, %rdx\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call calloc\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    mov %rsi, %r13\n");
        out.push_str("    lea 1(%r13), %rdi\n");
        out.push_str("    mov $8, %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
    }
    out.push_str("    mov %r12, (%rax)\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_print_struct
    out.push_str("alya_print_struct:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    test %r12, %r12\n");
        out.push_str("    jnz .L_x64_struct_not_null\n");
        out.push_str("    lea alya_fmt_struct_null(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    jmp .L_x64_struct_exit\n");
        out.push_str(".L_x64_struct_not_null:\n");
        out.push_str("    mov (%r12), %r13\n");
        out.push_str("    mov 8(%r13), %r14\n");
        out.push_str("    lea alya_fmt_struct_open(%rip), %rcx\n");
        out.push_str("    mov (%r13), %rdx\n");
        out.push_str("    call printf\n");
        out.push_str("    xor %r15, %r15\n");
        out.push_str(".L_x64_struct_loop:\n");
        out.push_str("    cmp %r14, %r15\n");
        out.push_str("    jge .L_x64_struct_close\n");
        out.push_str("    test %r15, %r15\n");
        out.push_str("    jz .L_x64_struct_print_f\n");
        out.push_str("    lea alya_fmt_struct_comma(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str(".L_x64_struct_print_f:\n");
        out.push_str("    lea alya_fmt_struct_field(%rip), %rcx\n");
        out.push_str("    mov 16(%r13, %r15, 8), %rdx\n");
        out.push_str("    mov 8(%r12, %r15, 8), %r8\n");
        out.push_str("    call printf\n");
        out.push_str("    inc %r15\n");
        out.push_str("    jmp .L_x64_struct_loop\n");
        out.push_str(".L_x64_struct_close:\n");
        out.push_str("    lea alya_fmt_struct_close(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str(".L_x64_struct_exit:\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    test %r12, %r12\n");
        out.push_str("    jnz .L_x64_struct_not_null\n");
        out.push_str("    lea alya_fmt_struct_null(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    jmp .L_x64_struct_exit\n");
        out.push_str(".L_x64_struct_not_null:\n");
        out.push_str("    mov (%r12), %r13\n");
        out.push_str("    mov 8(%r13), %r14\n");
        out.push_str("    lea alya_fmt_struct_open(%rip), %rdi\n");
        out.push_str("    mov (%r13), %rsi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    xor %r15, %r15\n");
        out.push_str(".L_x64_struct_loop:\n");
        out.push_str("    cmp %r14, %r15\n");
        out.push_str("    jge .L_x64_struct_close\n");
        out.push_str("    test %r15, %r15\n");
        out.push_str("    jz .L_x64_struct_print_f\n");
        out.push_str("    lea alya_fmt_struct_comma(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str(".L_x64_struct_print_f:\n");
        out.push_str("    lea alya_fmt_struct_field(%rip), %rdi\n");
        out.push_str("    mov 16(%r13, %r15, 8), %rsi\n");
        out.push_str("    mov 8(%r12, %r15, 8), %rdx\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    inc %r15\n");
        out.push_str("    jmp .L_x64_struct_loop\n");
        out.push_str(".L_x64_struct_close:\n");
        out.push_str("    lea alya_fmt_struct_close(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str(".L_x64_struct_exit:\n");
    }
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_error_index_out_of_bounds
    out.push_str("alya_error_index_out_of_bounds:\n");
    out.push_str("    mov alya_catch_idx(%rip), %r8\n");
    out.push_str("    test %r8, %r8\n");
    out.push_str("    jz .L_x64_fatal_bounds\n");
    out.push_str("    dec %r8\n");
    out.push_str("    mov %r8, alya_catch_idx(%rip)\n");
    out.push_str("    lea alya_str_bounds(%rip), %rax\n");
    out.push_str("    mov %rax, alya_err_msg(%rip)\n");
    out.push_str("    lea alya_catch_stack_sp(%rip), %r9\n");
    out.push_str("    mov (%r9, %r8, 8), %rsp\n");
    out.push_str("    lea alya_catch_stack_bp(%rip), %r9\n");
    out.push_str("    mov (%r9, %r8, 8), %rbp\n");
    out.push_str("    lea alya_catch_stack_handler(%rip), %r9\n");
    out.push_str("    mov (%r9, %r8, 8), %r10\n");
    out.push_str("    jmp *%r10\n");
    out.push_str(".L_x64_fatal_bounds:\n");
    out.push_str("    and $-16, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    lea alya_fmt_bounds(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    mov $1, %rcx\n");
        out.push_str("    call exit\n\n");
    } else {
        out.push_str("    lea alya_fmt_bounds(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    mov $1, %rdi\n");
        out.push_str(&format!("    call {}exit\n\n", p));
    }

    // fn_args
    out.push_str(".global fn_args\n");
    out.push_str("fn_args:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    movq alya_argc(%rip), %rax\n");
    out.push_str("    cmpq $1, %rax\n");
    out.push_str("    jg .L_x64_args_has_items\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    xorq %rcx, %rcx\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call alya_array_new\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    xorq %rdi, %rdi\n");
        out.push_str("    call alya_array_new\n");
    }
    out.push_str("    jmp .L_x64_args_ret\n");
    out.push_str(".L_x64_args_has_items:\n");
    out.push_str("    decq %rax\n");
    out.push_str("    movq %rax, %r12\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    movq %r12, %rcx\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call alya_array_new\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    movq %r12, %rdi\n");
        out.push_str("    call alya_array_new\n");
    }
    out.push_str("    movq %rax, %r13\n");
    out.push_str("    movq alya_argv(%rip), %r14\n");
    out.push_str("    xorq %rbx, %rbx\n");
    out.push_str(".L_x64_args_loop:\n");
    out.push_str("    cmpq %r12, %rbx\n");
    out.push_str("    jge .L_x64_args_done\n");
    out.push_str("    movq 8(%r14, %rbx, 8), %rax\n");
    out.push_str("    movq 16(%r13), %rdx\n");
    out.push_str("    movq %rax, (%rdx, %rbx, 8)\n");
    out.push_str("    incq %rbx\n");
    out.push_str("    jmp .L_x64_args_loop\n");
    out.push_str(".L_x64_args_done:\n");
    out.push_str("    movq %r13, %rax\n");
    out.push_str(".L_x64_args_ret:\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_join
    out.push_str(".global fn_join\n");
    out.push_str("fn_join:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    out.push_str("    sub $24, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    movq %rcx, %r12\n"); // arr
        out.push_str("    movq %rdx, %r13\n"); // delim
    } else {
        out.push_str("    movq %rdi, %r12\n");
        out.push_str("    movq %rsi, %r13\n");
    }
    out.push_str("    testq %r12, %r12\n");
    out.push_str("    jz .L_x64_join_empty\n");
    out.push_str("    movq (%r12), %rax\n"); // arr->len
    out.push_str("    testq %rax, %rax\n");
    out.push_str("    jz .L_x64_join_empty\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    movq alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmpq $48000, %rbx\n");
    out.push_str("    jl .L_x64_join_buf_ok\n");
    out.push_str("    xorq %rbx, %rbx\n");
    out.push_str(".L_x64_join_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %rdi\n");
    out.push_str("    movq %rdi, %r14\n"); // r14 = start_ptr
    out.push_str("    xorq %r15, %r15\n"); // r15 = i (0)
    out.push_str(".L_x64_join_loop:\n");
    out.push_str("    cmpq (%r12), %r15\n");
    out.push_str("    jge .L_x64_join_finish\n");
    out.push_str("    testq %r15, %r15\n");
    out.push_str("    jz .L_x64_join_copy_elem\n");
    out.push_str("    movq %r13, %rsi\n"); // copy delim
    out.push_str(".L_x64_join_copy_delim:\n");
    out.push_str("    movb (%rsi), %al\n");
    out.push_str("    testb %al, %al\n");
    out.push_str("    jz .L_x64_join_copy_elem\n");
    out.push_str("    movb %al, (%rdi)\n");
    out.push_str("    incq %rsi\n");
    out.push_str("    incq %rdi\n");
    out.push_str("    jmp .L_x64_join_copy_delim\n");
    out.push_str(".L_x64_join_copy_elem:\n");
    out.push_str("    movq 16(%r12), %rsi\n"); // arr->data
    out.push_str("    movq (%rsi, %r15, 8), %rsi\n"); // arr->data[i]
    out.push_str(".L_x64_join_copy_elem_loop:\n");
    out.push_str("    movb (%rsi), %al\n");
    out.push_str("    testb %al, %al\n");
    out.push_str("    jz .L_x64_join_elem_done\n");
    out.push_str("    movb %al, (%rdi)\n");
    out.push_str("    incq %rsi\n");
    out.push_str("    incq %rdi\n");
    out.push_str("    jmp .L_x64_join_copy_elem_loop\n");
    out.push_str(".L_x64_join_elem_done:\n");
    out.push_str("    incq %r15\n");
    out.push_str("    jmp .L_x64_join_loop\n");
    out.push_str(".L_x64_join_finish:\n");
    out.push_str("    movb $0, (%rdi)\n");
    out.push_str("    incq %rdi\n");
    out.push_str("    subq %r8, %rdi\n");
    out.push_str("    addq $7, %rdi\n");
    out.push_str("    andq $-8, %rdi\n");
    out.push_str("    movq %rdi, alya_str_idx(%rip)\n");
    out.push_str("    movq %r14, %rax\n");
    out.push_str("    jmp .L_x64_join_ret\n");
    out.push_str(".L_x64_join_empty:\n");
    out.push_str("    lea alya_str_empty(%rip), %rax\n");
    out.push_str(".L_x64_join_ret:\n");
    out.push_str("    addq $24, %rsp\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_split
    out.push_str(".global fn_split\n");
    out.push_str("fn_split:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    out.push_str("    sub $56, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    movq %rcx, %r12\n"); // str
        out.push_str("    movq %rdx, %r13\n"); // delim
        out.push_str("    xorq %rcx, %rcx\n");
        out.push_str("    call alya_array_new\n");
    } else {
        out.push_str("    movq %rdi, %r12\n");
        out.push_str("    movq %rsi, %r13\n");
        out.push_str("    xorq %rdi, %rdi\n");
        out.push_str("    call alya_array_new\n");
    }
    out.push_str("    movq %rax, %r14\n"); // r14 = arr
    out.push_str("    movq %r13, %rsi\n");
    out.push_str("    xorq %r15, %r15\n");
    out.push_str(".L_x64_split_dlen_loop:\n");
    out.push_str("    cmpb $0, (%rsi, %r15)\n");
    out.push_str("    je .L_x64_split_dlen_done\n");
    out.push_str("    incq %r15\n");
    out.push_str("    jmp .L_x64_split_dlen_loop\n");
    out.push_str(".L_x64_split_dlen_done:\n");
    out.push_str("    testq %r15, %r15\n");
    out.push_str("    jnz .L_x64_split_non_empty_delim\n");
    out.push_str("    movq %r12, 32(%rsp)\n");
    out.push_str(".L_x64_split_empty_loop:\n");
    out.push_str("    movq 32(%rsp), %rsi\n");
    out.push_str("    movb (%rsi), %al\n");
    out.push_str("    testb %al, %al\n");
    out.push_str("    jz .L_x64_split_ret\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    movq alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmpq $48000, %rbx\n");
    out.push_str("    jl .L_x64_se1\n");
    out.push_str("    xorq %rbx, %rbx\n");
    out.push_str(".L_x64_se1:\n");
    out.push_str("    lea (%r8, %rbx), %rdi\n");
    out.push_str("    movq %rdi, %r10\n");
    out.push_str("    movb %al, (%rdi)\n");
    out.push_str("    movb $0, 1(%rdi)\n");
    out.push_str("    addq $2, %rdi\n");
    out.push_str("    subq %r8, %rdi\n");
    out.push_str("    addq $7, %rdi\n");
    out.push_str("    andq $-8, %rdi\n");
    out.push_str("    movq %rdi, alya_str_idx(%rip)\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    movq %r14, %rcx\n");
        out.push_str("    movq %r10, %rdx\n");
        out.push_str("    call alya_array_push\n");
    } else {
        out.push_str("    movq %r14, %rdi\n");
        out.push_str("    movq %r10, %rsi\n");
        out.push_str("    call alya_array_push\n");
    }
    out.push_str("    incq 32(%rsp)\n");
    out.push_str("    jmp .L_x64_split_empty_loop\n");
    out.push_str(".L_x64_split_non_empty_delim:\n");
    out.push_str("    movq %r12, 32(%rsp)\n"); // curr = str
    out.push_str("    movq %r12, 40(%rsp)\n"); // token_start = str
    out.push_str(".L_x64_split_main_loop:\n");
    out.push_str("    movq 32(%rsp), %rsi\n");
    out.push_str("    cmpb $0, (%rsi)\n");
    out.push_str("    je .L_x64_split_emit_final\n");
    out.push_str("    movq %r13, %rdx\n"); // delim
    out.push_str("    xorq %rbx, %rbx\n"); // k = 0
    out.push_str(".L_x64_split_cmp_loop:\n");
    out.push_str("    cmpq %r15, %rbx\n");
    out.push_str("    jge .L_x64_split_matched\n");
    out.push_str("    movb (%rdx, %rbx), %al\n");
    out.push_str("    cmpb %al, (%rsi, %rbx)\n");
    out.push_str("    jne .L_x64_split_cmp_fail\n");
    out.push_str("    incq %rbx\n");
    out.push_str("    jmp .L_x64_split_cmp_loop\n");
    out.push_str(".L_x64_split_cmp_fail:\n");
    out.push_str("    incq 32(%rsp)\n");
    out.push_str("    jmp .L_x64_split_main_loop\n");
    out.push_str(".L_x64_split_matched:\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    movq alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmpq $48000, %rbx\n");
    out.push_str("    jl .L_x64_se2\n");
    out.push_str("    xorq %rbx, %rbx\n");
    out.push_str(".L_x64_se2:\n");
    out.push_str("    lea (%r8, %rbx), %rdi\n");
    out.push_str("    movq %rdi, 48(%rsp)\n"); // token ptr
    out.push_str("    movq 40(%rsp), %rsi\n"); // src = token_start
    out.push_str("    movq 32(%rsp), %rdx\n"); // end = curr
    out.push_str(".L_x64_split_tok_copy:\n");
    out.push_str("    cmpq %rdx, %rsi\n");
    out.push_str("    jge .L_x64_split_tok_done\n");
    out.push_str("    movb (%rsi), %al\n");
    out.push_str("    movb %al, (%rdi)\n");
    out.push_str("    incq %rsi\n");
    out.push_str("    incq %rdi\n");
    out.push_str("    jmp .L_x64_split_tok_copy\n");
    out.push_str(".L_x64_split_tok_done:\n");
    out.push_str("    movb $0, (%rdi)\n");
    out.push_str("    incq %rdi\n");
    out.push_str("    subq %r8, %rdi\n");
    out.push_str("    addq $7, %rdi\n");
    out.push_str("    andq $-8, %rdi\n");
    out.push_str("    movq %rdi, alya_str_idx(%rip)\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    movq %r14, %rcx\n");
        out.push_str("    movq 48(%rsp), %rdx\n");
        out.push_str("    call alya_array_push\n");
    } else {
        out.push_str("    movq %r14, %rdi\n");
        out.push_str("    movq 48(%rsp), %rsi\n");
        out.push_str("    call alya_array_push\n");
    }
    out.push_str("    addq %r15, 32(%rsp)\n"); // curr += delim_len
    out.push_str("    movq 32(%rsp), %rax\n");
    out.push_str("    movq %rax, 40(%rsp)\n"); // token_start = curr
    out.push_str("    jmp .L_x64_split_main_loop\n");
    out.push_str(".L_x64_split_emit_final:\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    movq alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmpq $48000, %rbx\n");
    out.push_str("    jl .L_x64_se3\n");
    out.push_str("    xorq %rbx, %rbx\n");
    out.push_str(".L_x64_se3:\n");
    out.push_str("    lea (%r8, %rbx), %rdi\n");
    out.push_str("    movq %rdi, 48(%rsp)\n");
    out.push_str("    movq 40(%rsp), %rsi\n"); // token_start
    out.push_str("    movq 32(%rsp), %rdx\n"); // curr
    out.push_str(".L_x64_split_final_copy:\n");
    out.push_str("    cmpq %rdx, %rsi\n");
    out.push_str("    jge .L_x64_split_final_done\n");
    out.push_str("    movb (%rsi), %al\n");
    out.push_str("    movb %al, (%rdi)\n");
    out.push_str("    incq %rsi\n");
    out.push_str("    incq %rdi\n");
    out.push_str("    jmp .L_x64_split_final_copy\n");
    out.push_str(".L_x64_split_final_done:\n");
    out.push_str("    movb $0, (%rdi)\n");
    out.push_str("    incq %rdi\n");
    out.push_str("    subq %r8, %rdi\n");
    out.push_str("    addq $7, %rdi\n");
    out.push_str("    andq $-8, %rdi\n");
    out.push_str("    movq %rdi, alya_str_idx(%rip)\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    movq %r14, %rcx\n");
        out.push_str("    movq 48(%rsp), %rdx\n");
        out.push_str("    call alya_array_push\n");
    } else {
        out.push_str("    movq %r14, %rdi\n");
        out.push_str("    movq 48(%rsp), %rsi\n");
        out.push_str("    call alya_array_push\n");
    }
    out.push_str(".L_x64_split_ret:\n");
    out.push_str("    movq %r14, %rax\n");
    out.push_str("    addq $56, %rsp\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_map_hash
    out.push_str(".global alya_map_hash\n");
    out.push_str("alya_map_hash:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rdx\n");
    } else {
        out.push_str("    mov %rdi, %rdx\n");
    }
    out.push_str("    cmp $256, %rdx\n");
    out.push_str("    jb .L_x64_mhash_int\n");
    out.push_str("    mov $5381, %rax\n");
    out.push_str(".L_x64_mhash_loop:\n");
    out.push_str("    movzbq (%rdx), %rcx\n");
    out.push_str("    test %rcx, %rcx\n");
    out.push_str("    jz .L_x64_mhash_done\n");
    out.push_str("    mov %rax, %r8\n");
    out.push_str("    shl $5, %r8\n");
    out.push_str("    add %r8, %rax\n");
    out.push_str("    add %rcx, %rax\n");
    out.push_str("    inc %rdx\n");
    out.push_str("    jmp .L_x64_mhash_loop\n");
    out.push_str(".L_x64_mhash_int:\n");
    out.push_str("    mov %rdx, %rax\n");
    out.push_str(".L_x64_mhash_done:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_map_key_eq
    out.push_str(".global alya_map_key_eq\n");
    out.push_str("alya_map_key_eq:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r8\n");
        out.push_str("    mov %rdx, %r9\n");
    } else {
        out.push_str("    mov %rdi, %r8\n");
        out.push_str("    mov %rsi, %r9\n");
    }
    out.push_str("    cmp %r8, %r9\n");
    out.push_str("    je .L_x64_mkeq_true\n");
    out.push_str("    cmp $256, %r8\n");
    out.push_str("    jb .L_x64_mkeq_false\n");
    out.push_str("    cmp $256, %r9\n");
    out.push_str("    jb .L_x64_mkeq_false\n");
    out.push_str(".L_x64_mkeq_str:\n");
    out.push_str("    movb (%r8), %al\n");
    out.push_str("    movb (%r9), %cl\n");
    out.push_str("    cmp %al, %cl\n");
    out.push_str("    jne .L_x64_mkeq_false\n");
    out.push_str("    test %al, %al\n");
    out.push_str("    jz .L_x64_mkeq_true\n");
    out.push_str("    inc %r8\n");
    out.push_str("    inc %r9\n");
    out.push_str("    jmp .L_x64_mkeq_str\n");
    out.push_str(".L_x64_mkeq_true:\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str("    jmp .L_x64_mkeq_end\n");
    out.push_str(".L_x64_mkeq_false:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_mkeq_end:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_map
    out.push_str(".global fn_map\n");
    out.push_str("fn_map:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $40, %rsp\n");
        out.push_str("    mov $1, %rcx\n");
        out.push_str("    mov $24, %rdx\n");
        out.push_str("    call calloc\n");
        out.push_str("    mov %rax, %rbx\n");
        out.push_str("    movq $0, (%rbx)\n");
        out.push_str("    movq $64, 8(%rbx)\n");
        out.push_str("    mov $64, %rcx\n");
        out.push_str("    mov $24, %rdx\n");
        out.push_str("    call calloc\n");
        out.push_str("    mov %rax, 16(%rbx)\n");
        out.push_str("    mov %rbx, %rax\n");
        out.push_str("    add $40, %rsp\n");
    } else {
        out.push_str("    sub $8, %rsp\n");
        out.push_str("    mov $1, %rdi\n");
        out.push_str("    mov $24, %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
        out.push_str("    mov %rax, %rbx\n");
        out.push_str("    movq $0, (%rbx)\n");
        out.push_str("    movq $64, 8(%rbx)\n");
        out.push_str("    mov $64, %rdi\n");
        out.push_str("    mov $24, %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
        out.push_str("    mov %rax, 16(%rbx)\n");
        out.push_str("    mov %rbx, %rax\n");
        out.push_str("    add $8, %rsp\n");
    }
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_set
    out.push_str(".global fn_set\n");
    out.push_str("fn_set:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    out.push_str("    sub $72, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    mov %rdx, %r13\n");
        out.push_str("    mov %r8, %r14\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    mov %rsi, %r13\n");
        out.push_str("    mov %rdx, %r14\n");
    }
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_set_done\n");
    out.push_str("    mov (%r12), %rax\n");
    out.push_str("    shl $1, %rax\n");
    out.push_str("    cmp 8(%r12), %rax\n");
    out.push_str("    jl .L_x64_set_no_resize\n");
    out.push_str("    mov 8(%r12), %rax\n");
    out.push_str("    shl $1, %rax\n");
    out.push_str("    mov %rax, 48(%rsp)\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rax, %rcx\n");
        out.push_str("    mov $24, %rdx\n");
        out.push_str("    call calloc\n");
    } else {
        out.push_str("    mov %rax, %rdi\n");
        out.push_str("    mov $24, %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
    }
    out.push_str("    mov %rax, 40(%rsp)\n");
    out.push_str("    mov 48(%rsp), %rax\n");
    out.push_str("    dec %rax\n");
    out.push_str("    mov %rax, 56(%rsp)\n");
    out.push_str("    movq $0, 64(%rsp)\n");
    out.push_str(".L_x64_rehash_loop:\n");
    out.push_str("    mov 64(%rsp), %rax\n");
    out.push_str("    cmp 8(%r12), %rax\n");
    out.push_str("    jge .L_x64_rehash_done\n");
    out.push_str("    mov 16(%r12), %rcx\n");
    out.push_str("    lea (%rax, %rax, 2), %rbx\n");
    out.push_str("    shl $3, %rbx\n");
    out.push_str("    add %rcx, %rbx\n");
    out.push_str("    cmpq $1, 16(%rbx)\n");
    out.push_str("    jne .L_x64_rehash_next\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov (%rbx), %rcx\n");
        out.push_str("    call alya_map_hash\n");
    } else {
        out.push_str("    mov (%rbx), %rdi\n");
        out.push_str("    call alya_map_hash\n");
    }
    out.push_str("    and 56(%rsp), %rax\n");
    out.push_str("    mov 40(%rsp), %rdi\n");
    out.push_str(".L_x64_rehash_probe:\n");
    out.push_str("    lea (%rax, %rax, 2), %r8\n");
    out.push_str("    shl $3, %r8\n");
    out.push_str("    add %rdi, %r8\n");
    out.push_str("    cmpq $0, 16(%r8)\n");
    out.push_str("    je .L_x64_rehash_put\n");
    out.push_str("    inc %rax\n");
    out.push_str("    and 56(%rsp), %rax\n");
    out.push_str("    jmp .L_x64_rehash_probe\n");
    out.push_str(".L_x64_rehash_put:\n");
    out.push_str("    mov 16(%r12), %rcx\n");
    out.push_str("    mov 64(%rsp), %rax\n");
    out.push_str("    lea (%rax, %rax, 2), %rbx\n");
    out.push_str("    shl $3, %rbx\n");
    out.push_str("    add %rcx, %rbx\n");
    out.push_str("    mov (%rbx), %r9\n");
    out.push_str("    mov %r9, (%r8)\n");
    out.push_str("    mov 8(%rbx), %r9\n");
    out.push_str("    mov %r9, 8(%r8)\n");
    out.push_str("    movq $1, 16(%r8)\n");
    out.push_str(".L_x64_rehash_next:\n");
    out.push_str("    incq 64(%rsp)\n");
    out.push_str("    jmp .L_x64_rehash_loop\n");
    out.push_str(".L_x64_rehash_done:\n");
    out.push_str("    mov 48(%rsp), %rax\n");
    out.push_str("    mov %rax, 8(%r12)\n");
    out.push_str("    mov 40(%rsp), %rax\n");
    out.push_str("    mov %rax, 16(%r12)\n");
    out.push_str(".L_x64_set_no_resize:\n");
    out.push_str("    mov 8(%r12), %rax\n");
    out.push_str("    dec %rax\n");
    out.push_str("    mov %rax, 48(%rsp)\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    call alya_map_hash\n");
    } else {
        out.push_str("    mov %r13, %rdi\n");
        out.push_str("    call alya_map_hash\n");
    }
    out.push_str("    and 48(%rsp), %rax\n");
    out.push_str("    mov %rax, 40(%rsp)\n");
    out.push_str("    movq $-1, 32(%rsp)\n");
    out.push_str("    movq $0, 56(%rsp)\n");
    out.push_str(".L_x64_set_probe:\n");
    out.push_str("    mov 56(%rsp), %rax\n");
    out.push_str("    cmp 8(%r12), %rax\n");
    out.push_str("    jge .L_x64_set_use_tomb\n");
    out.push_str("    mov 40(%rsp), %rax\n");
    out.push_str("    lea (%rax, %rax, 2), %r15\n");
    out.push_str("    shl $3, %r15\n");
    out.push_str("    add 16(%r12), %r15\n");
    out.push_str("    mov 16(%r15), %rax\n");
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz .L_x64_set_empty\n");
    out.push_str("    cmp $2, %rax\n");
    out.push_str("    jne .L_x64_set_check_key\n");
    out.push_str("    cmpq $-1, 32(%rsp)\n");
    out.push_str("    jne .L_x64_set_next\n");
    out.push_str("    mov 40(%rsp), %rax\n");
    out.push_str("    mov %rax, 32(%rsp)\n");
    out.push_str("    jmp .L_x64_set_next\n");
    out.push_str(".L_x64_set_check_key:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov (%r15), %rcx\n");
        out.push_str("    mov %r13, %rdx\n");
        out.push_str("    call alya_map_key_eq\n");
    } else {
        out.push_str("    mov (%r15), %rdi\n");
        out.push_str("    mov %r13, %rsi\n");
        out.push_str("    call alya_map_key_eq\n");
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz .L_x64_set_next\n");
    out.push_str("    mov 40(%rsp), %rax\n");
    out.push_str("    lea (%rax, %rax, 2), %r15\n");
    out.push_str("    shl $3, %r15\n");
    out.push_str("    add 16(%r12), %r15\n");
    out.push_str("    mov %r14, 8(%r15)\n");
    out.push_str("    jmp .L_x64_set_done\n");
    out.push_str(".L_x64_set_next:\n");
    out.push_str("    incq 56(%rsp)\n");
    out.push_str("    mov 40(%rsp), %rax\n");
    out.push_str("    inc %rax\n");
    out.push_str("    and 48(%rsp), %rax\n");
    out.push_str("    mov %rax, 40(%rsp)\n");
    out.push_str("    jmp .L_x64_set_probe\n");
    out.push_str(".L_x64_set_empty:\n");
    out.push_str("    cmpq $-1, 32(%rsp)\n");
    out.push_str("    je .L_x64_set_insert_here\n");
    out.push_str(".L_x64_set_use_tomb:\n");
    out.push_str("    cmpq $-1, 32(%rsp)\n");
    out.push_str("    je .L_x64_set_done\n");
    out.push_str("    mov 32(%rsp), %rax\n");
    out.push_str("    lea (%rax, %rax, 2), %r15\n");
    out.push_str("    shl $3, %r15\n");
    out.push_str("    add 16(%r12), %r15\n");
    out.push_str(".L_x64_set_insert_here:\n");
    out.push_str("    mov %r13, (%r15)\n");
    out.push_str("    mov %r14, 8(%r15)\n");
    out.push_str("    movq $1, 16(%r15)\n");
    out.push_str("    incq (%r12)\n");
    out.push_str(".L_x64_set_done:\n");
    out.push_str("    add $72, %rsp\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_get
    out.push_str(".global fn_get\n");
    out.push_str("fn_get:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    out.push_str("    sub $56, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    mov %rdx, %r13\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    mov %rsi, %r13\n");
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_get_ret\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    call alya_map_hash\n");
    } else {
        out.push_str("    mov %r13, %rdi\n");
        out.push_str("    call alya_map_hash\n");
    }
    out.push_str("    mov 8(%r12), %rdx\n");
    out.push_str("    dec %rdx\n");
    out.push_str("    and %rdx, %rax\n");
    out.push_str("    mov %rax, 32(%rsp)\n");
    out.push_str("    mov %rdx, 40(%rsp)\n");
    out.push_str("    movq $0, 48(%rsp)\n");
    out.push_str(".L_x64_get_loop:\n");
    out.push_str("    mov 48(%rsp), %rax\n");
    out.push_str("    cmp 8(%r12), %rax\n");
    out.push_str("    jge .L_x64_get_not_found\n");
    out.push_str("    mov 32(%rsp), %rax\n");
    out.push_str("    lea (%rax, %rax, 2), %r14\n");
    out.push_str("    shl $3, %r14\n");
    out.push_str("    add 16(%r12), %r14\n");
    out.push_str("    cmpq $0, 16(%r14)\n");
    out.push_str("    je .L_x64_get_not_found\n");
    out.push_str("    cmpq $1, 16(%r14)\n");
    out.push_str("    jne .L_x64_get_next\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov (%r14), %rcx\n");
        out.push_str("    mov %r13, %rdx\n");
        out.push_str("    call alya_map_key_eq\n");
    } else {
        out.push_str("    mov (%r14), %rdi\n");
        out.push_str("    mov %r13, %rsi\n");
        out.push_str("    call alya_map_key_eq\n");
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jnz .L_x64_get_found\n");
    out.push_str(".L_x64_get_next:\n");
    out.push_str("    mov 32(%rsp), %rax\n");
    out.push_str("    inc %rax\n");
    out.push_str("    and 40(%rsp), %rax\n");
    out.push_str("    mov %rax, 32(%rsp)\n");
    out.push_str("    incq 48(%rsp)\n");
    out.push_str("    jmp .L_x64_get_loop\n");
    out.push_str(".L_x64_get_found:\n");
    out.push_str("    mov 8(%r14), %rax\n");
    out.push_str("    jmp .L_x64_get_ret\n");
    out.push_str(".L_x64_get_not_found:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_get_ret:\n");
    out.push_str("    add $56, %rsp\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_has
    out.push_str(".global fn_has\n");
    out.push_str("fn_has:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    out.push_str("    sub $56, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    mov %rdx, %r13\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    mov %rsi, %r13\n");
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_has_ret\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    call alya_map_hash\n");
    } else {
        out.push_str("    mov %r13, %rdi\n");
        out.push_str("    call alya_map_hash\n");
    }
    out.push_str("    mov 8(%r12), %rdx\n");
    out.push_str("    dec %rdx\n");
    out.push_str("    and %rdx, %rax\n");
    out.push_str("    mov %rax, 32(%rsp)\n");
    out.push_str("    mov %rdx, 40(%rsp)\n");
    out.push_str("    movq $0, 48(%rsp)\n");
    out.push_str(".L_x64_has_loop:\n");
    out.push_str("    mov 48(%rsp), %rax\n");
    out.push_str("    cmp 8(%r12), %rax\n");
    out.push_str("    jge .L_x64_has_not_found\n");
    out.push_str("    mov 32(%rsp), %rax\n");
    out.push_str("    lea (%rax, %rax, 2), %r14\n");
    out.push_str("    shl $3, %r14\n");
    out.push_str("    add 16(%r12), %r14\n");
    out.push_str("    cmpq $0, 16(%r14)\n");
    out.push_str("    je .L_x64_has_not_found\n");
    out.push_str("    cmpq $1, 16(%r14)\n");
    out.push_str("    jne .L_x64_has_next\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov (%r14), %rcx\n");
        out.push_str("    mov %r13, %rdx\n");
        out.push_str("    call alya_map_key_eq\n");
    } else {
        out.push_str("    mov (%r14), %rdi\n");
        out.push_str("    mov %r13, %rsi\n");
        out.push_str("    call alya_map_key_eq\n");
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jnz .L_x64_has_found\n");
    out.push_str(".L_x64_has_next:\n");
    out.push_str("    mov 32(%rsp), %rax\n");
    out.push_str("    inc %rax\n");
    out.push_str("    and 40(%rsp), %rax\n");
    out.push_str("    mov %rax, 32(%rsp)\n");
    out.push_str("    incq 48(%rsp)\n");
    out.push_str("    jmp .L_x64_has_loop\n");
    out.push_str(".L_x64_has_found:\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str("    jmp .L_x64_has_ret\n");
    out.push_str(".L_x64_has_not_found:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_has_ret:\n");
    out.push_str("    add $56, %rsp\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_remove
    out.push_str(".global fn_remove\n");
    out.push_str("fn_remove:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    out.push_str("    sub $56, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    mov %rdx, %r13\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    mov %rsi, %r13\n");
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_rem_ret\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    call alya_map_hash\n");
    } else {
        out.push_str("    mov %r13, %rdi\n");
        out.push_str("    call alya_map_hash\n");
    }
    out.push_str("    mov 8(%r12), %rdx\n");
    out.push_str("    dec %rdx\n");
    out.push_str("    and %rdx, %rax\n");
    out.push_str("    mov %rax, 32(%rsp)\n");
    out.push_str("    mov %rdx, 40(%rsp)\n");
    out.push_str("    movq $0, 48(%rsp)\n");
    out.push_str(".L_x64_rem_loop:\n");
    out.push_str("    mov 48(%rsp), %rax\n");
    out.push_str("    cmp 8(%r12), %rax\n");
    out.push_str("    jge .L_x64_rem_not_found\n");
    out.push_str("    mov 32(%rsp), %rax\n");
    out.push_str("    lea (%rax, %rax, 2), %r14\n");
    out.push_str("    shl $3, %r14\n");
    out.push_str("    add 16(%r12), %r14\n");
    out.push_str("    cmpq $0, 16(%r14)\n");
    out.push_str("    je .L_x64_rem_not_found\n");
    out.push_str("    cmpq $1, 16(%r14)\n");
    out.push_str("    jne .L_x64_rem_next\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov (%r14), %rcx\n");
        out.push_str("    mov %r13, %rdx\n");
        out.push_str("    call alya_map_key_eq\n");
    } else {
        out.push_str("    mov (%r14), %rdi\n");
        out.push_str("    mov %r13, %rsi\n");
        out.push_str("    call alya_map_key_eq\n");
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jnz .L_x64_rem_found\n");
    out.push_str(".L_x64_rem_next:\n");
    out.push_str("    mov 32(%rsp), %rax\n");
    out.push_str("    inc %rax\n");
    out.push_str("    and 40(%rsp), %rax\n");
    out.push_str("    mov %rax, 32(%rsp)\n");
    out.push_str("    incq 48(%rsp)\n");
    out.push_str("    jmp .L_x64_rem_loop\n");
    out.push_str(".L_x64_rem_found:\n");
    out.push_str("    movq $2, 16(%r14)\n");
    out.push_str("    decq (%r12)\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str("    jmp .L_x64_rem_ret\n");
    out.push_str(".L_x64_rem_not_found:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_rem_ret:\n");
    out.push_str("    add $56, %rsp\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_keys
    out.push_str(".global fn_keys\n");
    out.push_str("fn_keys:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    out.push_str("    sub $56, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
    }
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    xor %rcx, %rcx\n");
        out.push_str("    call alya_array_new\n");
    } else {
        out.push_str("    xor %rdi, %rdi\n");
        out.push_str("    call alya_array_new\n");
    }
    out.push_str("    mov %rax, %r13\n");
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_keys_done\n");
    out.push_str("    mov 16(%r12), %r14\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_keys_loop:\n");
    out.push_str("    cmp 8(%r12), %rbx\n");
    out.push_str("    jge .L_x64_keys_done\n");
    out.push_str("    lea (%rbx, %rbx, 2), %r15\n");
    out.push_str("    shl $3, %r15\n");
    out.push_str("    add %r14, %r15\n");
    out.push_str("    cmpq $1, 16(%r15)\n");
    out.push_str("    jne .L_x64_keys_next\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    mov (%r15), %rdx\n");
        out.push_str("    call alya_array_push\n");
    } else {
        out.push_str("    mov %r13, %rdi\n");
        out.push_str("    mov (%r15), %rsi\n");
        out.push_str("    call alya_array_push\n");
    }
    out.push_str(".L_x64_keys_next:\n");
    out.push_str("    inc %rbx\n");
    out.push_str("    jmp .L_x64_keys_loop\n");
    out.push_str(".L_x64_keys_done:\n");
    out.push_str("    mov %r13, %rax\n");
    out.push_str("    add $56, %rsp\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_values
    out.push_str(".global fn_values\n");
    out.push_str("fn_values:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %r15\n");
    out.push_str("    sub $56, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    xor %rcx, %rcx\n");
        out.push_str("    call alya_array_new\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    xor %rdi, %rdi\n");
        out.push_str("    call alya_array_new\n");
    }
    out.push_str("    mov %rax, %r13\n");
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_vals_done\n");
    out.push_str("    mov 16(%r12), %r14\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_vals_loop:\n");
    out.push_str("    cmp 8(%r12), %rbx\n");
    out.push_str("    jge .L_x64_vals_done\n");
    out.push_str("    lea (%rbx, %rbx, 2), %r15\n");
    out.push_str("    shl $3, %r15\n");
    out.push_str("    add %r14, %r15\n");
    out.push_str("    cmpq $1, 16(%r15)\n");
    out.push_str("    jne .L_x64_vals_next\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    mov 8(%r15), %rdx\n");
        out.push_str("    call alya_array_push\n");
    } else {
        out.push_str("    mov %r13, %rdi\n");
        out.push_str("    mov 8(%r15), %rsi\n");
        out.push_str("    call alya_array_push\n");
    }
    out.push_str(".L_x64_vals_next:\n");
    out.push_str("    inc %rbx\n");
    out.push_str("    jmp .L_x64_vals_loop\n");
    out.push_str(".L_x64_vals_done:\n");
    out.push_str("    mov %r13, %rax\n");
    out.push_str("    add $56, %rsp\n");
    out.push_str("    pop %r15\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // alya_print_map
    out.push_str(".global alya_print_map\n");
    out.push_str("alya_print_map:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    push %rbx\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    test %r12, %r12\n");
        out.push_str("    jnz .L_x64_pmap_not_null\n");
        out.push_str("    lea alya_fmt_map_null(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    jmp .L_x64_pmap_exit\n");
        out.push_str(".L_x64_pmap_not_null:\n");
        out.push_str("    cmpq $0, (%r12)\n");
        out.push_str("    jne .L_x64_pmap_has_items\n");
        out.push_str("    lea alya_fmt_map_empty(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    jmp .L_x64_pmap_exit\n");
        out.push_str(".L_x64_pmap_has_items:\n");
        out.push_str("    lea alya_fmt_map_open(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    xor %rbx, %rbx\n");
        out.push_str("    mov 16(%r12), %r13\n");
        out.push_str("    xor %r14, %r14\n");
        out.push_str(".L_x64_pmap_loop:\n");
        out.push_str("    cmp 8(%r12), %r14\n");
        out.push_str("    jge .L_x64_pmap_close\n");
        out.push_str("    lea (%r14, %r14, 2), %rax\n");
        out.push_str("    shl $3, %rax\n");
        out.push_str("    add %r13, %rax\n");
        out.push_str("    cmpq $1, 16(%rax)\n");
        out.push_str("    jne .L_x64_pmap_next\n");
        out.push_str("    test %rbx, %rbx\n");
        out.push_str("    jz .L_x64_pmap_print_pair\n");
        out.push_str("    lea alya_fmt_arr_comma(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str(".L_x64_pmap_print_pair:\n");
        out.push_str("    lea (%r14, %r14, 2), %rax\n");
        out.push_str("    shl $3, %rax\n");
        out.push_str("    add %r13, %rax\n");
        out.push_str("    mov (%rax), %rdx\n");
        out.push_str("    cmp $256, %rdx\n");
        out.push_str("    jb .L_x64_pmap_key_num\n");
        out.push_str("    lea alya_fmt_prompt(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    jmp .L_x64_pmap_colon\n");
        out.push_str(".L_x64_pmap_key_num:\n");
        out.push_str("    lea alya_fmt_arr_elem(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str(".L_x64_pmap_colon:\n");
        out.push_str("    lea alya_fmt_map_colon(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    lea (%r14, %r14, 2), %rax\n");
        out.push_str("    shl $3, %rax\n");
        out.push_str("    add %r13, %rax\n");
        out.push_str("    mov 8(%rax), %rdx\n");
        out.push_str("    lea alya_fmt_arr_elem(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    inc %rbx\n");
        out.push_str(".L_x64_pmap_next:\n");
        out.push_str("    inc %r14\n");
        out.push_str("    jmp .L_x64_pmap_loop\n");
        out.push_str(".L_x64_pmap_close:\n");
        out.push_str("    lea alya_fmt_map_close(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str(".L_x64_pmap_exit:\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    test %r12, %r12\n");
        out.push_str("    jnz .L_x64_pmap_not_null\n");
        out.push_str("    lea alya_fmt_map_null(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    jmp .L_x64_pmap_exit\n");
        out.push_str(".L_x64_pmap_not_null:\n");
        out.push_str("    cmpq $0, (%r12)\n");
        out.push_str("    jne .L_x64_pmap_has_items\n");
        out.push_str("    lea alya_fmt_map_empty(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    jmp .L_x64_pmap_exit\n");
        out.push_str(".L_x64_pmap_has_items:\n");
        out.push_str("    lea alya_fmt_map_open(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    xor %rbx, %rbx\n");
        out.push_str("    mov 16(%r12), %r13\n");
        out.push_str("    xor %r14, %r14\n");
        out.push_str(".L_x64_pmap_loop:\n");
        out.push_str("    cmp 8(%r12), %r14\n");
        out.push_str("    jge .L_x64_pmap_close\n");
        out.push_str("    lea (%r14, %r14, 2), %rax\n");
        out.push_str("    shl $3, %rax\n");
        out.push_str("    add %r13, %rax\n");
        out.push_str("    cmpq $1, 16(%rax)\n");
        out.push_str("    jne .L_x64_pmap_next\n");
        out.push_str("    test %rbx, %rbx\n");
        out.push_str("    jz .L_x64_pmap_print_pair\n");
        out.push_str("    lea alya_fmt_arr_comma(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str(".L_x64_pmap_print_pair:\n");
        out.push_str("    lea (%r14, %r14, 2), %rax\n");
        out.push_str("    shl $3, %rax\n");
        out.push_str("    add %r13, %rax\n");
        out.push_str("    mov (%rax), %rsi\n");
        out.push_str("    cmp $256, %rsi\n");
        out.push_str("    jb .L_x64_pmap_key_num\n");
        out.push_str("    lea alya_fmt_prompt(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    jmp .L_x64_pmap_colon\n");
        out.push_str(".L_x64_pmap_key_num:\n");
        out.push_str("    lea alya_fmt_arr_elem(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str(".L_x64_pmap_colon:\n");
        out.push_str("    lea alya_fmt_map_colon(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    lea (%r14, %r14, 2), %rax\n");
        out.push_str("    shl $3, %rax\n");
        out.push_str("    add %r13, %rax\n");
        out.push_str("    mov 8(%rax), %rsi\n");
        out.push_str("    lea alya_fmt_arr_elem(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    inc %rbx\n");
        out.push_str(".L_x64_pmap_next:\n");
        out.push_str("    inc %r14\n");
        out.push_str("    jmp .L_x64_pmap_loop\n");
        out.push_str(".L_x64_pmap_close:\n");
        out.push_str("    lea alya_fmt_map_close(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str(".L_x64_pmap_exit:\n");
    }
    out.push_str("    pop %rbx\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");
}
