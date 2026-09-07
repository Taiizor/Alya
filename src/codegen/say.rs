use super::CodeGen;
use crate::ast::{BinaryOp, Expr};
use crate::codegen::analysis::{escape_string, is_string_expr};
use crate::codegen::arch;
use crate::codegen::context::VarType;
use crate::codegen::target::Architecture;

impl CodeGen {
    pub(crate) fn generate_say(&mut self, expr: &Expr) {
        match expr {
            Expr::String(s) => {
                let label = self.ctx.next_string_label();
                self.emit_rodata_section();
                self.output.push_str(&format!("{}:\n", label));
                self.emit_string_directive(&format!("{}\\n", escape_string(s)));
                self.output.push_str(".text\n");

                arch::emit_say_str_lit(
                    &mut self.output,
                    self.arch,
                    &label,
                    self.ctx.stack_offset,
                    self.os,
                );
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
                self.emit_rodata_section();
                self.output.push_str(&format!("{}:\n", fmt_label));
                self.emit_string_directive(&format_str);
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

                arch::emit_say_interpolated(
                    &mut self.output,
                    self.arch,
                    &fmt_label,
                    exprs.len(),
                    self.ctx.stack_offset,
                    self.os,
                );
                self.output.push('\n');
            }
            Expr::Binary { left, op, right } => {
                if matches!(op, BinaryOp::Add)
                    && (is_string_expr(left, &self.ctx.variables)
                        || is_string_expr(right, &self.ctx.variables))
                {
                    self.generate_string_concat(left, right);
                    let fmt_label = self.ctx.next_string_label();
                    self.emit_rodata_section();
                    self.output.push_str(&format!("{}:\n", fmt_label));
                    self.emit_string_directive("%s\\n");
                    self.output.push_str(".text\n");

                    arch::emit_say_acc(
                        &mut self.output,
                        self.arch,
                        &fmt_label,
                        self.ctx.stack_offset,
                        self.os,
                    );
                    self.output.push('\n');
                    return;
                }
                self.generate_expression(expr);

                let fmt_label = self.ctx.next_string_label();
                self.emit_rodata_section();
                self.output.push_str(&format!("{}:\n", fmt_label));
                self.emit_string_directive("%ld\\n");
                self.output.push_str(".text\n");

                arch::emit_say_acc(
                    &mut self.output,
                    self.arch,
                    &fmt_label,
                    self.ctx.stack_offset,
                    self.os,
                );
                self.output.push('\n');
            }
            Expr::Identifier(name) => {
                if let Some(var_type) = self.ctx.variables.get(name).cloned() {
                    match var_type {
                        VarType::StringLabel(label) => {
                            let fmt_label = self.ctx.next_string_label();
                            self.emit_rodata_section();
                            self.output.push_str(&format!("{}:\n", fmt_label));
                            self.emit_string_directive("%s\\n");
                            self.output.push_str(".text\n");

                            arch::emit_say_str(
                                &mut self.output,
                                self.arch,
                                &label,
                                &fmt_label,
                                self.ctx.stack_offset,
                                self.os,
                            );
                            self.output.push('\n');
                        }
                        VarType::StringOffset(offset) => {
                            let fmt_label = self.ctx.next_string_label();
                            self.emit_rodata_section();
                            self.output.push_str(&format!("{}:\n", fmt_label));
                            self.emit_string_directive("%s\\n");
                            self.output.push_str(".text\n");

                            arch::emit_say_offset(
                                &mut self.output,
                                self.arch,
                                offset,
                                self.ctx.stack_offset,
                                &fmt_label,
                                self.os,
                            );
                            self.output.push('\n');
                        }
                        VarType::Number(offset) => {
                            let fmt_label = self.ctx.next_string_label();
                            self.emit_rodata_section();
                            self.output.push_str(&format!("{}:\n", fmt_label));
                            self.emit_string_directive("%ld\\n");
                            self.output.push_str(".text\n");

                            arch::emit_say_offset(
                                &mut self.output,
                                self.arch,
                                offset,
                                self.ctx.stack_offset,
                                &fmt_label,
                                self.os,
                            );
                            self.output.push('\n');
                        }
                    }
                }
            }
            Expr::Number(n) => {
                let fmt_label = self.ctx.next_string_label();
                self.emit_rodata_section();
                self.output.push_str(&format!("{}:\n", fmt_label));
                self.emit_string_directive("%ld\\n");
                self.output.push_str(".text\n");

                arch::emit_say_num_const(
                    &mut self.output,
                    self.arch,
                    *n as i64,
                    &fmt_label,
                    self.ctx.stack_offset,
                    self.os,
                );
                self.output.push('\n');
            }
            _ => {
                let is_str = is_string_expr(expr, &self.ctx.variables);
                self.generate_expression(expr);

                let fmt_label = self.ctx.next_string_label();
                self.emit_rodata_section();
                self.output.push_str(&format!("{}:\n", fmt_label));
                if is_str {
                    self.emit_string_directive("%s\\n");
                } else {
                    self.emit_string_directive("%ld\\n");
                }
                self.output.push_str(".text\n");

                arch::emit_say_acc(
                    &mut self.output,
                    self.arch,
                    &fmt_label,
                    self.ctx.stack_offset,
                    self.os,
                );
                self.output.push('\n');
            }
        }
    }
}
