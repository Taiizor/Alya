use alya::codegen::{self, Architecture, OperatingSystem};
use alya::lexer::Lexer;
use alya::parser::Parser;
use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

fn run_alya_code_with_input(source: &str, input: Option<&str>) -> Option<(i32, String)> {
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

    let pid = std::process::id();
    let id = TEST_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let asm_path = format!("temp_e2e_{}_{}.s", pid, id);
    let exe_path = if cfg!(target_os = "windows") {
        format!("temp_e2e_{}_{}.exe", pid, id)
    } else {
        format!("temp_e2e_{}_{}", pid, id)
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
        panic!(
            "GCC compilation error:\n{}",
            String::from_utf8_lossy(&gcc_out.stderr)
        );
    }

    let run_cmd = if cfg!(target_os = "windows") {
        format!(".\\{}", exe_path)
    } else {
        format!("./{}", exe_path)
    };

    let mut child = Command::new(&run_cmd)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Program execution failed");

    if let Some(in_str) = input {
        use std::io::Write;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(in_str.as_bytes());
        }
    }

    let prog_out = child
        .wait_with_output()
        .expect("Failed to wait on child process");
    let _ = fs::remove_file(&exe_path);

    let code = prog_out.status.code().unwrap_or(-1);
    let mut output = String::from_utf8_lossy(&prog_out.stdout).replace("\r\n", "\n");
    let stderr = String::from_utf8_lossy(&prog_out.stderr).replace("\r\n", "\n");
    output.push_str(&stderr);

    Some((code, output))
}

fn run_alya_code_full(source: &str) -> Option<(i32, String)> {
    run_alya_code_with_input(source, None)
}

fn run_alya_code(source: &str) -> Option<String> {
    run_alya_code_full(source).map(|(_, out)| out)
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

#[test]
fn test_e2e_else_if_and_elif() {
    let code = r#"
let score = 85
if score >= 90
    say "A"
else if score >= 80
    say "B"
else
    say "C"
end

let val = 5
if val == 1
    say "one"
elif val == 5
    say "five"
else
    say "other"
end
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "B\nfive\n");
    }
}

#[test]
fn test_e2e_compound_assignment() {
    let code = r#"
let x = 10
x += 5
x -= 2
x *= 3
x /= 2
say x
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "19\n"); // ((10 + 5) - 2) * 3 / 2 = 39 / 2 = 19
    }
}

#[test]
fn test_e2e_slash_comments_and_logical_symbols() {
    let code = r#"
// This is a C-style comment
/* Multi-line
   comment */
let a = 10
let b = 20
if (a < 15) && (b == 20)
    say "and works"
end

if (a == 99) || (b > 10)
    say "or works"
end

if !(a == 99)
    say "not works"
end
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "and works\nor works\nnot works\n");
    }
}

#[test]
fn test_e2e_builtins() {
    let code = r#"
say len("Hello, Alya!")
say abs(-42)
say abs(42)
say min(15, 30)
say max(15, 30)
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "12\n42\n42\n15\n30\n");
    }
}

#[test]
fn test_e2e_div_by_zero_protection() {
    let code = r#"
let a = 50
let b = 0
say a / b
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_ne!(code, 0);
        assert!(output.contains("Runtime error: division by zero"));
    }
}

#[test]
fn test_e2e_modulo_by_zero_protection() {
    let code = r#"
say 100 % 0
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_ne!(code, 0);
        assert!(output.contains("Runtime error: division by zero"));
    }
}

#[test]
fn test_e2e_ask_input_automated() {
    let code = r#"
let name = ask "Name: "
let city = ask "City: "
say "Hello, {name} from {city}!"
"#;
    if let Some((code, output)) = run_alya_code_with_input(code, Some("Alya\nIstanbul\n")) {
        assert_eq!(code, 0);
        assert!(output.contains("Name: "));
        assert!(output.contains("City: "));
        assert!(output.contains("Hello, Alya from Istanbul!"));
    }
}

#[test]
fn test_e2e_try_catch_basic() {
    let code = r#"
say "Before try"
try
    say "Inside try before error"
    let x = 10 / 0
    say "Should not print"
catch
    say "Caught error successfully"
end
say "After try"
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            "Before try\nInside try before error\nCaught error successfully\nAfter try\n"
        );
    }
}

#[test]
fn test_e2e_try_catch_with_err_var() {
    let code = r#"
try
    let a = 100 % 0
catch err
    say "Caught: " + err
end
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(output, "Caught: division by zero\n");
    }
}

#[test]
fn test_e2e_try_catch_no_error() {
    let code = r#"
try
    let x = 10 / 2
    say x
catch
    say "Should not print"
end
say "Done"
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(output, "5\nDone\n");
    }
}
