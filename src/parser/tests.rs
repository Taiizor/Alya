use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::lexer::Lexer;
use super::Parser;

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
                Expr::Binary { left: rleft, op: rop, right: rright } => {
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
                Expr::Binary { left: lleft, op: lop, right: lright } => {
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
        Stmt::If { condition, then_block, else_block } => {
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
            assert_eq!(else_stmts[0], Stmt::Say(Expr::String("non-positive".into())));
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
fn test_parse_for_loop() {
    let code = r#"
for i in 1..10
    say i
end
"#;
    let program = parse_code(code).expect("Parse failed");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::For { var, start, end, body } => {
            assert_eq!(var, "i");
            assert_eq!(*start, Expr::Number(1.0));
            assert_eq!(*end, Expr::Number(10.0));
            assert_eq!(body.len(), 1);
        }
        other => panic!("Expected For loop, got {:?}", other),
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
