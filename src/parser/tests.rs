use super::Parser;
use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::lexer::Lexer;

fn parse_code(code: &str) -> Result<crate::ast::Program, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn test_parse_binary_precedence() {
    let program = parse_code("say 1 + 2 * 3").expect("Parse failed");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::Say(Expr::Binary { left, op, right }) => {
            assert_eq!(**left, Expr::Number(1.0));
            assert_eq!(*op, BinaryOp::Add);
            match &**right {
                Expr::Binary {
                    left: rleft,
                    op: rop,
                    right: rright,
                } => {
                    assert_eq!(**rleft, Expr::Number(2.0));
                    assert_eq!(*rop, BinaryOp::Multiply);
                    assert_eq!(**rright, Expr::Number(3.0));
                }
                other => panic!("Expected multiplication on right, got {:?}", other),
            }
        }
        other => panic!("Expected Say with Binary, got {:?}", other),
    }
}

#[test]
fn test_parse_parentheses_precedence() {
    let program = parse_code("say (1 + 2) * 3").expect("Parse failed");

    match &program.statements[0] {
        Stmt::Say(Expr::Binary { left, op, right }) => {
            assert_eq!(*op, BinaryOp::Multiply);
            assert_eq!(**right, Expr::Number(3.0));
            match &**left {
                Expr::Binary {
                    left: lleft,
                    op: lop,
                    right: lright,
                } => {
                    assert_eq!(**lleft, Expr::Number(1.0));
                    assert_eq!(*lop, BinaryOp::Add);
                    assert_eq!(**lright, Expr::Number(2.0));
                }
                other => panic!("Expected addition inside parens, got {:?}", other),
            }
        }
        other => panic!("Expected Say with Binary, got {:?}", other),
    }
}

#[test]
fn test_parse_unary_not_and_negate() {
    let program = parse_code("say -5\nsay not true").expect("Parse failed");
    assert_eq!(program.statements.len(), 2);

    match &program.statements[0] {
        Stmt::Say(Expr::Unary { op, expr }) => {
            assert_eq!(*op, UnaryOp::Negate);
            assert_eq!(**expr, Expr::Number(5.0));
        }
        other => panic!("Expected negate unary, got {:?}", other),
    }

    match &program.statements[1] {
        Stmt::Say(Expr::Unary { op, expr }) => {
            assert_eq!(*op, UnaryOp::Not);
            assert_eq!(**expr, Expr::Number(1.0)); // true is parsed as 1.0
        }
        other => panic!("Expected not unary, got {:?}", other),
    }
}

#[test]
fn test_parse_let_and_assign() {
    let program = parse_code("let score = 100\nscore = score + 10").expect("Parse failed");
    assert_eq!(program.statements.len(), 2);

    assert_eq!(
        program.statements[0],
        Stmt::Let {
            name: "score".into(),
            value: Expr::Number(100.0)
        }
    );

    assert_eq!(
        program.statements[1],
        Stmt::Assign {
            name: "score".into(),
            value: Expr::Binary {
                left: Box::new(Expr::Identifier("score".into())),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(10.0))
            }
        }
    );
}

#[test]
fn test_parse_if_else() {
    let code = r#"
if x > 0
    say "positive"
else
    say "non-positive"
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::If {
            condition,
            then_block,
            else_block,
        } => {
            assert_eq!(
                *condition,
                Expr::Binary {
                    left: Box::new(Expr::Identifier("x".into())),
                    op: BinaryOp::Greater,
                    right: Box::new(Expr::Number(0.0)),
                }
            );
            assert_eq!(then_block.len(), 1);
            assert_eq!(then_block[0], Stmt::Say(Expr::String("positive".into())));
            let else_stmts = else_block.as_ref().expect("Expected else block");
            assert_eq!(else_stmts.len(), 1);
            assert_eq!(
                else_stmts[0],
                Stmt::Say(Expr::String("non-positive".into()))
            );
        }
        other => panic!("Expected If statement, got {:?}", other),
    }
}

#[test]
fn test_parse_while_loop() {
    let code = r#"
while count < 5
    say count
    count = count + 1
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While { condition, body } => {
            assert_eq!(
                *condition,
                Expr::Binary {
                    left: Box::new(Expr::Identifier("count".into())),
                    op: BinaryOp::Less,
                    right: Box::new(Expr::Number(5.0)),
                }
            );
            assert_eq!(body.len(), 2);
        }
        other => panic!("Expected While loop, got {:?}", other),
    }
}

#[test]
fn test_parse_repeat_loop() {
    let code = r#"
repeat
    count += 1
    if count >= 3
        break
    end
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::Repeat { body } => {
            assert_eq!(body.len(), 2);
        }
        other => panic!("Expected Repeat loop, got {:?}", other),
    }
}

#[test]
fn test_parse_for_loop() {
    let code = r#"
for i in 1..10
    say i
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::For {
            var,
            start,
            end,
            body,
        } => {
            assert_eq!(var, "i");
            assert_eq!(*start, Expr::Number(1.0));
            assert_eq!(*end, Expr::Number(10.0));
            assert_eq!(body.len(), 1);
        }
        other => panic!("Expected For loop, got {:?}", other),
    }
}

