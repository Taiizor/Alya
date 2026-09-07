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
    let mut ast = parser.parse().expect("Parser error");
    let _ = alya::parser::resolve_imports(&mut ast, std::path::Path::new("."));

    let os = if cfg!(target_os = "windows") {
        OperatingSystem::Windows
    } else if cfg!(target_os = "macos") {
        OperatingSystem::MacOS
    } else {
        OperatingSystem::Linux
    };

    let arch = if cfg!(target_arch = "aarch64") {
        Architecture::ARM64
    } else if cfg!(target_arch = "x86") {
        Architecture::X86
    } else {
        Architecture::X64
    };

    let asm_code = codegen::generate(&ast, arch, os);

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
    if matches!(arch, Architecture::X86) {
        gcc.arg("-m32");
    }
    if matches!(os, OperatingSystem::Linux) {
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

#[test]
fn test_e2e_repeat_loop() {
    let code = r#"
let loops = 0
repeat
    loops += 1
    if loops >= 3
        break
    end
end
say loops
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(output, "3\n");
    }
}

#[test]
fn test_e2e_sqrt_and_pow() {
    let code = r#"
say sqrt(16)
say sqrt(25)
say sqrt(1)
say sqrt(0)
say pow(2, 8)
say pow(5, 3)
say pow(10, 0)
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(output, "4\n5\n1\n0\n256\n125\n1\n");
    }
}

#[test]
fn test_e2e_arrays() {
    let code = r#"
let arr = [10, 20, 30]
say len(arr)
say arr[0]
say arr[1]
say arr[2]
arr[1] = 99
arr[0] += 5
say arr
let sum = 0
for i in 0..(len(arr) - 1)
    sum += arr[i]
end
say sum
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(output, "3\n10\n20\n30\n[15, 99, 30]\n144\n");
    }
}

#[test]
fn test_e2e_array_bounds_catch() {
    let code = r#"
let arr = [1, 2, 3]
try
    let x = arr[5]
    say x
catch err
    say "caught: " + err
end
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(output, "caught: index out of bounds\n");
    }
}

#[test]
fn test_e2e_module_import() {
    let pid = std::process::id();
    let mod_filename = format!("temp_imported_helper_{}.alya", pid);
    let mod_content = r#"
function compute_bonus(salary)
    return salary * 2
end
"#;
    fs::write(&mod_filename, mod_content).expect("Failed to write temporary module file");

    let main_code = format!(
        r#"
import "{}"
let base = 1000
let total = compute_bonus(base)
say total
"#,
        mod_filename
    );

    let res = run_alya_code_full(&main_code);
    let _ = fs::remove_file(&mod_filename);

    if let Some((code, output)) = res {
        assert_eq!(code, 0);
        assert_eq!(output, "2000\n");
    }
}

#[test]
fn test_e2e_floating_point() {
    let code = r#"
let pi = 3.14
let r = 2.0
let area = pi * r * r
say area

let a = 10.5
let b = 2.5
say a + b
say a - b
say a * b
say a / b

let c = 1.25
c += 0.75
say c

if a > b
    say "greater"
end

let int_val = 10
let flt_val = float(int_val)
say flt_val

let back_to_int = int(3.99)
say back_to_int

say "Interpolated: {pi}"
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            "12.56\n13\n8\n26.25\n4.2\n2\ngreater\n10\n3\nInterpolated: 3.14\n"
        );
    }
}

#[test]
fn test_e2e_structs() {
    let code = r#"
struct Point
    x
    y
end

let p = Point { x: 10, y: 20 }
say p.x
say p.y
say p

p.x = 99
p.y += 5
say p.x
say p.y

let p2 = Point(1, 2)
say p2.x
say p2.y

function translate(pt, dx, dy)
    pt.x += dx
    pt.y += dy
    return pt
end

let p3 = translate(p2, 10, 20)
say p3.x
say p3.y

say "Formatted point: ({p.x}, {p.y})"
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            "10\n20\nPoint { x: 10, y: 20 }\n99\n25\n1\n2\n11\n22\nFormatted point: (99, 25)\n"
        );
    }
}

#[test]
fn test_e2e_structs_advanced() {
    let code = r#"
struct Person
    name
    age
    score
end

let alice = Person { name: "Alice", age: 30, score: 95.5 }
say alice.name
say alice.age
say alice.score
say "Student: {alice.name}, Age: {alice.age}, Score: {alice.score}"

struct Vector3
    x
    y
    z
end

let v1 = Vector3(1.0, 2.0, 3.5)
let v2 = Vector3(0.5, 1.5, 0.5)
let dot = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
say "Dot product: {dot}"
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            "Alice\n30\n95.5\nStudent: Alice, Age: 30, Score: 95.5\nDot product: 5.25\n"
        );
    }
}

#[test]
fn test_e2e_break_and_continue() {
    let code = r#"
let sum = 0
for i in 1..10
    if i % 2 == 0
        continue
    end
    if i > 6
        break
    end
    sum += i
end
say sum

let w = 0
let w_sum = 0
while w < 10
    w += 1
    if w == 2
        continue
    end
    if w == 5
        break
    end
    w_sum += w
end
say w_sum
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        // for loop: 1 + 3 + 5 = 9
        // while loop: w=1 (sum=1), w=2 (continue), w=3 (sum=4), w=4 (sum=8), w=5 (break) -> 8
        assert_eq!(output, "9\n8\n");
    }
}

