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
        self.emit_header();

        for stmt in &program.statements {
            self.generate_statement(stmt);
        }

        self.emit_footer();
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
                        // For string variables, create a label and store the label name
                        let label = self.next_string_label();
                        self.emit(".section .rodata");
                        self.emit(&format!("{}:", label));
                        self.emit(&format!("    .string \"{}\"", escape_string(s)));
                        self.emit(".text");
                        self.variables.insert(name.clone(), VarType::StringLabel(label));
                    }
                    _ => {
                        // For numeric expressions, evaluate and store on stack
                        self.generate_expression(value);
                        
                        // Allocate space on stack for the variable
                        match self.arch {
                            Architecture::ARM64 => {
                                self.stack_offset += 8;
                                self.emit(&format!("    str x0, [sp, #-{}]!", 8));
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
                        self.variables.insert(name.clone(), VarType::Number(self.stack_offset));
                    }
                }
            }
            Stmt::Assign { name, value } => {
                // Evaluate the new value
                self.generate_expression(value);
                
                // Get variable location from symbol table
                if let Some(var_type) = self.variables.get(name) {
                    match var_type {
                        VarType::Number(offset) => {
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
                            // String reassignment not supported yet
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
            _ => {
                // Other statements not yet implemented
            }
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
                self.generate_expression(expr);
                
                let fmt_label = self.next_string_label();
                self.emit(".section .rodata");
                self.emit(&format!("{}:", fmt_label));
                self.emit("    .string \"%ld\\n\"");
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
            Expr::Identifier(name) => {
                // Load variable from stack
                if let Some(var_type) = self.variables.get(name) {
                    match var_type {
                        VarType::Number(offset) => {
                            let offset = *offset;
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
                        VarType::StringLabel(_) => {
                            // String identifiers are handled specially in generate_say
                            // This shouldn't be reached in normal expression evaluation
                        }
                    }
                }
            }
            Expr::Binary { left, op, right } => {
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
            _ => {
                // Other expressions not yet implemented
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

pub fn generate(program: &Program, arch: Architecture, os: OperatingSystem) -> String {
    let mut codegen = CodeGen::new(arch, os);
    codegen.generate_program(program);
    codegen.output
}
