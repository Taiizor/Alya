use crate::ast::{BinaryOp, UnaryOp};
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
        out.push_str(".extern _usleep\n\n");
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
            out.push_str(".extern Sleep\n\n");
        } else {
            out.push_str(".extern usleep\n\n");
        }
        out.push_str(".text\n");
        out.push_str("main:\n");
        out.push_str("    push %rbp\n");
        out.push_str("    mov %rsp, %rbp\n");
        if matches!(os, OperatingSystem::Windows) {
            out.push_str("    movq %rcx, alya_argc(%rip)\n");
            out.push_str("    movq %rdx, alya_argv(%rip)\n\n");
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

pub fn emit_call_printf(out: &mut String, stack_offset: i32, os: OperatingSystem) {
    let p = if matches!(os, OperatingSystem::MacOS) {
        "_"
    } else {
        ""
    };
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
        out.push_str(&format!("    call {}printf\n", p));
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_load_num(out: &mut String, val: i64) {
    out.push_str(&format!("    mov ${}, %rax\n", val));
}

pub fn emit_load_float(out: &mut String, val: f64) {
    let bits = val.to_bits() as i64;
    out.push_str(&format!("    mov ${}, %rax\n", bits));
    out.push_str("    movq %rax, %xmm0\n");
}

pub fn emit_int_to_float(out: &mut String) {
    out.push_str("    cvtsi2sdq %rax, %xmm0\n");
    out.push_str("    movq %xmm0, %rax\n");
}

pub fn emit_float_to_int(out: &mut String) {
    out.push_str("    movq %rax, %xmm0\n");
    out.push_str("    cvttsd2siq %xmm0, %rax\n");
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
            out.push_str("    test %rbx, %rbx\n");
            out.push_str("    jz alya_error_div_zero\n");
            out.push_str("    cqo\n");
            out.push_str("    idiv %rbx\n");
        }
        BinaryOp::Modulo => {
            out.push_str("    test %rbx, %rbx\n");
            out.push_str("    jz alya_error_div_zero\n");
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

pub fn emit_float_binary_op(out: &mut String, op: BinaryOp) {
    out.push_str("    movq %rax, %xmm1\n");
    out.push_str("    pop %rax\n");
    out.push_str("    movq %rax, %xmm0\n");
    match op {
        BinaryOp::Add => {
            out.push_str("    addsd %xmm1, %xmm0\n");
            out.push_str("    movq %xmm0, %rax\n");
        }
        BinaryOp::Subtract => {
            out.push_str("    subsd %xmm1, %xmm0\n");
            out.push_str("    movq %xmm0, %rax\n");
        }
        BinaryOp::Multiply => {
            out.push_str("    mulsd %xmm1, %xmm0\n");
            out.push_str("    movq %xmm0, %rax\n");
        }
        BinaryOp::Divide => {
            out.push_str("    divsd %xmm1, %xmm0\n");
            out.push_str("    movq %xmm0, %rax\n");
        }
        BinaryOp::Modulo => {
            out.push_str("    movapd %xmm0, %xmm2\n");
            out.push_str("    divsd %xmm1, %xmm2\n");
            out.push_str("    cvttsd2siq %xmm2, %rcx\n");
            out.push_str("    cvtsi2sdq %rcx, %xmm2\n");
            out.push_str("    mulsd %xmm1, %xmm2\n");
            out.push_str("    subsd %xmm2, %xmm0\n");
            out.push_str("    movq %xmm0, %rax\n");
        }
        BinaryOp::Equal => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    sete %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::NotEqual => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    setne %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::Less => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    setb %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::Greater => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    seta %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    setbe %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    setae %al\n");
            out.push_str("    movzbq %al, %rax\n");
        }
        BinaryOp::And => out.push_str("    and %rbx, %rax\n"),
        BinaryOp::Or => out.push_str("    or %rbx, %rax\n"),
    }
}

pub fn emit_float_unary_op(out: &mut String, op: UnaryOp) {
    match op {
        UnaryOp::Negate => {
            out.push_str("    mov $0x8000000000000000, %rcx\n");
            out.push_str("    xor %rcx, %rax\n");
            out.push_str("    movq %rax, %xmm0\n");
        }
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

pub fn emit_say_str(
    out: &mut String,
    label: &str,
    fmt_label: &str,
    stack_offset: i32,
    os: OperatingSystem,
) {
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

pub fn emit_say_offset(
    out: &mut String,
    offset: i32,
    fmt_label: &str,
    stack_offset: i32,
    os: OperatingSystem,
) {
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

pub fn emit_say_num_const(
    out: &mut String,
    val: i64,
    fmt_label: &str,
    stack_offset: i32,
    os: OperatingSystem,
) {
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

pub fn emit_say_float(out: &mut String, fmt_label: &str, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rax, %rdx\n");
        out.push_str("    movq %rax, %xmm1\n");
        out.push_str(&format!("    lea {}(%rip), %rcx\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    } else {
        out.push_str("    movq %rax, %xmm0\n");
        out.push_str(&format!("    lea {}(%rip), %rdi\n", fmt_label));
        out.push_str("    mov $1, %al\n");
        emit_call_printf(out, stack_offset, os);
    }
}

pub fn emit_say_interpolated_pop_and_call(
    out: &mut String,
    fmt_label: &str,
    is_floats: &[bool],
    stack_offset: i32,
    os: OperatingSystem,
) {
    let count = is_floats.len();
    if matches!(os, OperatingSystem::Windows) {
        for i in (0..count).rev() {
            let reg = match i {
                0 => "%rdx",
                1 => "%r8",
                2 => "%r9",
                _ => "%rdx",
            };
            out.push_str(&format!("    pop {}\n", reg));
            match i {
                0 => out.push_str("    movq %rdx, %xmm1\n"),
                1 => out.push_str("    movq %r8, %xmm2\n"),
                2 => out.push_str("    movq %r9, %xmm3\n"),
                _ => {}
            }
        }
        out.push_str(&format!("    lea {}(%rip), %rcx\n", fmt_label));
        out.push_str("    xor %rax, %rax\n");
        emit_call_printf(out, stack_offset, os);
    } else {
        let mut int_reg_indices = Vec::with_capacity(count);
        let mut sse_reg_indices = Vec::with_capacity(count);
        let mut int_count = 0;
        let mut sse_count = 0;
        for &is_flt in is_floats {
            if is_flt {
                int_reg_indices.push(None);
                sse_reg_indices.push(Some(sse_count));
                sse_count += 1;
            } else {
                int_reg_indices.push(Some(int_count));
                sse_reg_indices.push(None);
                int_count += 1;
            }
        }

        for i in (0..count).rev() {
            out.push_str("    pop %rax\n");
            if let Some(s_idx) = sse_reg_indices[i] {
                if s_idx < 8 {
                    out.push_str(&format!("    movq %rax, %xmm{}\n", s_idx));
                }
            } else if let Some(i_idx) = int_reg_indices[i] {
                let reg = match i_idx {
                    0 => "%rsi",
                    1 => "%rdx",
                    2 => "%rcx",
                    3 => "%r8",
                    4 => "%r9",
                    _ => "%rsi",
                };
                out.push_str(&format!("    mov %rax, {}\n", reg));
            }
        }
        out.push_str(&format!("    lea {}(%rip), %rdi\n", fmt_label));
        out.push_str(&format!("    mov ${}, %al\n", sse_count));
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

pub fn emit_try_begin(out: &mut String, catch_label: &str) {
    out.push_str("    mov alya_catch_idx(%rip), %r8\n");
    out.push_str(&format!("    lea {}(%rip), %rax\n", catch_label));
    out.push_str("    lea alya_catch_stack_handler(%rip), %r9\n");
    out.push_str("    mov %rax, (%r9, %r8, 8)\n");
    out.push_str("    lea alya_catch_stack_sp(%rip), %r9\n");
    out.push_str("    mov %rsp, (%r9, %r8, 8)\n");
    out.push_str("    lea alya_catch_stack_bp(%rip), %r9\n");
    out.push_str("    mov %rbp, (%r9, %r8, 8)\n");
    out.push_str("    incq alya_catch_idx(%rip)\n");
}

pub fn emit_try_end(out: &mut String, end_label: &str, stack_delta: i32) {
    out.push_str("    decq alya_catch_idx(%rip)\n");
    if stack_delta > 0 {
        out.push_str(&format!("    add ${}, %rsp\n", stack_delta));
    }
    out.push_str(&format!("    jmp {}\n", end_label));
}

pub fn emit_catch_begin(out: &mut String, catch_label: &str) {
    out.push_str(&format!("{}:\n", catch_label));
}

pub fn emit_catch_load_err(out: &mut String) {
    out.push_str("    mov alya_err_msg(%rip), %rax\n");
}

pub fn emit_catch_end(out: &mut String, stack_delta: i32) {
    if stack_delta > 0 {
        out.push_str(&format!("    add ${}, %rsp\n", stack_delta));
    }
}

pub fn emit_pop_temp(out: &mut String) {
    out.push_str("    pop %rax\n");
}

pub fn emit_array_new(out: &mut String, count: usize, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str(&format!("    mov ${}, %rcx\n", count));
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call alya_array_new\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str(&format!("    mov ${}, %rdi\n", count));
        out.push_str("    call alya_array_new\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_array_set_imm(out: &mut String, index: usize) {
    out.push_str("    mov (%rsp), %rdx\n");
    out.push_str("    mov 16(%rdx), %rdx\n");
    out.push_str(&format!("    movq %rax, {}(%rdx)\n", index * 8));
}

pub fn emit_array_get(out: &mut String) {
    out.push_str("    mov %rax, %rcx\n");
    out.push_str("    pop %rdx\n");
    out.push_str("    test %rcx, %rcx\n");
    out.push_str("    jl alya_error_index_out_of_bounds\n");
    out.push_str("    cmpq (%rdx), %rcx\n");
    out.push_str("    jge alya_error_index_out_of_bounds\n");
    out.push_str("    mov 16(%rdx), %rdx\n");
    out.push_str("    movq (%rdx, %rcx, 8), %rax\n");
}

pub fn emit_array_set(out: &mut String) {
    out.push_str("    mov %rax, %r8\n");
    out.push_str("    pop %rax\n");
    out.push_str("    pop %rdx\n");
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jl alya_error_index_out_of_bounds\n");
    out.push_str("    cmpq (%rdx), %rax\n");
    out.push_str("    jge alya_error_index_out_of_bounds\n");
    out.push_str("    mov 16(%rdx), %rdx\n");
    out.push_str("    movq %r8, (%rdx, %rax, 8)\n");
}

pub fn emit_array_push(out: &mut String, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str("    mov %rax, %rdx\n");
        out.push_str("    pop %rcx\n");
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call alya_array_push\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        let misaligned = stack_offset % 16 != 0;
        out.push_str("    mov %rax, %rsi\n");
        out.push_str("    pop %rdi\n");
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str("    call alya_array_push\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_array_pop(out: &mut String, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str("    mov %rax, %rcx\n");
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call alya_array_pop\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str("    mov %rax, %rdi\n");
        out.push_str("    call alya_array_pop\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_array_len(out: &mut String) {
    out.push_str("    test %rax, %rax\n");
    out.push_str("    jz 1f\n");
    out.push_str("    movq (%rax), %rax\n");
    out.push_str("1:\n");
}

pub fn emit_print_array(out: &mut String, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str("    mov %rax, %rcx\n");
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call alya_print_array\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str("    mov %rax, %rdi\n");
        out.push_str("    call alya_print_array\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_print_map(out: &mut String, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str("    mov %rax, %rcx\n");
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call alya_print_map\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str("    mov %rax, %rdi\n");
        out.push_str("    call alya_print_map\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_struct_new(
    out: &mut String,
    desc_label: &str,
    field_count: usize,
    stack_offset: i32,
    os: OperatingSystem,
) {
    if matches!(os, OperatingSystem::Windows) {
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str(&format!("    lea {}(%rip), %rcx\n", desc_label));
        out.push_str(&format!("    mov ${}, %rdx\n", field_count));
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call alya_struct_new\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str(&format!("    lea {}(%rip), %rdi\n", desc_label));
        out.push_str(&format!("    mov ${}, %rsi\n", field_count));
        out.push_str("    call alya_struct_new\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_struct_field_get(out: &mut String, field_idx: usize) {
    out.push_str(&format!("    movq {}(%rax), %rax\n", (field_idx + 1) * 8));
}

pub fn emit_struct_field_set_imm(out: &mut String, field_idx: usize) {
    out.push_str("    mov (%rsp), %rdx\n");
    out.push_str(&format!("    movq %rax, {}(%rdx)\n", (field_idx + 1) * 8));
}

pub fn emit_struct_field_set(out: &mut String, field_idx: usize) {
    out.push_str("    pop %rdx\n");
    out.push_str(&format!("    movq %rax, {}(%rdx)\n", (field_idx + 1) * 8));
}

pub fn emit_print_struct(out: &mut String, stack_offset: i32, os: OperatingSystem) {
    if matches!(os, OperatingSystem::Windows) {
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str("    mov %rax, %rcx\n");
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call alya_print_struct\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str("    mov %rax, %rdi\n");
        out.push_str("    call alya_print_struct\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
}

pub fn emit_for_each_load_element(
    out: &mut String,
    arr_offset: i32,
    idx_offset: i32,
    var_offset: i32,
    end_label: &str,
) {
    out.push_str(&format!("    movq -{}(%rbp), %rax\n", arr_offset));
    out.push_str("    test %rax, %rax\n");
    out.push_str(&format!("    jz {}\n", end_label));
    out.push_str("    movq (%rax), %rdx\n");
    out.push_str(&format!("    movq -{}(%rbp), %rcx\n", idx_offset));
    out.push_str("    cmpq %rdx, %rcx\n");
    out.push_str(&format!("    jge {}\n", end_label));
    out.push_str("    movq 16(%rax), %rdx\n");
    out.push_str("    movq (%rdx, %rcx, 8), %rax\n");
    out.push_str(&format!("    movq %rax, -{}(%rbp)\n", var_offset));
}

pub fn emit_string_equality_call(
    out: &mut String,
    op: BinaryOp,
    stack_offset: i32,
    os: OperatingSystem,
) {
    if matches!(os, OperatingSystem::Windows) {
        out.push_str("    mov %rax, %rdx\n");
        out.push_str("    pop %rcx\n");
        let padding = if stack_offset % 16 == 0 { 32 } else { 40 };
        out.push_str(&format!("    sub ${}, %rsp\n", padding));
        out.push_str("    call fn_streq\n");
        out.push_str(&format!("    add ${}, %rsp\n", padding));
    } else {
        out.push_str("    mov %rax, %rsi\n");
        out.push_str("    pop %rdi\n");
        let misaligned = stack_offset % 16 != 0;
        if misaligned {
            out.push_str("    sub $8, %rsp\n");
        }
        out.push_str("    call fn_streq\n");
        if misaligned {
            out.push_str("    add $8, %rsp\n");
        }
    }
    if matches!(op, BinaryOp::NotEqual) {
        out.push_str("    xor $1, %rax\n");
    }
}
