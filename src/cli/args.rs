use crate::codegen::{Architecture, OperatingSystem};
use std::env;
use std::process;

pub struct CliArgs {
    pub input_file: String,
    pub output_file: Option<String>,
    pub output_binary: bool,
    pub arch: Architecture,
    pub os: OperatingSystem,
}

impl CliArgs {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            Self::print_usage();
            process::exit(1);
        }

        let mut input_file = None;
        let mut output_file = None;
        let mut output_binary = false;
        let mut arch = Architecture::X64;
        let mut os = if cfg!(target_os = "windows") {
            OperatingSystem::Windows
        } else if cfg!(target_os = "macos") {
            OperatingSystem::MacOS
        } else {
            OperatingSystem::Linux
        };

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => {
                    Self::print_usage();
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
                            "x64" => Architecture::X64,
                            "x86" => Architecture::X86,
                            "arm64" => Architecture::ARM64,
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
                            "linux" => OperatingSystem::Linux,
                            "windows" => OperatingSystem::Windows,
                            "macos" => OperatingSystem::MacOS,
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
                    Self::print_usage();
                    process::exit(1);
                }
            }
            i += 1;
        }

        let input_file = match input_file {
            Some(f) => f,
            None => {
                eprintln!("Error: No input file specified");
                Self::print_usage();
                process::exit(1);
            }
        };

        Self {
            input_file,
            output_file,
            output_binary,
            arch,
            os,
        }
    }

    pub fn print_usage() {
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
}
