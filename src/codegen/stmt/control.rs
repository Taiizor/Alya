use super::CodeGen;
use crate::ast::{Expr, Stmt};
use crate::codegen::analysis::{is_float_array, is_string_array, is_string_expr};
use crate::codegen::arch;
use crate::codegen::context::VarType;

impl CodeGen {
    pub(super) fn generate_if(
        &mut self,
        condition: &Expr,
        then_block: &[Stmt],
        else_block: Option<&[Stmt]>,
    ) {
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

    pub(super) fn generate_while(&mut self, condition: &Expr, body: &[Stmt]) {
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

    pub(super) fn generate_repeat(&mut self, body: &[Stmt]) {
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

    pub(super) fn generate_for(&mut self, var: &str, start: &Expr, end: &Expr, body: &[Stmt]) {
        let var = var.to_string();
        self.generate_expression(start);

        let var_offset = match self.ctx.variables.get(&var) {
            Some(VarType::Number(offset)) => {
                let off = *offset;
                arch::emit_store_var(&mut self.output, self.arch, off, self.ctx.stack_offset);
                off
            }
            _ => {
                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
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

    pub(super) fn generate_for_each(&mut self, var: &str, iterable: &Expr, body: &[Stmt]) {
        let var = var.to_string();
        self.generate_expression(iterable);
        arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
        let arr_offset = self.ctx.stack_offset;

        arch::emit_load_num(&mut self.output, self.arch, 0);
        arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
        let idx_offset = self.ctx.stack_offset;

        let is_str = is_string_array(iterable, &self.ctx.variables);
        let is_flt = is_float_array(iterable, &self.ctx.variables);

        let var_offset = match self.ctx.variables.get(&var) {
            Some(
                VarType::Number(offset) | VarType::Float(offset) | VarType::StringOffset(offset),
            ) => {
                let off = *offset;
                let var_type = if is_str {
                    VarType::StringOffset(off)
                } else if is_flt {
                    VarType::Float(off)
                } else {
                    VarType::Number(off)
                };
                self.ctx.variables.insert(var.clone(), var_type);
                off
            }
            _ => {
                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
                let off = self.ctx.stack_offset;
                let var_type = if is_str {
                    VarType::StringOffset(off)
                } else if is_flt {
                    VarType::Float(off)
                } else {
                    VarType::Number(off)
                };
                self.ctx.variables.insert(var.clone(), var_type);
                off
            }
        };

        let start_label = self.ctx.next_label();
        let step_label = self.ctx.next_label();
        let end_label = self.ctx.next_label();

        self.ctx.push_loop(step_label.clone(), end_label.clone());

        self.output.push_str(&format!("{}:\n", start_label));
        arch::emit_for_each_load_element(
            &mut self.output,
            self.arch,
            arr_offset,
            idx_offset,
            var_offset,
            &end_label,
        );

        for s in body {
            self.generate_statement(s);
        }

        self.output.push_str(&format!("{}:\n", step_label));
        arch::emit_increment_var(
            &mut self.output,
            self.arch,
            idx_offset,
            self.ctx.stack_offset,
            &start_label,
        );
        self.output.push_str(&format!("{}:\n", end_label));

        self.ctx.pop_loop();
    }

    pub(super) fn generate_throw(&mut self, opt_expr: Option<&Expr>) {
        if let Some(expr) = opt_expr {
            if is_string_expr(expr, &self.ctx.variables) {
                self.generate_expression(expr);
                arch::emit_push_temp(&mut self.output, self.arch);
            } else {
                self.generate_expression(expr);
                arch::emit_push_temp(&mut self.output, self.arch);
                arch::emit_function_call(
                    &mut self.output,
                    self.arch,
                    "str",
                    1,
                    self.ctx.stack_offset,
                    self.os,
                );
                arch::emit_push_temp(&mut self.output, self.arch);
            }
            arch::emit_function_call(
                &mut self.output,
                self.arch,
                "throw",
                1,
                self.ctx.stack_offset,
                self.os,
            );
        } else {
            arch::emit_function_call(
                &mut self.output,
                self.arch,
                "rethrow",
                0,
                self.ctx.stack_offset,
                self.os,
            );
        }
    }

    pub(super) fn generate_try_catch(
        &mut self,
        try_block: &[Stmt],
        catch_var: Option<&str>,
        catch_block: &[Stmt],
        finally_block: Option<&[Stmt]>,
    ) {
        let catch_label = self.ctx.next_label();
        let end_label = self.ctx.next_label();
        let saved_stack_offset = self.ctx.stack_offset;
        let saved_variables = self.ctx.variables.clone();

        if let Some(finally_stmts) = finally_block {
            let finally_normal_label = self.ctx.next_label();
            let finally_rethrow_label = self.ctx.next_label();

            if !catch_block.is_empty() || catch_var.is_some() {
                // Try block with catch
                arch::emit_try_begin(&mut self.output, self.arch, &catch_label, self.os);
                for s in try_block {
                    self.generate_statement(s);
                }
                let try_delta = self.ctx.stack_offset - saved_stack_offset;
                arch::emit_try_end(
                    &mut self.output,
                    self.arch,
                    &finally_normal_label,
                    try_delta,
                    self.os,
                );

                // Catch block
                self.ctx.stack_offset = saved_stack_offset;
                self.ctx.variables = saved_variables.clone();
                arch::emit_catch_begin(&mut self.output, self.arch, &catch_label);

                // Temporary try handler so that errors inside catch run finally and rethrow
                arch::emit_try_begin(&mut self.output, self.arch, &finally_rethrow_label, self.os);

                if let Some(name) = catch_var {
                    let name = name.to_string();
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
                arch::emit_try_end(
                    &mut self.output,
                    self.arch,
                    &finally_normal_label,
                    catch_delta,
                    self.os,
                );
            } else {
                // Try block WITHOUT catch (only finally)
                arch::emit_try_begin(&mut self.output, self.arch, &finally_rethrow_label, self.os);
                for s in try_block {
                    self.generate_statement(s);
                }
                let try_delta = self.ctx.stack_offset - saved_stack_offset;
                arch::emit_try_end(
                    &mut self.output,
                    self.arch,
                    &finally_normal_label,
                    try_delta,
                    self.os,
                );
            }

            // Normal path to finally
            self.output
                .push_str(&format!("{}:\n", finally_normal_label));
            self.ctx.stack_offset = saved_stack_offset;
            self.ctx.variables = saved_variables.clone();
            for s in finally_stmts {
                self.generate_statement(s);
            }
            let finally_delta = self.ctx.stack_offset - saved_stack_offset;
            arch::emit_catch_end(&mut self.output, self.arch, finally_delta);
            arch::emit_jump(&mut self.output, self.arch, &end_label);

            // Error path to finally (runs finally and rethrows)
            self.output
                .push_str(&format!("{}:\n", finally_rethrow_label));
            self.ctx.stack_offset = saved_stack_offset;
            self.ctx.variables = saved_variables.clone();
            for s in finally_stmts {
                self.generate_statement(s);
            }
            let finally_rethrow_delta = self.ctx.stack_offset - saved_stack_offset;
            arch::emit_catch_end(&mut self.output, self.arch, finally_rethrow_delta);
            arch::emit_function_call(
                &mut self.output,
                self.arch,
                "rethrow",
                0,
                self.ctx.stack_offset,
                self.os,
            );

            self.ctx.stack_offset = saved_stack_offset;
            self.ctx.variables = saved_variables;
            self.output.push_str(&format!("{}:\n", end_label));
        } else {
            // Existing try-catch without finally
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
                let name = name.to_string();
                arch::emit_catch_load_err(&mut self.output, self.arch, self.os);
                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
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
    }
}
