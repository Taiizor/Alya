use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Architecture {
    X64,
    X86,
    ARM64,
}

#[derive(Debug, Clone, Copy)]
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOS,
}

#[derive(Debug, Clone)]
enum VarType {
    Number(i32),      // Stack offset for numeric variables
    StringLabel(String), // String label for string variables
    StringOffset(i32), // Stack offset for string pointer variables
}

struct CodeGen {
    arch: Architecture,
    os: OperatingSystem,
    output: String,
    label_counter: usize,
    string_counter: usize,
    variables: HashMap<String, VarType>, // Variable name to type and location
    stack_offset: i32, // Current stack offset for variables
    loop_stack: Vec<(String, String)>, // (continue_label, break_label)
}

impl CodeGen {
    fn new(arch: Architecture, os: OperatingSystem) -> Self {
        Self {
            arch,
            os,
            output: String::new(),
            label_counter: 0,
            string_counter: 0,
            variables: HashMap::new(),
            stack_offset: 0,
            loop_stack: Vec::new(),
        }
    }

    fn emit(&mut self, code: &str) {
        self.output.push_str(code);
        self.output.push('\n');
    }

    fn next_label(&mut self) -> String {
        let label = format!(".L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    fn next_string_label(&mut self) -> String {
        let label = format!("str_{}", self.string_counter);
        self.string_counter += 1;
        label
    }

    // Helper to emit call printf with proper stack alignment and shadow space
    fn emit_call_printf(&mut self) {
        match self.arch {
            Architecture::ARM64 => {
                self.emit("    bl printf");
            }
            Architecture::X64 => {
                if matches!(self.os, OperatingSystem::Windows) {
                    let padding = if self.stack_offset % 16 == 0 { 32 } else { 40 };
                    self.emit(&format!("    sub ${}, %rsp", padding));
                    self.emit("    call printf");
                    self.emit(&format!("    add ${}, %rsp", padding));
                } else {
                    let misaligned = self.stack_offset % 16 != 0;
                    if misaligned {
                        self.emit("    sub $8, %rsp");
                    }
                    self.emit("    call printf");
                    if misaligned {
                        self.emit("    add $8, %rsp");
                    }
                }
            }
            Architecture::X86 => {
                self.emit("    call printf");
            }
        }
    }

    fn generate_program(&mut self, program: &Program) {
        let mut functions = Vec::new();
        let mut top_level = Vec::new();

        for stmt in &program.statements {
            match stmt {
                Stmt::Function { .. } => functions.push(stmt),
                _ => top_level.push(stmt),
            }
        }

        self.emit_header();

        for stmt in top_level {
            self.generate_statement(stmt);
        }

        self.emit_footer();

        for func in functions {
            if let Stmt::Function { name, params, body } = func {
                self.generate_function(name, params, body, program);
            }
        }

        self.emit_runtime();
    }

    fn emit_header(&mut self) {
        match self.arch {
            Architecture::ARM64 => {
                self.emit(".global main");
                self.emit(".extern printf");
                self.emit("");
                self.emit(".text");
                self.emit(".align 2");
                self.emit("main:");
                self.emit("    stp x29, x30, [sp, #-16]!");
                self.emit("    mov x29, sp");
                self.emit("");
            }
            Architecture::X64 => {
                self.emit(".global main");
                self.emit(".extern printf");
                self.emit("");
                self.emit(".text");
                self.emit("main:");
                self.emit("    push %rbp");
                self.emit("    mov %rsp, %rbp");
                self.emit("");
            }
            Architecture::X86 => {
                self.emit(".global main");
                self.emit(".extern printf");
                self.emit("");
                self.emit(".text");
                self.emit("main:");
                self.emit("    push %ebp");
                self.emit("    mov %esp, %ebp");
                self.emit("");
            }
        }
    }

    fn emit_footer(&mut self) {
        match self.arch {
            Architecture::ARM64 => {
                self.emit("    mov w0, #0");
                self.emit("    mov sp, x29");
                self.emit("    ldp x29, x30, [sp], #16");
                self.emit("    ret");
            }
            Architecture::X64 => {
                self.emit("    xor %rax, %rax");
                self.emit("    mov %rbp, %rsp");
                self.emit("    pop %rbp");
                self.emit("    ret");
            }
            Architecture::X86 => {
                self.emit("    xor %eax, %eax");
                self.emit("    mov %ebp, %esp");
                self.emit("    pop %ebp");
                self.emit("    ret");
            }
        }
    }

    fn generate_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Say(expr) => self.generate_say(expr),
            Stmt::Let { name, value } => {
                match value {
                    Expr::String(s) => {
                        // For string literals, create a label and store the label name
                        let label = self.next_string_label();
                        self.emit(".section .rodata");
                        self.emit(&format!("{}:", label));
                        self.emit(&format!("    .string \"{}\"", escape_string(s)));
                        self.emit(".text");
                        self.variables.insert(name.clone(), VarType::StringLabel(label));
                    }
                    _ => {
                        let is_str = self.is_string_expr(value);
                        self.generate_expression(value);

                        // Allocate space on stack for the variable
                        match self.arch {
                            Architecture::ARM64 => {
                                self.stack_offset += 16;
                                self.emit("    str x0, [sp, #-16]!");
                            }
                            Architecture::X64 => {
                                self.stack_offset += 8;
                                self.emit("    push %rax");
                            }
                            Architecture::X86 => {
                                self.stack_offset += 4;
                                self.emit("    push %eax");
                            }
                        }

                        // Store the variable location
                        if is_str {
                            self.variables.insert(name.clone(), VarType::StringOffset(self.stack_offset));
                        } else {
                            self.variables.insert(name.clone(), VarType::Number(self.stack_offset));
                        }
                    }
                }
            }
            Stmt::Assign { name, value } => {
                // Evaluate the new value
                self.generate_expression(value);

                // Get variable location from symbol table
                if let Some(var_type) = self.variables.get(name) {
                    match var_type {
                        VarType::Number(offset) | VarType::StringOffset(offset) => {
                            // Store the new value at the variable's location
                            let offset = *offset;
                            match self.arch {
                                Architecture::ARM64 => {
                                    self.emit(&format!("    str x0, [sp, #{}]", self.stack_offset - offset));
                                }
                                Architecture::X64 => {
                                    self.emit(&format!("    mov %rax, -{}(%rbp)", offset));
                                }
                                Architecture::X86 => {
                                    self.emit(&format!("    mov %eax, -{}(%ebp)", offset));
                                }
                            }
                        }
                        VarType::StringLabel(_) => {
                            // String literal reassignment not supported
                        }
                    }
                }
            }
            Stmt::Expr(expr) => {
                self.generate_expression(expr);
            }
            Stmt::If { condition, then_block, else_block } => {
                let else_label = self.next_label();
                let end_label = self.next_label();

                // Evaluate condition
                self.generate_expression(condition);

                // If false (0), jump to else (or end if no else)
                let target_label = if else_block.is_some() { &else_label } else { &end_label };
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    cbz x0, {}", target_label));
                    }
                    Architecture::X64 => {
                        self.emit("    test %rax, %rax");
                        self.emit(&format!("    jz {}", target_label));
                    }
                    Architecture::X86 => {
                        self.emit("    test %eax, %eax");
                        self.emit(&format!("    jz {}", target_label));
                    }
                }

