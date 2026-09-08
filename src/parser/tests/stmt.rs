use super::*;

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
fn test_parse_multi_let_single_value() {
    let program = parse_code("let idx, val = 0").expect("Parse failed");
    assert_eq!(program.statements.len(), 2);
    assert_eq!(
        program.statements[0],
        Stmt::Let {
            name: "idx".into(),
            value: Expr::Number(0.0)
        }
    );
    assert_eq!(
        program.statements[1],
        Stmt::Let {
            name: "val".into(),
            value: Expr::Number(0.0)
        }
    );
}

#[test]
fn test_parse_multi_let_multiple_values() {
    let program = parse_code("let idx, val = 0, 1").expect("Parse failed");
    assert_eq!(program.statements.len(), 2);
    assert_eq!(
        program.statements[0],
        Stmt::Let {
            name: "idx".into(),
            value: Expr::Number(0.0)
        }
    );
    assert_eq!(
        program.statements[1],
        Stmt::Let {
            name: "val".into(),
            value: Expr::Number(1.0)
        }
    );
}

#[test]
fn test_parse_multi_let_mismatch_error() {
    let result1 = parse_code("let a, b = 1, 2, 3");
    assert!(result1.is_err());
    assert!(result1.unwrap_err().contains("Mismatch in 'let' statement"));

    let result2 = parse_code("let a, b, c = 1, 2");
    assert!(result2.is_err());
    assert!(result2.unwrap_err().contains("Mismatch in 'let' statement"));
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
            defaults: vec![None, None],
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
fn test_parse_function_default_parameters() {
    let code = r#"
function greet(name, greeting = "Hello", punctuation = "!")
    say greeting
end

greet("Alya")
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 2);

    assert_eq!(
        program.statements[0],
        Stmt::Function {
            name: "greet".into(),
            params: vec!["name".into(), "greeting".into(), "punctuation".into()],
            defaults: vec![
                None,
                Some(Expr::String("Hello".into())),
                Some(Expr::String("!".into())),
            ],
            body: vec![Stmt::Say(Expr::Identifier("greeting".into()))],
        }
    );

    // Call greet("Alya") should have been expanded to include default arguments
    assert_eq!(
        program.statements[1],
        Stmt::Expr(Expr::Call {
            name: "greet".into(),
            args: vec![
                Expr::String("Alya".into()),
                Expr::String("Hello".into()),
                Expr::String("!".into()),
            ],
        })
    );
}

#[test]
fn test_parse_unclosed_block_error() {
    let code = "if x > 0\nsay 1";
    let result = parse_code(code);
    assert!(result.is_err());
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
            finally_block,
        } => {
            assert_eq!(try_block.len(), 1);
            assert_eq!(catch_var.as_deref(), Some("err"));
            assert_eq!(catch_block.len(), 1);
            assert_eq!(*finally_block, None);
        }
        other => panic!("Expected TryCatch, got {:?}", other),
    }

    match &program.statements[1] {
        Stmt::TryCatch {
            try_block,
            catch_var,
            catch_block,
            finally_block,
        } => {
            assert_eq!(try_block.len(), 1);
            assert_eq!(*catch_var, None);
            assert_eq!(catch_block.len(), 1);
            assert_eq!(*finally_block, None);
        }
        other => panic!("Expected TryCatch, got {:?}", other),
    }
}

#[test]
fn test_parse_catch_parentheses_and_finally() {
    let code = r#"
try
    throw "error message"
catch (e)
    say e
finally
    say "cleanup"
end

try
    say 1
finally
    say 2
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 2);

    match &program.statements[0] {
        Stmt::TryCatch {
            try_block,
            catch_var,
            catch_block,
            finally_block,
        } => {
            assert_eq!(try_block.len(), 1);
            assert!(matches!(&try_block[0], Stmt::Throw(Some(_))));
            assert_eq!(catch_var.as_deref(), Some("e"));
            assert_eq!(catch_block.len(), 1);
            assert_eq!(finally_block.as_ref().map(|b| b.len()), Some(1));
        }
        other => panic!("Expected TryCatch with finally, got {:?}", other),
    }

    match &program.statements[1] {
        Stmt::TryCatch {
            try_block,
            catch_var,
            catch_block,
            finally_block,
        } => {
            assert_eq!(try_block.len(), 1);
            assert_eq!(*catch_var, None);
            assert_eq!(catch_block.len(), 0);
            assert_eq!(finally_block.as_ref().map(|b| b.len()), Some(1));
        }
        other => panic!("Expected TryCatch without catch, got {:?}", other),
    }
}

#[test]
fn test_parse_throw_statements() {
    let code = r#"
throw "custom"
throw
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 2);
    match &program.statements[0] {
        Stmt::Throw(Some(expr)) => {
            assert_eq!(*expr, Expr::String("custom".into()));
        }
        other => panic!("Expected Stmt::Throw(Some), got {:?}", other),
    }
    match &program.statements[1] {
        Stmt::Throw(None) => {}
        other => panic!("Expected Stmt::Throw(None), got {:?}", other),
    }
}

#[test]
fn test_parse_when_enhanced() {
    let code = r#"
when status
    is 200, 201 then
        say "OK"
        let count = 1
    is 400..499
        say "Client Error"
    is >= 500 then
        say "Server Error"
    else
        say "Unknown"
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::If {
            condition: _,
            then_block,
            else_block,
        } => {
            assert_eq!(then_block.len(), 2);
            assert!(else_block.is_some());
        }
        other => panic!("Expected Stmt::If from desugared when, got {:?}", other),
    }
}
