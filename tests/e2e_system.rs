mod common;
use common::*;

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
fn test_e2e_cli_args() {
    let code = r#"
let a = args()
say a.length()
for arg in a
    say arg
end
if a.length() > 0
    say "first: {a[0]}"
end
"#;
    // Test with CLI arguments
    if let Some((code, output)) = run_alya_code_with_args(code, &["hello", "alya", "42"]) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!("3\n", "hello\n", "alya\n", "42\n", "first: hello\n",)
        );
    }

    // Test with no CLI arguments
    if let Some((code, output)) = run_alya_code_with_args(code, &[]) {
        assert_eq!(code, 0);
        assert_eq!(output, "0\n");
    }
}

#[test]
fn test_e2e_file_io() {
    let test_path = "target/test_file_io.txt";
    let _ = std::fs::remove_file(test_path);

    let code = r#"
let path = "target/test_file_io.txt"
say file_exists(path)

let ok = write_file(path, "Hello Alya!\nSelf-hosting is coming.")
say ok
say file_exists(path)

let content = read_file(path)
say content

let del_ok = delete_file(path)
say del_ok
say file_exists(path)

let del_missing = remove_file("target/non_existent_12345.txt")
say del_missing

let missing = read_file("target/non_existent_12345.txt")
say "missing: [{missing}]"
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!(
                "0\n",
                "1\n",
                "1\n",
                "Hello Alya!\nSelf-hosting is coming.\n",
                "1\n",
                "0\n",
                "0\n",
                "missing: []\n",
            )
        );
    }

    let _ = std::fs::remove_file(test_path);
}

#[test]
fn test_e2e_stdlib_modules() {
    let code = r#"
import "std/math"
import "std/time"
import "std/os"
import "std/json"

say clamp(50, 0, 10)
say hypot(3, 4)
say is_even(10)
say is_odd(7)

let t = now()
if t > 0
    say 1
else
    say 0
end
delay(5)

say json_number(42)
say json_string("alya")
say json_bool(1)
say json_bool(0)
let items = ["apple", "banana"]
say json_array(items)
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!(
                "10\n",
                "5\n",
                "1\n",
                "1\n",
                "1\n",
                "42\n",
                "\"alya\"\n",
                "true\n",
                "false\n",
                "[apple, banana]\n",
            )
        );
    }
}

#[test]
fn test_e2e_memory_and_arena() {
    let code = r#"
import "std/mem"

let ptr = alloc(16)
poke_byte(ptr, 0, 89) # 'Y'
poke_byte(ptr, 1, 0)
say str_from_ptr(ptr)
say peek_byte(ptr, 0)
poke_int(ptr, 8, 424242)
say peek_int(ptr, 8)
free(ptr)

let a = arena_new(256)
let m1 = arena_alloc_mem(a, 32)
poke_int(m1, 0, 777)
say peek_int(m1, 0)
say arena_total_allocated(a)
arena_clear(a)
say arena_total_allocated(a)
arena_free_all(a)
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!("Y\n", "89\n", "424242\n", "777\n", "32\n", "0\n",)
        );
    }
}

#[test]
fn test_e2e_time_and_test_stdlib() {
    let code = r#"
import "std/time"
import "std/test"

# 1. Time functions
say is_leap_year(2024)
say is_leap_year(2023)
say is_leap_year(2000)
say is_leap_year(1900)

say days_in_month(1, 2024)
say days_in_month(2, 2024)
say days_in_month(2, 2023)
say days_in_month(4, 2024)

say format_duration(3665)
say minutes(5)
say hours(2)

# 2. Test runner & comparison assertions
let r = runner_new()
runner_assert(r, 10 > 5, "greater than")
assert_gt(20, 10, "20 gt 10")
assert_gte(10, 10, "10 gte 10")
assert_lt(5, 10, "5 lt 10")
assert_lte(5, 5, "5 lte 5")
assert_not_null("hello", "not null")
assert_null(0, "is null")

say "Runner total: " + str(r.total)
say "Runner passed: " + str(r.passed)
say "Runner failed: " + str(r.failed)
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!(
                "1\n",
                "0\n",
                "1\n",
                "0\n",
                "31\n",
                "29\n",
                "28\n",
                "30\n",
                "1h 1m 5s\n",
                "300\n",
                "7200\n",
                "  [PASS] greater than\n",
                "  [PASS] 20 gt 10\n",
                "  [PASS] 10 gte 10\n",
                "  [PASS] 5 lt 10\n",
                "  [PASS] 5 lte 5\n",
                "  [PASS] not null\n",
                "  [PASS] is null\n",
                "Runner total: 1\n",
                "Runner passed: 1\n",
                "Runner failed: 0\n",
            )
        );
    }
}
