use crate::ast::{BinaryOp, Expr};
use crate::codegen::analysis::{escape_string, is_string_expr};
use crate::codegen::arch;
use crate::codegen::context::VarType;
use crate::codegen::target::Architecture;
use super::CodeGen;

impl CodeGen {
    pub(crate) fn generate_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                arch::emit_load_num(&mut self.output, self.arch, *n as i64);
            }
            Expr::String(s) => {
                let label = self.ctx.next_string_label();
                self.output.push_str(".section .rodata\n");
                self.output.push_str(&format!("{}:\n", label));
                self.output.push_str(&format!("    .string \"{}\"\n", escape_string(s)));
                self.output.push_str(".text\n");

                arch::emit_load_str_label(&mut self.output, self.arch, &label);
            }
            Expr::Identifier(name) => {
                if let Some(var_type) = self.ctx.variables.get(name).cloned() {
                    match var_type {
                        VarType::Number(offset) | VarType::StringOffset(offset) => {
                            arch::emit_load_var(&mut self.output, self.arch, offset, self.ctx.stack_offset);
                        }
                        VarType::StringLabel(label) => {
                            arch::emit_load_str_label(&mut self.output, self.arch, &label);
                        }
                    }
                }
            }
            Expr::Binary { left, op, right } => {
                if matches!(op, BinaryOp::Add)
                    && (is_string_expr(left, &self.ctx.variables) || is_string_expr(right, &self.ctx.variables))
                {
                    self.generate_string_concat(left, right);
                    return;
                }

                self.generate_expression(left);
                arch::emit_push_temp(&mut self.output, self.arch);

                self.generate_expression(right);
                arch::emit_binary_op(&mut self.output, self.arch, *op);
            }
            Expr::Unary { op, expr } => {
                self.generate_expression(expr);
                arch::emit_unary_op(&mut self.output, self.arch, *op);
            }
            Expr::Call { name, args } => {
                match self.arch {
                    Architecture::X86 => {
                        for arg in args.iter().rev() {
                            self.generate_expression(arg);
                            arch::emit_push_temp(&mut self.output, self.arch);
                        }
                    }
                    _ => {
                        for arg in args {
                            self.generate_expression(arg);
                            arch::emit_push_temp(&mut self.output, self.arch);
                        }
                    }
                }
                arch::emit_function_call(&mut self.output, self.arch, name, args.len(), self.ctx.stack_offset, self.os);
            }
            Expr::InterpolatedString(parts) => {
                if parts.is_empty() {
                    self.generate_expression(&Expr::String(String::new()));
                } else {
                    let mut iter = parts.iter();
                    let mut acc = iter.next().unwrap().clone();
                    for next in iter {
                        acc = Expr::Binary {
                            left: Box::new(acc),
                            op: BinaryOp::Add,
                            right: Box::new(next.clone()),
                        };
                    }
                    self.generate_expression(&acc);
                }
            }
        }
    }

    pub(crate) fn generate_string_concat(&mut self, left: &Expr, right: &Expr) {
        self.generate_expression(left);
        arch::emit_push_temp(&mut self.output, self.arch);

        self.generate_expression(right);
        arch::emit_string_concat_call(&mut self.output, self.arch, self.ctx.stack_offset, self.os);
    }
}
