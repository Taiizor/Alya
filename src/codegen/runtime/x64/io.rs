use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

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
    out.push_str("    cmp $950000, %rsi\n");
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
    out.push_str("    cmp $950000, %rbx\n");
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

    // fn_target_os
    out.push_str(".global fn_target_os\n");
    out.push_str("fn_target_os:\n");
    out.push_str("    lea alya_str_target_os(%rip), %rax\n");
    out.push_str("    ret\n\n");

    // fn_target_arch
    out.push_str(".global fn_target_arch\n");
    out.push_str("fn_target_arch:\n");
    out.push_str("    lea alya_str_target_arch(%rip), %rax\n");
    out.push_str("    ret\n\n");

    // fn_set_console_output_cp
    out.push_str(".global fn_set_console_output_cp\n");
    out.push_str("fn_set_console_output_cp:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call SetConsoleOutputCP\n");
        out.push_str("    add $32, %rsp\n");
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    ret\n\n");
    } else {
        out.push_str("    mov $1, %rax\n");
        out.push_str("    ret\n\n");
    }

    // fn_set_console_input_cp
    out.push_str(".global fn_set_console_input_cp\n");
    out.push_str("fn_set_console_input_cp:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call SetConsoleCP\n");
        out.push_str("    add $32, %rsp\n");
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    ret\n\n");
    } else {
        out.push_str("    mov $1, %rax\n");
        out.push_str("    ret\n\n");
    }

    // fn_get_console_output_cp
    out.push_str(".global fn_get_console_output_cp\n");
    out.push_str("fn_get_console_output_cp:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call GetConsoleOutputCP\n");
        out.push_str("    add $32, %rsp\n");
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    ret\n\n");
    } else {
        out.push_str("    mov $65001, %rax\n");
        out.push_str("    ret\n\n");
    }

    // fn_get_console_input_cp
    out.push_str(".global fn_get_console_input_cp\n");
    out.push_str("fn_get_console_input_cp:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call GetConsoleCP\n");
        out.push_str("    add $32, %rsp\n");
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    ret\n\n");
    } else {
        out.push_str("    mov $65001, %rax\n");
        out.push_str("    ret\n\n");
    }

    // fn_enable_virtual_terminal
    out.push_str(".global fn_enable_virtual_terminal\n");
    out.push_str("fn_enable_virtual_terminal:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $48, %rsp\n");
        out.push_str("    mov $-11, %ecx\n");
        out.push_str("    call GetStdHandle\n");
        out.push_str("    mov %rax, %rcx\n");
        out.push_str("    lea 32(%rsp), %rdx\n");
        out.push_str("    call GetConsoleMode\n");
        out.push_str("    mov $-11, %ecx\n");
        out.push_str("    call GetStdHandle\n");
        out.push_str("    mov %rax, %rcx\n");
        out.push_str("    mov 32(%rsp), %edx\n");
        out.push_str("    or $4, %edx\n");
        out.push_str("    call SetConsoleMode\n");
        out.push_str("    add $48, %rsp\n");
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    ret\n\n");
    } else {
        out.push_str("    mov $1, %rax\n");
        out.push_str("    ret\n\n");
    }

    // fn_set_console_title
    out.push_str(".global fn_set_console_title\n");
    out.push_str("fn_set_console_title:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    call SetConsoleTitleA\n");
        out.push_str("    add $32, %rsp\n");
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    mov $1, %rax\n");
        out.push_str("    ret\n\n");
    } else {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $16, %rsp\n");
        out.push_str("    mov %rdi, %rsi\n");
        out.push_str("    lea alya_fmt_console_title(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    xor %rdi, %rdi\n");
        out.push_str(&format!("    call {}fflush\n", p));
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    mov $1, %rax\n");
        out.push_str("    ret\n\n");
    }

    // fn_beep_console
    out.push_str(".global fn_beep_console\n");
    out.push_str("fn_beep_console:\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $32, %rsp\n");
        out.push_str("    mov $750, %ecx\n");
        out.push_str("    mov $150, %edx\n");
        out.push_str("    call Beep\n");
        out.push_str("    add $32, %rsp\n");
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    mov $1, %rax\n");
        out.push_str("    ret\n\n");
    } else {
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    sub $16, %rsp\n");
        out.push_str("    lea alya_str_console_bell(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    xor %rdi, %rdi\n");
        out.push_str(&format!("    call {}fflush\n", p));
        out.push_str("    mov %rbp, %rsp\n");
        out.push_str("    pop %rbp\n");
        out.push_str("    mov $1, %rax\n");
        out.push_str("    ret\n\n");
    }

    // fn_clear_console
    out.push_str(".global fn_clear_console\n");
    out.push_str("fn_clear_console:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    sub $32, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    lea alya_str_console_clear(%rip), %rcx\n");
        out.push_str("    call printf\n");
        out.push_str("    xor %rcx, %rcx\n");
        out.push_str("    call fflush\n");
    } else {
        out.push_str("    lea alya_str_console_clear(%rip), %rdi\n");
        out.push_str("    xor %rax, %rax\n");
        out.push_str(&format!("    call {}printf\n", p));
        out.push_str("    xor %rdi, %rdi\n");
        out.push_str(&format!("    call {}fflush\n", p));
    }
    out.push_str("    add $32, %rsp\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    mov $1, %rax\n");
    out.push_str("    ret\n\n");
}

