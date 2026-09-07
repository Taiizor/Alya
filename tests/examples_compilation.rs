use alya::codegen::{self, Architecture, OperatingSystem};
use alya::lexer::Lexer;
use alya::parser::Parser;
use std::fs;

#[test]
fn test_all_examples_compile_to_assembly() {
    let mut examples: Vec<String> = fs::read_dir("examples")
        .expect("Failed to read examples directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("alya") {
                path.file_name()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();
    examples.sort();
    assert!(!examples.is_empty(), "No .alya files found in examples/");

    for example_name in examples {
        let path = format!("examples/{}", example_name);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read example file '{}': {}", path, e));

        // 1. Lexer
        let mut lexer = Lexer::new(&source);
        let tokens = lexer
            .tokenize()
            .unwrap_or_else(|e| panic!("Lexer failed for '{}': {}", example_name, e));

        // 2. Parser
        let mut parser = Parser::new(tokens);
        let mut ast = parser
            .parse()
            .unwrap_or_else(|e| panic!("Parser failed for '{}': {}", example_name, e));

        let base_dir = std::path::Path::new(&path)
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        alya::parser::resolve_imports(&mut ast, base_dir)
            .unwrap_or_else(|e| panic!("Import resolution failed for '{}': {}", example_name, e));

        // 3. Codegen for x64
        let x64_asm = codegen::generate(&ast, Architecture::X64, OperatingSystem::Windows);
        assert!(
            !x64_asm.is_empty(),
            "Empty x64 assembly generated for '{}'",
            example_name
        );
        assert!(
            x64_asm.contains(".global main"),
            "Missing main entry in x64 for '{}'",
            example_name
        );

        // 4. Codegen for x86
        let x86_asm = codegen::generate(&ast, Architecture::X86, OperatingSystem::Linux);
        assert!(
            !x86_asm.is_empty(),
            "Empty x86 assembly generated for '{}'",
            example_name
        );

        // 5. Codegen for arm64
        let arm64_asm = codegen::generate(&ast, Architecture::ARM64, OperatingSystem::Linux);
        assert!(
            !arm64_asm.is_empty(),
            "Empty arm64 assembly generated for '{}'",
            example_name
        );

        // 6. Codegen for macOS arm64
        let macos_arm64 = codegen::generate(&ast, Architecture::ARM64, OperatingSystem::MacOS);
        assert!(
            macos_arm64.contains(".globl _main"),
            "Missing _main in macOS ARM64 for '{}'",
            example_name
        );

        // 7. Codegen for macOS x64
        let macos_x64 = codegen::generate(&ast, Architecture::X64, OperatingSystem::MacOS);
        assert!(
            macos_x64.contains(".globl _main"),
            "Missing _main in macOS x64 for '{}'",
            example_name
        );
    }
}

#[test]
fn test_all_examples_execute_with_gcc() {
    if std::process::Command::new("gcc")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("Skipping GCC execution: GCC not found in PATH.");
        return;
    }

    let mut examples: Vec<String> = fs::read_dir("examples")
        .expect("Failed to read examples directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("alya") {
                path.file_name()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();
    examples.sort();

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

    for (idx, example_name) in examples.iter().enumerate() {
        let path = format!("examples/{}", example_name);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read example file '{}': {}", path, e));

        let mut lexer = Lexer::new(&source);
        let tokens = lexer
            .tokenize()
            .unwrap_or_else(|e| panic!("Lexer failed for '{}': {}", example_name, e));
        let mut parser = Parser::new(tokens);
        let mut ast = parser
            .parse()
            .unwrap_or_else(|e| panic!("Parser failed for '{}': {}", example_name, e));

        let base_dir = std::path::Path::new(&path)
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        alya::parser::resolve_imports(&mut ast, base_dir)
            .unwrap_or_else(|e| panic!("Import resolution failed for '{}': {}", example_name, e));

        let asm_code = codegen::generate(&ast, arch, os);
        let pid = std::process::id();
        let temp_asm = format!("temp_ex_test_{}_{}.s", pid, idx);
        let temp_exe = if cfg!(target_os = "windows") {
            format!("temp_ex_test_{}_{}.exe", pid, idx)
        } else {
            format!("temp_ex_test_{}_{}", pid, idx)
        };

        fs::write(&temp_asm, &asm_code).expect("Failed to write asm");

        let mut gcc = std::process::Command::new("gcc");
        gcc.arg(&temp_asm).arg("-o").arg(&temp_exe);
        if matches!(arch, Architecture::X86) {
            gcc.arg("-m32");
        }
        if matches!(os, OperatingSystem::Linux) {
            gcc.arg("-no-pie");
        }
        let gcc_status = gcc.status().expect("Failed to run gcc");
        let _ = fs::remove_file(&temp_asm);
        assert!(
            gcc_status.success(),
            "GCC failed to compile '{}'",
            example_name
        );

        let run_cmd = if cfg!(target_os = "windows") {
            format!(".\\{}", temp_exe)
        } else {
            format!("./{}", temp_exe)
        };

        let mut child = std::process::Command::new(&run_cmd)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn compiled example");

        // Pipe mock input for interactive examples like user_input.alya
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(b"TestUser\nAlya\n");
        }

        let output = child
            .wait_with_output()
            .expect("Failed to wait on example execution");
        let _ = fs::remove_file(&temp_exe);
        let _ = fs::remove_file("mini_output.s");

        assert!(
            output.status.success(),
            "Example '{}' failed during execution with status: {:?}",
            example_name,
            output.status
        );

        let actual_stdout = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
        let expected = get_expected_output(example_name).unwrap_or_else(|| {
            panic!(
                "Missing expected output definition for example '{}'!",
                example_name
            )
        });

        assert_eq!(
            actual_stdout, expected,
            "Example '{}' output did not match expected output!\nActual:\n{}\nExpected:\n{}",
            example_name, actual_stdout, expected
        );
    }
}

