mod common;
use common::*;

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


