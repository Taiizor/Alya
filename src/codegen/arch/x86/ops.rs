use crate::ast::{BinaryOp, UnaryOp};

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

pub fn emit_float_binary_op(out: &mut String, op: BinaryOp) {
    out.push_str("    movapd %xmm0, %xmm1\n");
    out.push_str("    movsd (%esp), %xmm0\n");
    out.push_str("    add $8, %esp\n");
    match op {
        BinaryOp::Add => out.push_str("    addsd %xmm1, %xmm0\n"),
        BinaryOp::Subtract => out.push_str("    subsd %xmm1, %xmm0\n"),
        BinaryOp::Multiply => out.push_str("    mulsd %xmm1, %xmm0\n"),
        BinaryOp::Divide => out.push_str("    divsd %xmm1, %xmm0\n"),
        BinaryOp::Modulo => {
            out.push_str("    movapd %xmm0, %xmm2\n");
            out.push_str("    divsd %xmm1, %xmm2\n");
            out.push_str("    cvttsd2si %xmm2, %ecx\n");
            out.push_str("    cvtsi2sd %ecx, %xmm2\n");
            out.push_str("    mulsd %xmm1, %xmm2\n");
            out.push_str("    subsd %xmm2, %xmm0\n");
        }
        BinaryOp::Equal => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    sete %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::NotEqual => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    setne %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::Less => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    setb %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::Greater => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    seta %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    setbe %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    ucomisd %xmm1, %xmm0\n");
            out.push_str("    setae %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
        BinaryOp::And => out.push_str("    and %ebx, %eax\n"),
        BinaryOp::Or => out.push_str("    or %ebx, %eax\n"),
    }
}

pub fn emit_float_unary_op(out: &mut String, op: UnaryOp) {
    match op {
        UnaryOp::Negate => {
            out.push_str("    mov $-1, %eax\n");
            out.push_str("    cvtsi2sd %eax, %xmm1\n");
            out.push_str("    mulsd %xmm1, %xmm0\n");
        }
        UnaryOp::Not => {
            out.push_str("    test %eax, %eax\n");
            out.push_str("    sete %al\n");
            out.push_str("    movzbl %al, %eax\n");
        }
    }
}

pub fn emit_bit_op(out: &mut String, op: &str) {
    out.push_str("    mov %eax, %ebx\n");
    out.push_str("    pop %eax\n");
    match op {
        "bit_and" => out.push_str("    and %ebx, %eax\n"),
        "bit_or" => out.push_str("    or %ebx, %eax\n"),
        "bit_xor" => out.push_str("    xor %ebx, %eax\n"),
        "bit_shl" => {
            out.push_str("    mov %ebx, %ecx\n");
            out.push_str("    shl %cl, %eax\n");
        }
        "bit_shr" => {
            out.push_str("    mov %ebx, %ecx\n");
            out.push_str("    shr %cl, %eax\n");
        }
        _ => {}
    }
}

pub fn emit_bit_not(out: &mut String) {
    out.push_str("    not %eax\n");
}
