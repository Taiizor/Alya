use crate::ast::{BinaryOp, UnaryOp};

pub fn emit_header(out: &mut String) {
    out.push_str(".global main\n");
    out.push_str(".extern printf\n");
    out.push_str(".extern exit\n");
    out.push_str(".extern getchar\n");
    out.push_str(".extern fflush\n");
    out.push_str(".extern calloc\n\n");
    out.push_str(".text\n");
    out.push_str("main:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n\n");
}

pub fn emit_footer(out: &mut String) {
    out.push_str("    xor %eax, %eax\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n");
}

pub fn emit_call_printf(out: &mut String) {
    out.push_str("    call printf\n");
}

pub fn emit_load_num(out: &mut String, val: i64) {
    out.push_str(&format!("    mov ${}, %eax\n", val));
}

pub fn emit_load_str_label(out: &mut String, label: &str) {
    out.push_str(&format!("    mov ${}, %eax\n", label));
}

pub fn emit_load_var(out: &mut String, offset: i32) {
    out.push_str(&format!("    mov -{}(%ebp), %eax\n", offset));
}

pub fn emit_store_var(out: &mut String, offset: i32) {
    out.push_str(&format!("    mov %eax, -{}(%ebp)\n", offset));
}

pub fn emit_allocate_var(out: &mut String, stack_offset: &mut i32) {
    *stack_offset += 4;
    out.push_str("    push %eax\n");
}

pub fn emit_push_temp(out: &mut String) {
    out.push_str("    push %eax\n");
}

