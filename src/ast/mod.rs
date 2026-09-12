pub mod expr;
pub mod stmt;

pub use expr::{BinaryOp, Expr, UnaryOp};
pub use stmt::{ExternFnDecl, ExternParam, Stmt};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
