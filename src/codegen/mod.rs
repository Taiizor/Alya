pub mod analysis;
pub mod arch;
pub mod context;
mod expr;
pub mod runtime;
mod say;
mod stmt;
pub mod target;
#[cfg(test)]
mod tests;

pub use target::{Architecture, OperatingSystem};

use crate::ast::*;
use analysis::{infer_param_struct_type, ProgramInference};
use context::{CodeGenContext, VarType};

pub struct CodeGen {
    pub(crate) arch: Architecture,
    pub(crate) os: OperatingSystem,
    pub(crate) output: String,
    pub(crate) ctx: CodeGenContext,
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
        // Collect all struct definitions first
        for stmt in &program.statements {
            if let Stmt::StructDef { name, fields } = stmt {
                self.ctx.structs.insert(
                    name.clone(),
                    context::StructDefInfo {
                        name: name.clone(),
                        fields: fields.clone(),
                    },
                );
            }
        }

        let inference = ProgramInference::analyze(program);
        for s in &inference.known_strings {
            if s.starts_with("map_field_str:")
                || s.starts_with("map_str:")
                || s.starts_with("fn_ret_str:")
                || s.starts_with("fn_ret_tuple_str:")
                || s.starts_with("tuple_elem_str:")
                || s.starts_with("struct_field_str:")
                || s.starts_with("fn_param_str:")
                || s.starts_with("fn_param_str_arr:")
            {
                self.ctx
                    .variables
                    .insert(s.clone(), VarType::StringOffset(0));
            }
        }

        for s in &inference.known_floats {
            if s.starts_with("fn_ret_flt:") || s.starts_with("struct_field_flt:") {
                self.ctx.variables.insert(s.clone(), VarType::Float(0));
            }
        }

        for stmt in &program.statements {
            if let Stmt::Function { name, .. } = stmt {
                self.ctx.functions.insert(name.clone());
                if let Some(sname) = analysis::infer_function_return_struct_type(name, program) {
                    self.ctx.variables.insert(
                        format!("fn_ret_struct:{}", name),
                        VarType::Struct {
                            struct_name: sname,
                            offset: 0,
                        },
                    );
                }
            }
        }

        let mut functions = Vec::new();
        let mut top_level = Vec::new();

        for stmt in &program.statements {
            match stmt {
                Stmt::Function { .. } => functions.push(stmt),
                _ => top_level.push(stmt),
            }
        }

        arch::emit_header(&mut self.output, self.arch, self.os);

        for stmt in top_level {
            self.generate_statement(stmt);
        }

        self.emit_cleanup_scope(None);

        arch::emit_footer(&mut self.output, self.arch);

        for func in functions {
            if let Stmt::Function {
                name, params, body, ..
            } = func
            {
                self.generate_function(name, params, body, program, &inference);
            }
        }

