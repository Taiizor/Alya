use crate::ast::*;
use crate::codegen::analysis::traversal::find_call_arg;

pub fn infer_param_struct_type(
    func_name: &str,
    param_idx: usize,
    program: &Program,
) -> Option<String> {
    for s in &program.statements {
        if let Some(arg) = find_call_arg(s, func_name, param_idx) {
            if let Some(st) = infer_expr_struct_type(arg, program) {
                return Some(st);
            }
        }
    }
    None
}

fn infer_expr_struct_type(expr: &Expr, program: &Program) -> Option<String> {
    match expr {
        Expr::StructInit { name, .. } => Some(name.clone()),
        Expr::Call { name, .. } => {
            for s in &program.statements {
                if let Stmt::StructDef { name: sname, .. } = s {
                    if sname == name {
                        return Some(name.clone());
                    }
                }
            }
            None
        }
        Expr::Identifier(var_name) => {
            for s in &program.statements {
                match s {
                    Stmt::Let { name, value } if name == var_name => {
                        return infer_expr_struct_type(value, program);
                    }
                    Stmt::ForEach { var, iterable, .. } if var == var_name => {
                        return infer_expr_struct_type(iterable, program);
                    }
                    _ => {}
                }
            }
            None
        }
        Expr::Array(elements) => elements
            .first()
            .and_then(|e| infer_expr_struct_type(e, program)),
        _ => None,
    }
}
