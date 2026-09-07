use crate::cli::CliArgs;
use crate::codegen::{self, Architecture, OperatingSystem};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::process::Command;

pub fn run(args: CliArgs) -> Result<(), String> {
    // Validate architecture/OS compatibility warnings
    if cfg!(target_os = "windows") {
        if matches!(args.arch, Architecture::ARM64) {
            eprintln!("Warning: ARM64 assembly generation is not supported on Windows MinGW/GCC.");
            eprintln!("         Windows GCC can only assemble x64 and x86 code.");
            eprintln!("         The generated ARM64 assembly is valid but requires an ARM64 assembler.");
            eprintln!();
        }
        if matches!(args.arch, Architecture::X86) {
            eprintln!("Warning: x86 (32-bit) compilation requires gcc with multilib support.");
            eprintln!("         On Windows, you may need MinGW-w64 with 32-bit support.");
            eprintln!("         Try: gcc -m32 output.s -o program");
            eprintln!();
        }
    }

    let (asm_file, final_output) = if args.output_binary {
        let temp_asm = "temp_alya_output.s".to_string();
        let exe_name = args.output_file.unwrap_or_else(|| {
            if cfg!(target_os = "windows") {
                "program.exe".to_string()
            } else {
                "program".to_string()
            }
        });
        (temp_asm, Some(exe_name))
    } else {
        let asm_name = args.output_file.unwrap_or_else(|| "output.s".to_string());
        (asm_name, None)
    };

    let source = fs::read_to_string(&args.input_file)
        .map_err(|e| format!("Error: Cannot read file '{}': {}", args.input_file, e))?;

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    let code = codegen::generate(&ast, args.arch, args.os);

    fs::write(&asm_file, code)
        .map_err(|e| format!("Error: Cannot write to '{}': {}", asm_file, e))?;

    if let Some(exe_file) = final_output {
        println!("Compiling to executable: {}", exe_file);

        let mut gcc_args = vec![asm_file.as_str(), "-o", exe_file.as_str()];

        if matches!(args.arch, Architecture::X86) {
            gcc_args.insert(0, "-m32");
        }

        if !matches!(args.os, OperatingSystem::Windows) {
            gcc_args.push("-no-pie");
        }

        let gcc_result = Command::new("gcc")
            .args(&gcc_args)
            .output();

        match gcc_result {
            Ok(output) => {
                let _ = fs::remove_file(&asm_file);

                if !output.status.success() {
                    return Err(format!(
                        "GCC compilation failed:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }

                println!("✓ Successfully compiled to {}", exe_file);
                println!("\nRun your program:");
                if cfg!(target_os = "windows") {
                    println!("  .\\{}", exe_file);
                } else {
                    println!("  ./{}", exe_file);
                }
            }
            Err(e) => {
                let _ = fs::remove_file(&asm_file);
                return Err(format!(
                    "Error: Failed to run GCC: {}\nMake sure GCC is installed and in your PATH.",
                    e
                ));
            }
        }
    } else {
        println!("Compiled successfully to {}", asm_file);
        println!("\nTo create executable:");
        println!("  gcc {} -o program -no-pie", asm_file);
        println!("  ./program");
    }

    Ok(())
}