        runtime::emit_runtime(&mut self.output, self.arch, self.os, &self.ctx.structs);
    }

    fn generate_function(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
        program: &Program,
        inference: &ProgramInference,
    ) {
        let saved = self.ctx.enter_function();

        arch::emit_function_prologue(&mut self.output, self.arch, name);

        let mut heap_param_offsets = Vec::new();
        for (i, param) in params.iter().enumerate() {
            arch::emit_function_param_push(
                &mut self.output,
                self.arch,
                i,
                &mut self.ctx.stack_offset,
                self.os,
            );

            let is_str = inference.infer_param_is_string(name, i, program);
            let is_flt = inference.infer_param_is_float(name, i, program);
            let is_arr = inference.infer_param_is_array(name, i, program);
            let is_str_arr = inference.infer_param_is_string_array(name, i, program);
            let is_flt_arr = inference.infer_param_is_float_array(name, i, program);
            let is_map = inference.infer_param_is_map(name, i, program);
            let struct_type = infer_param_struct_type(name, i, program);
            if let Some(ref sname) = struct_type {
                self.ctx.variables.insert(
                    param.clone(),
                    VarType::Struct {
                        struct_name: sname.clone(),
                        offset: self.ctx.stack_offset,
                    },
                );
            } else if is_str {
                self.ctx
                    .variables
                    .insert(param.clone(), VarType::StringOffset(self.ctx.stack_offset));
            } else if is_flt {
                self.ctx
                    .variables
                    .insert(param.clone(), VarType::Float(self.ctx.stack_offset));
            } else if is_arr || is_str_arr || is_flt_arr {
                self.ctx
                    .variables
                    .insert(param.clone(), VarType::Array(self.ctx.stack_offset));
                if is_str_arr {
                    self.ctx
                        .variables
                        .insert(format!("arr_is_str:{}", param), VarType::Number(0));
                }
                if is_flt_arr {
                    self.ctx
                        .variables
                        .insert(format!("arr_is_flt:{}", param), VarType::Number(0));
                }
            } else if is_map {
                self.ctx
                    .variables
                    .insert(param.clone(), VarType::Map(self.ctx.stack_offset));
            } else {
                self.ctx
                    .variables
                    .insert(param.clone(), VarType::Number(self.ctx.stack_offset));
            }

            let is_heap_param =
                struct_type.is_some() || is_arr || is_str_arr || is_flt_arr || is_map;
            if is_heap_param {
                heap_param_offsets.push(self.ctx.stack_offset);
            }
        }

        for offset in heap_param_offsets {
            arch::emit_load_var(
                &mut self.output,
                self.arch,
                offset,
                self.ctx.stack_offset,
            );
            arch::emit_rc_retain(&mut self.output, self.arch, self.ctx.stack_offset, self.os);
        }

        for stmt in body {
            self.generate_statement(stmt);
        }

        self.emit_cleanup_scope(None);

        arch::emit_function_epilogue(&mut self.output, self.arch);

        self.ctx.exit_function(saved);
    }

    pub(crate) fn get_scope_heap_offsets(&self, skip_offset: Option<i32>) -> Vec<i32> {
        let mut offsets: Vec<i32> = self
            .ctx
            .variables
            .iter()
            .filter(|(name, _)| !name.contains(':') && !name.contains('.'))
            .filter_map(|(_, vtype)| match vtype {
                VarType::Array(off) | VarType::Map(off) | VarType::Struct { offset: off, .. } => {
                    if *off > 0 && Some(*off) != skip_offset {
                        Some(*off)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        offsets.sort_unstable();
        offsets.dedup();
        offsets
    }

    pub(crate) fn emit_cleanup_scope(&mut self, skip_offset: Option<i32>) {
        let offsets = self.get_scope_heap_offsets(skip_offset);
        for offset in offsets {
            arch::emit_rc_release_stack(
                &mut self.output,
                self.arch,
                offset,
                self.ctx.stack_offset,
                self.os,
            );
        }
    }

    pub(crate) fn is_heap_expression(&self, expr: &crate::ast::Expr) -> bool {
        use crate::ast::Expr;
        match expr {
            Expr::Array(_) | Expr::Map(_) | Expr::StructInit { .. } => true,
            Expr::Identifier(name) => matches!(
                self.ctx.variables.get(name),
                Some(VarType::Array(_)) | Some(VarType::Map(_)) | Some(VarType::Struct { .. })
            ),
            Expr::Call { name, .. } => {
                self.ctx.structs.contains_key(name)
                    || matches!(
                        self.ctx.variables.get(&format!("fn_ret_struct:{}", name)),
                        Some(VarType::Struct { .. })
                    )
            }
            _ => false,
        }
    }

    pub(crate) fn emit_rodata_section(&mut self) {
        if matches!(self.os, OperatingSystem::MacOS) {
            self.output
                .push_str(".section __TEXT,__cstring,cstring_literals\n");
        } else {
            self.output.push_str(".section .rodata\n");
        }
    }

    pub(crate) fn emit_string_directive(&mut self, text: &str) {
        if matches!(self.os, OperatingSystem::MacOS) {
            self.output.push_str(&format!("    .asciz \"{}\"\n", text));
        } else {
            self.output.push_str(&format!("    .string \"{}\"\n", text));
        }
    }
}

pub fn generate(program: &Program, arch: Architecture, os: OperatingSystem) -> String {
    let mut codegen = CodeGen::new(arch, os);
    codegen.generate_program(program);
    codegen.output
}
