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
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    lea 1(%r12), %rcx\n");
        out.push_str("    mov $8, %rdx\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call calloc\n");
        out.push_str("    add $32, %rsp\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    lea 1(%r12), %rdi\n");
        out.push_str("    mov $8, %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
    }
    out.push_str("    mov %r12, (%rax)\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
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
        out.push_str("    mov 8(%r12, %r14, 8), %rdx\n");
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
        out.push_str("    mov 8(%r12, %r14, 8), %rsi\n");
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
    out.push_str("    push %rbx\n");
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
    out.push_str("    pop %rbx\n");
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
}