                // Generate then block
                for stmt in then_block {
                    self.generate_statement(stmt);
                }

                if let Some(else_stmts) = else_block {
                    // Jump past else block
                    match self.arch {
                        Architecture::ARM64 => {
                            self.emit(&format!("    b {}", end_label));
                        }
                        Architecture::X64 => {
                            self.emit(&format!("    jmp {}", end_label));
                        }
                        Architecture::X86 => {
                            self.emit(&format!("    jmp {}", end_label));
                        }
                    }

                    // Else label
                    self.emit(&format!("{}:", else_label));

                    // Generate else block
                    for stmt in else_stmts {
                        self.generate_statement(stmt);
                    }
                }

                // End label
                self.emit(&format!("{}:", end_label));
            }
            Stmt::While { condition, body } => {
                let start_label = self.next_label();
                let end_label = self.next_label();

                self.loop_stack.push((start_label.clone(), end_label.clone()));

                // Start of loop
                self.emit(&format!("{}:", start_label));

                // Evaluate condition
                self.generate_expression(condition);

                // If condition is false, jump to end
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    cbz x0, {}", end_label));
                    }
                    Architecture::X64 => {
                        self.emit("    test %rax, %rax");
                        self.emit(&format!("    jz {}", end_label));
                    }
                    Architecture::X86 => {
                        self.emit("    test %eax, %eax");
                        self.emit(&format!("    jz {}", end_label));
                    }
                }

                // Loop body
                for stmt in body {
                    self.generate_statement(stmt);
                }

                // Jump back to start
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    b {}", start_label));
                    }
                    Architecture::X64 => {
                        self.emit(&format!("    jmp {}", start_label));
                    }
                    Architecture::X86 => {
                        self.emit(&format!("    jmp {}", start_label));
                    }
                }

                // End of loop
                self.emit(&format!("{}:", end_label));

                self.loop_stack.pop();
            }
            Stmt::For { var, start, end, body } => {
                // Evaluate start value
                self.generate_expression(start);

                // Allocate or assign variable
                let var_offset = match self.variables.get(var) {
                    Some(VarType::Number(offset)) => {
                        let offset = *offset;
                        match self.arch {
                            Architecture::ARM64 => {
                                self.emit(&format!("    str x0, [sp, #{}]", self.stack_offset - offset));
                            }
                            Architecture::X64 => {
                                self.emit(&format!("    mov %rax, -{}(%rbp)", offset));
                            }
                            Architecture::X86 => {
                                self.emit(&format!("    mov %eax, -{}(%ebp)", offset));
                            }
                        }
                        offset
                    }
                    _ => {
                        match self.arch {
                            Architecture::ARM64 => {
                                self.stack_offset += 16;
                                self.emit("    str x0, [sp, #-16]!");
                            }
                            Architecture::X64 => {
                                self.stack_offset += 8;
                                self.emit("    push %rax");
                            }
                            Architecture::X86 => {
                                self.stack_offset += 4;
                                self.emit("    push %eax");
                            }
                        }
                        self.variables.insert(var.clone(), VarType::Number(self.stack_offset));
                        self.stack_offset
                    }
                };

                let start_label = self.next_label();
                let step_label = self.next_label();
                let end_label = self.next_label();

                self.loop_stack.push((step_label.clone(), end_label.clone()));

                // Start of loop
                self.emit(&format!("{}:", start_label));

                // Load var and push onto stack
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    ldr x0, [sp, #{}]", self.stack_offset - var_offset));
                        self.emit("    str x0, [sp, #-16]!");
                    }
                    Architecture::X64 => {
                        self.emit(&format!("    mov -{}(%rbp), %rax", var_offset));
                        self.emit("    push %rax");
                    }
                    Architecture::X86 => {
                        self.emit(&format!("    mov -{}(%ebp), %eax", var_offset));
                        self.emit("    push %eax");
                    }
                }

                // Evaluate end
                self.generate_expression(end);

                // Compare var with end (if var > end, exit)
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit("    ldr x1, [sp], #16");
                        self.emit("    cmp x1, x0");
                        self.emit(&format!("    b.gt {}", end_label));
                    }
                    Architecture::X64 => {
                        self.emit("    mov %rax, %rbx");
                        self.emit("    pop %rax");
                        self.emit("    cmp %rbx, %rax");
                        self.emit(&format!("    jg {}", end_label));
                    }
                    Architecture::X86 => {
                        self.emit("    mov %eax, %ebx");
                        self.emit("    pop %eax");
                        self.emit("    cmp %ebx, %eax");
                        self.emit(&format!("    jg {}", end_label));
                    }
                }

                // Loop body
                for stmt in body {
                    self.generate_statement(stmt);
                }

                // Step label (for continue)
                self.emit(&format!("{}:", step_label));

                // Increment var (var = var + 1)
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    ldr x0, [sp, #{}]", self.stack_offset - var_offset));
                        self.emit("    add x0, x0, #1");
                        self.emit(&format!("    str x0, [sp, #{}]", self.stack_offset - var_offset));
                        self.emit(&format!("    b {}", start_label));
                    }
                    Architecture::X64 => {
                        self.emit(&format!("    addq $1, -{}(%rbp)", var_offset));
                        self.emit(&format!("    jmp {}", start_label));
                    }
                    Architecture::X86 => {
                        self.emit(&format!("    addl $1, -{}(%ebp)", var_offset));
                        self.emit(&format!("    jmp {}", start_label));
                    }
                }

                // End label
                self.emit(&format!("{}:", end_label));

                self.loop_stack.pop();
            }
            Stmt::Break => {
                if let Some((_, break_label)) = self.loop_stack.last() {
                    match self.arch {
                        Architecture::ARM64 => self.emit(&format!("    b {}", break_label)),
                        Architecture::X64 => self.emit(&format!("    jmp {}", break_label)),
                        Architecture::X86 => self.emit(&format!("    jmp {}", break_label)),
                    }
                }
            }
            Stmt::Continue => {
                if let Some((continue_label, _)) = self.loop_stack.last() {
                    match self.arch {
                        Architecture::ARM64 => self.emit(&format!("    b {}", continue_label)),
                        Architecture::X64 => self.emit(&format!("    jmp {}", continue_label)),
                        Architecture::X86 => self.emit(&format!("    jmp {}", continue_label)),
                    }
                }
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.generate_expression(expr);
                }
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit("    mov sp, x29");
                        self.emit("    ldp x29, x30, [sp], #16");
                        self.emit("    ret");
                    }
                    Architecture::X64 => {
                        self.emit("    mov %rbp, %rsp");
                        self.emit("    pop %rbp");
                        self.emit("    ret");
                    }
                    Architecture::X86 => {
                        self.emit("    mov %ebp, %esp");
                        self.emit("    pop %ebp");
                        self.emit("    ret");
                    }
                }
            }
            Stmt::Function { .. } => {}
        }
    }

    fn generate_say(&mut self, expr: &Expr) {
        match expr {
            Expr::String(s) => {
                let label = self.next_string_label();
                
                // Emit string data
                self.emit(".section .rodata");
                self.emit(&format!("{}:", label));
                self.emit(&format!("    .string \"{}\\n\"", escape_string(s)));
                self.emit(".text");

                // Print string
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    adrp x0, {}@PAGE", label));
                        self.emit(&format!("    add x0, x0, {}@PAGEOFF", label));
                        self.emit_call_printf();
                    }
                    Architecture::X64 => {
                        if matches!(self.os, OperatingSystem::Windows) {
                            self.emit(&format!("    lea {}(%rip), %rcx", label));
                            self.emit("    xor %rax, %rax");
                            self.emit_call_printf();
                        } else {
                            self.emit(&format!("    lea {}(%rip), %rdi", label));
                            self.emit("    xor %rax, %rax");
                            self.emit_call_printf();
                        }
                    }
                    Architecture::X86 => {
                        self.emit(&format!("    push ${}", label));
                        self.emit_call_printf();
                        self.emit("    add $4, %esp");
                    }
                }
                self.emit("");
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
                            if self.is_string_expr(part) {
                                format_str.push_str("%s");
                            } else {
                                format_str.push_str("%ld");
                            }
                            exprs.push(part);
                        }
                    }
                }
                format_str.push_str("\\n");

                let fmt_label = self.next_string_label();
                self.emit(".section .rodata");
                self.emit(&format!("{}:", fmt_label));
                self.emit(&format!("    .string \"{}\"", format_str));
                self.emit(".text");

                match self.arch {
                    Architecture::X86 => {
                        for expr in exprs.iter().rev() {
                            self.generate_expression(expr);
                            self.emit("    push %eax");
                        }
                        self.emit(&format!("    push ${}", fmt_label));
                        self.emit_call_printf();
                        self.emit(&format!("    add ${}, %esp", (exprs.len() + 1) * 4));
                    }
                    Architecture::X64 => {
                        for expr in &exprs {
                            self.generate_expression(expr);
                            self.emit("    push %rax");
                        }
                        if matches!(self.os, OperatingSystem::Windows) {
                            for i in (0..exprs.len()).rev() {
                                let reg = match i {
                                    0 => "%rdx",
                                    1 => "%r8",
                                    2 => "%r9",
                                    _ => "%rdx",
                                };
                                self.emit(&format!("    pop {}", reg));
                            }
                            self.emit(&format!("    lea {}(%rip), %rcx", fmt_label));
                            self.emit("    xor %rax, %rax");
                            self.emit_call_printf();
                        } else {
                            for i in (0..exprs.len()).rev() {
                                let reg = match i {
                                    0 => "%rsi",
                                    1 => "%rdx",
                                    2 => "%rcx",
                                    3 => "%r8",
                                    4 => "%r9",
                                    _ => "%rsi",
                                };
                                self.emit(&format!("    pop {}", reg));
                            }
                            self.emit(&format!("    lea {}(%rip), %rdi", fmt_label));
                            self.emit("    xor %rax, %rax");
                            self.emit_call_printf();
                        }
                    }
                    Architecture::ARM64 => {
                        for expr in &exprs {
                            self.generate_expression(expr);
                            self.emit("    str x0, [sp, #-16]!");
                        }
                        for i in (0..exprs.len()).rev() {
                            self.emit(&format!("    ldr x{}, [sp], #16", i + 1));
                        }
                        self.emit(&format!("    adrp x0, {}@PAGE", fmt_label));
                        self.emit(&format!("    add x0, x0, {}@PAGEOFF", fmt_label));
                        self.emit_call_printf();
                    }
                }
                self.emit("");
            }
            Expr::Identifier(name) => {
                // Check if this is a string or numeric variable
                if let Some(var_type) = self.variables.get(name).cloned() {
                    match var_type {
                        VarType::StringLabel(label) => {
                            // Print string variable
                            let fmt_label = self.next_string_label();
                            self.emit(".section .rodata");
                            self.emit(&format!("{}:", fmt_label));
                            self.emit("    .string \"%s\\n\"");
                            self.emit(".text");
                            
                            match self.arch {
                                Architecture::ARM64 => {
                                    self.emit(&format!("    adrp x1, {}@PAGE", label));
                                    self.emit(&format!("    add x1, x1, {}@PAGEOFF", label));
                                    self.emit(&format!("    adrp x0, {}@PAGE", fmt_label));
                                    self.emit(&format!("    add x0, x0, {}@PAGEOFF", fmt_label));
                                    self.emit_call_printf();
                                }
                                Architecture::X64 => {
                                    if matches!(self.os, OperatingSystem::Windows) {
                                        self.emit(&format!("    lea {}(%rip), %rcx", fmt_label));
                                        self.emit(&format!("    lea {}(%rip), %rdx", label));
                                        self.emit("    xor %rax, %rax");
                                        self.emit_call_printf();
                                    } else {
                                        self.emit(&format!("    lea {}(%rip), %rsi", label));
                                        self.emit(&format!("    lea {}(%rip), %rdi", fmt_label));
                                        self.emit("    xor %rax, %rax");
                                        self.emit_call_printf();
                                    }
                                }
                                Architecture::X86 => {
                                    self.emit(&format!("    push ${}", label));
                                    self.emit(&format!("    push ${}", fmt_label));
                                    self.emit_call_printf();
                                    self.emit("    add $8, %esp");
                                }
                            }
                            self.emit("");
                        }
                        VarType::StringOffset(offset) => {
                            let fmt_label = self.next_string_label();
                            self.emit(".section .rodata");
                            self.emit(&format!("{}:", fmt_label));
                            self.emit("    .string \"%s\\n\"");
                            self.emit(".text");
                            
                            match self.arch {
                                Architecture::ARM64 => {
                                    self.emit(&format!("    ldr x1, [sp, #{}]", self.stack_offset - offset));
                                    self.emit(&format!("    adrp x0, {}@PAGE", fmt_label));
                                    self.emit(&format!("    add x0, x0, {}@PAGEOFF", fmt_label));
                                    self.emit_call_printf();
                                }
                                Architecture::X64 => {
                                    if matches!(self.os, OperatingSystem::Windows) {
                                        self.emit(&format!("    mov -{}(%rbp), %rdx", offset));
                                        self.emit(&format!("    lea {}(%rip), %rcx", fmt_label));
                                        self.emit("    xor %rax, %rax");
                                        self.emit_call_printf();
                                    } else {
                                        self.emit(&format!("    mov -{}(%rbp), %rsi", offset));
                                        self.emit(&format!("    lea {}(%rip), %rdi", fmt_label));
                                        self.emit("    xor %rax, %rax");
                                        self.emit_call_printf();
                                    }
                                }
                                Architecture::X86 => {
                                    self.emit(&format!("    push -{}(%ebp)", offset));
                                    self.emit(&format!("    push ${}", fmt_label));
                                    self.emit_call_printf();
                                    self.emit("    add $8, %esp");
                                }
                            }
                            self.emit("");
                        }
                        VarType::Number(offset) => {
                            // Print numeric variable - load it first, then print
                            let fmt_label = self.next_string_label();
                            
                            // Emit format string
                            self.emit(".section .rodata");
                            self.emit(&format!("{}:", fmt_label));
                            self.emit("    .string \"%ld\\n\"");
                            self.emit(".text");
                            
                            // Load variable value
                            match self.arch {
                                Architecture::ARM64 => {
                                    self.emit(&format!("    ldr x1, [sp, #{}]", self.stack_offset - offset));
                                    self.emit(&format!("    adrp x0, {}@PAGE", fmt_label));
                                    self.emit(&format!("    add x0, x0, {}@PAGEOFF", fmt_label));
                                    self.emit_call_printf();
                                }
                                Architecture::X64 => {
                                    if matches!(self.os, OperatingSystem::Windows) {
                                        self.emit(&format!("    mov -{}(%rbp), %rdx", offset));
                                        self.emit(&format!("    lea {}(%rip), %rcx", fmt_label));
                                        self.emit("    xor %rax, %rax");
                                        self.emit_call_printf();
                                    } else {
                                        self.emit(&format!("    mov -{}(%rbp), %rsi", offset));
                                        self.emit(&format!("    lea {}(%rip), %rdi", fmt_label));
                                        self.emit("    xor %rax, %rax");
                                        self.emit_call_printf();
                                    }
                                }
                                Architecture::X86 => {
                                    self.emit(&format!("    push -{}(%ebp)", offset));
                                    self.emit(&format!("    push ${}", fmt_label));
                                    self.emit_call_printf();
                                    self.emit("    add $8, %esp");
                                }
                            }
                            self.emit("");
                        }
                    }
                }
            }
            Expr::Number(n) => {
                let fmt_label = self.next_string_label();
                
                // Emit format string
                self.emit(".section .rodata");
                self.emit(&format!("{}:", fmt_label));
                self.emit("    .string \"%ld\\n\"");
                self.emit(".text");

                // Print number
                let num_val = *n as i64;

                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    mov x1, #{}", num_val));
                        self.emit(&format!("    adrp x0, {}@PAGE", fmt_label));
                        self.emit(&format!("    add x0, x0, {}@PAGEOFF", fmt_label));
                        self.emit_call_printf();
                    }
                    Architecture::X64 => {
                        if matches!(self.os, OperatingSystem::Windows) {
                            self.emit(&format!("    lea {}(%rip), %rcx", fmt_label));
                            self.emit(&format!("    mov ${}, %rdx", num_val));
                            self.emit("    xor %rax, %rax");
                            self.emit_call_printf();
                        } else {
                            self.emit(&format!("    mov ${}, %rsi", num_val));
                            self.emit(&format!("    lea {}(%rip), %rdi", fmt_label));
                            self.emit("    xor %rax, %rax");
                            self.emit_call_printf();
                        }
                    }
                    Architecture::X86 => {
                        self.emit(&format!("    push ${}", num_val));
                        self.emit(&format!("    push ${}", fmt_label));
                        self.emit_call_printf();
                        self.emit("    add $8, %esp");
                    }
                }
                self.emit("");
            }
            _ => {
                // For complex expressions, evaluate and print result
                let is_str = self.is_string_expr(expr);
                self.generate_expression(expr);
                
                let fmt_label = self.next_string_label();
                self.emit(".section .rodata");
                self.emit(&format!("{}:", fmt_label));
                if is_str {
                    self.emit("    .string \"%s\\n\"");
                } else {
                    self.emit("    .string \"%ld\\n\"");
                }
                self.emit(".text");

                match self.arch {
                    Architecture::ARM64 => {
                        self.emit("    mov x1, x0");
                        self.emit(&format!("    adrp x0, {}@PAGE", fmt_label));
                        self.emit(&format!("    add x0, x0, {}@PAGEOFF", fmt_label));
                        self.emit_call_printf();
                    }
                    Architecture::X64 => {
                        if matches!(self.os, OperatingSystem::Windows) {
                            self.emit("    mov %rax, %rdx");
                            self.emit(&format!("    lea {}(%rip), %rcx", fmt_label));
                            self.emit("    xor %rax, %rax");
                            self.emit_call_printf();
                        } else {
                            self.emit("    mov %rax, %rsi");
                            self.emit(&format!("    lea {}(%rip), %rdi", fmt_label));
                            self.emit("    xor %rax, %rax");
                            self.emit_call_printf();
                        }
                    }
                    Architecture::X86 => {
                        self.emit("    push %eax");
                        self.emit(&format!("    push ${}", fmt_label));
                        self.emit_call_printf();
                        self.emit("    add $8, %esp");
                    }
                }
                self.emit("");
            }
        }
    }

    fn generate_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                let num_val = *n as i64;

                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    mov x0, #{}", num_val));
                    }
                    Architecture::X64 => {
                        self.emit(&format!("    mov ${}, %rax", num_val));
                    }
                    Architecture::X86 => {
                        self.emit(&format!("    mov ${}, %eax", num_val));
                    }
                }
            }
            Expr::String(s) => {
                let label = self.next_string_label();
                self.emit(".section .rodata");
                self.emit(&format!("{}:", label));
                self.emit(&format!("    .string \"{}\"", escape_string(s)));
                self.emit(".text");

                match self.arch {
                    Architecture::ARM64 => {
                        self.emit(&format!("    adrp x0, {}@PAGE", label));
                        self.emit(&format!("    add x0, x0, {}@PAGEOFF", label));
                    }
                    Architecture::X64 => {
                        self.emit(&format!("    lea {}(%rip), %rax", label));
                    }
                    Architecture::X86 => {
                        self.emit(&format!("    mov ${}, %eax", label));
                    }
                }
            }
            Expr::Identifier(name) => {
                // Load variable from stack
                if let Some(var_type) = self.variables.get(name).cloned() {
                    match var_type {
                        VarType::Number(offset) | VarType::StringOffset(offset) => {
                            match self.arch {
                                Architecture::ARM64 => {
                                    self.emit(&format!("    ldr x0, [sp, #{}]", self.stack_offset - offset));
                                }
                                Architecture::X64 => {
                                    self.emit(&format!("    mov -{}(%rbp), %rax", offset));
                                }
                                Architecture::X86 => {
                                    self.emit(&format!("    mov -{}(%ebp), %eax", offset));
                                }
                            }
                        }
                        VarType::StringLabel(label) => {
                            match self.arch {
                                Architecture::ARM64 => {
                                    self.emit(&format!("    adrp x0, {}@PAGE", label));
                                    self.emit(&format!("    add x0, x0, {}@PAGEOFF", label));
                                }
                                Architecture::X64 => {
                                    self.emit(&format!("    lea {}(%rip), %rax", label));
                                }
                                Architecture::X86 => {
                                    self.emit(&format!("    mov ${}, %eax", label));
                                }
                            }
                        }
                    }
                }
            }
            Expr::Binary { left, op, right } => {
                if matches!(op, BinaryOp::Add) && (self.is_string_expr(left) || self.is_string_expr(right)) {
                    self.generate_string_concat(left, right);
                    return;
                }

                // Evaluate left side
                self.generate_expression(left);
                
                // Save left result
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit("    str x0, [sp, #-16]!");
                    }
                    Architecture::X64 => {
                        self.emit("    push %rax");
                    }
                    Architecture::X86 => {
                        self.emit("    push %eax");
                    }
                }

                // Evaluate right side
                self.generate_expression(right);

                // Restore left and perform operation
                match self.arch {
                    Architecture::ARM64 => {
                        self.emit("    ldr x1, [sp], #16");
                        match op {
                            BinaryOp::Add => self.emit("    add x0, x1, x0"),
                            BinaryOp::Subtract => self.emit("    sub x0, x1, x0"),
                            BinaryOp::Multiply => self.emit("    mul x0, x1, x0"),
                            BinaryOp::Divide => self.emit("    sdiv x0, x1, x0"),
                            BinaryOp::Modulo => {
                                self.emit("    sdiv x2, x1, x0");
                                self.emit("    msub x0, x2, x0, x1");
                            }
                            BinaryOp::Equal => {
                                self.emit("    cmp x1, x0");
                                self.emit("    cset x0, eq");
                            }
                            BinaryOp::NotEqual => {
                                self.emit("    cmp x1, x0");
                                self.emit("    cset x0, ne");
                            }
                            BinaryOp::Less => {
                                self.emit("    cmp x1, x0");
                                self.emit("    cset x0, lt");
                            }
                            BinaryOp::Greater => {
                                self.emit("    cmp x1, x0");
                                self.emit("    cset x0, gt");
                            }
                            BinaryOp::LessEqual => {
                                self.emit("    cmp x1, x0");
                                self.emit("    cset x0, le");
                            }
                            BinaryOp::GreaterEqual => {
                                self.emit("    cmp x1, x0");
                                self.emit("    cset x0, ge");
                            }
                            BinaryOp::And => self.emit("    and x0, x1, x0"),
                            BinaryOp::Or => self.emit("    orr x0, x1, x0"),
                        }
                    }
                    Architecture::X64 => {
                        self.emit("    mov %rax, %rbx");
                        self.emit("    pop %rax");
                        match op {
                            BinaryOp::Add => self.emit("    add %rbx, %rax"),
                            BinaryOp::Subtract => self.emit("    sub %rbx, %rax"),
                            BinaryOp::Multiply => self.emit("    imul %rbx, %rax"),
                            BinaryOp::Divide => {
                                self.emit("    cqo");
                                self.emit("    idiv %rbx");
                            }
                            BinaryOp::Modulo => {
                                self.emit("    cqo");
                                self.emit("    idiv %rbx");
                                self.emit("    mov %rdx, %rax");
                            }
                            BinaryOp::Equal => {
                                self.emit("    cmp %rbx, %rax");
                                self.emit("    sete %al");
                                self.emit("    movzbq %al, %rax");
                            }
                            BinaryOp::NotEqual => {
                                self.emit("    cmp %rbx, %rax");
                                self.emit("    setne %al");
                                self.emit("    movzbq %al, %rax");
                            }
                            BinaryOp::Less => {
                                self.emit("    cmp %rbx, %rax");
                                self.emit("    setl %al");
                                self.emit("    movzbq %al, %rax");
                            }
                            BinaryOp::Greater => {
                                self.emit("    cmp %rbx, %rax");
                                self.emit("    setg %al");
                                self.emit("    movzbq %al, %rax");
                            }
                            BinaryOp::LessEqual => {
                                self.emit("    cmp %rbx, %rax");
                                self.emit("    setle %al");
                                self.emit("    movzbq %al, %rax");
                            }
                            BinaryOp::GreaterEqual => {
                                self.emit("    cmp %rbx, %rax");
                                self.emit("    setge %al");
                                self.emit("    movzbq %al, %rax");
                            }
                            BinaryOp::And => self.emit("    and %rbx, %rax"),
                            BinaryOp::Or => self.emit("    or %rbx, %rax"),
                        }
                    }
                    Architecture::X86 => {
                        self.emit("    mov %eax, %ebx");
                        self.emit("    pop %eax");
                        match op {
                            BinaryOp::Add => self.emit("    add %ebx, %eax"),
                            BinaryOp::Subtract => self.emit("    sub %ebx, %eax"),
                            BinaryOp::Multiply => self.emit("    imul %ebx, %eax"),
                            BinaryOp::Divide => {
                                self.emit("    cdq");
                                self.emit("    idiv %ebx");
                            }
                            BinaryOp::Modulo => {
                                self.emit("    cdq");
                                self.emit("    idiv %ebx");
                                self.emit("    mov %edx, %eax");
                            }
                            BinaryOp::Equal => {
                                self.emit("    cmp %ebx, %eax");
                                self.emit("    sete %al");
                                self.emit("    movzbl %al, %eax");
                            }
                            BinaryOp::NotEqual => {
                                self.emit("    cmp %ebx, %eax");
                                self.emit("    setne %al");
                                self.emit("    movzbl %al, %eax");
                            }
                            BinaryOp::Less => {
                                self.emit("    cmp %ebx, %eax");
                                self.emit("    setl %al");
                                self.emit("    movzbl %al, %eax");
                            }
                            BinaryOp::Greater => {
                                self.emit("    cmp %ebx, %eax");
                                self.emit("    setg %al");
                                self.emit("    movzbl %al, %eax");
                            }
                            BinaryOp::LessEqual => {
                                self.emit("    cmp %ebx, %eax");
                                self.emit("    setle %al");
                                self.emit("    movzbl %al, %eax");
                            }
                            BinaryOp::GreaterEqual => {
                                self.emit("    cmp %ebx, %eax");
                                self.emit("    setge %al");
                                self.emit("    movzbl %al, %eax");
                            }
                            BinaryOp::And => self.emit("    and %ebx, %eax"),
                            BinaryOp::Or => self.emit("    or %ebx, %eax"),
                        }
                    }
                }
            }
            Expr::Unary { op, expr } => {
                self.generate_expression(expr);
                match op {
                    UnaryOp::Negate => match self.arch {
                        Architecture::ARM64 => self.emit("    neg x0, x0"),
                        Architecture::X64 => self.emit("    neg %rax"),
                        Architecture::X86 => self.emit("    neg %eax"),
                    },
                    UnaryOp::Not => match self.arch {
                        Architecture::ARM64 => {
                            self.emit("    cmp x0, #0");
                            self.emit("    cset x0, eq");
                        }
                        Architecture::X64 => {
                            self.emit("    test %rax, %rax");
                            self.emit("    sete %al");
                            self.emit("    movzbq %al, %rax");
                        }
                        Architecture::X86 => {
                            self.emit("    test %eax, %eax");
                            self.emit("    sete %al");
                            self.emit("    movzbl %al, %eax");
                        }
                    },
                }
            }
            Expr::Call { name, args } => {
                match self.arch {
                    Architecture::X86 => {
                        for arg in args.iter().rev() {
                            self.generate_expression(arg);
                            self.emit("    push %eax");
                        }
                        self.emit(&format!("    call fn_{}", name));
                        if !args.is_empty() {
                            self.emit(&format!("    add ${}, %esp", args.len() * 4));
                        }
                    }
                    Architecture::X64 => {
                        for arg in args {
                            self.generate_expression(arg);
                            self.emit("    push %rax");
                        }
                        if matches!(self.os, OperatingSystem::Windows) {
                            for i in (0..args.len()).rev() {
                                let reg = match i {
                                    0 => "%rcx",
                                    1 => "%rdx",
                                    2 => "%r8",
                                    3 => "%r9",
                                    _ => "%rcx",
                                };
                                self.emit(&format!("    pop {}", reg));
                            }
                            let padding = if self.stack_offset % 16 == 0 { 32 } else { 40 };
                            self.emit(&format!("    sub ${}, %rsp", padding));
                            self.emit(&format!("    call fn_{}", name));
                            self.emit(&format!("    add ${}, %rsp", padding));
                        } else {
                            for i in (0..args.len()).rev() {
                                let reg = match i {
                                    0 => "%rdi",
                                    1 => "%rsi",
                                    2 => "%rdx",
                                    3 => "%rcx",
                                    4 => "%r8",
                                    5 => "%r9",
                                    _ => "%rdi",
                                };
                                self.emit(&format!("    pop {}", reg));
                            }
                            let misaligned = self.stack_offset % 16 != 0;
                            if misaligned {
                                self.emit("    sub $8, %rsp");
                            }
                            self.emit(&format!("    call fn_{}", name));
                            if misaligned {
                                self.emit("    add $8, %rsp");
                            }
                        }
                    }
                    Architecture::ARM64 => {
                        for arg in args {
                            self.generate_expression(arg);
                            self.emit("    str x0, [sp, #-16]!");
                        }
                        for i in (0..args.len()).rev() {
                            self.emit(&format!("    ldr x{}, [sp], #16", i));
                        }
                        self.emit(&format!("    bl fn_{}", name));
                    }
                }
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

    fn is_string_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::String(_) => true,
            Expr::InterpolatedString(_) => true,
            Expr::Identifier(name) => {
                if let Some(var_type) = self.variables.get(name) {
                    matches!(var_type, VarType::StringLabel(_) | VarType::StringOffset(_))
                } else {
                    false
                }
            }
            Expr::Binary { left, op: BinaryOp::Add, right } => {
                self.is_string_expr(left) || self.is_string_expr(right)
            }
            _ => false,
        }
    }

    fn generate_string_concat(&mut self, left: &Expr, right: &Expr) {
        self.generate_expression(left);
        match self.arch {
            Architecture::ARM64 => self.emit("    str x0, [sp, #-16]!"),
            Architecture::X64 => self.emit("    push %rax"),
            Architecture::X86 => self.emit("    push %eax"),
        }

        self.generate_expression(right);

        match self.arch {
            Architecture::ARM64 => {
                self.emit("    mov x1, x0");
                self.emit("    ldr x0, [sp], #16");
                self.emit("    bl alya_concat");
            }
            Architecture::X64 => {
                if matches!(self.os, OperatingSystem::Windows) {
                    self.emit("    mov %rax, %rdx");
                    self.emit("    pop %rcx");
                    let padding = if self.stack_offset % 16 == 0 { 32 } else { 40 };
                    self.emit(&format!("    sub ${}, %rsp", padding));
                    self.emit("    call alya_concat");
                    self.emit(&format!("    add ${}, %rsp", padding));
                } else {
                    self.emit("    mov %rax, %rsi");
                    self.emit("    pop %rdi");
                    let misaligned = self.stack_offset % 16 != 0;
                    if misaligned {
                        self.emit("    sub $8, %rsp");
                    }
                    self.emit("    call alya_concat");
                    if misaligned {
                        self.emit("    add $8, %rsp");
                    }
                }
            }
            Architecture::X86 => {
                self.emit("    mov %eax, %edx");
                self.emit("    pop %eax");
                self.emit("    push %edx");
                self.emit("    push %eax");
                self.emit("    call alya_concat");
                self.emit("    add $8, %esp");
            }
        }
    }

    fn generate_function(&mut self, name: &str, params: &[String], body: &[Stmt], program: &Program) {
        let saved_vars = self.variables.clone();
        let saved_stack_offset = self.stack_offset;
        let saved_loop_stack = std::mem::take(&mut self.loop_stack);

        self.stack_offset = 0;
        self.variables.clear();

        self.emit("");
        self.emit(&format!(".global fn_{}", name));
        self.emit(&format!("fn_{}:", name));

        match self.arch {
            Architecture::ARM64 => {
                self.emit("    stp x29, x30, [sp, #-16]!");
                self.emit("    mov x29, sp");
            }
            Architecture::X64 => {
                self.emit("    push %rbp");
                self.emit("    mov %rsp, %rbp");
            }
            Architecture::X86 => {
                self.emit("    push %ebp");
                self.emit("    mov %esp, %ebp");
            }
        }

        for (i, param) in params.iter().enumerate() {
            match self.arch {
                Architecture::ARM64 => {
                    self.stack_offset += 16;
                    self.emit(&format!("    str x{}, [sp, #-16]!", i));
                }
                Architecture::X64 => {
                    self.stack_offset += 8;
                    if matches!(self.os, OperatingSystem::Windows) {
                        let reg = match i {
                            0 => "%rcx",
                            1 => "%rdx",
                            2 => "%r8",
                            3 => "%r9",
                            _ => "%rcx",
                        };
                        self.emit(&format!("    push {}", reg));
                    } else {
                        let reg = match i {
                            0 => "%rdi",
                            1 => "%rsi",
                            2 => "%rdx",
                            3 => "%rcx",
                            4 => "%r8",
                            5 => "%r9",
                            _ => "%rdi",
                        };
                        self.emit(&format!("    push {}", reg));
                    }
                }
                Architecture::X86 => {
                    self.stack_offset += 4;
                    let src_offset = 8 + i * 4;
                    self.emit(&format!("    push {}(%ebp)", src_offset));
                }
            }

            let is_str = program.statements.iter().any(|s| {
                if let Some(arg) = find_call_arg(s, name, i) {
                    matches!(arg, Expr::String(_) | Expr::InterpolatedString(_))
                } else {
                    false
                }
            });

            if is_str {
                self.variables.insert(param.clone(), VarType::StringOffset(self.stack_offset));
            } else {
                self.variables.insert(param.clone(), VarType::Number(self.stack_offset));
            }
        }

        for stmt in body {
            self.generate_statement(stmt);
        }

        match self.arch {
            Architecture::ARM64 => {
                self.emit("    mov sp, x29");
                self.emit("    ldp x29, x30, [sp], #16");
                self.emit("    ret");
            }
            Architecture::X64 => {
                self.emit("    mov %rbp, %rsp");
                self.emit("    pop %rbp");
                self.emit("    ret");
            }
            Architecture::X86 => {
                self.emit("    mov %ebp, %esp");
                self.emit("    pop %ebp");
                self.emit("    ret");
            }
        }

        self.variables = saved_vars;
        self.stack_offset = saved_stack_offset;
        self.loop_stack = saved_loop_stack;
    }

    fn emit_runtime(&mut self) {
        self.emit("");
        self.emit(".section .bss");
        self.emit(".align 16");
        self.emit("alya_str_buf:");
        self.emit("    .space 65536");
        match self.arch {
            Architecture::ARM64 | Architecture::X64 => {
                self.emit("alya_str_idx:");
                self.emit("    .quad 0");
            }
            Architecture::X86 => {
                self.emit("alya_str_idx:");
                self.emit("    .long 0");
            }
        }
        self.emit("");
        self.emit(".text");
        match self.arch {
            Architecture::ARM64 => {
                self.emit(".align 2");
                self.emit("alya_concat:");
                self.emit("    stp x29, x30, [sp, #-16]!");
                self.emit("    mov x29, sp");
                self.emit("    stp x19, x20, [sp, #-16]!");
                self.emit("    stp x21, x22, [sp, #-16]!");
                self.emit("    adrp x19, alya_str_buf");
                self.emit("    add x19, x19, :lo12:alya_str_buf");
                self.emit("    adrp x20, alya_str_idx");
                self.emit("    add x20, x20, :lo12:alya_str_idx");
                self.emit("    ldr x21, [x20]");
                self.emit("    cmp x21, #48000");
                self.emit("    b.lt .L_arm_concat_ok");
                self.emit("    mov x21, #0");
                self.emit(".L_arm_concat_ok:");
                self.emit("    add x22, x19, x21");
                self.emit(".L_arm_copy1:");
                self.emit("    ldrb w2, [x0], #1");
                self.emit("    cbz w2, .L_arm_copy2_start");
                self.emit("    strb w2, [x22], #1");
                self.emit("    b .L_arm_copy1");
                self.emit(".L_arm_copy2_start:");
                self.emit(".L_arm_copy2:");
                self.emit("    ldrb w2, [x1], #1");
                self.emit("    cbz w2, .L_arm_concat_end");
                self.emit("    strb w2, [x22], #1");
                self.emit("    b .L_arm_copy2");
                self.emit(".L_arm_concat_end:");
                self.emit("    strb wzr, [x22], #1");
                self.emit("    sub x2, x22, x19");
                self.emit("    add x2, x2, #7");
                self.emit("    and x2, x2, #~7");
                self.emit("    str x2, [x20]");
                self.emit("    add x0, x19, x21");
                self.emit("    ldp x21, x22, [sp], #16");
                self.emit("    ldp x19, x20, [sp], #16");
                self.emit("    ldp x29, x30, [sp], #16");
                self.emit("    ret");
            }
            Architecture::X64 => {
                self.emit("alya_concat:");
                self.emit("    push %rsi");
                self.emit("    push %rdi");
                self.emit("    push %rbx");
                if matches!(self.os, OperatingSystem::Windows) {
                    self.emit("    mov %rcx, %rsi");
                    self.emit("    mov %rdx, %r10");
                } else {
                    self.emit("    mov %rsi, %r10");
                    self.emit("    mov %rdi, %rsi");
                }
                self.emit("    lea alya_str_buf(%rip), %r8");
                self.emit("    mov alya_str_idx(%rip), %rbx");
                self.emit("    cmp $48000, %rbx");
                self.emit("    jl .L_x64_concat_ok");
                self.emit("    xor %rbx, %rbx");
                self.emit(".L_x64_concat_ok:");
                self.emit("    lea (%r8, %rbx), %rdi");
                self.emit("    mov %rdi, %rax");
                self.emit(".L_x64_copy1:");
                self.emit("    movb (%rsi), %cl");
                self.emit("    test %cl, %cl");
                self.emit("    jz .L_x64_copy2_start");
                self.emit("    movb %cl, (%rdi)");
                self.emit("    inc %rsi");
                self.emit("    inc %rdi");
                self.emit("    jmp .L_x64_copy1");
                self.emit(".L_x64_copy2_start:");
                self.emit("    mov %r10, %rsi");
                self.emit(".L_x64_copy2:");
                self.emit("    movb (%rsi), %cl");
                self.emit("    test %cl, %cl");
                self.emit("    jz .L_x64_concat_end");
                self.emit("    movb %cl, (%rdi)");
                self.emit("    inc %rsi");
                self.emit("    inc %rdi");
                self.emit("    jmp .L_x64_copy2");
                self.emit(".L_x64_concat_end:");
                self.emit("    movb $0, (%rdi)");
                self.emit("    inc %rdi");
                self.emit("    sub %r8, %rdi");
                self.emit("    add $7, %rdi");
                self.emit("    and $-8, %rdi");
                self.emit("    mov %rdi, alya_str_idx(%rip)");
                self.emit("    pop %rbx");
                self.emit("    pop %rdi");
                self.emit("    pop %rsi");
                self.emit("    ret");
            }
            Architecture::X86 => {
                self.emit("alya_concat:");
                self.emit("    push %ebp");
                self.emit("    mov %esp, %ebp");
                self.emit("    push %esi");
                self.emit("    push %edi");
                self.emit("    push %ebx");
                self.emit("    mov 8(%ebp), %esi");
                self.emit("    mov 12(%ebp), %edx");
                self.emit("    mov $alya_str_buf, %ecx");
                self.emit("    mov alya_str_idx, %ebx");
                self.emit("    cmp $48000, %ebx");
                self.emit("    jl .L_x86_concat_ok");
                self.emit("    xor %ebx, %ebx");
                self.emit(".L_x86_concat_ok:");
                self.emit("    lea (%ecx, %ebx), %edi");
                self.emit("    mov %edi, %eax");
                self.emit(".L_x86_copy1:");
                self.emit("    movb (%esi), %bl");
                self.emit("    test %bl, %bl");
                self.emit("    jz .L_x86_copy2_start");
                self.emit("    movb %bl, (%edi)");
                self.emit("    inc %esi");
                self.emit("    inc %edi");
                self.emit("    jmp .L_x86_copy1");
                self.emit(".L_x86_copy2_start:");
                self.emit("    mov %edx, %esi");
                self.emit(".L_x86_copy2:");
                self.emit("    movb (%esi), %bl");
                self.emit("    test %bl, %bl");
                self.emit("    jz .L_x86_concat_end");
                self.emit("    movb %bl, (%edi)");
                self.emit("    inc %esi");
                self.emit("    inc %edi");
                self.emit("    jmp .L_x86_copy2");
                self.emit(".L_x86_concat_end:");
                self.emit("    movb $0, (%edi)");
                self.emit("    inc %edi");
                self.emit("    sub %ecx, %edi");
                self.emit("    add $3, %edi");
                self.emit("    and $-4, %edi");
                self.emit("    mov %edi, alya_str_idx");
                self.emit("    pop %ebx");
                self.emit("    pop %edi");
                self.emit("    pop %esi");
                self.emit("    mov %ebp, %esp");
                self.emit("    pop %ebp");
                self.emit("    ret");
            }
        }
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}