fn get_expected_output(example_name: &str) -> Option<&'static str> {
    match example_name {
        "arithmetic.alya" => Some(
            "=== Basic Arithmetic ===\n\
20 + 6 = 26\n\
20 - 6 = 14\n\
20 * 6 = 120\n\
20 / 6 = 3\n\
20 % 6 = 2\n\
\n\
=== Expression Precedence ===\n\
(20 + 6) * 2 = 52\n",
        ),
        "arrays.alya" => Some(concat!(
            "Initial array:\n",
            "[10, 20, 30, 40]\n",
            "Array length: 4\n",
            "First element: 10\n",
            "Second element: 20\n",
            "Mutated array:\n",
            "[15, 20, 99, 40]\n",
            "After push operations:\n",
            "[15, 20, 99, 40, 50, 60]\n",
            "Popped element: 60\n",
            "After pop:\n",
            "[15, 20, 99, 40, 50]\n",
            "Traversal with while loop:\n",
            "  Element at 0: 15\n",
            "  Element at 1: 20\n",
            "  Element at 2: 99\n",
            "  Element at 3: 40\n",
            "  Element at 4: 50\n",
            "Sum of elements: 224\n",
            "Caught error safely: index out of bounds\n",
        )),
        "builtins.alya" => Some(
            "Length: 12\n\
Absolute value: 25\n\
Min: 15\n\
Max: 42\n\
Square root of 64: 8\n\
2 to the power of 10: 1024\n\
Trimmed: Alya Programming Language\n\
Uppercase: ALYA PROGRAMMING LANGUAGE\n\
Lowercase: alya programming language\n\
Contains 'Language': 1\n\
Substring: Alya\n\
Split count: 3\n\
Joined: apple - banana - orange\n",
        ),
        "calculator.alya" => Some(
            "=== Simple Calculator ===\n\
Addition:       15 + 7 = 22\n\
Subtraction:    15 - 7 = 8\n\
Multiplication: 15 * 7 = 105\n\
Division:       15 / 7 = 2\n\
Modulo:         15 % 7 = 1\n\
\n\
=== Complex Expressions ===\n\
(15 + 7) * 2 = 44\n\
(15 - 7) / 2 = 4\n\
100 / 4 - 5 = 20\n\
(3 + 5) * (10 - 2) = 64\n",
        ),
        "character_tools.alya" => Some(
            "=== Character Indexing ===\n\
First char: A\n\
Second char: l\n\
\n\
=== ASCII Codes ===\n\
ASCII of 'A': 65\n\
Char from 66: B\n\
\n\
=== Character Classification ===\n\
Is '5' a digit? 1\n\
Is 'a' a digit? 0\n\
Is 'x' alpha? 1\n\
Is '9' alpha? 0\n\
Is 'x' alnum? 1\n\
Is '!' alnum? 0\n\
Is ' ' space? 1\n\
Is 'A' space? 0\n",
        ),
        "cli_args.alya" => Some(
            "=== Command-Line Arguments ===\n\
Argument count: 0\n",
        ),
        "comments.alya" => Some("Sum: 30\n"),
        "compound_operators.alya" => Some(
            "Initial: 10\n\
After += 5: 15\n\
After -= 3: 12\n\
After *= 4: 48\n\
After /= 2: 24\n",
        ),
        "conditionals.alya" => Some(
            "=== Academic Evaluation ===\n\
Score:      85\n\
Attendance: 92%\n\
Result: Grade B - Good job!\n\
Status: Eligible for honors\n\
Award:  Scholarship considered\n",
        ),
        "fibonacci.alya" => Some(
            "=== Fibonacci Sequence in Alya ===\n\
Computing first 10 Fibonacci numbers with a loop:\n\
0\n\
1\n\
1\n\
2\n\
3\n\
5\n\
8\n\
13\n\
21\n\
34\n\
\n\
Step-by-step recurrence demonstration:\n\
F(5) = F(4) + F(3) = 3 + 2 = 5\n\
F(6) = F(5) + F(4) = 5 + 3 = 8\n\
F(7) = F(6) + F(5) = 8 + 5 = 13\n\
F(8) = F(7) + F(6) = 13 + 8 = 21\n",
        ),
        "file_io.alya" => Some(
            "=== File Existence ===\n\
Exists before writing: 0\n\
\n\
=== Writing File ===\n\
Write successful: 1\n\
Exists after writing: 1\n\
\n\
=== Reading File ===\n\
Hello from Alya File I/O!\n\
Building compilers with self-hosting.\n\
=== Deleting File ===\n\
Delete successful: 1\n\
Exists after deleting: 0\n",
        ),
        "floats.alya" => Some(
            "=== Floating-Point Operations ===\n\
Radius: 2.5\n\
Circumference: 15.708\n\
Area: 19.6349\n\
Average: 25.25\n\
Converted to float: 42.5\n\
Converted back to int: 42\n\
Final score: 30\n",
        ),
        "functions.alya" => Some(
            "=== Functions Demo ===\n\
Hello, Developer! Welcome to Alya.\n\
Area of 8x5 rectangle: 40\n\
Total after discount:  $90\n",
        ),
        "hello.alya" => Some(
            "Hello, World!\n\
Welcome to Alya programming language!\n",
        ),
        "interpolation.alya" => Some(
            "=== Developer Profile ===\n\
Name:        Alice\n\
Role:        Software Engineer\n\
Experience:  5 years\n\
Projects:    12 completed\n\
\n\
Summary: Alice is a Software Engineer with 5 years of experience across 12 projects.\n",
        ),
        "loops.alya" => Some(
            "=== For Loop (Range 1..5) ===\n\
Iteration 1\n\
Iteration 2\n\
Iteration 3\n\
Iteration 4\n\
Iteration 5\n\
\n\
=== While Loop with += ===\n\
Count: 1\n\
Count: 2\n\
Count: 3\n\
Count: 4\n\
\n\
=== While Loop with Break ===\n\
While item: 1\n\
While item: 2\n\
While item: 3\n\
While broke early at w = 4\n\
\n\
=== Repeat Loop with Break ===\n\
Repeat item: 1\n\
Repeat item: 2\n\
Repeat item: 3\n\
Repeat broke at r = 4\n\
\n\
=== Loop with Continue ===\n\
Odd number: 1\n\
Odd number: 3\n\
Odd number: 5\n\
\n\
=== For-Each Array Iteration ===\n\
Fruit: apple\n\
Fruit: banana\n\
Fruit: cherry\n",
        ),
        "main.alya" => Some(
            "Enter name: Hello, TestUser! Welcome to Alya.\n\
Speed: 50 ops/sec\n",
        ),
        "maps.alya" => Some(
            "=== Hash Map Initialization ===\n\
Initial size: 0\n\
\n\
=== Accessing Entries ===\n\
Alice score: 95\n\
Bob score: 88\n\
Charlie score: 70\n\
Size after inserts: 3\n\
\n\
=== Key Membership ===\n\
Has Alice? 1\n\
Has David? 0\n\
\n\
=== Removing Entries ===\n\
Removed Charlie: 1\n\
Has Charlie? 0\n\
Size after removal: 2\n\
\n\
=== Keys and Values ===\n\
Keys count: 2\n\
Values count: 2\n\
Total score: 183\n",
        ),
        "mini_compiler.alya" => Some(concat!(
            "==================================================\n",
            "      Alya Mini-Compiler (Written in Alya)        \n",
            "==================================================\n",
            "No file provided. Compiling embedded demo program:\n",
            "1. Tokenizing source code...\n",
            "   Generated 53 tokens.\n",
            "2. Parsing and generating x64 Assembly...\n",
            "3. Writing native assembly to mini_output.s...\n\n",
            "[SUCCESS] Compilation complete!\n",
            "To produce a native standalone binary, run:\n",
            "   gcc mini_output.s -o mini_program.exe\n",
            "   ./mini_program.exe\n",
            "==================================================\n",
        )),
        "modern_features.alya" => Some(
            "Calculated score: 46\n\
Access granted!\n\
Length of greeting: 13\n\
Absolute value of -42: 42\n\
Minimum of 10 and 20: 10\n\
Maximum of 10 and 20: 20\n",
        ),
        "modules.alya" => Some(
            "=== Modules & Imports ===\n\
Sum: 16\n\
Product: 48\n\
Square of 12: 144\n\
Inline call: 2\n",
        ),
        "pattern_matching.alya" => Some(
            "=== HTTP Status Code Resolver ===\n\
Status 200: OK\n\
\n\
=== Priority Level Resolver ===\n\
Priority 2: Medium\n",
        ),
        "quickstart_arithmetic.alya" => Some("15\n5\n50\n2\n"),
        "structs.alya" => Some(concat!(
            "=== Structs and Custom Types ===\n",
            "Point 1:\n",
            "Point { x: 10, y: 20 }\n",
            "Point 1 coordinates: (10, 20)\n",
            "Mutated Point 1:\n",
            "Point { x: 100, y: 25 }\n",
            "Point 2: (5, 12)\n",
            "Distance squared of Point 2: 169\n",
            "Shifted Point: (15, 32)\n",
            "Alice is a Systems Architect with 8 years of experience.\n",
        )),
        "try_catch.alya" => Some(
            "=== Try-Catch Demo ===\n\
\n\
1. Basic try-catch:\n\
Attempting division...\n\
Caught error: Division by zero was safely handled!\n\
\n\
2. Try-catch with error message:\n\
Calculating modulo...\n\
Caught exception message: division by zero\n\
\n\
3. Successful try block:\n\
Safe division result: 25\n\
\n\
Program completed successfully without crashing.\n",
        ),
        "user_input.alya" => Some(
            "What is your name? Hello, TestUser! Welcome to Alya.\n\
What is your favorite programming language? Awesome, Alya is great!\n",
        ),
        "variables.alya" => Some(
            "=== Language Information ===\n\
Language:    Alya\n\
Version:     1\n\
Year:        2026\n\
Open Source: 1\n\
Next version will be: 2\n\
Project age: 2 years\n",
        ),
        _ => None,
    }
}
