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

    // fn_get_env
    out.push_str(".global fn_get_env\n");
    out.push_str("fn_get_env:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %rbx\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    sub $32, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %rbx\n");
        out.push_str("    test %rbx, %rbx\n");
        out.push_str("    jz .L_x64_getenv_empty\n");
        out.push_str("    mov %rbx, %rcx\n");
        out.push_str("    call getenv\n");
    } else {
        out.push_str("    mov %rdi, %rbx\n");
        out.push_str("    test %rbx, %rbx\n");
        out.push_str("    jz .L_x64_getenv_empty\n");
        out.push_str("    mov %rbx, %rdi\n");
        out.push_str(&format!("    call {}getenv\n", p));
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz .L_x64_getenv_empty\n");
    out.push_str("    mov %rax, %rsi\n");
    out.push_str("    lea alya_str_buf(%rip), %r8\n");
    out.push_str("    mov alya_str_idx(%rip), %rbx\n");
    out.push_str("    cmp $48000, %rbx\n");
    out.push_str("    jl .L_x64_getenv_buf_ok\n");
    out.push_str("    xor %rbx, %rbx\n");
    out.push_str(".L_x64_getenv_buf_ok:\n");
    out.push_str("    lea (%r8, %rbx), %r12\n");
    out.push_str("    mov %r12, %r14\n");
    out.push_str(".L_x64_getenv_copy:\n");
    out.push_str("    movb (%rsi), %al\n");
    out.push_str("    movb %al, (%r14)\n");
    out.push_str("    testb %al, %al\n");
    out.push_str("    jz .L_x64_getenv_done\n");
    out.push_str("    inc %rsi\n");
    out.push_str("    inc %r14\n");
    out.push_str("    jmp .L_x64_getenv_copy\n");
    out.push_str(".L_x64_getenv_done:\n");
    out.push_str("    inc %r14\n");
    out.push_str("    sub %r8, %r14\n");
    out.push_str("    add $7, %r14\n");
    out.push_str("    and $-8, %r14\n");
    out.push_str("    mov %r14, alya_str_idx(%rip)\n");
    out.push_str("    mov %r12, %rax\n");
    out.push_str("    jmp .L_x64_getenv_ret\n");
    out.push_str(".L_x64_getenv_empty:\n");
    out.push_str("    lea alya_str_empty(%rip), %rax\n");
    out.push_str(".L_x64_getenv_ret:\n");
    out.push_str("    add $32, %rsp\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    pop %rbx\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

}
