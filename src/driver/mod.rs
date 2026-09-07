use crate::cli::{CliArgs, CommandKind};
use crate::codegen::{self, Architecture, OperatingSystem};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::path::Path;
pub mod runner;

pub fn run(args: CliArgs) -> Result<(), String> {
    let source = fs::read_to_string(&args.input_file)
        .map_err(|e| format!("Error: Cannot read file '{}': {}", args.input_file, e))?;

    // 1. Lexical Analysis
    let mut lexer = Lexer::new(&source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| crate::diagnostics::render_error(&args.input_file, &source, &e))?;

    if args.command == CommandKind::EmitTokens {
        println!("{:<12} {:<30}", "POSITION", "TOKEN");
        println!("{:-<12} {:-<30}", "", "");
        for t in &tokens {
            println!(
                "{:<12} {:?}",
                format!("{}:{}", t.line, t.column),
                t.token_type
            );
        }
        return Ok(());
    }

    // 2. Syntactic Analysis (Parsing)
    let mut parser = Parser::new(tokens);
    let ast = parser
        .parse()
        .map_err(|e| crate::diagnostics::render_error(&args.input_file, &source, &e))?;

    if args.command == CommandKind::EmitAst {
        println!("{:#?}", ast);
        return Ok(());
    }

    if args.command == CommandKind::Check {
        if !args.quiet {
            println!("✓ Syntax OK: {}", args.input_file);
        }
        return Ok(());
    }

    // Warnings for cross-compilation on Windows host
    if cfg!(target_os = "windows") {
        if matches!(args.arch, Architecture::ARM64) {
            eprintln!("Warning: ARM64 assembly generation is not supported on Windows MinGW/GCC.");
            eprintln!("         Windows GCC can only assemble x64 and x86 code.");
            eprintln!(
                "         The generated ARM64 assembly is valid but requires an ARM64 assembler."
            );
            eprintln!();
        }
        if matches!(args.arch, Architecture::X86) {
            eprintln!("Warning: x86 (32-bit) compilation requires gcc with multilib support.");
            eprintln!("         On Windows, you may need MinGW-w64 with 32-bit support.");
            eprintln!("         Try: gcc -m32 output.s -o program");
            eprintln!();
        }
    }

    // Determine smart base name from input file
    let default_stem = Path::new(&args.input_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let is_binary = args.output_binary || args.command == CommandKind::Run;

    let (asm_file, final_output) = if is_binary {
        let temp_asm = format!("temp_{}_{}.s", default_stem, std::process::id());
        let exe_name = args.output_file.clone().unwrap_or_else(|| {
            if matches!(args.os, OperatingSystem::Windows) {
                format!("{}.exe", default_stem)
            } else {
                default_stem.to_string()
            }
        });
        (temp_asm, Some(exe_name))
    } else {
        let asm_name = args
            .output_file
            .clone()
            .unwrap_or_else(|| format!("{}.s", default_stem));
        (asm_name, None)
    };

    // 3. Code Generation
    let code = codegen::generate(&ast, args.arch, args.os);

    fs::write(&asm_file, code)
        .map_err(|e| format!("Error: Cannot write to '{}': {}", asm_file, e))?;

    if let Some(exe_file) = final_output {
        if !args.quiet && args.command != CommandKind::Run {
            println!("Compiling to executable: {}", exe_file);
        }

        runner::compile_with_gcc(&asm_file, &exe_file, args.arch, args.os)?;

        if args.command == CommandKind::Run {
            runner::execute_binary(&exe_file, args.output_file.is_none())?;
        } else if !args.quiet {
            println!("✓ Successfully compiled to {}", exe_file);
            println!("\nRun your program:");
            if cfg!(target_os = "windows") {
                println!("  .\\{}", exe_file);
            } else {
                println!("  ./{}", exe_file);
            }
        }
    } else if !args.quiet {
        println!("Compiled successfully to {}", asm_file);
        println!("\nTo create executable:");
        println!("  gcc {} -o {} -no-pie", asm_file, default_stem);
        if cfg!(target_os = "windows") {
            println!("  .\\{}.exe", default_stem);
        } else {
            println!("  ./{}", default_stem);
        }
    }

    Ok(())
}
