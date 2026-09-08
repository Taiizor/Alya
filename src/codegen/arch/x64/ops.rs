use crate::ast::{BinaryOp, UnaryOp};

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

pub fn emit_bit_op(out: &mut String, op: &str) {
    out.push_str("    mov %rax, %rbx\n");
    out.push_str("    pop %rax\n");
    match op {
        "bit_and" => out.push_str("    and %rbx, %rax\n"),
        "bit_or" => out.push_str("    or %rbx, %rax\n"),
        "bit_xor" => out.push_str("    xor %rbx, %rax\n"),
        "bit_shl" => {
            out.push_str("    mov %rbx, %rcx\n");
            out.push_str("    shl %cl, %rax\n");
        }
        "bit_shr" => {
            out.push_str("    mov %rbx, %rcx\n");
            out.push_str("    shr %cl, %rax\n");
        }
        _ => {}
    }
}

pub fn emit_bit_not(out: &mut String) {
    out.push_str("    not %rax\n");
}