pub fn emit_binary_op(out: &mut String, op: BinaryOp) {
    out.push_str("    mov %eax, %ebx\n");
    out.push_str("    pop %eax\n");
    match op {
        BinaryOp::Add => out.push_str("    add %ebx, %eax\n"),
        BinaryOp::Subtract => out.push_str("    sub %ebx, %eax\n"),
        BinaryOp::Multiply => out.push_str("    imul %ebx, %eax\n"),
        BinaryOp::Divide => {
            out.push_str("    test %ebx, %ebx\n");
            out.push_str("    jz alya_error_div_zero\n");
            out.push_str("    cdq\n");
            out.push_str("    idiv %ebx\n");
        }
        BinaryOp::Modulo => {
            out.push_str("    test %ebx, %ebx\n");
            out.push_str("    jz alya_error_div_zero\n");
            out.push_str("    cdq\n");
            out.push_str("    idiv %ebx\n");
            out.push_str("    mov %edx, %eax\n");
        }
        BinaryOp::Equal => {
            out.push_str("    cmp %ebx, %eax\n");
            out.push_str("    sete %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::NotEqual => {
            out.push_str("    cmp %ebx, %eax\n");
            out.push_str("    setne %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::Less => {
            out.push_str("    cmp %ebx, %eax\n");
            out.push_str("    setl %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::Greater => {
            out.push_str("    cmp %ebx, %eax\n");
            out.push_str("    setg %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    cmp %ebx, %eax\n");
            out.push_str("    setle %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    cmp %ebx, %eax\n");
            out.push_str("    setge %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::And => out.push_str("    and %ebx, %eax\n"),
        BinaryOp::Or => out.push_str("    or %ebx, %eax\n"),
    }
}

pub fn emit_unary_op(out: &mut String, op: UnaryOp) {
    match op {
        UnaryOp::Negate => out.push_str("    neg %eax\n"),
        UnaryOp::Not => {
            out.push_str("    test %eax, %eax\n");
            out.push_str("    sete %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
    }
}

pub fn emit_jump_if_zero(out: &mut String, label: &str) {
    out.push_str("    test %eax, %eax\n");
    out.push_str(&format!("    jz {}\n", label));
}

pub fn emit_jump(out: &mut String, label: &str) {
    out.push_str(&format!("    jmp {}\n", label));
}

pub fn emit_compare_and_jump_if_greater(out: &mut String, label: &str) {
    out.push_str("    mov %eax, %ebx\n");
    out.push_str("    pop %eax\n");
    out.push_str("    cmp %ebx, %eax\n");
    out.push_str(&format!("    jg {}\n", label));
}

pub fn emit_increment_var(out: &mut String, var_offset: i32, start_label: &str) {
    out.push_str(&format!("    addl $1, -{}(%ebp)\n", var_offset));
    out.push_str(&format!("    jmp {}\n", start_label));
}

pub fn emit_function_prologue(out: &mut String, name: &str) {
    out.push_str(&format!("\n.global fn_{}\n", name));
    out.push_str(&format!("fn_{}:\n", name));
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
}

pub fn emit_function_epilogue(out: &mut String) {
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n");
}

pub fn emit_function_param_push(out: &mut String, param_idx: usize, stack_offset: &mut i32) {
    *stack_offset += 4;
    let src_offset = 8 + param_idx * 4;
    out.push_str(&format!("    push {}(%ebp)\n", src_offset));
}

pub fn emit_function_call(out: &mut String, name: &str, args_count: usize) {
    out.push_str(&format!("    call fn_{}\n", name));
    if args_count > 0 {
        out.push_str(&format!("    add ${}, %esp\n", args_count * 4));
    }
}

pub fn emit_say_str(out: &mut String, label: &str, fmt_label: &str) {
    out.push_str(&format!("    push ${}\n", label));
    out.push_str(&format!("    push ${}\n", fmt_label));
    emit_call_printf(out);
    out.push_str("    add $8, %esp\n");
}

pub fn emit_say_str_lit(out: &mut String, label: &str) {
    out.push_str(&format!("    push ${}\n", label));
    emit_call_printf(out);
    out.push_str("    add $4, %esp\n");
}

pub fn emit_say_offset(out: &mut String, offset: i32, fmt_label: &str) {
    out.push_str(&format!("    push -{}(%ebp)\n", offset));
    out.push_str(&format!("    push ${}\n", fmt_label));
    emit_call_printf(out);
    out.push_str("    add $8, %esp\n");
}

pub fn emit_say_num_const(out: &mut String, val: i64, fmt_label: &str) {
    out.push_str(&format!("    push ${}\n", val));
    out.push_str(&format!("    push ${}\n", fmt_label));
    emit_call_printf(out);
    out.push_str("    add $8, %esp\n");
}

pub fn emit_say_acc(out: &mut String, fmt_label: &str) {
    out.push_str("    push %eax\n");
    out.push_str(&format!("    push ${}\n", fmt_label));
    emit_call_printf(out);
    out.push_str("    add $8, %esp\n");
}

pub fn emit_say_interpolated_call(out: &mut String, fmt_label: &str, count: usize) {
    out.push_str(&format!("    push ${}\n", fmt_label));
    emit_call_printf(out);
    out.push_str(&format!("    add ${}, %esp\n", (count + 1) * 4));
}

pub fn emit_string_concat_call(out: &mut String) {
    out.push_str("    mov %eax, %edx\n");
    out.push_str("    pop %eax\n");
    out.push_str("    push %edx\n");
    out.push_str("    push %eax\n");
    out.push_str("    call alya_concat\n");
    out.push_str("    add $8, %esp\n");
}

pub fn emit_try_begin(out: &mut String, catch_label: &str) {
    out.push_str("    mov alya_catch_idx, %ecx\n");
    out.push_str(&format!("    mov ${}, %eax\n", catch_label));
    out.push_str("    mov $alya_catch_stack_handler, %edx\n");
    out.push_str("    mov %eax, (%edx, %ecx, 4)\n");
    out.push_str("    mov $alya_catch_stack_sp, %edx\n");
    out.push_str("    mov %esp, (%edx, %ecx, 4)\n");
    out.push_str("    mov $alya_catch_stack_bp, %edx\n");
    out.push_str("    mov %ebp, (%edx, %ecx, 4)\n");
    out.push_str("    incl alya_catch_idx\n");
}

pub fn emit_try_end(out: &mut String, end_label: &str, stack_delta: i32) {
    out.push_str("    decl alya_catch_idx\n");
    if stack_delta > 0 {
        out.push_str(&format!("    add ${}, %esp\n", stack_delta));
    }
    out.push_str(&format!("    jmp {}\n", end_label));
}

pub fn emit_catch_begin(out: &mut String, catch_label: &str) {
    out.push_str(&format!("{}:\n", catch_label));
}

pub fn emit_catch_load_err(out: &mut String) {
    out.push_str("    mov alya_err_msg, %eax\n");
}

pub fn emit_catch_end(out: &mut String, stack_delta: i32) {
    if stack_delta > 0 {
        out.push_str(&format!("    add ${}, %esp\n", stack_delta));
    }
}

pub fn emit_pop_temp(out: &mut String) {
    out.push_str("    pop %eax\n");
}

pub fn emit_array_new(out: &mut String, count: usize) {
    out.push_str(&format!("    push ${}\n", count));
    out.push_str("    call alya_array_new\n");
    out.push_str("    add $4, %esp\n");
}

pub fn emit_array_set_imm(out: &mut String, index: usize) {
    out.push_str("    mov (%esp), %edx\n");
    out.push_str(&format!("    mov %eax, {}(%edx)\n", (index + 1) * 4));
}

pub fn emit_array_get(out: &mut String) {
    out.push_str("    mov %eax, %ecx\n");
    out.push_str("    pop %edx\n");
    out.push_str("    test %ecx, %ecx\n");
    out.push_str("    jl alya_error_index_out_of_bounds\n");
    out.push_str("    cmp (%edx), %ecx\n");
    out.push_str("    jge alya_error_index_out_of_bounds\n");
    out.push_str("    mov 4(%edx, %ecx, 4), %eax\n");
}

pub fn emit_array_set(out: &mut String) {
    out.push_str("    mov %eax, %ebx\n");
    out.push_str("    pop %eax\n");
    out.push_str("    pop %edx\n");
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jl alya_error_index_out_of_bounds\n");
    out.push_str("    cmp (%edx), %eax\n");
    out.push_str("    jge alya_error_index_out_of_bounds\n");
    out.push_str("    mov %ebx, 4(%edx, %eax, 4)\n");
}

pub fn emit_array_len(out: &mut String) {
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jz 1f\n");
    out.push_str("    mov (%eax), %eax\n");
    out.push_str("1:\n");
}

pub fn emit_print_array(out: &mut String) {
    out.push_str("    push %eax\n");
    out.push_str("    call alya_print_array\n");
    out.push_str("    add $4, %esp\n");
}
