mod assign;
mod control;

use super::CodeGen;
use crate::ast::Stmt;
use crate::codegen::arch;

impl CodeGen {
    pub(crate) fn generate_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Import(_) => {}
            Stmt::Say(expr) => self.generate_say(expr),
            Stmt::Expr(expr) => {
                self.generate_expression(expr);
            }
            Stmt::Let { name, value } => self.generate_let(name, value),
            Stmt::Assign { name, value } => self.generate_assign(name, value),
            Stmt::FieldAssign { object, field, value } => {
                self.generate_field_assign(object, field, value);
            }
            Stmt::IndexAssign { array, index, value } => {
                self.generate_index_assign(array, index, value);
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => self.generate_if(condition, then_block, else_block.as_deref()),
            Stmt::While { condition, body } => self.generate_while(condition, body),
            Stmt::Repeat { body } => self.generate_repeat(body),
            Stmt::For {
                var,
                start,
                end,
                body,
            } => self.generate_for(var, start, end, body),
            Stmt::ForEach {
                var,
                iterable,
                body,
            } => self.generate_for_each(var, iterable, body),
            Stmt::Break => {
                if let Some((_, break_label)) = self.ctx.current_loop().cloned() {
                    arch::emit_jump(&mut self.output, self.arch, &break_label);
                }
            }
            Stmt::Continue => {
                if let Some((continue_label, _)) = self.ctx.current_loop().cloned() {
                    arch::emit_jump(&mut self.output, self.arch, &continue_label);
                }
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.generate_expression(expr);
                }
                arch::emit_function_epilogue(&mut self.output, self.arch);
            }
            Stmt::TryCatch {
                try_block,
                catch_var,
                catch_block,
            } => self.generate_try_catch(try_block, catch_var.as_deref(), catch_block),
            Stmt::Function { .. } => {}
            Stmt::StructDef { name, fields } => {
                self.ctx.structs.insert(
                    name.clone(),
                    crate::codegen::context::StructDefInfo {
                        name: name.clone(),
                        fields: fields.clone(),
                    },
                );
            }
        }
    }
}
