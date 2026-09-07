pub mod arm64;
pub mod data;
pub mod x64;
pub mod x86;

use super::target::{Architecture, OperatingSystem};

pub fn emit_runtime(out: &mut String, arch: Architecture, os: OperatingSystem) {
    data::emit_data_sections(out, arch, os);

    match arch {
        Architecture::ARM64 => arm64::emit_arm64_runtime(out, os),
        Architecture::X64 => x64::emit_x64_runtime(out, os),
        Architecture::X86 => x86::emit_x86_runtime(out),
    }
}
