use super::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Say(Expr),
    Let {
        name: String,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Repeat {
        body: Vec<Stmt>,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Expr(Expr),
    TryCatch {
        try_block: Vec<Stmt>,
        catch_var: Option<String>,
        catch_block: Vec<Stmt>,
    },
}
