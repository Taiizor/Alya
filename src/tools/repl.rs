use crate::ast::Stmt;
use crate::codegen::{self, Architecture, OperatingSystem};
use crate::driver::runner;
use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Checks whether an input buffer represents an incomplete statement or block
/// that requires further lines of input before being compiled and executed.
pub fn is_input_incomplete(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return false;
    }

    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => {
            // Lexer errors usually indicate unclosed string literals or multiline quotes
            return true;
        }
    };

    let mut block_depth: i32 = 0;
    let mut paren_depth: i32 = 0;

    for token in &tokens {
        match &token.token_type {
            TokenType::Function
            | TokenType::If
            | TokenType::While
            | TokenType::Repeat
            | TokenType::For
            | TokenType::When
            | TokenType::Try
            | TokenType::Struct => {
                block_depth += 1;
            }
            TokenType::End => {
                block_depth = (block_depth - 1).max(0);
            }
            TokenType::LeftParen | TokenType::LeftBracket | TokenType::LeftBrace => {
                paren_depth += 1;
            }
            TokenType::RightParen | TokenType::RightBracket | TokenType::RightBrace => {
                paren_depth = (paren_depth - 1).max(0);
            }
            _ => {}
        }
    }

    if block_depth > 0 || paren_depth > 0 {
        return true;
    }

    // Check if the last meaningful token is a continuation operator
    let last_meaningful_token = tokens
        .iter()
        .rev()
        .find(|t| !matches!(t.token_type, TokenType::Newline | TokenType::Eof));

    if let Some(t) = last_meaningful_token {
        match &t.token_type {
            TokenType::Comma
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Multiply
            | TokenType::Divide
            | TokenType::Modulo
            | TokenType::Assign
            | TokenType::Equal
            | TokenType::NotEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::And
            | TokenType::Or
            | TokenType::BitAnd
            | TokenType::BitOr
            | TokenType::BitXor
            | TokenType::Colon
            | TokenType::Question => return true,
            _ => {}
        }
    }

    false
}

/// Persistent state of an interactive REPL session.
pub struct ReplSession {
    pub imports: Vec<String>,
    pub structs: Vec<(String, String)>,
    pub functions: Vec<(String, String)>,
    pub statements: Vec<String>,
    pub var_names: Vec<String>,
    pub arch: Architecture,
    pub os: OperatingSystem,
}

impl ReplSession {
    pub fn new(arch: Architecture, os: OperatingSystem) -> Self {
        Self {
            imports: Vec::new(),
            structs: Vec::new(),
            functions: Vec::new(),
            statements: Vec::new(),
            var_names: Vec::new(),
            arch,
            os,
        }
    }

    /// Assembles all persistent definitions and statements into a single source string,
    /// optionally appending a trailing snippet to be evaluated.
    pub fn assemble_program(&self, trailing_code: &str) -> String {
        let mut code = String::new();

        for imp in &self.imports {
            code.push_str(imp);
            code.push('\n');
        }

        for (_, s_code) in &self.structs {
            code.push_str(s_code);
            code.push('\n');
        }

        for (_, f_code) in &self.functions {
            code.push_str(f_code);
            code.push('\n');
        }

        for stmt in &self.statements {
            code.push_str(stmt);
            code.push('\n');
        }

        if !trailing_code.is_empty() {
            code.push_str(trailing_code);
            code.push('\n');
        }

        code
    }

    /// Clears all session definitions and history.
    pub fn clear(&mut self) {
        self.imports.clear();
        self.structs.clear();
        self.functions.clear();
        self.statements.clear();
        self.var_names.clear();
    }

    /// Compiles and executes the current session combined with the provided snippet.
    pub fn execute_snippet(&self, snippet: &str) -> Result<(bool, String, String), String> {
        let source = self.assemble_program(snippet);
        execute_code_snippet(&source, self.arch, self.os)
    }

    /// Processes a complete user input chunk.
    pub fn eval_input(&mut self, input: &str) {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return;
        }

        // Handle REPL commands
        if trimmed.starts_with(':') {
            self.handle_command(trimmed);
            return;
        }

        // 1. Tokenize input chunk
        let mut lexer = Lexer::new(trimmed);
        let tokens = match lexer.tokenize() {
            Ok(toks) => toks,
            Err(err) => {
                eprintln!("\x1b[1;31mSyntax Error:\x1b[0m {}", err);
                return;
            }
        };

        // 2. Parse input chunk
        let mut parser = Parser::new(tokens);
        let parsed_program = match parser.parse() {
            Ok(prog) => prog,
            Err(err) => {
                eprintln!("\x1b[1;31mParse Error:\x1b[0m {}", err);
                return;
            }
        };

