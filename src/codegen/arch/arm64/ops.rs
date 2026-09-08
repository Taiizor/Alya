use crate::ast::{BinaryOp, UnaryOp};

pub fn emit_binary_op(out: &mut String, op: BinaryOp) {
    out.push_str("    ldr x1, [sp], #16\n");
    match op {
        BinaryOp::Add => out.push_str("    add x0, x1, x0\n"),
        BinaryOp::Subtract => out.push_str("    sub x0, x1, x0\n"),
        BinaryOp::Multiply => out.push_str("    mul x0, x1, x0\n"),
        BinaryOp::Divide => {
            out.push_str("    cbz x0, alya_error_div_zero\n");
            out.push_str("    sdiv x0, x1, x0\n");
        }
        BinaryOp::Modulo => {
            out.push_str("    cbz x0, alya_error_div_zero\n");
            out.push_str("    sdiv x2, x1, x0\n");
            out.push_str("    msub x0, x2, x0, x1\n");
        }
        BinaryOp::Equal => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, eq\n");
        }
        BinaryOp::NotEqual => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, ne\n");
        }
        BinaryOp::Less => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, lt\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, le\n");
        }
        BinaryOp::Greater => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, gt\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    cmp x1, x0\n");
            out.push_str("    cset x0, ge\n");
        }
        BinaryOp::And => {
            out.push_str("    and x0, x1, x0\n");
        }
        BinaryOp::Or => {
            out.push_str("    orr x0, x1, x0\n");
        }
    }
}

pub fn emit_unary_op(out: &mut String, op: UnaryOp) {
    match op {
        UnaryOp::Negate => out.push_str("    neg x0, x0\n"),
        UnaryOp::Not => {
            out.push_str("    cmp x0, #0\n");
            out.push_str("    cset x0, eq\n");
        }
    }
}

pub fn emit_float_binary_op(out: &mut String, op: BinaryOp) {
    out.push_str("    fmov d1, x0\n");
    out.push_str("    ldr x1, [sp], #16\n");
    out.push_str("    fmov d0, x1\n");
    match op {
        BinaryOp::Add => {
            out.push_str("    fadd d0, d0, d1\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Subtract => {
            out.push_str("    fsub d0, d0, d1\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Multiply => {
            out.push_str("    fmul d0, d0, d1\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Divide => {
            out.push_str("    fdiv d0, d0, d1\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Modulo => {
            out.push_str("    fdiv d2, d0, d1\n");
            out.push_str("    fcvtzs x2, d2\n");
            out.push_str("    scvtf d2, x2\n");
            out.push_str("    fmul d2, d2, d1\n");
            out.push_str("    fsub d0, d0, d2\n");
            out.push_str("    fmov x0, d0\n");
        }
        BinaryOp::Equal => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, eq\n");
        }
        BinaryOp::NotEqual => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, ne\n");
        }
        BinaryOp::Less => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, mi\n");
        }
        BinaryOp::Greater => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, gt\n");
        }
        BinaryOp::LessEqual => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, le\n");
        }
        BinaryOp::GreaterEqual => {
            out.push_str("    fcmp d0, d1\n");
            out.push_str("    cset x0, ge\n");
        }
        BinaryOp::And => {
            out.push_str("    and x0, x1, x0\n");
        }
        BinaryOp::Or => {
            out.push_str("    orr x0, x1, x0\n");
        }
    }
}

pub fn emit_float_unary_op(out: &mut String, op: UnaryOp) {
    match op {
        UnaryOp::Negate => {
            out.push_str("    fmov d0, x0\n");
            out.push_str("    fneg d0, d0\n");
            out.push_str("    fmov x0, d0\n");
        }
        UnaryOp::Not => {
            out.push_str("    cmp x0, #0\n");
            out.push_str("    cset x0, eq\n");
        }
    }
}

pub fn emit_bit_op(out: &mut String, op: &str) {
    out.push_str("    ldr x1, [sp], #16\n");
    match op {
        "bit_and" => out.push_str("    and x0, x1, x0\n"),
        "bit_or" => out.push_str("    orr x0, x1, x0\n"),
        "bit_xor" => out.push_str("    eor x0, x1, x0\n"),
        "bit_shl" => out.push_str("    lsl x0, x1, x0\n"),
        "bit_shr" => out.push_str("    lsr x0, x1, x0\n"),
        _ => {}
    }
}

pub fn emit_bit_not(out: &mut String) {
    out.push_str("    mvn x0, x0\n");
}

