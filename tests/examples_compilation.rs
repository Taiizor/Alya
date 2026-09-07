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
        let ast = parser
            .parse()
            .unwrap_or_else(|e| panic!("Parser failed for '{}': {}", example_name, e));

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
    }
}

#[test]
fn test_all_examples_execute_with_gcc() {
    if cfg!(target_os = "macos") {
        eprintln!("Skipping GCC execution on macOS: Apple Clang requires Mach-O toolchain.");
        return;
    }

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
    } else {
        OperatingSystem::Linux
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
        let ast = parser
            .parse()
            .unwrap_or_else(|e| panic!("Parser failed for '{}': {}", example_name, e));

        let asm_code = codegen::generate(&ast, Architecture::X64, os);
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
        if !matches!(os, OperatingSystem::Windows) {
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
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to spawn compiled example");

        // Pipe mock input for interactive examples like user_input.alya
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(b"TestUser\nAlya\n");
        }

        let status = child.wait().expect("Failed to wait on example execution");
        let _ = fs::remove_file(&temp_exe);

        assert!(
            status.success(),
            "Example '{}' failed during execution with status: {:?}",
            example_name,
            status
        );
    }
}