#[test]
fn test_parse_for_each_loop() {
    let code = r#"
for item in [1, 2, 3]
    say item
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::ForEach {
            var,
            iterable,
            body,
        } => {
            assert_eq!(var, "item");
            assert_eq!(
                *iterable,
                Expr::Array(vec![
                    Expr::Number(1.0),
                    Expr::Number(2.0),
                    Expr::Number(3.0)
                ])
            );
            assert_eq!(body.len(), 1);
        }
        other => panic!("Expected ForEach loop, got {:?}", other),
    }
}

#[test]
fn test_parse_function_and_call() {
    let code = r#"
function multiply(a, b)
    return a * b
end

let result = multiply(6, 7)
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 2);

    assert_eq!(
        program.statements[0],
        Stmt::Function {
            name: "multiply".into(),
            params: vec!["a".into(), "b".into()],
            body: vec![Stmt::Return(Some(Expr::Binary {
                left: Box::new(Expr::Identifier("a".into())),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Identifier("b".into())),
            }))],
        }
    );

    assert_eq!(
        program.statements[1],
        Stmt::Let {
            name: "result".into(),
            value: Expr::Call {
                name: "multiply".into(),
                args: vec![Expr::Number(6.0), Expr::Number(7.0)],
            }
        }
    );
}

#[test]
fn test_parse_unclosed_block_error() {
    let code = "if x > 0\nsay 1";
    let result = parse_code(code);
    assert!(result.is_err());
}

#[test]
fn test_parse_compound_assignments() {
    let code = "x += 5\ny -= 3\nz *= 2\nw /= 4";
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 4);

    assert_eq!(
        program.statements[0],
        Stmt::Assign {
            name: "x".into(),
            value: Expr::Binary {
                left: Box::new(Expr::Identifier("x".into())),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(5.0)),
            }
        }
    );

    assert_eq!(
        program.statements[1],
        Stmt::Assign {
            name: "y".into(),
            value: Expr::Binary {
                left: Box::new(Expr::Identifier("y".into())),
                op: BinaryOp::Subtract,
                right: Box::new(Expr::Number(3.0)),
            }
        }
    );

    assert_eq!(
        program.statements[2],
        Stmt::Assign {
            name: "z".into(),
            value: Expr::Binary {
                left: Box::new(Expr::Identifier("z".into())),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Number(2.0)),
            }
        }
    );

    assert_eq!(
        program.statements[3],
        Stmt::Assign {
            name: "w".into(),
            value: Expr::Binary {
                left: Box::new(Expr::Identifier("w".into())),
                op: BinaryOp::Divide,
                right: Box::new(Expr::Number(4.0)),
            }
        }
    );
}

#[test]
fn test_parse_elif_and_logical_symbols() {
    let code = r#"
if a > 0 && b > 0
    say "both"
elif a > 0 || !c
    say "one or not c"
else
    say "none"
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::If {
            condition,
            then_block: _,
            else_block,
        } => {
            assert_eq!(
                *condition,
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Identifier("a".into())),
                        op: BinaryOp::Greater,
                        right: Box::new(Expr::Number(0.0)),
                    }),
                    op: BinaryOp::And,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Identifier("b".into())),
                        op: BinaryOp::Greater,
                        right: Box::new(Expr::Number(0.0)),
                    }),
                }
            );
            assert!(else_block.is_some());
        }
        other => panic!("Expected If, got {:?}", other),
    }
}

#[test]
fn test_parse_ask_expression() {
    let code = "let name = ask \"Your name: \"\nlet city = ask(\"City: \")\nlet general = ask";
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 3);

    assert_eq!(
        program.statements[0],
        Stmt::Let {
            name: "name".into(),
            value: Expr::Call {
                name: "ask".into(),
                args: vec![Expr::String("Your name: ".into())],
            }
        }
    );
}

#[test]
fn test_parse_try_catch() {
    let code = r#"
try
    let x = 10 / 0
catch err
    say err
end

try
    say 42
catch
    say "error"
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 2);

    match &program.statements[0] {
        Stmt::TryCatch {
            try_block,
            catch_var,
            catch_block,
        } => {
            assert_eq!(try_block.len(), 1);
            assert_eq!(catch_var.as_deref(), Some("err"));
            assert_eq!(catch_block.len(), 1);
        }
        other => panic!("Expected TryCatch, got {:?}", other),
    }

    match &program.statements[1] {
        Stmt::TryCatch {
            try_block,
            catch_var,
            catch_block,
        } => {
            assert_eq!(try_block.len(), 1);
            assert_eq!(*catch_var, None);
            assert_eq!(catch_block.len(), 1);
        }
        other => panic!("Expected TryCatch, got {:?}", other),
    }
}

