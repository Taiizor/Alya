use crate::codegen::{Architecture, OperatingSystem};
use std::fs;
use std::process::Command;

pub fn compile_with_gcc(
    asm_file: &str,
    exe_file: &str,
    arch: Architecture,
    os: OperatingSystem,
) -> Result<(), String> {
    let mut gcc_args = vec![asm_file, "-o", exe_file];

    if matches!(arch, Architecture::X86) {
        gcc_args.insert(0, "-m32");
    }

    if !matches!(os, OperatingSystem::Windows) {
        gcc_args.push("-no-pie");
    }

    let gcc_result = Command::new("gcc")
        .args(&gcc_args)
        .output();

    let _ = fs::remove_file(asm_file);

    match gcc_result {
        Ok(output) => {
            if !output.status.success() {
                Err(format!(
                    "GCC compilation failed:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(format!(
            "Error: Failed to run GCC: {}\nMake sure GCC is installed and in your PATH.",
            e
        )),
    }
}

pub fn execute_binary(exe_file: &str, delete_after: bool) -> Result<(), String> {
    let run_path = if cfg!(target_os = "windows") {
        format!(".\\{}", exe_file)
    } else {
        format!("./{}", exe_file)
    };

    let mut child = Command::new(&run_path)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .map_err(|e| format!("Error: Failed to execute '{}': {}", run_path, e))?;

    let status = child.wait().map_err(|e| format!("Execution error: {}", e))?;

    if delete_after {
        let _ = fs::remove_file(exe_file);
    }

    if !status.success() {
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }

    Ok(())
}
