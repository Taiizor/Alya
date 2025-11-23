mod lexer;
mod parser;
mod codegen;
mod ast;

use std::env;
use std::fs;
use std::process::{self, Command};

fn print_usage() {
    println!("Alya Programming Language Compiler v2.0");
    println!("Usage: alyac <source-file> [options]");
    println!();
    println!("Options:");
    println!("  -o <file>         Output file (default: output.s or program.exe)");
    println!("  --output-binary   Compile directly to executable (calls GCC automatically)");
    println!("  --arch <arch>     Target architecture: x64, x86, arm64 (default: x64)");
    println!("  --os <os>         Target OS: linux, windows, macos (default: auto-detect)");
    println!("  -h, --help        Show this help message");
    println!();
    println!("Example:");
    println!("  alyac hello.alya");
    println!("  alyac hello.alya -o hello.s --arch arm64");
    println!("  alyac hello.alya --output-binary -o hello.exe");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let mut input_file = None;
    let mut output_file = None;
    let mut output_binary = false;
    let mut arch = codegen::Architecture::X64;
    let mut os = if cfg!(target_os = "windows") {
        codegen::OperatingSystem::Windows
    } else if cfg!(target_os = "macos") {
        codegen::OperatingSystem::MacOS
    } else {
        codegen::OperatingSystem::Linux
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-o" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--output-binary" => {
                output_binary = true;
            }
            "--arch" => {
                if i + 1 < args.len() {
                    arch = match args[i + 1].as_str() {
                        "x64" => codegen::Architecture::X64,
                        "x86" => codegen::Architecture::X86,
                        "arm64" => codegen::Architecture::ARM64,
                        _ => {
                            eprintln!("Error: Unknown architecture '{}'", args[i + 1]);
                            process::exit(1);
                        }
                    };
                    i += 1;
                }
            }
            "--os" => {
                if i + 1 < args.len() {
                    os = match args[i + 1].as_str() {
                        "linux" => codegen::OperatingSystem::Linux,
                        "windows" => codegen::OperatingSystem::Windows,
                        "macos" => codegen::OperatingSystem::MacOS,
                        _ => {
                            eprintln!("Error: Unknown OS '{}'", args[i + 1]);
                            process::exit(1);
                        }
                    };
                    i += 1;
                }
            }
            arg if !arg.starts_with('-') => {
                input_file = Some(arg.to_string());
            }
            _ => {
                eprintln!("Error: Unknown option '{}'", args[i]);
                print_usage();
                process::exit(1);
            }
        }
        i += 1;
    }

    let input_file = match input_file {
        Some(f) => f,
        None => {
            eprintln!("Error: No input file specified");
            print_usage();
            process::exit(1);
        }
    };

    // Validate architecture/OS compatibility
    if cfg!(target_os = "windows") {
        if matches!(arch, codegen::Architecture::ARM64) {
            eprintln!("Warning: ARM64 assembly generation is not supported on Windows MinGW/GCC.");
            eprintln!("         Windows GCC can only assemble x64 and x86 code.");
            eprintln!("         The generated ARM64 assembly is valid but requires an ARM64 assembler.");
            eprintln!("");
        }
        if matches!(arch, codegen::Architecture::X86) {
            eprintln!("Warning: x86 (32-bit) compilation requires gcc with multilib support.");
            eprintln!("         On Windows, you may need MinGW-w64 with 32-bit support.");
            eprintln!("         Try: gcc -m32 output.s -o program");
            eprintln!("");
        }
    }

    // Determine output files
    let (asm_file, final_output) = if output_binary {
        // Binary mode: temp .s file and final executable
        let temp_asm = "temp_alya_output.s".to_string();
        let exe_name = output_file.unwrap_or_else(|| {
            if cfg!(target_os = "windows") {
                "program.exe".to_string()
            } else {
                "program".to_string()
            }
        });
        (temp_asm, Some(exe_name))
    } else {
        // Assembly mode: just output .s file
        let asm_name = output_file.unwrap_or_else(|| "output.s".to_string());
        (asm_name, None)
    };

    // Read source file
    let source = match fs::read_to_string(&input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: Cannot read file '{}': {}", input_file, e);
            process::exit(1);
        }
    };

    // Lexical analysis
    let mut lexer = lexer::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            process::exit(1);
        }
    };

    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            process::exit(1);
        }
    };

    // Code generation
    let code = codegen::generate(&ast, arch, os);

    // Write assembly output
    if let Err(e) = fs::write(&asm_file, code) {
        eprintln!("Error: Cannot write to '{}': {}", asm_file, e);
        process::exit(1);
    }

    if let Some(exe_file) = final_output {
        // Binary mode: compile with GCC
        println!("Compiling to executable: {}", exe_file);
        
        let mut gcc_args = vec![&asm_file, "-o", &exe_file];
        
        // Add architecture-specific flags
        match arch {
            codegen::Architecture::X86 => {
                gcc_args.insert(0, "-m32");
            }
            _ => {}
        }
        
        // Add platform-specific flags
        if !matches!(os, codegen::OperatingSystem::Windows) {
            gcc_args.push("-no-pie");
        }
        
        let gcc_result = Command::new("gcc")
            .args(&gcc_args)
            .output();
        
        match gcc_result {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!("GCC compilation failed:");
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    
                    // Clean up temp file
                    let _ = fs::remove_file(&asm_file);
                    process::exit(1);
                }
                
                // Clean up temp assembly file
                let _ = fs::remove_file(&asm_file);
                
                println!("✓ Successfully compiled to {}", exe_file);
                println!("\nRun your program:");
                if cfg!(target_os = "windows") {
                    println!("  .\\{}", exe_file);
                } else {
                    println!("  ./{}", exe_file);
                }
            }
            Err(e) => {
                eprintln!("Error: Failed to run GCC: {}", e);
                eprintln!("Make sure GCC is installed and in your PATH.");
                
                // Clean up temp file
                let _ = fs::remove_file(&asm_file);
                process::exit(1);
            }
        }
    } else {
        // Assembly mode: just inform user
        println!("Compiled successfully to {}", asm_file);
        println!("\nTo create executable:");
        println!("  gcc {} -o program -no-pie", asm_file);
        println!("  ./program");
    }
}
