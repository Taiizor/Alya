pub mod collections;
pub mod errors;
pub mod fs;
pub mod math;
pub mod mem;
pub mod strings;

use crate::codegen::target::OperatingSystem;

pub fn emit_x64_runtime(out: &mut String, os: OperatingSystem) {
    strings::emit(out, os);
    math::emit(out, os);
    fs::emit(out, os);
    collections::emit(out, os);
    mem::emit(out, os);
    errors::emit(out, os);
}
