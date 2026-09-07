use super::CodeGen;
use crate::ast::Expr;
use crate::codegen::analysis::{
    escape_string, is_array_expr, is_float_array, is_float_expr, is_map_expr, is_string_array,
    is_string_expr,
};
use crate::codegen::arch;
use crate::codegen::context::VarType;
use crate::codegen::target::Architecture;

impl CodeGen {
    pub(super) fn generate_let(&mut self, name: &str, value: &Expr) {
        let name = name.to_string();
        match value {
            Expr::String(s) => {
                let label = self.ctx.next_string_label();
                self.emit_rodata_section();
                self.output.push_str(&format!("{}:\n", label));
                self.emit_string_directive(&escape_string(s));
                self.output.push_str(".text\n");
                arch::emit_load_str_label(&mut self.output, self.arch, &label, self.os);
                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);
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

                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);

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

                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);

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

                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);

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

                arch::emit_allocate_var(&mut self.output, self.arch, &mut self.ctx.stack_offset);

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
                    if let Expr::Map(entries) = value {
                        for (k, v) in entries {
                            if is_string_expr(v, &self.ctx.variables) {
                                if let Expr::String(field) = k {
                                    self.ctx.variables.insert(
                                        format!("map_field_str:{}", field),
                                        VarType::StringOffset(0),
                                    );
                                    self.ctx.variables.insert(
                                        format!("map_str:{}.{}", name, field),
                                        VarType::StringOffset(0),
                                    );
                                }
                            }
                            if is_map_expr(v, &self.ctx.variables) {
                                if let Expr::String(field) = k {
                                    self.ctx.variables.insert(
                                        format!("map_field_map:{}", field),
                                        VarType::Map(0),
                                    );
                                    self.ctx.variables.insert(
                                        format!("map_map:{}.{}", name, field),
                                        VarType::Map(0),
                                    );
                                }
                            }
                        }
                    }
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
        }
    }

    pub(super) fn generate_assign(&mut self, name: &str, value: &Expr) {
        let name = name.to_string();
        let is_flt = is_float_expr(value, &self.ctx.variables);
        let is_str = is_string_expr(value, &self.ctx.variables);
        let is_arr = is_array_expr(value, &self.ctx.variables);
        let is_map = is_map_expr(value, &self.ctx.variables);
        self.generate_expression(value);

        if let Some(var_type) = self.ctx.variables.get(&name).cloned() {
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
                        if let Expr::Map(entries) = value {
                            for (k, v) in entries {
                                if is_string_expr(v, &self.ctx.variables) {
                                    if let Expr::String(field) = k {
                                        self.ctx.variables.insert(
                                            format!("map_field_str:{}", field),
                                            VarType::StringOffset(0),
                                        );
                                        self.ctx.variables.insert(
                                            format!("map_str:{}.{}", name, field),
                                            VarType::StringOffset(0),
                                        );
                                    }
                                }
                                if is_map_expr(v, &self.ctx.variables) {
                                    if let Expr::String(field) = k {
                                        self.ctx.variables.insert(
                                            format!("map_field_map:{}", field),
                                            VarType::Map(0),
                                        );
                                        self.ctx.variables.insert(
                                            format!("map_map:{}.{}", name, field),
                                            VarType::Map(0),
                                        );
                                    }
                                }
                            }
                        }
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

    pub(super) fn generate_field_assign(&mut self, object: &Expr, field: &str, value: &Expr) {
        let mut field_idx = 0;
        let mut struct_found = false;

        if let Expr::Identifier(obj_name) = object {
            if let Some(VarType::Struct { struct_name, .. }) = self.ctx.variables.get(obj_name) {
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

    pub(super) fn generate_index_assign(&mut self, array: &Expr, index: &Expr, value: &Expr) {
        if is_map_expr(array, &self.ctx.variables) || matches!(index, Expr::String(_)) {
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
                    self.ctx
                        .variables
                        .insert(format!("map_field_str:{}", field), VarType::StringOffset(0));
                }
                if let (Expr::Identifier(map_name), Expr::String(field)) = (array, index) {
                    self.ctx.variables.insert(
                        format!("map_str:{}.{}", map_name, field),
                        VarType::StringOffset(0),
                    );
                }
            }
            if is_map_expr(value, &self.ctx.variables) {
                if let Expr::String(field) = index {
                    self.ctx
                        .variables
                        .insert(format!("map_field_map:{}", field), VarType::Map(0));
                }
                if let (Expr::Identifier(map_name), Expr::String(field)) = (array, index) {
                    self.ctx
                        .variables
                        .insert(format!("map_map:{}.{}", map_name, field), VarType::Map(0));
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
}
