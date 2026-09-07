use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let p = if matches!(os, OperatingSystem::MacOS) { "_" } else { "" };
    let _ = (is_win, p);

    // fn_alloc
    out.push_str(".global fn_alloc\n");
    out.push_str("fn_alloc:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push 8(%ebp)\n");
    out.push_str("    call malloc\n");
    out.push_str("    add $4, %esp\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_free
    out.push_str(".global fn_free\n");
    out.push_str("fn_free:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jz .L_x86_free_done\n");
    out.push_str("    push %eax\n");
    out.push_str("    call free\n");
    out.push_str("    add $4, %esp\n");
    out.push_str(".L_x86_free_done:\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_realloc
    out.push_str(".global fn_realloc\n");
    out.push_str("fn_realloc:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push 12(%ebp)\n");
    out.push_str("    push 8(%ebp)\n");
    out.push_str("    call realloc\n");
    out.push_str("    add $8, %esp\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_copy_mem
    out.push_str(".global fn_copy_mem\n");
    out.push_str("fn_copy_mem:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push 16(%ebp)\n");
    out.push_str("    push 12(%ebp)\n");
    out.push_str("    push 8(%ebp)\n");
    out.push_str("    call memcpy\n");
    out.push_str("    add $12, %esp\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_zero_mem
    out.push_str(".global fn_zero_mem\n");
    out.push_str("fn_zero_mem:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push 12(%ebp)\n");
    out.push_str("    push $0\n");
    out.push_str("    push 8(%ebp)\n");
    out.push_str("    call memset\n");
    out.push_str("    add $12, %esp\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_peek_byte
    out.push_str(".global fn_peek_byte\n");
    out.push_str("fn_peek_byte:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %edx\n");
    out.push_str("    mov 12(%ebp), %ecx\n");
    out.push_str("    add %ecx, %edx\n");
    out.push_str("    movzbl (%edx), %eax\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_poke_byte
    out.push_str(".global fn_poke_byte\n");
    out.push_str("fn_poke_byte:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %edx\n");
    out.push_str("    mov 12(%ebp), %ecx\n");
    out.push_str("    mov 16(%ebp), %eax\n");
    out.push_str("    add %ecx, %edx\n");
    out.push_str("    movb %al, (%edx)\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_peek_int
    out.push_str(".global fn_peek_int\n");
    out.push_str("fn_peek_int:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %edx\n");
    out.push_str("    mov 12(%ebp), %ecx\n");
    out.push_str("    add %ecx, %edx\n");
    out.push_str("    mov (%edx), %eax\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_poke_int
    out.push_str(".global fn_poke_int\n");
    out.push_str("fn_poke_int:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %edx\n");
    out.push_str("    mov 12(%ebp), %ecx\n");
    out.push_str("    mov 16(%ebp), %eax\n");
    out.push_str("    add %ecx, %edx\n");
    out.push_str("    mov %eax, (%edx)\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_str_from_ptr
    out.push_str(".global fn_str_from_ptr\n");
    out.push_str("fn_str_from_ptr:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jnz .L_x86_strfromptr_ret\n");
    out.push_str("    mov $alya_str_empty, %eax\n");
    out.push_str(".L_x86_strfromptr_ret:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_str_to_ptr
    out.push_str(".global fn_str_to_ptr\n");
    out.push_str("fn_str_to_ptr:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %eax\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_arena_create
    out.push_str(".global fn_arena_create\n");
    out.push_str("fn_arena_create:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %ebx\n");
    out.push_str("    push %esi\n");
    out.push_str("    mov 8(%ebp), %ebx\n");
    out.push_str("    cmp $0, %ebx\n");
    out.push_str("    jg .L_x86_ac_size_ok\n");
    out.push_str("    mov $65536, %ebx\n");
    out.push_str(".L_x86_ac_size_ok:\n");
    out.push_str("    push $12\n");
    out.push_str("    push $1\n");
    out.push_str("    call calloc\n");
    out.push_str("    add $8, %esp\n");
    out.push_str("    mov %eax, %esi\n");
    out.push_str("    lea 12(%ebx), %eax\n");
    out.push_str("    push %eax\n");
    out.push_str("    call malloc\n");
    out.push_str("    add $4, %esp\n");
    out.push_str("    movl $0, (%eax)\n");
    out.push_str("    movl %ebx, 4(%eax)\n");
    out.push_str("    movl $0, 8(%eax)\n");
    out.push_str("    movl %eax, (%esi)\n");
    out.push_str("    movl %ebx, 4(%esi)\n");
    out.push_str("    movl $0, 8(%esi)\n");
    out.push_str("    mov %esi, %eax\n");
    out.push_str("    pop %esi\n");
    out.push_str("    pop %ebx\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_arena_alloc
    out.push_str(".global fn_arena_alloc\n");
    out.push_str("fn_arena_alloc:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %ebx\n");
    out.push_str("    push %esi\n");
    out.push_str("    push %edi\n");
    out.push_str("    mov 8(%ebp), %esi\n");
    out.push_str("    mov 12(%ebp), %ebx\n");
    out.push_str("    test %esi, %esi\n");
    out.push_str("    jz .L_x86_aa_fail\n");
    out.push_str("    cmp $0, %ebx\n");
    out.push_str("    jle .L_x86_aa_fail\n");
    out.push_str("    add $7, %ebx\n");
    out.push_str("    and $-8, %ebx\n");
    out.push_str("    mov (%esi), %edi\n");
    out.push_str("    test %edi, %edi\n");
    out.push_str("    jz .L_x86_aa_new_chunk\n");
    out.push_str("    mov 8(%edi), %eax\n");
    out.push_str("    add %ebx, %eax\n");
    out.push_str("    cmp 4(%edi), %eax\n");
    out.push_str("    jg .L_x86_aa_new_chunk\n");
    out.push_str("    mov 8(%edi), %edx\n");
    out.push_str("    lea 12(%edi, %edx), %eax\n");
    out.push_str("    mov 8(%edi), %ecx\n");
    out.push_str("    add %ebx, %ecx\n");
    out.push_str("    mov %ecx, 8(%edi)\n");
    out.push_str("    add %ebx, 8(%esi)\n");
    out.push_str("    jmp .L_x86_aa_ret\n");
    out.push_str(".L_x86_aa_new_chunk:\n");
    out.push_str("    mov 4(%esi), %edx\n");
    out.push_str("    cmp %edx, %ebx\n");
    out.push_str("    cmovg %ebx, %edx\n");
    out.push_str("    lea 12(%edx), %eax\n");
    out.push_str("    push %edx\n");
    out.push_str("    push %eax\n");
    out.push_str("    call malloc\n");
    out.push_str("    add $4, %esp\n");
    out.push_str("    pop %edx\n");
    out.push_str("    mov (%esi), %ecx\n");
    out.push_str("    mov %ecx, (%eax)\n");
    out.push_str("    mov %edx, 4(%eax)\n");
    out.push_str("    mov %ebx, 8(%eax)\n");
    out.push_str("    mov %eax, (%esi)\n");
    out.push_str("    add %ebx, 8(%esi)\n");
    out.push_str("    lea 12(%eax), %eax\n");
    out.push_str("    jmp .L_x86_aa_ret\n");
    out.push_str(".L_x86_aa_fail:\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str(".L_x86_aa_ret:\n");
    out.push_str("    pop %edi\n");
    out.push_str("    pop %esi\n");
    out.push_str("    pop %ebx\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_arena_reset
    out.push_str(".global fn_arena_reset\n");
    out.push_str("fn_arena_reset:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %esi\n");
    out.push_str("    mov 8(%ebp), %esi\n");
    out.push_str("    test %esi, %esi\n");
    out.push_str("    jz .L_x86_ar_done\n");
    out.push_str("    mov (%esi), %eax\n");
    out.push_str(".L_x86_ar_loop:\n");
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jz .L_x86_ar_fin\n");
    out.push_str("    movl $0, 8(%eax)\n");
    out.push_str("    mov (%eax), %eax\n");
    out.push_str("    jmp .L_x86_ar_loop\n");
    out.push_str(".L_x86_ar_fin:\n");
    out.push_str("    movl $0, 8(%esi)\n");
    out.push_str(".L_x86_ar_done:\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str("    pop %esi\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_arena_destroy
    out.push_str(".global fn_arena_destroy\n");
    out.push_str("fn_arena_destroy:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %esi\n");
    out.push_str("    push %ebx\n");
    out.push_str("    mov 8(%ebp), %esi\n");
    out.push_str("    test %esi, %esi\n");
    out.push_str("    jz .L_x86_ad_done\n");
    out.push_str("    mov (%esi), %ebx\n");
    out.push_str(".L_x86_ad_loop:\n");
    out.push_str("    test %ebx, %ebx\n");
    out.push_str("    jz .L_x86_ad_free_arena\n");
    out.push_str("    mov (%ebx), %esi\n");
    out.push_str("    push %ebx\n");
    out.push_str("    call free\n");
    out.push_str("    add $4, %esp\n");
    out.push_str("    mov %esi, %ebx\n");
    out.push_str("    jmp .L_x86_ad_loop\n");
    out.push_str(".L_x86_ad_free_arena:\n");
    out.push_str("    mov 8(%ebp), %esi\n");
    out.push_str("    push %esi\n");
    out.push_str("    call free\n");
    out.push_str("    add $4, %esp\n");
    out.push_str(".L_x86_ad_done:\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str("    pop %ebx\n");
    out.push_str("    pop %esi\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_arena_allocated
    out.push_str(".global fn_arena_allocated\n");
    out.push_str("fn_arena_allocated:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    mov 8(%ebp), %edx\n");
    out.push_str("    test %edx, %edx\n");
    out.push_str("    jz .L_x86_aal_zero\n");
    out.push_str("    mov 8(%edx), %eax\n");
    out.push_str("    jmp .L_x86_aal_ret\n");
    out.push_str(".L_x86_aal_zero:\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str(".L_x86_aal_ret:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");
}