        // 3. Process statements
        if parsed_program.statements.is_empty() {
            return;
        }

        // Check if single statement is a bare expression
        if parsed_program.statements.len() == 1 {
            match &parsed_program.statements[0] {
                Stmt::Expr(_) => {
                    // Evaluate as an expression wrapped in say (...)
                    let eval_code = format!("say ({})", trimmed);
                    match self.execute_snippet(&eval_code) {
                        Ok((true, stdout, stderr)) => {
                            let out = stdout.trim_end();
                            if !out.is_empty() {
                                println!("\x1b[1;36m=>\x1b[0m \x1b[1;32m{}\x1b[0m", out);
                            }
                            if !stderr.trim().is_empty() {
                                eprintln!("\x1b[1;33m{}\x1b[0m", stderr.trim_end());
                            }
                        }
                        Ok((false, stdout, stderr)) => {
                            let err_msg = if !stderr.trim().is_empty() {
                                stderr.trim()
                            } else {
                                stdout.trim()
                            };
                            eprintln!("\x1b[1;31mRuntime Error:\x1b[0m {}", err_msg);
                        }
                        Err(err) => {
                            eprintln!("\x1b[1;31mError:\x1b[0m {}", err);
                        }
                    }
                    return;
                }
                Stmt::Import { path, .. } => {
                    let imp_str = trimmed.to_string();
                    if !self.imports.contains(&imp_str) {
                        self.imports.push(imp_str.clone());
                    }
                    match self.execute_snippet("") {
                        Ok((true, _, _)) => {
                            println!("\x1b[1;36m=>\x1b[0m module '{}' loaded", path);
                        }
                        Ok((false, stdout, stderr)) => {
                            self.imports.retain(|s| s != &imp_str);
                            let err_msg = if !stderr.trim().is_empty() {
                                stderr.trim()
                            } else {
                                stdout.trim()
                            };
                            eprintln!("\x1b[1;31mImport Error:\x1b[0m {}", err_msg);
                        }
                        Err(err) => {
                            self.imports.retain(|s| s != &imp_str);
                            eprintln!("\x1b[1;31mImport Error:\x1b[0m {}", err);
                        }
                    }
                    return;
                }
                Stmt::Function { name, .. } => {
                    let func_name = name.clone();
                    let func_code = trimmed.to_string();

                    // Temporarily update function definition
                    let mut prev = None;
                    if let Some(pos) = self.functions.iter().position(|(n, _)| n == &func_name) {
                        prev = Some((pos, self.functions.remove(pos)));
                    }
                    self.functions.push((func_name.clone(), func_code));

                    match self.execute_snippet("") {
                        Ok((true, _, _)) => {
                            println!("\x1b[1;36m=>\x1b[0m function {} defined", func_name);
                        }
                        Ok((false, stdout, stderr)) => {
                            self.functions.pop();
                            if let Some((pos, entry)) = prev {
                                self.functions.insert(pos, entry);
                            }
                            let err_msg = if !stderr.trim().is_empty() {
                                stderr.trim()
                            } else {
                                stdout.trim()
                            };
                            eprintln!("\x1b[1;31mFunction Error:\x1b[0m {}", err_msg);
                        }
                        Err(err) => {
                            self.functions.pop();
                            if let Some((pos, entry)) = prev {
                                self.functions.insert(pos, entry);
                            }
                            eprintln!("\x1b[1;31mFunction Error:\x1b[0m {}", err);
                        }
                    }
                    return;
                }
                Stmt::StructDef { name, .. } => {
                    let struct_name = name.clone();
                    let struct_code = trimmed.to_string();

                    let mut prev = None;
                    if let Some(pos) = self.structs.iter().position(|(n, _)| n == &struct_name) {
                        prev = Some((pos, self.structs.remove(pos)));
                    }
                    self.structs.push((struct_name.clone(), struct_code));

                    match self.execute_snippet("") {
                        Ok((true, _, _)) => {
                            println!("\x1b[1;36m=>\x1b[0m struct {} defined", struct_name);
                        }
                        Ok((false, stdout, stderr)) => {
                            self.structs.pop();
                            if let Some((pos, entry)) = prev {
                                self.structs.insert(pos, entry);
                            }
                            let err_msg = if !stderr.trim().is_empty() {
                                stderr.trim()
                            } else {
                                stdout.trim()
                            };
                            eprintln!("\x1b[1;31mStruct Error:\x1b[0m {}", err_msg);
                        }
                        Err(err) => {
                            self.structs.pop();
                            if let Some((pos, entry)) = prev {
                                self.structs.insert(pos, entry);
                            }
                            eprintln!("\x1b[1;31mStruct Error:\x1b[0m {}", err);
                        }
                    }
                    return;
                }
                Stmt::Let { name, .. } => {
                    let var_name = name.clone();
                    // Execute let statement and inspect its value
                    let eval_code = format!("{}\nsay ({})", trimmed, var_name);
                    match self.execute_snippet(&eval_code) {
                        Ok((true, stdout, stderr)) => {
                            self.statements.push(trimmed.to_string());
                            if !self.var_names.contains(&var_name) {
                                self.var_names.push(var_name);
                            }
                            let out = stdout.trim_end();
                            if !out.is_empty() {
                                println!("\x1b[1;36m=>\x1b[0m \x1b[1;32m{}\x1b[0m", out);
                            }
                            if !stderr.trim().is_empty() {
                                eprintln!("\x1b[1;33m{}\x1b[0m", stderr.trim_end());
                            }
                        }
                        Ok((false, stdout, stderr)) => {
                            let err_msg = if !stderr.trim().is_empty() {
                                stderr.trim()
                            } else {
                                stdout.trim()
                            };
                            eprintln!("\x1b[1;31mRuntime Error:\x1b[0m {}", err_msg);
                        }
                        Err(err) => {
                            eprintln!("\x1b[1;31mError:\x1b[0m {}", err);
                        }
                    }
                    return;
                }
                Stmt::Assign { name, .. } => {
                    let var_name = name.clone();
                    let eval_code = format!("{}\nsay ({})", trimmed, var_name);
                    match self.execute_snippet(&eval_code) {
                        Ok((true, stdout, stderr)) => {
                            self.statements.push(trimmed.to_string());
                            let out = stdout.trim_end();
                            if !out.is_empty() {
                                println!("\x1b[1;36m=>\x1b[0m \x1b[1;32m{}\x1b[0m", out);
                            }
                            if !stderr.trim().is_empty() {
                                eprintln!("\x1b[1;33m{}\x1b[0m", stderr.trim_end());
                            }
                        }
                        Ok((false, stdout, stderr)) => {
                            let err_msg = if !stderr.trim().is_empty() {
                                stderr.trim()
                            } else {
                                stdout.trim()
                            };
                            eprintln!("\x1b[1;31mRuntime Error:\x1b[0m {}", err_msg);
                        }
                        Err(err) => {
                            eprintln!("\x1b[1;31mError:\x1b[0m {}", err);
                        }
                    }
                    return;
                }
                _ => {}
            }
        }

