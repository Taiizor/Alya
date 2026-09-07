use crate::codegen::target::{Architecture, OperatingSystem};

pub fn emit_data_sections(out: &mut String, arch: Architecture, os: OperatingSystem) {
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str("\n.section __DATA,__bss\n");
        out.push_str(".p2align 4\n");
    } else {
        out.push_str("\n.section .bss\n");
        out.push_str(".align 16\n");
    }

    out.push_str("alya_str_buf:\n");
    out.push_str("    .space 65536\n");
    match arch {
        Architecture::ARM64 | Architecture::X64 => {
            out.push_str("alya_str_idx:\n");
            out.push_str("    .quad 0\n");
            out.push_str("alya_catch_idx:\n");
            out.push_str("    .quad 0\n");
            out.push_str("alya_catch_stack_handler:\n");
            out.push_str("    .space 1024\n");
            out.push_str("alya_catch_stack_sp:\n");
            out.push_str("    .space 1024\n");
            out.push_str("alya_catch_stack_bp:\n");
            out.push_str("    .space 1024\n");
            out.push_str("alya_err_msg:\n");
            out.push_str("    .quad 0\n");
        }
        Architecture::X86 => {
            out.push_str("alya_str_idx:\n");
            out.push_str("    .long 0\n");
            out.push_str("alya_catch_idx:\n");
            out.push_str("    .long 0\n");
            out.push_str("alya_catch_stack_handler:\n");
            out.push_str("    .space 512\n");
            out.push_str("alya_catch_stack_sp:\n");
            out.push_str("    .space 512\n");
            out.push_str("alya_catch_stack_bp:\n");
            out.push_str("    .space 512\n");
            out.push_str("alya_err_msg:\n");
            out.push_str("    .long 0\n");
        }
    }

    if matches!(os, OperatingSystem::MacOS) {
        out.push_str("\n.section __TEXT,__cstring,cstring_literals\n");
        out.push_str("alya_fmt_prompt:\n");
        out.push_str("    .asciz \"%s\"\n");
        out.push_str("alya_fmt_div_zero:\n");
        out.push_str("    .asciz \"Runtime error: division by zero\\n\"\n");
        out.push_str("alya_str_div_zero:\n");
        out.push_str("    .asciz \"division by zero\"\n");
        out.push_str("alya_fmt_bounds:\n");
        out.push_str("    .asciz \"Runtime error: index out of bounds\\n\"\n");
        out.push_str("alya_str_bounds:\n");
        out.push_str("    .asciz \"index out of bounds\"\n");
        out.push_str("alya_fmt_arr_empty:\n");
        out.push_str("    .asciz \"[]\\n\"\n");
        out.push_str("alya_fmt_arr_open:\n");
        out.push_str("    .asciz \"[\"\n");
        out.push_str("alya_fmt_arr_close:\n");
        out.push_str("    .asciz \"]\\n\"\n");
        out.push_str("alya_fmt_arr_elem:\n");
        if matches!(arch, Architecture::X86) {
            out.push_str("    .asciz \"%d\"\n");
        } else {
            out.push_str("    .asciz \"%ld\"\n");
        }
        out.push_str("alya_fmt_arr_comma:\n");
        out.push_str("    .asciz \", \"\n");
    } else {
        out.push_str("\n.section .rodata\n");
        out.push_str("alya_fmt_prompt:\n");
        out.push_str("    .string \"%s\"\n");
        out.push_str("alya_fmt_div_zero:\n");
        out.push_str("    .string \"Runtime error: division by zero\\n\"\n");
        out.push_str("alya_str_div_zero:\n");
        out.push_str("    .string \"division by zero\"\n");
        out.push_str("alya_fmt_bounds:\n");
        out.push_str("    .string \"Runtime error: index out of bounds\\n\"\n");
        out.push_str("alya_str_bounds:\n");
        out.push_str("    .string \"index out of bounds\"\n");
        out.push_str("alya_fmt_arr_empty:\n");
        out.push_str("    .string \"[]\\n\"\n");
        out.push_str("alya_fmt_arr_open:\n");
        out.push_str("    .string \"[\"\n");
        out.push_str("alya_fmt_arr_close:\n");
        out.push_str("    .string \"]\\n\"\n");
        out.push_str("alya_fmt_arr_elem:\n");
        if matches!(arch, Architecture::X86) {
            out.push_str("    .string \"%d\"\n");
        } else {
            out.push_str("    .string \"%ld\"\n");
        }
        out.push_str("alya_fmt_arr_comma:\n");
        out.push_str("    .string \", \"\n");
    }
    out.push_str(".text\n");
}
