mod common;
use common::*;

#[test]
fn test_e2e_string_interpolation() {
    let code = r#"
function calc(a, b)
    return a * b + 2
end

let name = "Alya"
say "Welcome to {name}!"
say "Call: {calc(3, 4)}"
say "Math: {10 + 5 * 2}"
let greeting = "   hello   "
say "Method: {greeting.trim().upper()}"
say "Escaped: {{bracket}}"
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(
            output,
            "Welcome to Alya!\nCall: 14\nMath: 20\nMethod: HELLO\nEscaped: {bracket}\n"
        );
    }
}

#[test]
fn test_e2e_string_helpers() {
    let code = r#"
let s = "  Hello, World!  "

// trim
let trimmed = s.trim()
say trimmed
say trim("   spaced out   ")

// upper and lower
let up = trimmed.upper()
say up
let low = trimmed.lower()
say low
say upper("alya language")
say lower("ALYA COMPILER")

// contains
say trimmed.contains("World")
say trimmed.contains("xyz")
say contains("abcdef", "cd")
say contains("abcdef", "gh")
say trimmed.contains("")

// substring / substr (3 args and 2 args)
let sub1 = trimmed.substring(0, 5)
say sub1
let sub2 = substr(trimmed, 7, 5)
say sub2
let sub3 = trimmed.substring(7)
say sub3
let sub4 = substr(trimmed, 7)
say sub4

// chaining and concatenation
let combo = s.trim().upper()
say combo + " - SUCCESS"
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!(
                "Hello, World!\n",
                "spaced out\n",
                "HELLO, WORLD!\n",
                "hello, world!\n",
                "ALYA LANGUAGE\n",
                "alya compiler\n",
                "1\n",
                "0\n",
                "1\n",
                "0\n",
                "1\n",
                "Hello\n",
                "World\n",
                "World!\n",
                "World!\n",
                "HELLO, WORLD! - SUCCESS\n",
            )
        );
    }
}

#[test]
fn test_e2e_split_and_join() {
    let code = r#"
let fruits_str = "apple,banana,cherry"
let fruits = fruits_str.split(",")
say fruits.len()
for f in fruits
    say f
end

let joined = fruits.join(" - ")
say joined

// Function syntax
let words = split("hello world alya", " ")
say join(words, "_")

// Indexing split result
say fruits[0]
say fruits[2]

// Method chaining
let chained = "x:y:z".split(":").join("/")
say chained

// Empty delimiter (char-by-char split)
let chars = "abc".split("")
say chars.len()
for c in chars
    say c
end

// Single element & empty array join
let single = ["solo"].join(",")
say single

let empty = [].join(",")
say "empty: [{empty}]"

// Split delimiter not found
let no_match = "standalone".split(",")
say no_match.len()
say no_match[0]
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!(
                "3\n",
                "apple\n",
                "banana\n",
                "cherry\n",
                "apple - banana - cherry\n",
                "hello_world_alya\n",
                "apple\n",
                "cherry\n",
                "x/y/z\n",
                "3\n",
                "a\n",
                "b\n",
                "c\n",
                "solo\n",
                "empty: []\n",
                "1\n",
                "standalone\n",
            )
        );
    }
}

#[test]
fn test_e2e_character_tools() {
    let code = r#"
let s = "Alya 2026"
say char_at(s, 0)
say s.char_at(1)
say s[2]
say s[3]

say ord("A")
say ord("a")
say "Z".ord()
say chr(66)
say chr(98)

say is_digit("7")
say is_digit("a")
say "9".is_digit()

say is_alpha("X")
say is_alpha("_")
say is_alpha("5")
say "m".is_alpha()

say is_alnum("A")
say is_alnum("3")
say is_alnum("!")

say is_space(" ")
say is_space("\t")
say is_space("x")
say " ".is_space()

// Out of bounds safety
let out1 = char_at(s, 50)
say "oob: [{out1}]"
let out2 = s[-1]
say "neg: [{out2}]"
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!(
                "A\n",
                "l\n",
                "y\n",
                "a\n",
                "65\n",
                "97\n",
                "90\n",
                "B\n",
                "b\n",
                "1\n",
                "0\n",
                "1\n",
                "1\n",
                "1\n",
                "0\n",
                "1\n",
                "1\n",
                "1\n",
                "0\n",
                "1\n",
                "1\n",
                "0\n",
                "1\n",
                "oob: []\n",
                "neg: []\n",
            )
        );
    }
}

#[test]
fn test_e2e_str_conversion_and_concat() {
    let code = r#"
let a = 12345
say str(a)
let b = -987
say str(b)
say str(0)
say "Count: " + 42
say 100 + " percent"
let x = 50
say "Val is {x}"
say "item_" + 1 + "_part_" + 2
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!(
                "12345\n",
                "-987\n",
                "0\n",
                "Count: 42\n",
                "100 percent\n",
                "Val is 50\n",
                "item_1_part_2\n",
            )
        );
    }
}