        // For other statements or multi-statement blocks (loops, ifs, says)
        let has_mutations = parsed_program.statements.iter().any(|s| {
            matches!(
                s,
                Stmt::Let { .. }
                    | Stmt::Assign { .. }
                    | Stmt::IndexAssign { .. }
                    | Stmt::FieldAssign { .. }
            )
        });

        match self.execute_snippet(trimmed) {
            Ok((true, stdout, stderr)) => {
                if has_mutations {
                    self.statements.push(trimmed.to_string());
                    for s in &parsed_program.statements {
                        if let Stmt::Let { name, .. } = s {
                            if !self.var_names.contains(name) {
                                self.var_names.push(name.clone());
                            }
                        }
                    }
                }
                if !stdout.is_empty() {
                    print!("{}", stdout);
                }
                if !stderr.is_empty() {
                    eprint!("{}", stderr);
                }
            }
            Ok((false, stdout, stderr)) => {
                let err_msg = if !stderr.trim().is_empty() {
                    stderr.trim()
                } else {
                    stdout.trim()
                };
                eprintln!("\x1b[1;31mRuntime Error:\x1b[0m {}", err_msg);
            }
            Err(err) => {
                eprintln!("\x1b[1;31mError:\x1b[0m {}", err);
            }
        }
    }

    fn handle_command(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let name = parts[0];

        match name {
            ":exit" | ":quit" | ":q" => {
                println!("\x1b[1;33mGoodbye!\x1b[0m");
                std::process::exit(0);
            }
            ":clear" | ":c" => {
                self.clear();
                println!("\x1b[1;32m✓ Session state cleared.\x1b[0m");
            }
            ":vars" | ":v" => {
                println!("\x1b[1;36m=== Active Session State ===\x1b[0m");
                if self.imports.is_empty()
                    && self.structs.is_empty()
                    && self.functions.is_empty()
                    && self.var_names.is_empty()
                {
                    println!("  (No variables or functions defined yet)");
                    return;
                }

                if !self.imports.is_empty() {
                    println!("\x1b[1;33mImports:\x1b[0m");
                    for imp in &self.imports {
                        println!("  {}", imp);
                    }
                }

                if !self.structs.is_empty() {
                    println!("\x1b[1;33mStructs:\x1b[0m");
                    for (name, _) in &self.structs {
                        println!("  struct {}", name);
                    }
                }

                if !self.functions.is_empty() {
                    println!("\x1b[1;33mFunctions:\x1b[0m");
                    for (name, _) in &self.functions {
                        println!("  function {}", name);
                    }
                }

                if !self.var_names.is_empty() {
                    println!("\x1b[1;33mVariables:\x1b[0m");
                    for name in &self.var_names {
                        println!("  {}", name);
                    }
                }
            }
            ":code" => {
                let code = self.assemble_program("");
                println!("\x1b[1;36m=== Current Session Code ===\x1b[0m");
                if code.trim().is_empty() {
                    println!("  (Empty session)");
                } else {
                    println!("{}", code.trim());
                }
                println!("\x1b[1;36m============================\x1b[0m");
            }
            ":help" | ":h" => {
                println!(
                    "\x1b[1;36m╔════════════════════════════════════════════════════════╗\x1b[0m"
                );
                println!("\x1b[1;36m║                  \x1b[1;33mALYA REPL COMMANDS\x1b[0m                    \x1b[1;36m║\x1b[0m");
                println!(
                    "\x1b[1;36m╠════════════════════════════════════════════════════════╣\x1b[0m"
                );
                println!("\x1b[1;36m║\x1b[0m  \x1b[1;32m:help, :h\x1b[0m     Show this help guide                     \x1b[1;36m║\x1b[0m");
                println!("\x1b[1;36m║\x1b[0m  \x1b[1;32m:vars, :v\x1b[0m     List defined variables and functions     \x1b[1;36m║\x1b[0m");
                println!("\x1b[1;36m║\x1b[0m  \x1b[1;32m:code\x1b[0m         View accumulated session code            \x1b[1;36m║\x1b[0m");
                println!("\x1b[1;36m║\x1b[0m  \x1b[1;32m:clear, :c\x1b[0m    Reset session state                      \x1b[1;36m║\x1b[0m");
                println!("\x1b[1;36m║\x1b[0m  \x1b[1;32m:exit, :q\x1b[0m     Exit the REPL                            \x1b[1;36m║\x1b[0m");
                println!(
                    "\x1b[1;36m╠════════════════════════════════════════════════════════╣\x1b[0m"
                );
                println!("\x1b[1;36m║\x1b[0m  \x1b[1;33mFeatures:\x1b[0m                                             \x1b[1;36m║\x1b[0m");
                println!("\x1b[1;36m║\x1b[0m  • Expressions (e.g. 1 + 2, [1, 2, 3]) auto-evaluate   \x1b[1;36m║\x1b[0m");
                println!("\x1b[1;36m║\x1b[0m  • Multi-line blocks continue until 'end' keyword       \x1b[1;36m║\x1b[0m");
                println!("\x1b[1;36m║\x1b[0m  • Standard library support (e.g. import \"std/math\")   \x1b[1;36m║\x1b[0m");
                println!(
                    "\x1b[1;36m╚════════════════════════════════════════════════════════╝\x1b[0m"
                );
            }
            other => {
                eprintln!(
                    "\x1b[1;31mUnknown command '{}'. Type :help for available commands.\x1b[0m",
                    other
                );
            }
        }
    }
}

