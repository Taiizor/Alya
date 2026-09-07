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
use analysis::{
    collect_known_float_vars, collect_known_string_vars, infer_param_is_array,
    infer_param_is_float, infer_param_is_float_array, infer_param_is_map, infer_param_is_string,
    infer_param_is_string_array, infer_param_struct_type,
};
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

        let known_strings = collect_known_string_vars(program);
        for s in &known_strings {
            if s.starts_with("map_field_str:")
                || s.starts_with("map_str:")
                || s.starts_with("fn_ret_str:")
            {
                self.ctx
                    .variables
                    .insert(s.clone(), VarType::StringOffset(0));
            }
        }

        let known_floats = collect_known_float_vars(program);
        for s in &known_floats {
            if s.starts_with("fn_ret_flt:") {
                self.ctx.variables.insert(s.clone(), VarType::Float(0));
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

        arch::emit_footer(&mut self.output, self.arch);

        for func in functions {
            if let Stmt::Function { name, params, body } = func {
                self.generate_function(name, params, body, program);
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
    ) {
        let saved = self.ctx.enter_function();

        arch::emit_function_prologue(&mut self.output, self.arch, name);

        for (i, param) in params.iter().enumerate() {
            arch::emit_function_param_push(
                &mut self.output,
                self.arch,
                i,
                &mut self.ctx.stack_offset,
                self.os,
            );

            let is_str = infer_param_is_string(name, i, program);
            let is_flt = infer_param_is_float(name, i, program);
            let is_arr = infer_param_is_array(name, i, program);
            let is_str_arr = infer_param_is_string_array(name, i, program);
            let is_flt_arr = infer_param_is_float_array(name, i, program);
            let is_map = infer_param_is_map(name, i, program);
            let struct_type = infer_param_struct_type(name, i, program);
            if let Some(sname) = struct_type {
                self.ctx.variables.insert(
                    param.clone(),
                    VarType::Struct {
                        struct_name: sname,
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
        }

        for stmt in body {
            self.generate_statement(stmt);
        }

        arch::emit_function_epilogue(&mut self.output, self.arch);

        self.ctx.exit_function(saved);
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
