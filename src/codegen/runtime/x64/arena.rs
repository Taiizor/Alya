use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

    // fn_arena_create
    out.push_str(".global fn_arena_create\n");
    out.push_str("fn_arena_create:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    sub $32, %rsp\n");
    if is_win {
        out.push_str("    mov %rcx, %r12\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
    }
    out.push_str("    cmp $0, %r12\n");
    out.push_str("    jg .L_x64_ac_size_ok\n");
    out.push_str("    mov $65536, %r12\n");
    out.push_str(".L_x64_ac_size_ok:\n");
    if is_win {
        out.push_str("    mov $1, %rcx\n");
        out.push_str("    mov $24, %rdx\n");
        out.push_str("    call calloc\n");
    } else {
        out.push_str("    mov $1, %rdi\n");
        out.push_str("    mov $24, %rsi\n");
        out.push_str(&format!("    call {}calloc\n", p));
    }
    out.push_str("    mov %rax, %r13\n");
    out.push_str("    lea 24(%r12), %rax\n");
    if is_win {
        out.push_str("    mov %rax, %rcx\n");
        out.push_str("    call malloc\n");
    } else {
        out.push_str("    mov %rax, %rdi\n");
        out.push_str(&format!("    call {}malloc\n", p));
    }
    out.push_str("    movq $0, (%rax)\n");
    out.push_str("    movq %r12, 8(%rax)\n");
    out.push_str("    movq $0, 16(%rax)\n");
    out.push_str("    movq %rax, (%r13)\n");
    out.push_str("    movq %r12, 8(%r13)\n");
    out.push_str("    movq $0, 16(%r13)\n");
    out.push_str("    mov %r13, %rax\n");
    out.push_str("    add $32, %rsp\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_arena_alloc
    out.push_str(".global fn_arena_alloc\n");
    out.push_str("fn_arena_alloc:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    sub $40, %rsp\n");
    if is_win {
        out.push_str("    mov %rcx, %r12\n");
        out.push_str("    mov %rdx, %r13\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
        out.push_str("    mov %rsi, %r13\n");
    }
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_aa_fail\n");
    out.push_str("    cmp $0, %r13\n");
    out.push_str("    jle .L_x64_aa_fail\n");
    out.push_str("    add $7, %r13\n");
    out.push_str("    and $-8, %r13\n");
    out.push_str("    mov (%r12), %r14\n");
    out.push_str("    test %r14, %r14\n");
    out.push_str("    jz .L_x64_aa_new_chunk\n");
    out.push_str("    mov 16(%r14), %rax\n");
    out.push_str("    add %r13, %rax\n");
    out.push_str("    cmp 8(%r14), %rax\n");
    out.push_str("    jg .L_x64_aa_new_chunk\n");
    out.push_str("    mov 16(%r14), %rdx\n");
    out.push_str("    lea 24(%r14, %rdx), %rax\n");
    out.push_str("    mov 16(%r14), %rcx\n");
    out.push_str("    add %r13, %rcx\n");
    out.push_str("    mov %rcx, 16(%r14)\n");
    out.push_str("    add %r13, 16(%r12)\n");
    out.push_str("    jmp .L_x64_aa_ret\n");
    out.push_str(".L_x64_aa_new_chunk:\n");
    out.push_str("    mov 8(%r12), %rdx\n");
    out.push_str("    cmp %rdx, %r13\n");
    out.push_str("    cmovg %r13, %rdx\n");
    out.push_str("    mov %rdx, -8(%rbp)\n");
    out.push_str("    lea 24(%rdx), %rax\n");
    if is_win {
        out.push_str("    mov %rax, %rcx\n");
        out.push_str("    call malloc\n");
    } else {
        out.push_str("    mov %rax, %rdi\n");
        out.push_str(&format!("    call {}malloc\n", p));
    }
    out.push_str("    mov -8(%rbp), %rdx\n");
    out.push_str("    mov (%r12), %rcx\n");
    out.push_str("    mov %rcx, (%rax)\n");
    out.push_str("    mov %rdx, 8(%rax)\n");
    out.push_str("    mov %r13, 16(%rax)\n");
    out.push_str("    mov %rax, (%r12)\n");
    out.push_str("    add %r13, 16(%r12)\n");
    out.push_str("    lea 24(%rax), %rax\n");
    out.push_str("    jmp .L_x64_aa_ret\n");
    out.push_str(".L_x64_aa_fail:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_aa_ret:\n");
    out.push_str("    add $40, %rsp\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_arena_reset
    out.push_str(".global fn_arena_reset\n");
    out.push_str("fn_arena_reset:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if is_win {
        out.push_str("    mov %rcx, %rax\n");
    } else {
        out.push_str("    mov %rdi, %rax\n");
    }
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz .L_x64_ar_done\n");
    out.push_str("    mov (%rax), %rdx\n");
    out.push_str(".L_x64_ar_loop:\n");
    out.push_str("    test %rdx, %rdx\n");
    out.push_str("    jz .L_x64_ar_fin\n");
    out.push_str("    movq $0, 16(%rdx)\n");
    out.push_str("    mov (%rdx), %rdx\n");
    out.push_str("    jmp .L_x64_ar_loop\n");
    out.push_str(".L_x64_ar_fin:\n");
    out.push_str("    movq $0, 16(%rax)\n");
    out.push_str(".L_x64_ar_done:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_arena_destroy
    out.push_str(".global fn_arena_destroy\n");
    out.push_str("fn_arena_destroy:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    out.push_str("    push %r12\n");
    out.push_str("    push %r13\n");
    out.push_str("    push %r14\n");
    out.push_str("    sub $40, %rsp\n");
    if is_win {
        out.push_str("    mov %rcx, %r12\n");
    } else {
        out.push_str("    mov %rdi, %r12\n");
    }
    out.push_str("    test %r12, %r12\n");
    out.push_str("    jz .L_x64_ad_done\n");
    out.push_str("    mov (%r12), %r13\n");
    out.push_str(".L_x64_ad_loop:\n");
    out.push_str("    test %r13, %r13\n");
    out.push_str("    jz .L_x64_ad_free_arena\n");
    out.push_str("    mov (%r13), %r14\n");
    if is_win {
        out.push_str("    mov %r13, %rcx\n");
        out.push_str("    call free\n");
    } else {
        out.push_str("    mov %r13, %rdi\n");
        out.push_str(&format!("    call {}free\n", p));
    }
    out.push_str("    mov %r14, %r13\n");
    out.push_str("    jmp .L_x64_ad_loop\n");
    out.push_str(".L_x64_ad_free_arena:\n");
    if is_win {
        out.push_str("    mov %r12, %rcx\n");
        out.push_str("    call free\n");
    } else {
        out.push_str("    mov %r12, %rdi\n");
        out.push_str(&format!("    call {}free\n", p));
    }
    out.push_str(".L_x64_ad_done:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    add $40, %rsp\n");
    out.push_str("    pop %r14\n");
    out.push_str("    pop %r13\n");
    out.push_str("    pop %r12\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");

    // fn_arena_allocated
    out.push_str(".global fn_arena_allocated\n");
    out.push_str("fn_arena_allocated:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
    if is_win {
        out.push_str("    test %rcx, %rcx\n");
        out.push_str("    jz .L_x64_aal_zero\n");
        out.push_str("    mov 16(%rcx), %rax\n");
    } else {
        out.push_str("    test %rdi, %rdi\n");
        out.push_str("    jz .L_x64_aal_zero\n");
        out.push_str("    mov 16(%rdi), %rax\n");
    }
    out.push_str("    jmp .L_x64_aal_ret\n");
    out.push_str(".L_x64_aal_zero:\n");
    out.push_str("    xor %rax, %rax\n");
    out.push_str(".L_x64_aal_ret:\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n\n");
}
