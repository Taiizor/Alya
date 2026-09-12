mod common;
use common::*;

#[test]
fn test_e2e_c_ffi_abs() {
    let code = r#"
extern "C"
    function abs(n: i32) -> i32
end

say abs(-42)
say abs(100)
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "42\n100\n");
    }
}

#[test]
fn test_e2e_c_ffi_puts() {
    let code = r#"
extern "C"
    function puts(s: str) -> i32
end

puts("Hello from C FFI")
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "Hello from C FFI\n");
    }
}

#[test]
fn test_e2e_c_ffi_strlen() {
    let code = r#"
extern "C"
    function strlen(s: str) -> i32
end

say strlen("Alya FFI")
say strlen("")
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "8\n0\n");
    }
}

#[test]
fn test_e2e_c_ffi_strcmp() {
    let code = r#"
extern "C"
    function strcmp(s1: str, s2: str) -> i32
end

say strcmp("abc", "abc")
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "0\n");
    }
}

#[test]
fn test_e2e_c_ffi_getenv() {
    let code = r#"
extern "C"
    function getenv(name: str) -> str
end

let p = getenv("PATH")
if len(p) > 0
    say "path_ok"
end
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "path_ok\n");
    }
}