/// Compiles and runs a source snippet, returning (success, stdout, stderr).
pub fn execute_code_snippet(
    source: &str,
    arch: Architecture,
    os: OperatingSystem,
) -> Result<(bool, String, String), String> {
    if source.trim().is_empty() {
        return Ok((true, String::new(), String::new()));
    }

    // 1. Lexer
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    // 2. Parser
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

    // 3. Module Resolution
    let base_dir = Path::new(".");
    crate::parser::resolve_imports(&mut ast, base_dir)
        .map_err(|e| format!("Import error: {}", e))?;

    // 4. Codegen
    let asm_code = codegen::generate(&ast, arch, os);

    // 5. Compile with GCC to temp executable
    let pid = std::process::id();
    let rand_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        % 1_000_000_000;
    let temp_asm = format!("temp_repl_{}_{}.s", pid, rand_id);
    let temp_exe = if matches!(os, OperatingSystem::Windows) {
        format!("temp_repl_{}_{}.exe", pid, rand_id)
    } else {
        format!("temp_repl_{}_{}", pid, rand_id)
    };

    fs::write(&temp_asm, &asm_code)
        .map_err(|e| format!("Failed to write temporary assembly: {}", e))?;

    let gcc_res = runner::compile_with_gcc(&temp_asm, &temp_exe, arch, os);
    let _ = fs::remove_file(&temp_asm);

    gcc_res?;

    // 6. Execute binary
    let exe_path = if matches!(os, OperatingSystem::Windows) {
        format!(".\\{}", temp_exe)
    } else {
        format!("./{}", temp_exe)
    };

    let run_res = Command::new(&exe_path)
        .stdin(std::process::Stdio::inherit())
        .output();

    let _ = fs::remove_file(&temp_exe);

    match run_res {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok((output.status.success(), stdout, stderr))
        }
        Err(e) => Err(format!("Execution failed: {}", e)),
    }
}

