use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarType {
    Number(i32),         // Stack offset for numeric variables
    StringLabel(String), // Rodata label for string literals
    StringOffset(i32),   // Stack offset for string pointers
}

#[derive(Debug, Clone)]
pub struct ScopeState {
    pub variables: HashMap<String, VarType>,
    pub stack_offset: i32,
    pub loop_stack: Vec<(String, String)>,
}

#[derive(Debug, Default)]
pub struct CodeGenContext {
    pub label_counter: usize,
    pub string_counter: usize,
    pub variables: HashMap<String, VarType>,
    pub stack_offset: i32,
    pub loop_stack: Vec<(String, String)>,
}

impl CodeGenContext {
    pub fn new() -> Self {
        Self {
            label_counter: 0,
            string_counter: 0,
            variables: HashMap::new(),
            stack_offset: 0,
            loop_stack: Vec::new(),
        }
    }

    pub fn next_label(&mut self) -> String {
        let label = format!(".L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn next_string_label(&mut self) -> String {
        let label = format!("str_{}", self.string_counter);
        self.string_counter += 1;
        label
    }

    pub fn push_loop(&mut self, continue_lbl: String, break_lbl: String) {
        self.loop_stack.push((continue_lbl, break_lbl));
    }

    pub fn pop_loop(&mut self) -> Option<(String, String)> {
        self.loop_stack.pop()
    }

    pub fn current_loop(&self) -> Option<&(String, String)> {
        self.loop_stack.last()
    }

    pub fn enter_function(&mut self) -> ScopeState {
        let saved = ScopeState {
            variables: std::mem::take(&mut self.variables),
            stack_offset: self.stack_offset,
            loop_stack: std::mem::take(&mut self.loop_stack),
        };
        self.stack_offset = 0;
        saved
    }

    pub fn exit_function(&mut self, saved: ScopeState) {
        self.variables = saved.variables;
        self.stack_offset = saved.stack_offset;
        self.loop_stack = saved.loop_stack;
    }
}
