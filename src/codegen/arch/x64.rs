use crate::ast::{BinaryOp, UnaryOp};
use crate::codegen::target::OperatingSystem;

pub fn emit_header(out: &mut String) {
    out.push_str(".global main\n");
    out.push_str(".extern printf\n\n");
    out.push_str(".text\n");
    out.push_str("main:\n");
    out.push_str("    push %rbp\n");
    out.push_str("    mov %rsp, %rbp\n\n");
}

pub fn emit_footer(out: &mut String) {
    out.push_str("    xor %rax, %rax\n");
    out.push_str("    mov %rbp, %rsp\n");
    out.push_str("    pop %rbp\n");
    out.push_str("    ret\n");
}

pub fn emit_call_printf(out: &mut String, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call printf\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str("    call printf\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_load_num(out: &mut String, val: i64) {
    out.push_str(&format!("    mov ${}, %rax\n", val));
}

pub fn emit_load_str_label(out: &mut String, label: &str) {
    out.push_str(&format!("    lea {}(%rip), %rax\n", label));
}

pub fn emit_load_var(out: &mut String, offset: i32) {
    out.push_str(&format!("    mov -{}(%rbp), %rax\n", offset));
}

pub fn emit_store_var(out: &mut String, offset: i32) {
    out.push_str(&format!("    mov %rax, -{}(%rbp)\n", offset));
}

pub fn emit_allocate_var(out: &mut String, stack_offset: &mut i32) {
    *stack_offset += 8;
    out.push_str("    push %rax\n");
}

pub fn emit_push_temp(out: &mut String) {
    out.push_str("    push %rax\n");
}

pub fn emit_binary_op(out: &mut String, op: BinaryOp) {
    out.push_str("    mov %rax, %rbx\n");
    out.push_str("    pop %rax\n");
    match op {
        BinaryOp::Add => out.push_str("    add %rbx, %rax\n"),
        BinaryOp::Subtract => out.push_str("    sub %rbx, %rax\n"),
        BinaryOp::Multiply => out.push_str("    imul %rbx, %rax\n"),
        BinaryOp::Divide => {
            out.push_str("    cqo\n");
            out.push_str("    idiv %rbx\n");
        }
        BinaryOp::Modulo => {
            out.push_str("    cqo\n");
            out.push_str("    idiv %rbx\n");
            out.push_str("    mov %rdx, %rax\n");
        }
        BinaryOp::Equal => {
            out.push_str("    cmp %rbx, %rax\n");
            out.push_str("    sete %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::NotEqual => {
            out.push_str("    cmp %rbx, %rax\n");
            out.push_str("    setne %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::Less => {
            out.push_str("    cmp %rbx, %rax\n");
            out.push_str("    setl %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::Greater => {
            out.push_str("    cmp %rbx, %rax\n");
            out.push_str("    setg %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    cmp %rbx, %rax\n");
            out.push_str("    setle %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    cmp %rbx, %rax\n");
            out.push_str("    setge %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::And => out.push_str("    and %rbx, %rax\n"),
        BinaryOp::Or => out.push_str("    or %rbx, %rax\n"),
    }
}

pub fn emit_unary_op(out: &mut String, op: UnaryOp) {
    match op {
        UnaryOp::Negate => out.push_str("    neg %rax\n"),
        UnaryOp::Not => {
            out.push_str("    test %rax, %rax\n");
            out.push_str("    sete %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
    }
}

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

pub fn emit_function_param_push(out: &mut String, param_idx: usize, stack_offset: &mut i32, os: OperatingSystem) {
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

pub fn emit_function_call(out: &mut String, name: &str, args_count: usize, stack_offset: i32, os: OperatingSystem) {
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

pub fn emit_say_str(out: &mut String, label: &str, fmt_label: &str, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        out.push_str(&format!("    lea {}(%rip), %rcx\n", fmt_label));
        out.push_str(&format!("    lea {}(%rip), %rdx\n", label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    } else {
        out.push_str(&format!("    lea {}(%rip), %rsi\n", label));
        out.push_str(&format!("    lea {}(%rip), %rdi\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    }
}

pub fn emit_say_str_lit(out: &mut String, label: &str, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        out.push_str(&format!("    lea {}(%rip), %rcx\n", label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    } else {
        out.push_str(&format!("    lea {}(%rip), %rdi\n", label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    }
}

pub fn emit_say_offset(out: &mut String, offset: i32, fmt_label: &str, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        out.push_str(&format!("    mov -{}(%rbp), %rdx\n", offset));
        out.push_str(&format!("    lea {}(%rip), %rcx\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    } else {
        out.push_str(&format!("    mov -{}(%rbp), %rsi\n", offset));
        out.push_str(&format!("    lea {}(%rip), %rdi\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    }
}

pub fn emit_say_num_const(out: &mut String, val: i64, fmt_label: &str, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        out.push_str(&format!("    lea {}(%rip), %rcx\n", fmt_label));
        out.push_str(&format!("    mov ${}, %rdx\n", val));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    } else {
        out.push_str(&format!("    mov ${}, %rsi\n", val));
        out.push_str(&format!("    lea {}(%rip), %rdi\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    }
}

pub fn emit_say_acc(out: &mut String, fmt_label: &str, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rax, %rdx\n");
        out.push_str(&format!("    lea {}(%rip), %rcx\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    } else {
        out.push_str("    mov %rax, %rsi\n");
        out.push_str(&format!("    lea {}(%rip), %rdi\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    }
}

pub fn emit_say_interpolated_pop_and_call(out: &mut String, fmt_label: &str, count: usize, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        for i in (0..count).rev() {
            let reg = match i {
                0 => "%rdx",
                1 => "%r8",
                2 => "%r9",
                _ => "%rdx",
            };
            out.push_str(&format!("    pop {}\n", reg));
        }
        out.push_str(&format!("    lea {}(%rip), %rcx\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    } else {
        for i in (0..count).rev() {
            let reg = match i {
                0 => "%rsi",
                1 => "%rdx",
                2 => "%rcx",
                3 => "%r8",
                4 => "%r9",
                _ => "%rsi",
            };
            out.push_str(&format!("    pop {}\n", reg));
        }
        out.push_str(&format!("    lea {}(%rip), %rdi\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    }
}

pub fn emit_string_concat_call(out: &mut String, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rax, %rdx\n");
        out.push_str("    pop %rcx\n");
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call alya_concat\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        out.push_str("    mov %rax, %rsi\n");
        out.push_str("    pop %rdi\n");
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str("    call alya_concat\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}
