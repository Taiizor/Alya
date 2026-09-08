pub mod arrays;
pub mod common;
pub mod floats;
pub mod maps;
pub mod strings;
pub mod structs;

pub use arrays::*;
pub use common::*;
pub use floats::*;
pub use maps::*;
pub use strings::*;
pub use structs::*;

use crate::ast::Program;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ProgramInference {
    pub known_strings: HashSet<String>,
    pub known_floats: HashSet<String>,
    pub known_arrays: HashSet<String>,
    pub known_maps: HashSet<String>,
}

impl ProgramInference {
    pub fn analyze(program: &Program) -> Self {
        let known_strings = collect_known_string_vars(program);
        let known_floats = collect_known_float_vars(program);
        let known_arrays = collect_known_array_vars(program);
        let known_maps = collect_known_map_vars(program);
        Self {
            known_strings,
            known_floats,
            known_arrays,
            known_maps,
        }
    }

    #[inline]
    pub fn infer_param_is_string(
        &self,
        func_name: &str,
        param_idx: usize,
        program: &Program,
    ) -> bool {
        infer_param_is_string_with(func_name, param_idx, program, &self.known_strings)
    }

    #[inline]
    pub fn infer_param_is_string_array(
        &self,
        func_name: &str,
        param_idx: usize,
        program: &Program,
    ) -> bool {
        infer_param_is_string_array_with(func_name, param_idx, program, &self.known_strings)
    }

    #[inline]
    pub fn infer_param_is_float(
        &self,
        func_name: &str,
        param_idx: usize,
        program: &Program,
    ) -> bool {
        infer_param_is_float_with(func_name, param_idx, program, &self.known_floats)
    }

    #[inline]
    pub fn infer_param_is_float_array(
        &self,
        func_name: &str,
        param_idx: usize,
        program: &Program,
    ) -> bool {
        infer_param_is_float_array_with(func_name, param_idx, program, &self.known_floats)
    }

    #[inline]
    pub fn infer_param_is_array(
        &self,
        func_name: &str,
        param_idx: usize,
        program: &Program,
    ) -> bool {
        infer_param_is_array_with(func_name, param_idx, program, &self.known_arrays)
    }

    #[inline]
    pub fn infer_param_is_map(&self, func_name: &str, param_idx: usize, program: &Program) -> bool {
        infer_param_is_map_with(func_name, param_idx, program, &self.known_maps)
    }
}
