use super::{arm64, x64, x86};
use crate::ast::{BinaryOp, UnaryOp};
use crate::codegen::target::Architecture;

pub fn emit_binary_op(out: &mut String, arch: Architecture, op: BinaryOp) {
    match arch {
        Architecture::ARM64 => arm64::emit_binary_op(out, op),
        Architecture::X64 => x64::emit_binary_op(out, op),
        Architecture::X86 => x86::emit_binary_op(out, op),
    }
}

pub fn emit_unary_op(out: &mut String, arch: Architecture, op: UnaryOp) {
    match arch {
        Architecture::ARM64 => arm64::emit_unary_op(out, op),
        Architecture::X64 => x64::emit_unary_op(out, op),
        Architecture::X86 => x86::emit_unary_op(out, op),
    }
}

pub fn emit_float_binary_op(out: &mut String, arch: Architecture, op: BinaryOp) {
    match arch {
        Architecture::ARM64 => arm64::emit_float_binary_op(out, op),
        Architecture::X64 => x64::emit_float_binary_op(out, op),
        Architecture::X86 => x86::emit_float_binary_op(out, op),
    }
}

pub fn emit_float_unary_op(out: &mut String, arch: Architecture, op: UnaryOp) {
    match arch {
        Architecture::ARM64 => arm64::emit_float_unary_op(out, op),
        Architecture::X64 => x64::emit_float_unary_op(out, op),
        Architecture::X86 => x86::emit_float_unary_op(out, op),
    }
}

