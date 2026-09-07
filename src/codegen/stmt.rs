use super::CodeGen;
use crate::ast::{Expr, Stmt};
use crate::codegen::analysis::{
    escape_string, is_array_expr, is_float_array, is_float_expr, is_map_expr, is_string_array,
    is_string_expr,
};
use crate::codegen::arch;
use crate::codegen::context::VarType;
use crate::codegen::target::Architecture;

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
                    arch::emit_load_str_label(&mut self.output, self.arch, &label, self.os);
                    arch::emit_allocate_var(
                        &mut self.output,
                        self.arch,
                        &mut self.ctx.stack_offset,
                    );
                    self.ctx
                        .variables
                        .insert(name.clone(), VarType::StringOffset(self.ctx.stack_offset));
                }
                Expr::Array(elements) => {
                    let is_str_arr = elements
                        .first()
                        .is_some_and(|e| is_string_expr(e, &self.ctx.variables));
                    let is_flt_arr = elements
                        .first()
                        .is_some_and(|e| is_float_expr(e, &self.ctx.variables));
                    self.generate_expression(value);

                    arch::emit_allocate_var(
                        &mut self.output,
                        self.arch,
                        &mut self.ctx.stack_offset,
                    );

                    self.ctx
                        .variables
                        .insert(name.clone(), VarType::Array(self.ctx.stack_offset));
                    if is_str_arr {
                        self.ctx
                            .variables
                            .insert(format!("arr_is_str:{}", name), VarType::Number(0));
                    }
                    if is_flt_arr {
                        self.ctx
                            .variables
                            .insert(format!("arr_is_flt:{}", name), VarType::Number(0));
                    }
                }
                Expr::StructInit {
                    name: sname,
                    fields: init_fields,
                } => {
                    self.generate_expression(value);

                    arch::emit_allocate_var(
                        &mut self.output,
                        self.arch,
                        &mut self.ctx.stack_offset,
                    );

                    self.ctx.variables.insert(
                        name.clone(),
                        VarType::Struct {
                            struct_name: sname.clone(),
                            offset: self.ctx.stack_offset,
                        },
                    );

                    for (fname, fval) in init_fields {
                        let is_flt = is_float_expr(fval, &self.ctx.variables);
                        let is_str = is_string_expr(fval, &self.ctx.variables);
                        let field_key = format!("{}.{}", name, fname);
                        if is_str {
                            self.ctx
                                .variables
                                .insert(field_key, VarType::StringOffset(0));
                        } else if is_flt {
                            self.ctx.variables.insert(field_key, VarType::Float(0));
                        } else {
                            self.ctx.variables.insert(field_key, VarType::Number(0));
                        }
                    }
                }
                Expr::Call {
                    name: cname,
                    args: cargs,
                } if self.ctx.structs.contains_key(cname) => {
                    let sname = cname.clone();
                    let sfields = self
                        .ctx
                        .structs
                        .get(&sname)
                        .map(|s| s.fields.clone())
                        .unwrap_or_default();

                    self.generate_expression(value);

                    arch::emit_allocate_var(
                        &mut self.output,
                        self.arch,
                        &mut self.ctx.stack_offset,
                    );

                    self.ctx.variables.insert(
                        name.clone(),
                        VarType::Struct {
                            struct_name: sname,
                            offset: self.ctx.stack_offset,
                        },
                    );

                    for (i, arg) in cargs.iter().enumerate() {
                        if let Some(fname) = sfields.get(i) {
                            let is_flt = is_float_expr(arg, &self.ctx.variables);
                            let is_str = is_string_expr(arg, &self.ctx.variables);
                            let field_key = format!("{}.{}", name, fname);
                            if is_str {
                                self.ctx
                                    .variables
                                    .insert(field_key, VarType::StringOffset(0));
                            } else if is_flt {
                                self.ctx.variables.insert(field_key, VarType::Float(0));
                            } else {
                                self.ctx.variables.insert(field_key, VarType::Number(0));
                            }
                        }
                    }
                }
                _ => {
                    let is_str = is_string_expr(value, &self.ctx.variables);
                    let is_arr = is_array_expr(value, &self.ctx.variables);
                    let is_struct = match value {
                        Expr::Identifier(ident) => {
                            if let Some(VarType::Struct { struct_name, .. }) =
                                self.ctx.variables.get(ident)
                            {
                                Some(struct_name.clone())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let is_flt = is_float_expr(value, &self.ctx.variables);
                    let is_map = is_map_expr(value, &self.ctx.variables);
                    self.generate_expression(value);

                    arch::emit_allocate_var(
                        &mut self.output,
                        self.arch,
                        &mut self.ctx.stack_offset,
                    );

                    if let Some(sname) = is_struct {
                        self.ctx.variables.insert(
                            name.clone(),
                            VarType::Struct {
                                struct_name: sname,
                                offset: self.ctx.stack_offset,
                            },
                        );
                    } else if is_map {
                        self.ctx
                            .variables
                            .insert(name.clone(), VarType::Map(self.ctx.stack_offset));
                    } else if is_str {
                        self.ctx
                            .variables
                            .insert(name.clone(), VarType::StringOffset(self.ctx.stack_offset));
                    } else if is_arr {
                        self.ctx
                            .variables
                            .insert(name.clone(), VarType::Array(self.ctx.stack_offset));
                        if is_string_array(value, &self.ctx.variables) {
                            self.ctx
                                .variables
                                .insert(format!("arr_is_str:{}", name), VarType::Number(0));
                        }
                        if is_float_array(value, &self.ctx.variables) {
                            self.ctx
                                .variables
                                .insert(format!("arr_is_flt:{}", name), VarType::Number(0));
                        }
                    } else if is_flt {
                        self.ctx
                            .variables
                            .insert(name.clone(), VarType::Float(self.ctx.stack_offset));
                    } else {
                        self.ctx
                            .variables
                            .insert(name.clone(), VarType::Number(self.ctx.stack_offset));
                    }
                }
            },
            Stmt::Assign { name, value } => {
                let is_flt = is_float_expr(value, &self.ctx.variables);
                let is_str = is_string_expr(value, &self.ctx.variables);
                let is_arr = is_array_expr(value, &self.ctx.variables);
                let is_map = is_map_expr(value, &self.ctx.variables);
                self.generate_expression(value);

                if let Some(var_type) = self.ctx.variables.get(name).cloned() {
                    match var_type {
                        VarType::Number(offset)
                        | VarType::Float(offset)
                        | VarType::StringOffset(offset)
                        | VarType::Array(offset)
                        | VarType::Map(offset)
                        | VarType::Struct { offset, .. } => {
                            arch::emit_store_var(
                                &mut self.output,
                                self.arch,
                                offset,
                                self.ctx.stack_offset,
                            );
                            if is_map {
                                self.ctx
                                    .variables
                                    .insert(name.clone(), VarType::Map(offset));
                            } else if is_str {
                                self.ctx
                                    .variables
                                    .insert(name.clone(), VarType::StringOffset(offset));
                            } else if is_flt {
                                self.ctx
                                    .variables
                                    .insert(name.clone(), VarType::Float(offset));
                            } else if is_arr {
                                self.ctx
                                    .variables
                                    .insert(name.clone(), VarType::Array(offset));
                                if is_string_array(value, &self.ctx.variables) {
                                    self.ctx
                                        .variables
                                        .insert(format!("arr_is_str:{}", name), VarType::Number(0));
                                }
                                if is_float_array(value, &self.ctx.variables) {
                                    self.ctx
                                        .variables
                                        .insert(format!("arr_is_flt:{}", name), VarType::Number(0));
                                }
                            }
                        }
                        VarType::StringLabel(_) => {}
                    }
                }
            }
            Stmt::FieldAssign {
                object,
                field,
                value,
            } => {
                let mut field_idx = 0;
                let mut struct_found = false;

                if let Expr::Identifier(obj_name) = object {
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
                    let field_key = format!("{}.{}", obj_name, field);
                    let is_flt = is_float_expr(value, &self.ctx.variables);
                    let is_str = is_string_expr(value, &self.ctx.variables);
                    if is_str {
                        self.ctx
                            .variables
                            .insert(field_key, VarType::StringOffset(0));
                    } else if is_flt {
                        self.ctx.variables.insert(field_key, VarType::Float(0));
                    } else {
                        self.ctx.variables.insert(field_key, VarType::Number(0));
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
                arch::emit_push_temp(&mut self.output, self.arch);

                self.generate_expression(value);
                arch::emit_struct_field_set(&mut self.output, self.arch, field_idx);
            }
            Stmt::IndexAssign {
                array,
                index,
                value,
            } => {
                if is_map_expr(array, &self.ctx.variables) {
                    let actual_args = [array, index, value];
                    match self.arch {
                        Architecture::X86 => {
                            for arg in actual_args.iter().rev() {
                                self.generate_expression(arg);
                                arch::emit_push_temp(&mut self.output, self.arch);
                            }
                        }
                        _ => {
                            for arg in actual_args.iter() {
                                self.generate_expression(arg);
                                arch::emit_push_temp(&mut self.output, self.arch);
                            }
                        }
                    }
                    arch::emit_function_call(
                        &mut self.output,
                        self.arch,
                        "set",
                        3,
                        self.ctx.stack_offset,
                        self.os,
                    );
                    if is_string_expr(value, &self.ctx.variables) {
                        if let Expr::String(field) = index {
                            self.ctx.variables.insert(
                                format!("map_field_str:{}", field),
                                VarType::StringOffset(0),
                            );
                        }
                        if let (Expr::Identifier(map_name), Expr::String(field)) = (array, index) {
                            self.ctx.variables.insert(
                                format!("map_str:{}.{}", map_name, field),
                                VarType::StringOffset(0),
                            );
                        }
                    }
                } else {
                    self.generate_expression(array);
                    arch::emit_push_temp(&mut self.output, self.arch);

                    self.generate_expression(index);
                    arch::emit_push_temp(&mut self.output, self.arch);

                    self.generate_expression(value);
                    arch::emit_array_set(&mut self.output, self.arch);
                }
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
            Stmt::ForEach {
                var,
                iterable,
                body,
            } => {
                self.generate_expression(iterable);
                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
                let arr_offset = self.ctx.stack_offset;

                arch::emit_load_num(&mut self.output, self.arch, 0);
                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
                let idx_offset = self.ctx.stack_offset;

                let is_str = is_string_array(iterable, &self.ctx.variables);
                let is_flt = is_float_array(iterable, &self.ctx.variables);

                let var_offset = match self.ctx.variables.get(var) {
                    Some(
                        VarType::Number(offset)
                        | VarType::Float(offset)
                        | VarType::StringOffset(offset),
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
                        arch::emit_allocate_var(
                            &mut self.output,
                            self.arch,
                            &mut self.ctx.stack_offset,
                        );
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
