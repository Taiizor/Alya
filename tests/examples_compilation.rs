use std::fs;
use alya::lexer::Lexer;
use alya::parser::Parser;
use alya::codegen::{self, Architecture, OperatingSystem};

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
        let tokens = lexer.tokenize()
            .unwrap_or_else(|e| panic!("Lexer failed for '{}': {}", example_name, e));

        // 2. Parser
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()
            .unwrap_or_else(|e| panic!("Parser failed for '{}': {}", example_name, e));

        // 3. Codegen for x64
        let x64_asm = codegen::generate(&ast, Architecture::X64, OperatingSystem::Windows);
        assert!(!x64_asm.is_empty(), "Empty x64 assembly generated for '{}'", example_name);
        assert!(x64_asm.contains(".global main"), "Missing main entry in x64 for '{}'", example_name);

        // 4. Codegen for x86
        let x86_asm = codegen::generate(&ast, Architecture::X86, OperatingSystem::Linux);
        assert!(!x86_asm.is_empty(), "Empty x86 assembly generated for '{}'", example_name);

        // 5. Codegen for arm64
        let arm64_asm = codegen::generate(&ast, Architecture::ARM64, OperatingSystem::Linux);
        assert!(!arm64_asm.is_empty(), "Empty arm64 assembly generated for '{}'", example_name);
    }
}
