pub mod analysis;
pub mod arch;
pub mod context;
pub mod runtime;
pub mod target;
#[cfg(test)]
mod tests;

pub use target::{Architecture, OperatingSystem};

use crate::ast::*;
use analysis::{escape_string, infer_param_is_string, is_string_expr};
use context::{CodeGenContext, VarType};

pub struct CodeGen {
    arch: Architecture,
    os: OperatingSystem,
    output: String,
    ctx: CodeGenContext,
}

impl CodeGen {
    pub fn new(arch: Architecture, os: OperatingSystem) -> Self {
        Self {
            arch,
            os,
            output: String::new(),
            ctx: CodeGenContext::new(),
        }
    }

    pub fn generate_program(&mut self, program: &Program) {
        let mut functions = Vec::new();
        let mut top_level = Vec::new();

        for stmt in &program.statements {
            match stmt {
                Stmt::Function { .. } => functions.push(stmt),
                _ => top_level.push(stmt),
            }
        }

        arch::emit_header(&mut self.output, self.arch);

        for stmt in top_level {
            self.generate_statement(stmt);
        }

        arch::emit_footer(&mut self.output, self.arch);

        for func in functions {
            if let Stmt::Function { name, params, body } = func {
                self.generate_function(name, params, body, program);
            }
        }

        runtime::emit_runtime(&mut self.output, self.arch, self.os);
    }

    fn generate_function(&mut self, name: &str, params: &[String], body: &[Stmt], program: &Program) {
        let saved = self.ctx.enter_function();

        arch::emit_function_prologue(&mut self.output, self.arch, name);

        for (i, param) in params.iter().enumerate() {
            arch::emit_function_param_push(&mut self.output, self.arch, i, &mut self.ctx.stack_offset, self.os);

            let is_str = infer_param_is_string(name, i, program);
            if is_str {
                self.ctx.variables.insert(param.clone(), VarType::StringOffset(self.ctx.stack_offset));
            } else {
                self.ctx.variables.insert(param.clone(), VarType::Number(self.ctx.stack_offset));
            }
        }

        for stmt in body {
            self.generate_statement(stmt);
        }

        arch::emit_function_epilogue(&mut self.output, self.arch);

        self.ctx.exit_function(saved);
    }

    fn generate_statement(&mut self, stmt: &Stmt) {
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
                        for e in exprs.iter().rev() {
                            self.generate_expression(e);
                            arch::emit_push_temp(&mut self.output, self.arch);
                        }
                    }
                    _ => {
                        for e in &exprs {
                            self.generate_expression(e);
                            arch::emit_push_temp(&mut self.output, self.arch);
                        }
                    }
                }

                arch::emit_say_interpolated(&mut self.output, self.arch, &fmt_label, exprs.len(), self.ctx.stack_offset, self.os);
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

    fn generate_expression(&mut self, expr: &Expr) {
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
                if matches!(op, BinaryOp::Add) && (is_string_expr(left, &self.ctx.variables) || is_string_expr(right, &self.ctx.variables)) {
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

    fn generate_string_concat(&mut self, left: &Expr, right: &Expr) {
        self.generate_expression(left);
        arch::emit_push_temp(&mut self.output, self.arch);

        self.generate_expression(right);
        arch::emit_string_concat_call(&mut self.output, self.arch, self.ctx.stack_offset, self.os);
    }
}

pub fn generate(program: &Program, arch: Architecture, os: OperatingSystem) -> String {
    let mut codegen = CodeGen::new(arch, os);
    codegen.generate_program(program);
    codegen.output
}
