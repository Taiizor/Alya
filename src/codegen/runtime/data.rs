use crate::codegen::target::Architecture;

pub fn emit_data_sections(out: &mut String, arch: Architecture) {
    out.push_str("\n.section .bss\n");
    out.push_str(".align 16\n");
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

    out.push_str("\n.section .rodata\n");
    out.push_str("alya_fmt_prompt:\n");
    out.push_str("    .string \"%s\"\n");
    out.push_str("alya_fmt_div_zero:\n");
    out.push_str("    .string \"Runtime error: division by zero\\n\"\n");
    out.push_str("alya_str_div_zero:\n");
    out.push_str("    .string \"division by zero\"\n");
    out.push_str(".text\n");
}