#[test]
fn test_parse_arrays() {
    let source = "let arr = [1, 2, 3]\nsay arr[0]\narr[1] = 42\nmatrix[0][1] = 99";
    let mut lexer = crate::lexer::Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Failed to parse");

    assert_eq!(program.statements.len(), 4);

    // let arr = [1, 2, 3]
    match &program.statements[0] {
        Stmt::Let { name, value } => {
            assert_eq!(name, "arr");
            match value {
                Expr::Array(elements) => {
                    assert_eq!(elements.len(), 3);
                    assert_eq!(elements[0], Expr::Number(1.0));
                    assert_eq!(elements[1], Expr::Number(2.0));
                    assert_eq!(elements[2], Expr::Number(3.0));
                }
                other => panic!("Expected Expr::Array, got {:?}", other),
            }
        }
        other => panic!("Expected Stmt::Let, got {:?}", other),
    }

    // say arr[0]
    match &program.statements[1] {
        Stmt::Say(Expr::Index { array, index }) => {
            assert_eq!(**array, Expr::Identifier("arr".into()));
            assert_eq!(**index, Expr::Number(0.0));
        }
        other => panic!("Expected Stmt::Say(Expr::Index), got {:?}", other),
    }

    // arr[1] = 42
    match &program.statements[2] {
        Stmt::IndexAssign {
            array,
            index,
            value,
        } => {
            assert_eq!(*array, Expr::Identifier("arr".into()));
            assert_eq!(*index, Expr::Number(1.0));
            assert_eq!(*value, Expr::Number(42.0));
        }
        other => panic!("Expected Stmt::IndexAssign, got {:?}", other),
    }

    // matrix[0][1] = 99
    match &program.statements[3] {
        Stmt::IndexAssign {
            array,
            index,
            value,
        } => {
            match array {
                Expr::Index {
                    array: inner_array,
                    index: inner_index,
                } => {
                    assert_eq!(**inner_array, Expr::Identifier("matrix".into()));
                    assert_eq!(**inner_index, Expr::Number(0.0));
                }
                other => panic!("Expected Expr::Index, got {:?}", other),
            }
            assert_eq!(*index, Expr::Number(1.0));
            assert_eq!(*value, Expr::Number(99.0));
        }
        other => panic!("Expected Stmt::IndexAssign, got {:?}", other),
    }
}

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

    super::resolve_imports(&mut program, &temp_dir).expect("Failed to resolve imports");

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

    super::resolve_imports(&mut program, &temp_dir).expect("Failed to resolve imports");

    // The function is imported and duplicate avoided
    assert_eq!(program.statements.len(), 3);
    match &program.statements[0] {
        Stmt::Function { name, .. } => assert_eq!(name, "calc_sum"),
        other => panic!("Expected Stmt::Function, got {:?}", other),
    }

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_parse_struct_and_field_access() {
    let source = "struct Point\n  x\n  y\nend\nlet p = Point { x: 10, y: 20 }\np.x = 99\nsay p.x";
    let mut lexer = crate::lexer::Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Failed to parse");

    assert_eq!(program.statements.len(), 4);

    // 1. StructDef
    match &program.statements[0] {
        Stmt::StructDef { name, fields } => {
            assert_eq!(name, "Point");
            assert_eq!(fields, &["x", "y"]);
        }
        other => panic!("Expected Stmt::StructDef, got {:?}", other),
    }

    // 2. StructInit
    match &program.statements[1] {
        Stmt::Let { name, value } => {
            assert_eq!(name, "p");
            match value {
                Expr::StructInit {
                    name: sname,
                    fields,
                } => {
                    assert_eq!(sname, "Point");
                    assert_eq!(fields.len(), 2);
                    assert_eq!(fields[0].0, "x");
                    assert_eq!(fields[1].0, "y");
                }
                other => panic!("Expected Expr::StructInit, got {:?}", other),
            }
        }
        other => panic!("Expected Stmt::Let, got {:?}", other),
    }

    // 3. FieldAssign
    match &program.statements[2] {
        Stmt::FieldAssign {
            object,
            field,
            value,
        } => {
            assert_eq!(*object, Expr::Identifier("p".into()));
            assert_eq!(field, "x");
            assert_eq!(*value, Expr::Number(99.0));
        }
        other => panic!("Expected Stmt::FieldAssign, got {:?}", other),
    }

    // 4. Say with FieldAccess
    match &program.statements[3] {
        Stmt::Say(Expr::FieldAccess { object, field }) => {
            assert_eq!(**object, Expr::Identifier("p".into()));
            assert_eq!(field, "x");
        }
        other => panic!("Expected Stmt::Say(FieldAccess), got {:?}", other),
    }
}

#[test]
fn test_resolve_embedded_stdlib_modules() {
    let code = r#"
import "std/math"
import "std/time"
import "std/os"
import "std/json"
say PI
"#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse().expect("Parse failed");

    // Use a non-existent directory to force fallback to embedded stdlib
    let dummy_dir = std::path::Path::new("non_existent_dir_for_test");
    super::resolve_imports(&mut ast, dummy_dir).expect("Embedded stdlib resolution should succeed");

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

    assert!(has_hypot, "Missing hypot from std/math");
    assert!(has_now, "Missing now from std/time");
    assert!(has_env, "Missing env from std/os");
    assert!(has_json_bool, "Missing json_bool from std/json");
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
    super::resolve_imports(&mut ast, dummy_dir).expect("Embedded stdlib resolution should succeed");

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
