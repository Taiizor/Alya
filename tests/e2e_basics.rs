mod common;
use common::*;

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
