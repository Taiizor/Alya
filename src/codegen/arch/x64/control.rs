use crate::codegen::target::OperatingSystem;

pub fn emit_jump_if_zero(out: &mut String, label: &str) {
    out.push_str("    test %rax, %rax\n");
    out.push_str(&format!("    jz {}\n", label));
}

pub fn emit_jump(out: &mut String, label: &str) {
    out.push_str(&format!("    jmp {}\n", label));
}

pub fn emit_compare_and_jump_if_greater(out: &mut String, label: &str) {
    out.push_str("    mov %rax, %rbx\n");
    out.push_str("    pop %rax\n");
    out.push_str("    cmp %rbx, %rax\n");
    out.push_str(&format!("    jg {}\n", label));
}

pub fn emit_increment_var(out: &mut String, var_offset: i32, start_label: &str) {
    out.push_str(&format!("    addq $1, -{}(%rbp)\n", var_offset));
    out.push_str(&format!("    jmp {}\n", start_label));
}

pub fn emit_function_prologue(out: &mut String, name: &str) {
    out.push_str(&format!("\n.global fn_{}\n", name));
    out.push_str(&format!("fn_{}:\n", name));
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n");
}

pub fn emit_function_epilogue(out: &mut String) {
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n");
}

pub fn emit_function_param_push(
    out: &mut String,
    param_idx: usize,
    stack_offset: &mut i32,
    os: OperatingSystem,
) {
    *stack_offset += 8;
    let reg = if matches!(os, OperatingSystem::Windows) {
        match param_idx {
            0 => "%rcx",
            1 => "%rdx",
            2 => "%r8",
            3 => "%r9",
            _ => "%rcx",
        }
    } else {
        match param_idx {
            0 => "%rdi",
            1 => "%rsi",
            2 => "%rdx",
            3 => "%rcx",
            4 => "%r8",
            5 => "%r9",
            _ => "%rdi",
        }
    };
    out.push_str(&format!("    push {}\n", reg));
}

pub fn emit_function_call(
    out: &mut String,
    name: &str,
    args_count: usize,
    stack_offset: i32,
    os: OperatingSystem,
) {
    if matches!(os, OperatingSystem::Windows) {
        for i in (0..args_count).rev() {
            let reg = match i {
                0 => "%rcx",
                1 => "%rdx",
                2 => "%r8",
                3 => "%r9",
                _ => "%rcx",
            };
            out.push_str(&format!("    pop {}\n", reg));
        }
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str(&format!("    call fn_{}\n", name));
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        for i in (0..args_count).rev() {
            let reg = match i {
                0 => "%rdi",
                1 => "%rsi",
                2 => "%rdx",
                3 => "%rcx",
                4 => "%r8",
                5 => "%r9",
                _ => "%rdi",
            };
            out.push_str(&format!("    pop {}\n", reg));
        }
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str(&format!("    call fn_{}\n", name));
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}
