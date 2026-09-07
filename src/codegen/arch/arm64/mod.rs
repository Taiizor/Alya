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

pub fn emit_adrp_add(out: &mut String, reg: &str, label: &str, os: OperatingSystem) {
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str(&format!("    adrp {}, {}@PAGE\n", reg, label));
        out.push_str(&format!("    add {}, {}, {}@PAGEOFF\n", reg, reg, label));
    } else {
        out.push_str(&format!("    adrp {}, {}\n", reg, label));
        out.push_str(&format!("    add {}, {}, :lo12:{}\n", reg, reg, label));
    }
}

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
        out.push_str(".align 2\n");
        out.push_str("_main:\n");
        out.push_str("    stp x29, x30, [sp, #-16]!\n");
        out.push_str("    mov x29, sp\n");
        emit_adrp_add(out, "x2", "alya_argc", os);
        out.push_str("    str x0, [x2]\n");
        emit_adrp_add(out, "x2", "alya_argv", os);
        out.push_str("    str x1, [x2]\n\n");
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
        out.push_str(".extern usleep\n");
        out.push_str(".extern mkdir\n\n");
        out.push_str(".text\n");
        out.push_str(".align 2\n");
        out.push_str("main:\n");
        out.push_str("    stp x29, x30, [sp, #-16]!\n");
        out.push_str("    mov x29, sp\n");
        emit_adrp_add(out, "x2", "alya_argc", os);
        out.push_str("    str x0, [x2]\n");
        emit_adrp_add(out, "x2", "alya_argv", os);
        out.push_str("    str x1, [x2]\n\n");
    }
}

pub fn emit_footer(out: &mut String) {
    out.push_str("    mov w0, #0\n");
    out.push_str("    mov sp, x29\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n");
}
