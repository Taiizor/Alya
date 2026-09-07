pub mod collections;
pub mod errors;
pub mod fs;
pub mod math;
pub mod mem;
pub mod strings;

use crate::codegen::target::OperatingSystem;

pub(crate) fn emit_adrp_add(out: &mut String, reg: &str, label: &str, os: OperatingSystem) {
    if matches!(os, OperatingSystem::MacOS) {
        out.push_str(&format!("    adrp {}, {}@PAGE\n", reg, label));
        out.push_str(&format!("    add {}, {}, {}@PAGEOFF\n", reg, reg, label));
    } else {
        out.push_str(&format!("    adrp {}, {}\n", reg, label));
        out.push_str(&format!("    add {}, {}, :lo12:{}\n", reg, reg, label));
    }
}

pub fn emit_arm64_runtime(out: &mut String, os: OperatingSystem) {
    strings::emit(out, os);
    math::emit(out, os);
    fs::emit(out, os);
    collections::emit(out, os);
    mem::emit(out, os);
    errors::emit(out, os);
}
