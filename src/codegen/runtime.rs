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
    out.push_str("\n.text\n");

    match arch {
        Architecture::ARM64 => {
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
            out.push_str("    ret\n");
        }
        Architecture::X64 => {
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
            out.push_str("    ret\n");
        }
        Architecture::X86 => {
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
            out.push_str("    movb %bl, (%edi)\n");
            out.push_str("    inc %esi\n");
            out.push_str("    inc %edi\n");
            out.push_str("    jmp .L_x86_copy1\n");
            out.push_str(".L_x86_copy2_start:\n");
            out.push_str("    mov %edx, %esi\n");
            out.push_str(".L_x86_copy2:\n");
            out.push_str("    movb (%esi), %bl\n");
            out.push_str("    test %bl, %bl\n");
            out.push_str("    jz .L_x86_concat_end\n");
            out.push_str("    movb %bl, (%edi)\n");
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
            out.push_str("    ret\n");
        }
    }
}
