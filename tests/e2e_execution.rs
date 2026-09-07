use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use alya::codegen::{self, Architecture, OperatingSystem};
use alya::lexer::Lexer;
use alya::parser::Parser;

static TEST_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

fn run_alya_code(source: &str) -> Option<String> {
    // Check if gcc is available
    if Command::new("gcc").arg("--version").output().is_err() {
        eprintln!("Skipping E2E test: GCC is not available in PATH.");
        return None;
    }

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexer error");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parser error");

    let os = if cfg!(target_os = "windows") {
        OperatingSystem::Windows
    } else {
        OperatingSystem::Linux
    };

    let asm_code = codegen::generate(&ast, Architecture::X64, os);

    let id = TEST_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let asm_path = format!("temp_e2e_{}.s", id);
    let exe_path = if cfg!(target_os = "windows") {
        format!("temp_e2e_{}.exe", id)
    } else {
        format!("temp_e2e_{}", id)
    };

    fs::write(&asm_path, asm_code).expect("Failed to write temp asm file");

    let mut gcc = Command::new("gcc");
    gcc.arg(&asm_path).arg("-o").arg(&exe_path);
    if !matches!(os, OperatingSystem::Windows) {
        gcc.arg("-no-pie");
    }

    let gcc_out = gcc.output().expect("GCC invocation failed");
    let _ = fs::remove_file(&asm_path);

    if !gcc_out.status.success() {
        let _ = fs::remove_file(&exe_path);
        panic!("GCC compilation error:\n{}", String::from_utf8_lossy(&gcc_out.stderr));
    }

    let run_cmd = if cfg!(target_os = "windows") {
        format!(".\\{}", exe_path)
    } else {
        format!("./{}", exe_path)
    };

    let prog_out = Command::new(&run_cmd).output().expect("Program execution failed");
    let _ = fs::remove_file(&exe_path);

    let output_str = String::from_utf8_lossy(&prog_out.stdout)
        .replace("\r\n", "\n");

    Some(output_str)
}

#[test]
fn test_e2e_arithmetic() {
    let code = r#"
say 10 + 20
say 15 * 3
say 100 - 45
say 50 / 2
say 17 % 5
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "30\n45\n55\n25\n2\n");
    }
}

#[test]
fn test_e2e_variables_and_arithmetic() {
    let code = r#"
let a = 10
let b = 25
let sum = a + b
say sum
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "35\n");
    }
}

#[test]
fn test_e2e_conditionals() {
    let code = r#"
let x = 42
if x > 50
    say "gt"
else
    say "le"
end
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "le\n");
    }
}

#[test]
fn test_e2e_loops() {
    let code = r#"
for i in 1..4
    say i
end

let j = 10
while j < 13
    say j
    j = j + 1
end
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "1\n2\n3\n4\n10\n11\n12\n");
    }
}

#[test]
fn test_e2e_functions() {
    let code = r#"
function multiply(x, y)
    return x * y
end

say multiply(7, 6)
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "42\n");
    }
}

#[test]
fn test_e2e_string_interpolation() {
    let code = r#"
let name = "Alya"
say "Welcome to {name}!"
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "Welcome to Alya!\n");
    }
}
