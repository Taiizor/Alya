pub mod builtins;
pub mod control;
pub mod loads;
pub mod ops;
pub mod say;

pub use builtins::*;
pub use control::*;
pub use loads::*;
pub use ops::*;
pub use say::*;

use crate::codegen::target::OperatingSystem;

pub fn emit_header(out: &mut String, os: OperatingSystem) {
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str(".globl _main\n");
        out.push_str(".extern _printf\n");
        out.push_str(".extern _exit\n");
        out.push_str(".extern _getchar\n");
        out.push_str(".extern _fflush\n");
        out.push_str(".extern _calloc\n");
        out.push_str(".extern _malloc\n");
        out.push_str(".extern _free\n");
        out.push_str(".extern _realloc\n");
        out.push_str(".extern _memcpy\n");
        out.push_str(".extern _memset\n");
        out.push_str(".extern _time\n");
        out.push_str(".extern _getenv\n");
        out.push_str(".extern _system\n");
        out.push_str(".extern _usleep\n");
        out.push_str(".extern _mkdir\n\n");
        out.push_str(".text\n");
        out.push_str("_main:\n");
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        out.push_str("    movq %rdi, alya_argc(%rip)\n");
        out.push_str("    movq %rsi, alya_argv(%rip)\n\n");
    } else {
        out.push_str(".global main\n");
        out.push_str(".extern printf\n");
        out.push_str(".extern exit\n");
        out.push_str(".extern getchar\n");
        out.push_str(".extern fflush\n");
        out.push_str(".extern calloc\n");
        out.push_str(".extern malloc\n");
        out.push_str(".extern free\n");
        out.push_str(".extern realloc\n");
        out.push_str(".extern memcpy\n");
        out.push_str(".extern memset\n");
        out.push_str(".extern time\n");
        out.push_str(".extern getenv\n");
        out.push_str(".extern system\n");
        if matches!(os, OperatingSystem::Windows) {
            out.push_str(".extern SetConsoleOutputCP\n");
            out.push_str(".extern SetConsoleCP\n");
            out.push_str(".extern GetConsoleOutputCP\n");
            out.push_str(".extern GetConsoleCP\n");
            out.push_str(".extern GetStdHandle\n");
            out.push_str(".extern GetConsoleMode\n");
            out.push_str(".extern SetConsoleMode\n");
            out.push_str(".extern Sleep\n");
            out.push_str(".extern _mkdir\n\n");
        } else {
            out.push_str(".extern usleep\n");
            out.push_str(".extern mkdir\n\n");
        }
        out.push_str(".text\n");
        out.push_str("main:\n");
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        if matches!(os, OperatingSystem::Windows) {
            out.push_str("    movq %rcx, alya_argc(%rip)\n");
            out.push_str("    movq %rdx, alya_argv(%rip)\n\n");
            out.push_str("    sub $48, %rsp\n");
            out.push_str("    mov $65001, %ecx\n");
            out.push_str("    call SetConsoleOutputCP\n");
            out.push_str("    mov $65001, %ecx\n");
            out.push_str("    call SetConsoleCP\n");
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
            out.push_str("    add $48, %rsp\n\n");
        } else {
            out.push_str("    movq %rdi, alya_argc(%rip)\n");
            out.push_str("    movq %rsi, alya_argv(%rip)\n\n");
        }
    }
}

pub fn emit_footer(out: &mut String) {
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n");
}
