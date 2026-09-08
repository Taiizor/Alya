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

pub fn emit_header(out: &mut String) {
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
    out.push_str(".extern Sleep\n");
    out.push_str(".extern usleep\n");
    out.push_str(".extern _mkdir\n");
    out.push_str(".extern mkdir\n");
    out.push_str(".extern socket\n");
    out.push_str(".extern connect\n");
    out.push_str(".extern send\n");
    out.push_str(".extern recv\n");
    out.push_str(".extern close\n");
    out.push_str(".extern closesocket\n");
    out.push_str(".extern bind\n");
    out.push_str(".extern listen\n");
    out.push_str(".extern accept\n");
    out.push_str(".extern htons\n");
    out.push_str(".extern inet_addr\n");
    out.push_str(".extern gethostbyname\n");
    out.push_str(".extern setsockopt\n");
    out.push_str(".extern getpeername\n");
    out.push_str(".extern inet_ntoa\n");
    out.push_str(".extern sendto\n");
    out.push_str(".extern recvfrom\n\n");
    out.push_str(".text\n");
    out.push_str("main:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    movl 8(%ebp), %eax\n");
    out.push_str("    movl %eax, alya_argc\n");
    out.push_str("    movl 12(%ebp), %eax\n");
    out.push_str("    movl %eax, alya_argv\n\n");
}

pub fn emit_footer(out: &mut String) {
    out.push_str("    xor %eax, %eax\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n");
}
