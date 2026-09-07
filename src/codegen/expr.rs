use super::CodeGen;
use crate::ast::{BinaryOp, Expr};
use crate::codegen::analysis::{escape_string, is_array_expr, is_float_expr, is_string_expr};
use crate::codegen::arch;
use crate::codegen::context::VarType;
use crate::codegen::target::Architecture;

impl CodeGen {
    pub(crate) fn generate_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                if n.fract() != 0.0 {
                    arch::emit_load_float(&mut self.output, self.arch, *n);
                } else {
                    arch::emit_load_num(&mut self.output, self.arch, *n as i64);
                }
            }
            Expr::Float(n) => {
                arch::emit_load_float(&mut self.output, self.arch, *n);
            }
            Expr::String(s) => {
                let label = self.ctx.next_string_label();
                self.emit_rodata_section();
                self.output.push_str(&format!("{}:\n", label));
                self.emit_string_directive(&escape_string(s));
                self.output.push_str(".text\n");

                arch::emit_load_str_label(&mut self.output, self.arch, &label, self.os);
            }
            Expr::Identifier(name) => {
                if let Some(var_type) = self.ctx.variables.get(name).cloned() {
                    match var_type {
                        VarType::Number(offset)
                        | VarType::Float(offset)
                        | VarType::StringOffset(offset)
                        | VarType::Array(offset)
                        | VarType::Struct { offset, .. } => {
                            arch::emit_load_var(
                                &mut self.output,
                                self.arch,
                                offset,
                                self.ctx.stack_offset,
                            );
                            if matches!(var_type, VarType::Float(_)) {
                                match self.arch {
                                    Architecture::X64 => {
                                        self.output.push_str("    movq %rax, %xmm0\n");
                                    }
                                    Architecture::ARM64 => {
                                        self.output.push_str("    fmov d0, x0\n");
                                    }
                                    Architecture::X86 => {}
                                }
                            }
                        }
                        VarType::StringLabel(label) => {
                            arch::emit_load_str_label(&mut self.output, self.arch, &label, self.os);
                        }
                    }
                }
            }
            Expr::Binary { left, op, right } => {
                if matches!(op, BinaryOp::Add)
                    && (is_string_expr(left, &self.ctx.variables)
                        || is_string_expr(right, &self.ctx.variables))
                {
                    self.generate_string_concat(left, right);
                    return;
                }

                let left_is_float = is_float_expr(left, &self.ctx.variables);
                let right_is_float = is_float_expr(right, &self.ctx.variables);
                let is_float = left_is_float || right_is_float;

                if is_float {
                    self.generate_expression(left);
                    if !left_is_float {
                        arch::emit_int_to_float(&mut self.output, self.arch);
                    }
                    if matches!(self.arch, Architecture::X86) {
                        self.output
                            .push_str("    sub $8, %esp\n    movsd %xmm0, (%esp)\n");
                    } else {
                        arch::emit_push_temp(&mut self.output, self.arch);
                    }

                    self.generate_expression(right);
                    if !right_is_float {
                        arch::emit_int_to_float(&mut self.output, self.arch);
                    }

                    arch::emit_float_binary_op(&mut self.output, self.arch, *op);
                } else {
                    self.generate_expression(left);
                    arch::emit_push_temp(&mut self.output, self.arch);

                    self.generate_expression(right);
                    arch::emit_binary_op(&mut self.output, self.arch, *op);
                }
            }
            Expr::Unary { op, expr } => {
                let is_float = is_float_expr(expr, &self.ctx.variables);
                self.generate_expression(expr);
                if is_float {
                    arch::emit_float_unary_op(&mut self.output, self.arch, *op);
                } else {
                    arch::emit_unary_op(&mut self.output, self.arch, *op);
                }
            }
            Expr::Call { name, args } => {
                if let Some(sdef) = self.ctx.structs.get(name).cloned() {
                    let desc_label = format!("alya_struct_desc_{}", name);
                    arch::emit_struct_new(
                        &mut self.output,
                        self.arch,
                        &desc_label,
                        sdef.fields.len(),
                        self.ctx.stack_offset,
                        self.os,
                    );
                    arch::emit_push_temp(&mut self.output, self.arch);

                    for (i, arg) in args.iter().enumerate() {
                        self.generate_expression(arg);
                        arch::emit_struct_field_set_imm(&mut self.output, self.arch, i);
                    }

                    arch::emit_pop_temp(&mut self.output, self.arch);
                    return;
                }

                if name == "len" && args.len() == 1 && is_array_expr(&args[0], &self.ctx.variables)
                {
                    self.generate_expression(&args[0]);
                    arch::emit_array_len(&mut self.output, self.arch);
                    return;
                }

                if name == "push" && args.len() == 2 {
                    self.generate_expression(&args[0]);
                    arch::emit_push_temp(&mut self.output, self.arch);
                    self.generate_expression(&args[1]);
                    arch::emit_array_push(
                        &mut self.output,
                        self.arch,
                        self.ctx.stack_offset,
                        self.os,
                    );
                    return;
                }

                if name == "pop" && args.len() == 1 {
                    self.generate_expression(&args[0]);
                    arch::emit_array_pop(
                        &mut self.output,
                        self.arch,
                        self.ctx.stack_offset,
                        self.os,
                    );
                    return;
                }

                if name == "float" && args.len() == 1 {
                    self.generate_expression(&args[0]);
                    arch::emit_int_to_float(&mut self.output, self.arch);
                    return;
                }

                if name == "int" && args.len() == 1 {
                    self.generate_expression(&args[0]);
                    arch::emit_float_to_int(&mut self.output, self.arch);
                    return;
                }

                let (call_name, actual_args): (&str, Vec<Expr>) =
                    if (name == "substring" || name == "substr") && args.len() == 2 {
                        (
                            "substring",
                            vec![args[0].clone(), args[1].clone(), Expr::Number(-1.0)],
                        )
                    } else if name == "substr" {
                        ("substring", args.clone())
                    } else {
                        (name.as_str(), args.clone())
                    };

                match self.arch {
                    Architecture::X86 => {
                        for arg in actual_args.iter().rev() {
                            self.generate_expression(arg);
                            arch::emit_push_temp(&mut self.output, self.arch);
                        }
                    }
                    _ => {
                        for arg in &actual_args {
                            self.generate_expression(arg);
                            arch::emit_push_temp(&mut self.output, self.arch);
                        }
                    }
                }
                arch::emit_function_call(
                    &mut self.output,
                    self.arch,
                    call_name,
                    actual_args.len(),
                    self.ctx.stack_offset,
                    self.os,
                );
            }
            Expr::StructInit { name, fields } => {
                let field_count = self
                    .ctx
                    .structs
                    .get(name)
                    .map(|s| s.fields.len())
                    .unwrap_or(fields.len());
                let desc_label = format!("alya_struct_desc_{}", name);
                arch::emit_struct_new(
                    &mut self.output,
                    self.arch,
                    &desc_label,
                    field_count,
                    self.ctx.stack_offset,
                    self.os,
                );
                arch::emit_push_temp(&mut self.output, self.arch);

                if let Some(sdef) = self.ctx.structs.get(name).cloned() {
                    for (i, fname) in sdef.fields.iter().enumerate() {
                        if let Some((_, fval)) = fields.iter().find(|(k, _)| k == fname) {
                            self.generate_expression(fval);
                        } else {
                            self.generate_expression(&Expr::Number(0.0));
                        }
                        arch::emit_struct_field_set_imm(&mut self.output, self.arch, i);
                    }
                } else {
                    for (i, (_, fval)) in fields.iter().enumerate() {
                        self.generate_expression(fval);
                        arch::emit_struct_field_set_imm(&mut self.output, self.arch, i);
                    }
                }

                arch::emit_pop_temp(&mut self.output, self.arch);
            }
            Expr::FieldAccess { object, field } => {
                let mut field_idx = 0;
                let mut struct_found = false;

                if let Expr::Identifier(obj_name) = &**object {
                    if let Some(VarType::Struct { struct_name, .. }) =
                        self.ctx.variables.get(obj_name)
                    {
                        if let Some(sdef) = self.ctx.structs.get(struct_name) {
                            if let Some(idx) = sdef.fields.iter().position(|f| f == field) {
                                field_idx = idx;
                                struct_found = true;
                            }
                        }
                    }
                }

                if !struct_found {
                    for sdef in self.ctx.structs.values() {
                        if let Some(idx) = sdef.fields.iter().position(|f| f == field) {
                            field_idx = idx;
                            break;
                        }
                    }
                }

                self.generate_expression(object);
                arch::emit_struct_field_get(&mut self.output, self.arch, field_idx);
            }
            Expr::Array(elements) => {
                arch::emit_array_new(
                    &mut self.output,
                    self.arch,
                    elements.len(),
                    self.ctx.stack_offset,
                    self.os,
                );
                arch::emit_push_temp(&mut self.output, self.arch);

                for (i, elem) in elements.iter().enumerate() {
                    self.generate_expression(elem);
                    arch::emit_array_set_imm(&mut self.output, self.arch, i);
                }

                arch::emit_pop_temp(&mut self.output, self.arch);
            }
            Expr::Index { array, index } => {
                self.generate_expression(array);
                arch::emit_push_temp(&mut self.output, self.arch);

                self.generate_expression(index);
                arch::emit_array_get(&mut self.output, self.arch);
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
