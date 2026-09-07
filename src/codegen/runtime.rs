use super::target::{Architecture, OperatingSystem};

pub fn emit_runtime(out: &mut String, arch: Architecture, os: OperatingSystem) {
    out.push_str("\n.section .bss\n");
    out.push_str(".align 16\n");
    out.push_str("alya_str_buf:\n");
    out.push_str("    .space 65536\n");
    match arch {
        Architecture::ARM64 | Architecture::X64 => {
            out.push_str("alya_str_idx:\n");
            out.push_str("    .quad 0\n");
        }
        Architecture::X86 => {
            out.push_str("alya_str_idx:\n");
            out.push_str("    .long 0\n");
        }
    }

    out.push_str("\n.section .rodata\n");
    out.push_str("alya_fmt_prompt:\n");
    out.push_str("    .string \"%s\"\n");
    out.push_str("alya_fmt_div_zero:\n");
    out.push_str("    .string \"Runtime error: division by zero\\n\"\n");
    out.push_str(".text\n");

    match arch {
        Architecture::ARM64 => {
            // alya_concat
            out.push_str(".align 2\n");
            out.push_str("alya_concat:\n");
            out.push_str("    stp x29, x30, [sp, #-16]!\n");
            out.push_str("    mov x29, sp\n");
            out.push_str("    stp x19, x20, [sp, #-16]!\n");
            out.push_str("    stp x21, x22, [sp, #-16]!\n");
            out.push_str("    adrp x19, alya_str_buf\n");
            out.push_str("    add x19, x19, :lo12:alya_str_buf\n");
            out.push_str("    adrp x20, alya_str_idx\n");
            out.push_str("    add x20, x20, :lo12:alya_str_idx\n");
            out.push_str("    ldr x21, [x20]\n");
            out.push_str("    cmp x21, #48000\n");
            out.push_str("    b.lt .L_arm_concat_ok\n");
            out.push_str("    mov x21, #0\n");
            out.push_str(".L_arm_concat_ok:\n");
            out.push_str("    add x22, x19, x21\n");
            out.push_str(".L_arm_copy1:\n");
            out.push_str("    ldrb w2, [x0], #1\n");
            out.push_str("    cbz w2, .L_arm_copy2_start\n");
            out.push_str("    strb w2, [x22], #1\n");
            out.push_str("    b .L_arm_copy1\n");
            out.push_str(".L_arm_copy2_start:\n");
            out.push_str(".L_arm_copy2:\n");
            out.push_str("    ldrb w2, [x1], #1\n");
            out.push_str("    cbz w2, .L_arm_concat_end\n");
            out.push_str("    strb w2, [x22], #1\n");
            out.push_str("    b .L_arm_copy2\n");
            out.push_str(".L_arm_concat_end:\n");
            out.push_str("    strb wzr, [x22], #1\n");
            out.push_str("    sub x2, x22, x19\n");
            out.push_str("    add x2, x2, #7\n");
            out.push_str("    and x2, x2, #~7\n");
            out.push_str("    str x2, [x20]\n");
            out.push_str("    add x0, x19, x21\n");
            out.push_str("    ldp x21, x22, [sp], #16\n");
            out.push_str("    ldp x19, x20, [sp], #16\n");
            out.push_str("    ldp x29, x30, [sp], #16\n");
            out.push_str("    ret\n\n");

            // fn_len
            out.push_str("fn_len:\n");
            out.push_str("    mov x1, x0\n");
            out.push_str("    mov x0, #0\n");
            out.push_str(".L_arm_len_loop:\n");
            out.push_str("    ldrb w2, [x1, x0]\n");
            out.push_str("    cbz w2, .L_arm_len_end\n");
            out.push_str("    add x0, x0, #1\n");
            out.push_str("    b .L_arm_len_loop\n");
            out.push_str(".L_arm_len_end:\n");
            out.push_str("    ret\n\n");

            // fn_abs
            out.push_str("fn_abs:\n");
            out.push_str("    cmp x0, #0\n");
            out.push_str("    b.ge .L_arm_abs_end\n");
            out.push_str("    neg x0, x0\n");
            out.push_str(".L_arm_abs_end:\n");
            out.push_str("    ret\n\n");

            // fn_min
            out.push_str("fn_min:\n");
            out.push_str("    cmp x0, x1\n");
            out.push_str("    csel x0, x0, x1, le\n");
            out.push_str("    ret\n\n");

            // fn_max
            out.push_str("fn_max:\n");
            out.push_str("    cmp x0, x1\n");
            out.push_str("    csel x0, x0, x1, ge\n");
            out.push_str("    ret\n\n");

            // fn_exit
            out.push_str("fn_exit:\n");
            out.push_str("    b exit\n\n");

            // fn_ask
            out.push_str("fn_ask:\n");
            out.push_str("    stp x29, x30, [sp, #-16]!\n");
            out.push_str("    mov x29, sp\n");
            out.push_str("    stp x19, x20, [sp, #-16]!\n");
            out.push_str("    stp x21, x22, [sp, #-16]!\n");
            out.push_str("    cbz x0, .L_arm_ask_read\n");
            out.push_str("    mov x1, x0\n");
            out.push_str("    adrp x0, alya_fmt_prompt\n");
            out.push_str("    add x0, x0, :lo12:alya_fmt_prompt\n");
            out.push_str("    bl printf\n");
            out.push_str("    mov x0, #0\n");
            out.push_str("    bl fflush\n");
            out.push_str(".L_arm_ask_read:\n");
            out.push_str("    adrp x19, alya_str_buf\n");
            out.push_str("    add x19, x19, :lo12:alya_str_buf\n");
            out.push_str("    adrp x20, alya_str_idx\n");
            out.push_str("    add x20, x20, :lo12:alya_str_idx\n");
            out.push_str("    ldr x2, [x20]\n");
            out.push_str("    cmp x2, #48000\n");
            out.push_str("    b.lt .L_arm_ask_buf_ok\n");
            out.push_str("    mov x2, #0\n");
            out.push_str(".L_arm_ask_buf_ok:\n");
            out.push_str("    add x19, x19, x2\n");
            out.push_str("    mov x21, x19\n");
            out.push_str(".L_arm_ask_loop:\n");
            out.push_str("    bl getchar\n");
            out.push_str("    cmp w0, #-1\n");
            out.push_str("    b.eq .L_arm_ask_done\n");
            out.push_str("    cmp w0, #10\n");
            out.push_str("    b.eq .L_arm_ask_done\n");
            out.push_str("    cmp w0, #13\n");
            out.push_str("    b.eq .L_arm_ask_loop\n");
            out.push_str("    strb w0, [x19], #1\n");
            out.push_str("    b .L_arm_ask_loop\n");
            out.push_str(".L_arm_ask_done:\n");
            out.push_str("    strb wzr, [x19], #1\n");
            out.push_str("    adrp x1, alya_str_buf\n");
            out.push_str("    add x1, x1, :lo12:alya_str_buf\n");
            out.push_str("    sub x2, x19, x1\n");
            out.push_str("    add x2, x2, #7\n");
            out.push_str("    and x2, x2, #~7\n");
            out.push_str("    str x2, [x20]\n");
            out.push_str("    mov x0, x21\n");
            out.push_str("    ldp x21, x22, [sp], #16\n");
            out.push_str("    ldp x19, x20, [sp], #16\n");
            out.push_str("    ldp x29, x30, [sp], #16\n");
            out.push_str("    ret\n\n");

            // alya_error_div_zero
            out.push_str("alya_error_div_zero:\n");
            out.push_str("    mov x19, sp\n");
            out.push_str("    and x19, x19, #~15\n");
            out.push_str("    mov sp, x19\n");
            out.push_str("    adrp x0, alya_fmt_div_zero\n");
            out.push_str("    add x0, x0, :lo12:alya_fmt_div_zero\n");
            out.push_str("    bl printf\n");
            out.push_str("    mov w0, #1\n");
            out.push_str("    bl exit\n\n");
        }
        Architecture::X64 => {
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

            // fn_exit
            out.push_str("fn_exit:\n");
            out.push_str("    push %rbp\n");
            out.push_str("    mov %rsp, %rbp\n");
            if matches!(os, OperatingSystem::Windows) {
                out.push_str("    sub $32, %rsp\n");
                out.push_str("    call exit\n");
            } else {
                out.push_str("    sub $8, %rsp\n");
                out.push_str("    call exit\n");
            }

            // fn_ask
            out.push_str("fn_ask:\n");
            out.push_str("    push %rbp\n");
            out.push_str("    mov %rsp, %rbp\n");
            out.push_str("    push %rbx\n");
            out.push_str("    push %rsi\n");
            out.push_str("    push %rdi\n");
            out.push_str("    push %r12\n");
            out.push_str("    sub $48, %rsp\n");
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
                out.push_str("    call printf\n");
                out.push_str("    xor %rdi, %rdi\n");
                out.push_str("    call fflush\n");
            }
            out.push_str(".L_x64_ask_read:\n");
            out.push_str("    lea alya_str_buf(%rip), %rbx\n");
            out.push_str("    mov alya_str_idx(%rip), %rsi\n");
            out.push_str("    cmp $48000, %rsi\n");
            out.push_str("    jl .L_x64_ask_buf_ok\n");
            out.push_str("    xor %rsi, %rsi\n");
            out.push_str(".L_x64_ask_buf_ok:\n");
            out.push_str("    lea (%rbx, %rsi), %r12\n");
            out.push_str("    mov %r12, %rdi\n");
            out.push_str(".L_x64_ask_loop:\n");
            out.push_str("    call getchar\n");
            out.push_str("    cmp $-1, %rax\n");
            out.push_str("    je .L_x64_ask_done\n");
            out.push_str("    cmp $10, %rax\n");
            out.push_str("    je .L_x64_ask_done\n");
            out.push_str("    cmp $13, %rax\n");
            out.push_str("    je .L_x64_ask_loop\n");
            out.push_str("    movb %al, (%rdi)\n");
            out.push_str("    inc %rdi\n");
            out.push_str("    jmp .L_x64_ask_loop\n");
            out.push_str(".L_x64_ask_done:\n");
            out.push_str("    movb $0, (%rdi)\n");
            out.push_str("    inc %rdi\n");
            out.push_str("    sub %rbx, %rdi\n");
            out.push_str("    add $7, %rdi\n");
            out.push_str("    and $-8, %rdi\n");
            out.push_str("    mov %rdi, alya_str_idx(%rip)\n");
            out.push_str("    mov %r12, %rax\n");
            out.push_str("    add $48, %rsp\n");
            out.push_str("    pop %r12\n");
            out.push_str("    pop %rdi\n");
            out.push_str("    pop %rsi\n");
            out.push_str("    pop %rbx\n");
            out.push_str("    mov %rbp, %rsp\n");
            out.push_str("    pop %rbp\n");
            out.push_str("    ret\n\n");

            // alya_error_div_zero
            out.push_str("alya_error_div_zero:\n");
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
                out.push_str("    call printf\n");
                out.push_str("    mov $1, %rdi\n");
                out.push_str("    call exit\n\n");
            }
        }
        Architecture::X86 => {
            // alya_concat
            out.push_str("alya_concat:\n");
            out.push_str("    push %ebp\n");
            out.push_str("    mov %esp, %ebp\n");
            out.push_str("    push %esi\n");
            out.push_str("    push %edi\n");
            out.push_str("    push %ebx\n");
            out.push_str("    mov 8(%ebp), %esi\n");
            out.push_str("    mov 12(%ebp), %edx\n");
            out.push_str("    mov $alya_str_buf, %ecx\n");
            out.push_str("    mov alya_str_idx, %ebx\n");
            out.push_str("    cmp $48000, %ebx\n");
            out.push_str("    jl .L_x86_concat_ok\n");
            out.push_str("    xor %ebx, %ebx\n");
            out.push_str(".L_x86_concat_ok:\n");
            out.push_str("    lea (%ecx, %ebx), %edi\n");
            out.push_str("    mov %edi, %eax\n");
            out.push_str(".L_x86_copy1:\n");
            out.push_str("    movb (%esi), %bl\n");
            out.push_str("    test %bl, %bl\n");
            out.push_str("    jz .L_x86_copy2_start\n");
            out.push_str("    movb %cl, (%edi)\n");
            out.push_str("    inc %esi\n");
            out.push_str("    inc %edi\n");
            out.push_str("    jmp .L_x86_copy1\n");
            out.push_str(".L_x86_copy2_start:\n");
            out.push_str("    mov %edx, %esi\n");
            out.push_str(".L_x86_copy2:\n");
            out.push_str("    movb (%esi), %bl\n");
            out.push_str("    test %bl, %bl\n");
            out.push_str("    jz .L_x86_concat_end\n");
            out.push_str("    movb %cl, (%edi)\n");
            out.push_str("    inc %esi\n");
            out.push_str("    inc %edi\n");
            out.push_str("    jmp .L_x86_copy2\n");
            out.push_str(".L_x86_concat_end:\n");
            out.push_str("    movb $0, (%edi)\n");
            out.push_str("    inc %edi\n");
            out.push_str("    sub %ecx, %edi\n");
            out.push_str("    add $3, %edi\n");
            out.push_str("    and $-4, %edi\n");
            out.push_str("    mov %edi, alya_str_idx\n");
            out.push_str("    pop %ebx\n");
            out.push_str("    pop %edi\n");
            out.push_str("    pop %esi\n");
            out.push_str("    mov %ebp, %esp\n");
            out.push_str("    pop %ebp\n");
            out.push_str("    ret\n\n");

            // fn_len
            out.push_str("fn_len:\n");
            out.push_str("    push %ebp\n");
            out.push_str("    mov %esp, %ebp\n");
            out.push_str("    mov 8(%ebp), %edx\n");
            out.push_str("    xor %eax, %eax\n");
            out.push_str(".L_x86_len_loop:\n");
            out.push_str("    cmpb $0, (%edx, %eax)\n");
            out.push_str("    je .L_x86_len_end\n");
            out.push_str("    inc %eax\n");
            out.push_str("    jmp .L_x86_len_loop\n");
            out.push_str(".L_x86_len_end:\n");
            out.push_str("    mov %ebp, %esp\n");
            out.push_str("    pop %ebp\n");
            out.push_str("    ret\n\n");

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

            // fn_exit
            out.push_str("fn_exit:\n");
            out.push_str("    push %ebp\n");
            out.push_str("    mov %esp, %ebp\n");
            out.push_str("    push 8(%ebp)\n");
            out.push_str("    call exit\n\n");

            // fn_ask
            out.push_str("fn_ask:\n");
            out.push_str("    push %ebp\n");
            out.push_str("    mov %esp, %ebp\n");
            out.push_str("    push %esi\n");
            out.push_str("    push %edi\n");
            out.push_str("    push %ebx\n");
            out.push_str("    mov 8(%ebp), %eax\n");
            out.push_str("    test %eax, %eax\n");
            out.push_str("    jz .L_x86_ask_read\n");
            out.push_str("    push %eax\n");
            out.push_str("    push $alya_fmt_prompt\n");
            out.push_str("    call printf\n");
            out.push_str("    add $8, %esp\n");
            out.push_str("    push $0\n");
            out.push_str("    call fflush\n");
            out.push_str("    add $4, %esp\n");
            out.push_str(".L_x86_ask_read:\n");
            out.push_str("    mov $alya_str_buf, %ecx\n");
            out.push_str("    mov alya_str_idx, %ebx\n");
            out.push_str("    cmp $48000, %ebx\n");
            out.push_str("    jl .L_x86_ask_buf_ok\n");
            out.push_str("    xor %ebx, %ebx\n");
            out.push_str(".L_x86_ask_buf_ok:\n");
            out.push_str("    lea (%ecx, %ebx), %esi\n");
            out.push_str("    mov %esi, %edi\n");
            out.push_str(".L_x86_ask_loop:\n");
            out.push_str("    call getchar\n");
            out.push_str("    cmp $-1, %eax\n");
            out.push_str("    je .L_x86_ask_done\n");
            out.push_str("    cmp $10, %eax\n");
            out.push_str("    je .L_x86_ask_done\n");
            out.push_str("    cmp $13, %eax\n");
            out.push_str("    je .L_x86_ask_loop\n");
            out.push_str("    movb %al, (%edi)\n");
            out.push_str("    inc %edi\n");
            out.push_str("    jmp .L_x86_ask_loop\n");
            out.push_str(".L_x86_ask_done:\n");
            out.push_str("    movb $0, (%edi)\n");
            out.push_str("    inc %edi\n");
            out.push_str("    sub %ecx, %edi\n");
            out.push_str("    add $3, %edi\n");
            out.push_str("    and $-4, %edi\n");
            out.push_str("    mov %edi, alya_str_idx\n");
            out.push_str("    mov %esi, %eax\n");
            out.push_str("    pop %ebx\n");
            out.push_str("    pop %edi\n");
            out.push_str("    pop %esi\n");
            out.push_str("    mov %ebp, %esp\n");
            out.push_str("    pop %ebp\n");
            out.push_str("    ret\n\n");

            // alya_error_div_zero
            out.push_str("alya_error_div_zero:\n");
            out.push_str("    and $-16, %esp\n");
            out.push_str("    push $alya_fmt_div_zero\n");
            out.push_str("    call printf\n");
            out.push_str("    push $1\n");
            out.push_str("    call exit\n\n");
        }
    }
}
