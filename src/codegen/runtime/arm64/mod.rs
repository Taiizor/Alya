pub mod arena;
pub mod arrays;
pub mod errors;
pub mod fs;
pub mod heap;
pub mod io;
pub mod maps;
pub mod math;
pub mod net;
pub mod str_ops;
pub mod str_split;
pub mod structs;
pub mod thread;

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
    str_ops::emit(out, os);
    str_split::emit(out, os);
    io::emit(out, os);
    math::emit(out, os);
    fs::emit(out, os);
    arrays::emit(out, os);
    maps::emit(out, os);
    structs::emit(out, os);
    heap::emit(out, os);
    arena::emit(out, os);
    net::emit(out, os);
    thread::emit(out, os);
    errors::emit(out, os);
}
