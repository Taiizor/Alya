use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

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

    // fn_str
    out.push_str(".global fn_str\n");
    out.push_str("fn_str:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
    }
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_str_buf_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_str_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %r12\n");
    out.push_str("    mov %r12, %r14\n");
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jnz .L_x64_str_chk_neg\n");
    out.push_str("    movb $'0', (%r12)\n");
    out.push_str("    movb $0, 1(%r12)\n");
    out.push_str("    add $2, %r12\n");
    out.push_str("    jmp .L_x64_str_finish\n");
    out.push_str(".L_x64_str_chk_neg:\n");
    out.push_str("    jns .L_x64_str_pos\n");
    out.push_str("    movb $'-', (%r12)\n");
    out.push_str("    inc %r12\n");
    out.push_str("    neg %rax\n");
    out.push_str(".L_x64_str_pos:\n");
    out.push_str("    xor %r13, %r13\n");
    out.push_str("    mov $10, %rbx\n");
    out.push_str(".L_x64_str_div_loop:\n");
    out.push_str("    xor %rdx, %rdx\n");
    out.push_str("    div %rbx\n");
    out.push_str("    add $'0', %dl\n");
    out.push_str("    push %rdx\n");
    out.push_str("    inc %r13\n");
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jnz .L_x64_str_div_loop\n");
    out.push_str(".L_x64_str_copy_loop:\n");
    out.push_str("    pop %rdx\n");
    out.push_str("    movb %dl, (%r12)\n");
    out.push_str("    inc %r12\n");
    out.push_str("    dec %r13\n");
    out.push_str("    jnz .L_x64_str_copy_loop\n");
    out.push_str("    movb $0, (%r12)\n");
    out.push_str("    inc %r12\n");
    out.push_str(".L_x64_str_finish:\n");
    out.push_str("    sub %r8, %r12\n");
    out.push_str("    add $7, %r12\n");
    out.push_str("    and $-8, %r12\n");
    out.push_str("    mov %r12, alya_str_idx(%rip)\n");
    out.push_str("    mov %r14, %rax\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
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

}