/// Starts the interactive REPL read-eval-print loop.
pub fn start_repl(arch: Architecture, os: OperatingSystem) -> Result<(), String> {
    println!("\x1b[1;36m╔════════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;36m║             \x1b[1;33m⚡ ALYA INTERACTIVE REPL ⚡\x1b[0m                \x1b[1;36m║\x1b[0m");
    println!(
        "\x1b[1;36m║  \x1b[0mVersion {} ({}-{})              \x1b[1;36m║\x1b[0m",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    println!("\x1b[1;36m║  \x1b[0mType \x1b[1;32m:help\x1b[0m for commands, \x1b[1;31m:exit\x1b[0m to quit               \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m╚════════════════════════════════════════════════════════╝\x1b[0m\n");

    let mut session = ReplSession::new(arch, os);
    let mut input_buffer = String::new();

    let stdin = io::stdin();

    loop {
        if input_buffer.is_empty() {
            print!("\x1b[1;32malya>\x1b[0m ");
        } else {
            print!("\x1b[1;34m ...>\x1b[0m ");
        }
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut line = String::new();
        let bytes_read = stdin.read_line(&mut line).map_err(|e| e.to_string())?;

        if bytes_read == 0 {
            // EOF reached (Ctrl+D / Ctrl+Z)
            println!("\n\x1b[1;33mGoodbye!\x1b[0m");
            break;
        }

        let trimmed_line = line.trim();

        // Allow cancelling incomplete multiline inputs with :reset, :cancel
        if !input_buffer.is_empty() && (trimmed_line == ":reset" || trimmed_line == ":cancel") {
            input_buffer.clear();
            println!("\x1b[1;33mInput discarded.\x1b[0m");
            continue;
        }

        input_buffer.push_str(&line);

        if !is_input_incomplete(&input_buffer) {
            let full_input = std::mem::take(&mut input_buffer);
            session.eval_input(&full_input);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completeness_checker() {
        assert!(!is_input_incomplete("1 + 2"));
        assert!(!is_input_incomplete("let x = 10"));
        assert!(!is_input_incomplete("say \"hello\""));

        // Incomplete function
        assert!(is_input_incomplete("function add(a, b)"));
        assert!(!is_input_incomplete("function add(a, b) return a + b end"));

        // Incomplete loop
        assert!(is_input_incomplete("for i in 1..5"));
        assert!(!is_input_incomplete("for i in 1..5 say i end"));

        // Incomplete brackets
        assert!(is_input_incomplete("[1, 2,"));
        assert!(!is_input_incomplete("[1, 2, 3]"));

        // Incomplete operators
        assert!(is_input_incomplete("10 +"));
        assert!(is_input_incomplete("x =="));
    }

    #[test]
    fn test_repl_session_assemble() {
        let mut session = ReplSession::new(Architecture::X64, OperatingSystem::Windows);
        session.imports.push("import \"std/math\"".into());
        session.functions.push((
            "double".into(),
            "function double(x) return x * 2 end".into(),
        ));
        session.statements.push("let x = 10".into());

        let code = session.assemble_program("say double(x)");
        assert!(code.contains("import \"std/math\""));
        assert!(code.contains("function double(x) return x * 2 end"));
        assert!(code.contains("let x = 10"));
        assert!(code.contains("say double(x)"));
    }

    #[test]
    fn test_repl_execute_snippet() {
        let arch = if cfg!(target_arch = "aarch64") {
            Architecture::ARM64
        } else if cfg!(target_arch = "x86") {
            Architecture::X86
        } else {
            Architecture::X64
        };
        let os = if cfg!(target_os = "windows") {
            OperatingSystem::Windows
        } else if cfg!(target_os = "macos") {
            OperatingSystem::MacOS
        } else {
            OperatingSystem::Linux
        };

        let res = execute_code_snippet("say 25 * 4", arch, os);
        if let Ok((success, stdout, _)) = res {
            assert!(success);
            assert_eq!(stdout.trim(), "100");
        }
    }
}
