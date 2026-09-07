use super::*;

#[test]
fn test_parse_import() {
    let source = "import \"math_utils.alya\"\nsay 42";
    let mut lexer = crate::lexer::Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Failed to parse");

    assert_eq!(program.statements.len(), 2);
    match &program.statements[0] {
        Stmt::Import(path) => assert_eq!(path, "math_utils.alya"),
        other => panic!("Expected Stmt::Import, got {:?}", other),
    }
}

#[test]
fn test_resolve_imports_temporary_files() {
    use std::fs;
    let temp_dir = std::env::temp_dir().join(format!("alya_import_test_{}", std::process::id()));
    let _ = fs::create_dir_all(&temp_dir);

    let helper_path = temp_dir.join("helper.alya");
    fs::write(&helper_path, "function get_val()\n    return 42\nend\n").unwrap();

    let main_source = "import \"helper.alya\"\nlet ans = get_val()\nsay ans";
    let mut lexer = crate::lexer::Lexer::new(main_source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse().expect("Failed to parse");

    resolve_imports(&mut program, &temp_dir).expect("Failed to resolve imports");

    // After resolution, import is replaced by the function definition from helper.alya
    assert_eq!(program.statements.len(), 3);
    match &program.statements[0] {
        Stmt::Function { name, .. } => assert_eq!(name, "get_val"),
        other => panic!("Expected Stmt::Function, got {:?}", other),
    }

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_resolve_imports_subdirectory_and_backslash_normalization() {
    use std::fs;
    let temp_dir =
        std::env::temp_dir().join(format!("alya_import_sub_test_{}", std::process::id()));
    let sub_dir = temp_dir.join("sub");
    let _ = fs::create_dir_all(&sub_dir);

    let helper_path = sub_dir.join("calc.alya");
    fs::write(
        &helper_path,
        "function calc_sum(a, b)\n    return a + b\nend\n",
    )
    .unwrap();

    // Test both forward slash and backslash in import path (duplicate is deduplicated)
    let main_source =
        "import \"sub/calc.alya\"\nimport \"sub\\\\calc.alya\"\nlet ans = calc_sum(1, 2)\nsay ans";
    let mut lexer = crate::lexer::Lexer::new(main_source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse().expect("Failed to parse");

    resolve_imports(&mut program, &temp_dir).expect("Failed to resolve imports");

    // The function is imported and duplicate avoided
    assert_eq!(program.statements.len(), 3);
    match &program.statements[0] {
        Stmt::Function { name, .. } => assert_eq!(name, "calc_sum"),
        other => panic!("Expected Stmt::Function, got {:?}", other),
    }

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_resolve_embedded_stdlib_modules() {
    let code = r#"
import "std/math"
import "std/time"
import "std/os"
import "std/json"
import "std/mem"
import "std/str"
import "std/path"
import "std/fs"
import "std/hash"
import "std/collections"
import "std/test"
say PI
"#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse().expect("Parse failed");

    // Use a non-existent directory to force fallback to embedded stdlib
    let dummy_dir = std::path::Path::new("non_existent_dir_for_test");
    resolve_imports(&mut ast, dummy_dir).expect("Embedded stdlib resolution should succeed");

    // Check that functions and constants from stdlib were imported
    let has_hypot = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "hypot",
        _ => false,
    });
    let has_now = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "now",
        _ => false,
    });
    let has_env = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "env",
        _ => false,
    });
    let has_json_bool = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "json_bool",
        _ => false,
    });
    let has_arena_new = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "arena_new",
        _ => false,
    });
    let has_starts_with = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "starts_with",
        _ => false,
    });
    let has_path_join = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "path_join",
        _ => false,
    });
    let has_copy_file = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "copy_file",
        _ => false,
    });
    let has_fnv1a = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "fnv1a",
        _ => false,
    });
    let has_stack_new = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "stack_new",
        _ => false,
    });
    let has_assert_eq = ast.statements.iter().any(|s| match s {
        Stmt::Function { name, .. } => name == "assert_eq",
        _ => false,
    });

    assert!(has_hypot, "Missing hypot from std/math");
    assert!(has_now, "Missing now from std/time");
    assert!(has_env, "Missing env from std/os");
    assert!(has_json_bool, "Missing json_bool from std/json");
    assert!(has_arena_new, "Missing arena_new from std/mem");
    assert!(has_starts_with, "Missing starts_with from std/str");
    assert!(has_path_join, "Missing path_join from std/path");
    assert!(has_copy_file, "Missing copy_file from std/fs");
    assert!(has_fnv1a, "Missing fnv1a from std/hash");
    assert!(has_stack_new, "Missing stack_new from std/collections");
    assert!(has_assert_eq, "Missing assert_eq from std/test");
}

#[test]
fn test_embedded_stdlib_deduplication() {
    let code = r#"
import "std/math"
import "std/math"
say PI
"#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse().expect("Parse failed");

    let dummy_dir = std::path::Path::new("non_existent_dir_for_test");
    resolve_imports(&mut ast, dummy_dir).expect("Embedded stdlib resolution should succeed");

    let hypot_count = ast
        .statements
        .iter()
        .filter(|s| match s {
            Stmt::Function { name, .. } => name == "hypot",
            _ => false,
        })
        .count();

    assert_eq!(
        hypot_count, 1,
        "Duplicate import of std/math should only include hypot once"
    );
}
