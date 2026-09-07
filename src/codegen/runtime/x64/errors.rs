use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

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

    // fn_sleep
    out.push_str(".global fn_sleep\n");
    out.push_str("fn_sleep:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    sub $32, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    call Sleep\n");
    } else {
        out.push_str("    imul $1000, %rdi, %rdi\n");
        out.push_str(&format!("    call {}usleep\n", p));
    }
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    add $32, %rsp\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_system_exec
    out.push_str(".global fn_system_exec\n");
    out.push_str("fn_system_exec:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    sub $32, %rsp\n");
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    test %rcx, %rcx\n");
        out.push_str("    jz .L_x64_sysexec_empty\n");
        out.push_str("    call system\n");
    } else {
        out.push_str("    test %rdi, %rdi\n");
        out.push_str("    jz .L_x64_sysexec_empty\n");
        out.push_str(&format!("    call {}system\n", p));
    }
    out.push_str("    jmp .L_x64_sysexec_ret\n");
    out.push_str(".L_x64_sysexec_empty:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_sysexec_ret:\n");
    out.push_str("    add $32, %rsp\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");



}
