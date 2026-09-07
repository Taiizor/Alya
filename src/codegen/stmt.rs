use crate::ast::{BinaryOp, Expr, Stmt};
use crate::codegen::analysis::{escape_string, is_string_expr};
use crate::codegen::arch;
use crate::codegen::context::VarType;
use crate::codegen::target::Architecture;
use super::CodeGen;

impl CodeGen {
    pub(crate) fn generate_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Say(expr) => self.generate_say(expr),
            Stmt::Let { name, value } => {
                match value {
                    Expr::String(s) => {
                        let label = self.ctx.next_string_label();
                        self.output.push_str(".section .rodata\n");
                        self.output.push_str(&format!("{}:\n", label));
                        self.output.push_str(&format!("    .string \"{}\"\n", escape_string(s)));
                        self.output.push_str(".text\n");
                        self.ctx.variables.insert(name.clone(), VarType::StringLabel(label));
                    }
                    _ => {
                        let is_str = is_string_expr(value, &self.ctx.variables);
                        self.generate_expression(value);

                        arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);

                        if is_str {
                            self.ctx.variables.insert(name.clone(), VarType::StringOffset(self.ctx.stack_offset));
                        } else {
                            self.ctx.variables.insert(name.clone(), VarType::Number(self.ctx.stack_offset));
                        }
                    }
                }
            }
            Stmt::Assign { name, value } => {
                self.generate_expression(value);

                if let Some(var_type) = self.ctx.variables.get(name).cloned() {
                    match var_type {
                        VarType::Number(offset) | VarType::StringOffset(offset) => {
                            arch::emit_store_var(&mut self.output, self.arch, offset, self.ctx.stack_offset);
                        }
                        VarType::StringLabel(_) => {}
                    }
                }
            }
            Stmt::Expr(expr) => {
                self.generate_expression(expr);
            }
            Stmt::If { condition, then_block, else_block } => {
                let else_label = self.ctx.next_label();
                let end_label = self.ctx.next_label();

                self.generate_expression(condition);

                let target_label = if else_block.is_some() { &else_label } else { &end_label };
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
            Stmt::For { var, start, end, body } => {
                self.generate_expression(start);

                let var_offset = match self.ctx.variables.get(var) {
                    Some(VarType::Number(offset)) => {
                        let off = *offset;
                        arch::emit_store_var(&mut self.output, self.arch, off, self.ctx.stack_offset);
                        off
                    }
                    _ => {
                        arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
                        self.ctx.variables.insert(var.clone(), VarType::Number(self.ctx.stack_offset));
                        self.ctx.stack_offset
                    }
                };

                let start_label = self.ctx.next_label();
                let step_label = self.ctx.next_label();
                let end_label = self.ctx.next_label();

                self.ctx.push_loop(step_label.clone(), end_label.clone());

                self.output.push_str(&format!("{}:\n", start_label));
                arch::emit_load_var(&mut self.output, self.arch, var_offset, self.ctx.stack_offset);
                arch::emit_push_temp(&mut self.output, self.arch);

                self.generate_expression(end);
                arch::emit_compare_and_jump_if_greater(&mut self.output, self.arch, &end_label);

                for s in body {
                    self.generate_statement(s);
                }

                self.output.push_str(&format!("{}:\n", step_label));
                arch::emit_increment_var(&mut self.output, self.arch, var_offset, self.ctx.stack_offset, &start_label);
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
            Stmt::TryCatch { try_block, catch_var, catch_block } => {
                let catch_label = self.ctx.next_label();
                let end_label = self.ctx.next_label();
                let saved_stack_offset = self.ctx.stack_offset;
                let saved_variables = self.ctx.variables.clone();

                arch::emit_try_begin(&mut self.output, self.arch, &catch_label);

                for s in try_block {
                    self.generate_statement(s);
                }

                let try_delta = self.ctx.stack_offset - saved_stack_offset;
                arch::emit_try_end(&mut self.output, self.arch, &end_label, try_delta);

                // At catch entry, runtime SP has been restored to saved_stack_offset.
                self.ctx.stack_offset = saved_stack_offset;
                self.ctx.variables = saved_variables.clone();
                arch::emit_catch_begin(&mut self.output, self.arch, &catch_label);

                if let Some(name) = catch_var {
                    arch::emit_catch_load_err(&mut self.output, self.arch);
                    arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
                    self.ctx.variables.insert(name.clone(), VarType::StringOffset(self.ctx.stack_offset));
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

    fn generate_say(&mut self, expr: &Expr) {
        match expr {
            Expr::String(s) => {
                let label = self.ctx.next_string_label();
                self.output.push_str(".section .rodata\n");
                self.output.push_str(&format!("{}:\n", label));
                self.output.push_str(&format!("    .string \"{}\\n\"\n", escape_string(s)));
                self.output.push_str(".text\n");

                arch::emit_say_str_lit(&mut self.output, self.arch, &label, self.ctx.stack_offset, self.os);
                self.output.push('\n');
            }
            Expr::InterpolatedString(parts) => {
                let mut format_str = String::new();
                let mut exprs = Vec::new();

                for part in parts {
                    match part {
                        Expr::String(s) => {
                            format_str.push_str(&escape_string(s).replace('%', "%%"));
                        }
                        _ => {
                            if is_string_expr(part, &self.ctx.variables) {
                                format_str.push_str("%s");
                            } else {
                                format_str.push_str("%ld");
                            }
                            exprs.push(part);
                        }
                    }
                }
                format_str.push_str("\\n");

                let fmt_label = self.ctx.next_string_label();
                self.output.push_str(".section .rodata\n");
                self.output.push_str(&format!("{}:\n", fmt_label));
                self.output.push_str(&format!("    .string \"{}\"\n", format_str));
                self.output.push_str(".text\n");

                match self.arch {
                    Architecture::X86 => {
                        for expr in exprs.iter().rev() {
                            self.generate_expression(expr);
                            arch::emit_push_temp(&mut self.output, self.arch);
                        }
                    }
                    _ => {
                        for expr in exprs.iter() {
                            self.generate_expression(expr);
                            arch::emit_push_temp(&mut self.output, self.arch);
                        }
                    }
                }

                arch::emit_say_interpolated(&mut self.output, self.arch, &fmt_label, exprs.len(), self.ctx.stack_offset, self.os);
                self.output.push('\n');
            }
            Expr::Binary { left, op, right } => {
                if matches!(op, BinaryOp::Add)
                    && (is_string_expr(left, &self.ctx.variables) || is_string_expr(right, &self.ctx.variables))
                {
                    self.generate_string_concat(left, right);
                    let fmt_label = self.ctx.next_string_label();
                    self.output.push_str(".section .rodata\n");
                    self.output.push_str(&format!("{}:\n", fmt_label));
                    self.output.push_str("    .string \"%s\\n\"\n");
                    self.output.push_str(".text\n");

                    arch::emit_say_acc(&mut self.output, self.arch, &fmt_label, self.ctx.stack_offset, self.os);
                    self.output.push('\n');
                    return;
                }
                self.generate_expression(expr);

                let fmt_label = self.ctx.next_string_label();
                self.output.push_str(".section .rodata\n");
                self.output.push_str(&format!("{}:\n", fmt_label));
                self.output.push_str("    .string \"%ld\\n\"\n");
                self.output.push_str(".text\n");

                arch::emit_say_acc(&mut self.output, self.arch, &fmt_label, self.ctx.stack_offset, self.os);
                self.output.push('\n');
            }
            Expr::Identifier(name) => {
                if let Some(var_type) = self.ctx.variables.get(name).cloned() {
                    match var_type {
                        VarType::StringLabel(label) => {
                            let fmt_label = self.ctx.next_string_label();
                            self.output.push_str(".section .rodata\n");
                            self.output.push_str(&format!("{}:\n", fmt_label));
                            self.output.push_str("    .string \"%s\\n\"\n");
                            self.output.push_str(".text\n");

                            arch::emit_say_str(&mut self.output, self.arch, &label, &fmt_label, self.ctx.stack_offset, self.os);
                            self.output.push('\n');
                        }
                        VarType::StringOffset(offset) => {
                            let fmt_label = self.ctx.next_string_label();
                            self.output.push_str(".section .rodata\n");
                            self.output.push_str(&format!("{}:\n", fmt_label));
                            self.output.push_str("    .string \"%s\\n\"\n");
                            self.output.push_str(".text\n");

                            arch::emit_say_offset(&mut self.output, self.arch, offset, self.ctx.stack_offset, &fmt_label, self.os);
                            self.output.push('\n');
                        }
                        VarType::Number(offset) => {
                            let fmt_label = self.ctx.next_string_label();
                            self.output.push_str(".section .rodata\n");
                            self.output.push_str(&format!("{}:\n", fmt_label));
                            self.output.push_str("    .string \"%ld\\n\"\n");
                            self.output.push_str(".text\n");

                            arch::emit_say_offset(&mut self.output, self.arch, offset, self.ctx.stack_offset, &fmt_label, self.os);
                            self.output.push('\n');
                        }
                    }
                }
            }
            Expr::Number(n) => {
                let fmt_label = self.ctx.next_string_label();
                self.output.push_str(".section .rodata\n");
                self.output.push_str(&format!("{}:\n", fmt_label));
                self.output.push_str("    .string \"%ld\\n\"\n");
                self.output.push_str(".text\n");

                arch::emit_say_num_const(&mut self.output, self.arch, *n as i64, &fmt_label, self.ctx.stack_offset, self.os);
                self.output.push('\n');
            }
            _ => {
                let is_str = is_string_expr(expr, &self.ctx.variables);
                self.generate_expression(expr);

                let fmt_label = self.ctx.next_string_label();
                self.output.push_str(".section .rodata\n");
                self.output.push_str(&format!("{}:\n", fmt_label));
                if is_str {
                    self.output.push_str("    .string \"%s\\n\"\n");
                } else {
                    self.output.push_str("    .string \"%ld\\n\"\n");
                }
                self.output.push_str(".text\n");

                arch::emit_say_acc(&mut self.output, self.arch, &fmt_label, self.ctx.stack_offset, self.os);
                self.output.push('\n');
            }
        }
    }
}