fn find_call_arg<'a>(stmt: &'a Stmt, func_name: &str, param_idx: usize) -> Option<&'a Expr> {
    match stmt {
        Stmt::Expr(expr) | Stmt::Say(expr) => find_call_arg_in_expr(expr, func_name, param_idx),
        Stmt::Let { value, .. } | Stmt::Assign { value, .. } => {
            find_call_arg_in_expr(value, func_name, param_idx)
        }
        Stmt::If { condition, then_block, else_block } => {
            if let Some(arg) = find_call_arg_in_expr(condition, func_name, param_idx) {
                return Some(arg);
            }
            for s in then_block {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            if let Some(else_stmts) = else_block {
                for s in else_stmts {
                    if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                        return Some(arg);
                    }
                }
            }
            None
        }
        Stmt::While { condition, body } => {
            if let Some(arg) = find_call_arg_in_expr(condition, func_name, param_idx) {
                return Some(arg);
            }
            for s in body {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        Stmt::For { body, .. } => {
            for s in body {
                if let Some(arg) = find_call_arg(s, func_name, param_idx) {
                    return Some(arg);
                }
            }
            None
        }
        _ => None,
    }
}

fn find_call_arg_in_expr<'a>(expr: &'a Expr, func_name: &str, param_idx: usize) -> Option<&'a Expr> {
    match expr {
        Expr::Call { name, args } if name == func_name => {
            args.get(param_idx)
        }
        Expr::Binary { left, right, .. } => {
            find_call_arg_in_expr(left, func_name, param_idx)
                .or_else(|| find_call_arg_in_expr(right, func_name, param_idx))
        }
        Expr::Unary { expr, .. } => find_call_arg_in_expr(expr, func_name, param_idx),
        _ => None,
    }
}

pub fn generate(program: &Program, arch: Architecture, os: OperatingSystem) -> String {
    let mut codegen = CodeGen::new(arch, os);
    codegen.generate_program(program);
    codegen.output
}
