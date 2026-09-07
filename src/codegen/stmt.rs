use super::CodeGen;
use crate::ast::{Expr, Stmt};
use crate::codegen::analysis::{escape_string, is_string_expr};
use crate::codegen::arch;
use crate::codegen::context::VarType;

impl CodeGen {
    pub(crate) fn generate_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Import(_) => {}
            Stmt::Say(expr) => self.generate_say(expr),
            Stmt::Let { name, value } => match value {
                Expr::String(s) => {
                    let label = self.ctx.next_string_label();
                    self.emit_rodata_section();
                    self.output.push_str(&format!("{}:\n", label));
                    self.emit_string_directive(&escape_string(s));
                    self.output.push_str(".text\n");
                    self.ctx
                        .variables
                        .insert(name.clone(), VarType::StringLabel(label));
                }
                Expr::Array(_) => {
                    self.generate_expression(value);

                    arch::emit_allocate_var(
                        &mut self.output,
                        self.arch,
                        &mut self.ctx.stack_offset,
                    );

                    self.ctx
                        .variables
                        .insert(name.clone(), VarType::Array(self.ctx.stack_offset));
                }
                _ => {
                    let is_str = is_string_expr(value, &self.ctx.variables);
                    let is_arr = match value {
                        Expr::Identifier(ident) => {
                            matches!(self.ctx.variables.get(ident), Some(VarType::Array(_)))
                        }
                        _ => false,
                    };
                    self.generate_expression(value);

                    arch::emit_allocate_var(
                        &mut self.output,
                        self.arch,
                        &mut self.ctx.stack_offset,
                    );

                    if is_str {
                        self.ctx
                            .variables
                            .insert(name.clone(), VarType::StringOffset(self.ctx.stack_offset));
                    } else if is_arr {
                        self.ctx
                            .variables
                            .insert(name.clone(), VarType::Array(self.ctx.stack_offset));
                    } else {
                        self.ctx
                            .variables
                            .insert(name.clone(), VarType::Number(self.ctx.stack_offset));
                    }
                }
            },
            Stmt::Assign { name, value } => {
                self.generate_expression(value);

                if let Some(var_type) = self.ctx.variables.get(name).cloned() {
                    match var_type {
                        VarType::Number(offset)
                        | VarType::StringOffset(offset)
                        | VarType::Array(offset) => {
                            arch::emit_store_var(
                                &mut self.output,
                                self.arch,
                                offset,
                                self.ctx.stack_offset,
                            );
                        }
                        VarType::StringLabel(_) => {}
                    }
                }
            }
            Stmt::IndexAssign {
                array,
                index,
                value,
            } => {
                self.generate_expression(array);
                arch::emit_push_temp(&mut self.output, self.arch);

                self.generate_expression(index);
                arch::emit_push_temp(&mut self.output, self.arch);

                self.generate_expression(value);
                arch::emit_array_set(&mut self.output, self.arch);
            }
            Stmt::Expr(expr) => {
                self.generate_expression(expr);
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let else_label = self.ctx.next_label();
                let end_label = self.ctx.next_label();

                self.generate_expression(condition);

                let target_label = if else_block.is_some() {
                    &else_label
                } else {
                    &end_label
                };
                arch::emit_jump_if_zero(&mut self.output, self.arch, target_label);

                for s in then_block {
                    self.generate_statement(s);
                }

                if let Some(else_stmts) = else_block {
                    arch::emit_jump(&mut self.output, self.arch, &end_label);
                    self.output.push_str(&format!("{}:\n", else_label));

                    for s in else_stmts {
                        self.generate_statement(s);
                    }
                }

                self.output.push_str(&format!("{}:\n", end_label));
            }
            Stmt::While { condition, body } => {
                let start_label = self.ctx.next_label();
                let end_label = self.ctx.next_label();

                self.ctx.push_loop(start_label.clone(), end_label.clone());

                self.output.push_str(&format!("{}:\n", start_label));
                self.generate_expression(condition);
                arch::emit_jump_if_zero(&mut self.output, self.arch, &end_label);

                for s in body {
                    self.generate_statement(s);
                }

                arch::emit_jump(&mut self.output, self.arch, &start_label);
                self.output.push_str(&format!("{}:\n", end_label));

                self.ctx.pop_loop();
            }
            Stmt::Repeat { body } => {
                let start_label = self.ctx.next_label();
                let end_label = self.ctx.next_label();

                self.ctx.push_loop(start_label.clone(), end_label.clone());

                self.output.push_str(&format!("{}:\n", start_label));

                for s in body {
                    self.generate_statement(s);
                }

                arch::emit_jump(&mut self.output, self.arch, &start_label);
                self.output.push_str(&format!("{}:\n", end_label));

                self.ctx.pop_loop();
            }
            Stmt::For {
                var,
                start,
                end,
                body,
            } => {
                self.generate_expression(start);

                let var_offset = match self.ctx.variables.get(var) {
                    Some(VarType::Number(offset)) => {
                        let off = *offset;
                        arch::emit_store_var(
                            &mut self.output,
                            self.arch,
                            off,
                            self.ctx.stack_offset,
                        );
                        off
                    }
                    _ => {
                        arch::emit_allocate_var(
                            &mut self.output,
                            self.arch,
                            &mut self.ctx.stack_offset,
                        );
                        self.ctx
                            .variables
                            .insert(var.clone(), VarType::Number(self.ctx.stack_offset));
                        self.ctx.stack_offset
                    }
                };

                let start_label = self.ctx.next_label();
                let step_label = self.ctx.next_label();
                let end_label = self.ctx.next_label();

                self.ctx.push_loop(step_label.clone(), end_label.clone());

                self.output.push_str(&format!("{}:\n", start_label));
                arch::emit_load_var(
                    &mut self.output,
                    self.arch,
                    var_offset,
                    self.ctx.stack_offset,
                );
                arch::emit_push_temp(&mut self.output, self.arch);

                self.generate_expression(end);
                arch::emit_compare_and_jump_if_greater(&mut self.output, self.arch, &end_label);

                for s in body {
                    self.generate_statement(s);
                }

                self.output.push_str(&format!("{}:\n", step_label));
                arch::emit_increment_var(
                    &mut self.output,
                    self.arch,
                    var_offset,
                    self.ctx.stack_offset,
                    &start_label,
                );
                self.output.push_str(&format!("{}:\n", end_label));

                self.ctx.pop_loop();
            }
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
            } => {
                let catch_label = self.ctx.next_label();
                let end_label = self.ctx.next_label();
                let saved_stack_offset = self.ctx.stack_offset;
                let saved_variables = self.ctx.variables.clone();

                arch::emit_try_begin(&mut self.output, self.arch, &catch_label, self.os);

                for s in try_block {
                    self.generate_statement(s);
                }

                let try_delta = self.ctx.stack_offset - saved_stack_offset;
                arch::emit_try_end(&mut self.output, self.arch, &end_label, try_delta, self.os);

                // At catch entry, runtime SP has been restored to saved_stack_offset.
                self.ctx.stack_offset = saved_stack_offset;
                self.ctx.variables = saved_variables.clone();
                arch::emit_catch_begin(&mut self.output, self.arch, &catch_label);

                if let Some(name) = catch_var {
                    arch::emit_catch_load_err(&mut self.output, self.arch, self.os);
                    arch::emit_allocate_var(
                        &mut self.output,
                        self.arch,
                        &mut self.ctx.stack_offset,
                    );
                    self.ctx
                        .variables
                        .insert(name.clone(), VarType::StringOffset(self.ctx.stack_offset));
                }

                for s in catch_block {
                    self.generate_statement(s);
                }

                let catch_delta = self.ctx.stack_offset - saved_stack_offset;
                arch::emit_catch_end(&mut self.output, self.arch, catch_delta);

                self.ctx.stack_offset = saved_stack_offset;
                self.ctx.variables = saved_variables;
                self.output.push_str(&format!("{}:\n", end_label));
            }
            Stmt::Function { .. } => {}
        }
    }
}
