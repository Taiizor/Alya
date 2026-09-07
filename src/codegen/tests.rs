use super::generate;
use super::target::{Architecture, OperatingSystem};
use crate::ast::{BinaryOp, Expr, Program, Stmt};

fn simple_program(stmt: Stmt) -> Program {
    Program {
        statements: vec![stmt],
    }
}

#[test]
fn test_codegen_x64_windows_header_and_footer() {
    let program = simple_program(Stmt::Say(Expr::Number(42.0)));
    let asm = generate(&program, Architecture::X64, OperatingSystem::Windows);

    assert!(asm.contains(".global main"));
    assert!(asm.contains("main:"));
    assert!(asm.contains("push %rbp"));
    assert!(asm.contains("mov %rsp, %rbp"));
    assert!(asm.contains("ret"));
}

#[test]
fn test_codegen_x86_header_and_footer() {
    let program = simple_program(Stmt::Say(Expr::Number(42.0)));
    let asm = generate(&program, Architecture::X86, OperatingSystem::Linux);

    assert!(asm.contains(".global main"));
    assert!(asm.contains("main:"));
    assert!(asm.contains("push %ebp"));
    assert!(asm.contains("mov %esp, %ebp"));
    assert!(asm.contains("ret"));
}

#[test]
fn test_codegen_arm64_header_and_footer() {
    let program = simple_program(Stmt::Say(Expr::Number(42.0)));
    let asm = generate(&program, Architecture::ARM64, OperatingSystem::Linux);

    assert!(asm.contains(".global main"));
    assert!(asm.contains("main:"));
    assert!(asm.contains("stp x29, x30, [sp, #-16]!"));
    assert!(asm.contains("ldp x29, x30, [sp], #16"));
    assert!(asm.contains("ret"));
}

#[test]
fn test_codegen_string_rodata() {
    let program = simple_program(Stmt::Say(Expr::String("Test String".into())));
    let asm = generate(&program, Architecture::X64, OperatingSystem::Windows);

    assert!(asm.contains(".section .rodata"));
    assert!(asm.contains(".string \"Test String\\n\""));
}

#[test]
fn test_codegen_let_and_binary_op() {
    let program = Program {
        statements: vec![
            Stmt::Let {
                name: "x".into(),
                value: Expr::Binary {
                    left: Box::new(Expr::Number(10.0)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(20.0)),
                },
            },
            Stmt::Say(Expr::Identifier("x".into())),
        ],
    };

    let asm = generate(&program, Architecture::X64, OperatingSystem::Windows);
    assert!(asm.contains("mov $10, %rax"));
    assert!(asm.contains("push %rax"));
    assert!(asm.contains("mov $20, %rax"));
    assert!(asm.contains("add %rbx, %rax"));
}

#[test]
fn test_codegen_function_definition() {
    let program = Program {
        statements: vec![Stmt::Function {
            name: "my_func".into(),
            params: vec!["a".into()],
            body: vec![Stmt::Return(Some(Expr::Identifier("a".into())))],
        }],
    };

    let asm = generate(&program, Architecture::X64, OperatingSystem::Windows);
    assert!(asm.contains("fn_my_func:"));
    assert!(asm.contains("ret"));
}

#[test]
fn test_codegen_macos_arm64_header_and_sections() {
    let program = simple_program(Stmt::Say(Expr::String("Hello Mac".into())));
    let asm = generate(&program, Architecture::ARM64, OperatingSystem::MacOS);

    assert!(asm.contains(".globl _main"));
    assert!(asm.contains("_main:"));
    assert!(asm.contains(".section __TEXT,__cstring,cstring_literals"));
    assert!(asm.contains(".asciz \"Hello Mac\\n\""));
    assert!(asm.contains("_printf"));
}

#[test]
fn test_codegen_macos_x64_header_and_sections() {
    let program = simple_program(Stmt::Say(Expr::String("Hello Mac".into())));
    let asm = generate(&program, Architecture::X64, OperatingSystem::MacOS);

    assert!(asm.contains(".globl _main"));
    assert!(asm.contains("_main:"));
    assert!(asm.contains(".section __TEXT,__cstring,cstring_literals"));
    assert!(asm.contains(".asciz \"Hello Mac\\n\""));
    assert!(asm.contains("_printf"));
}
