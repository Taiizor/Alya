use crate::codegen::{Architecture, OperatingSystem};
use std::env;
use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandKind {
    Build,
    Run,
    Check,
    EmitTokens,
    EmitAst,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArgs {
    pub command: CommandKind,
    pub input_file: String,
    pub output_file: Option<String>,
    pub output_binary: bool,
    pub arch: Architecture,
    pub os: OperatingSystem,
    pub quiet: bool,
}

impl CliArgs {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        match Self::parse_from(&args) {
            Ok(Some(parsed)) => parsed,
            Ok(None) => process::exit(0),
            Err(err) => {
                eprintln!("{}", err);
                eprintln!("Run 'alyac --help' for usage instructions.");
                process::exit(1);
            }
        }
    }

    pub fn parse_from(args: &[String]) -> Result<Option<Self>, String> {
        if args.len() < 2 {
            Self::print_usage();
            return Ok(None);
        }

        let first = args[1].as_str();
        if first == "-h" || first == "--help" || first == "help" {
            Self::print_usage();
            return Ok(None);
        }
        if first == "-v" || first == "--version" || first == "version" {
            Self::print_version();
            return Ok(None);
        }

        let mut command = CommandKind::Build;
        let mut output_binary = false;
        let mut start_idx = 1;

        match first {
            "run" => {
                command = CommandKind::Run;
                output_binary = true;
                start_idx = 2;
            }
            "build" => {
                command = CommandKind::Build;
                output_binary = true;
                start_idx = 2;
            }
            "check" => {
                command = CommandKind::Check;
                start_idx = 2;
            }
            "ast" => {
                command = CommandKind::EmitAst;
                start_idx = 2;
            }
            "tokens" => {
                command = CommandKind::EmitTokens;
                start_idx = 2;
            }
            _ => {}
        }

        let mut input_file = None;
        let mut output_file = None;
        let mut quiet = false;
        let mut arch = Architecture::X64;
        let mut os = if cfg!(target_os = "windows") {
            OperatingSystem::Windows
        } else if cfg!(target_os = "macos") {
            OperatingSystem::MacOS
        } else {
            OperatingSystem::Linux
        };

        let mut i = start_idx;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => {
                    Self::print_usage();
                    return Ok(None);
                }
                "-v" | "--version" => {
                    Self::print_version();
                    return Ok(None);
                }
                "-o" | "--output" => {
                    if i + 1 < args.len() {
                        output_file = Some(args[i + 1].clone());
                        i += 1;
                    } else {
                        return Err("Error: Missing argument for '-o/--output'".to_string());
                    }
                }
                "-b" | "-c" | "--binary" | "--output-binary" => {
                    output_binary = true;
                }
                "-S" | "--asm" => {
                    output_binary = false;
                }
                "-r" | "--run" => {
                    command = CommandKind::Run;
                    output_binary = true;
                }
                "--check" => {
                    command = CommandKind::Check;
                }
                "--ast" => {
                    command = CommandKind::EmitAst;
                }
                "--tokens" => {
                    command = CommandKind::EmitTokens;
                }
                "-q" | "--quiet" => {
                    quiet = true;
                }
                "--arch" => {
                    if i + 1 < args.len() {
                        arch = match args[i + 1].as_str() {
                            "x64" => Architecture::X64,
                            "x86" => Architecture::X86,
                            "arm64" => Architecture::ARM64,
                            other => return Err(format!("Error: Unknown architecture '{}'. Supported: x64, x86, arm64", other)),
                        };
                        i += 1;
                    } else {
                        return Err("Error: Missing argument for '--arch'".to_string());
                    }
                }
                "--os" => {
                    if i + 1 < args.len() {
                        os = match args[i + 1].as_str() {
                            "linux" => OperatingSystem::Linux,
                            "windows" => OperatingSystem::Windows,
                            "macos" => OperatingSystem::MacOS,
                            other => return Err(format!("Error: Unknown OS '{}'. Supported: linux, windows, macos", other)),
                        };
                        i += 1;
                    } else {
                        return Err("Error: Missing argument for '--os'".to_string());
                    }
                }
                arg if !arg.starts_with('-') => {
                    if let Some(existing) = &input_file {
                        return Err(format!("Error: Unexpected multiple input files: '{}' and '{}'", existing, arg));
                    } else {
                        input_file = Some(arg.to_string());
                    }
                }
                other => {
                    return Err(format!("Error: Unknown option '{}'", other));
                }
            }
            i += 1;
        }

        let input_file = match input_file {
            Some(f) => f,
            None => return Err("Error: No input source file specified.".to_string()),
        };

        Ok(Some(Self {
            command,
            input_file,
            output_file,
            output_binary,
            arch,
            os,
            quiet,
        }))
    }

    pub fn print_version() {
        println!("alya {}", env!("CARGO_PKG_VERSION"));
    }

    pub fn print_usage() {
        println!("Alya Programming Language Compiler (alyac) v{}", env!("CARGO_PKG_VERSION"));
        println!("A modern, simple, compiled programming language.\n");
        println!("USAGE:");
        println!("  alyac <COMMAND> <file> [OPTIONS]");
        println!("  alyac <file> [OPTIONS]\n");
        println!("COMMANDS:");
        println!("  run <file>            Compile and execute program immediately");
        println!("  build <file>          Compile program directly to an executable binary (-b, -c)");
        println!("  check <file>          Verify syntax and structure without generating code");
        println!("  ast <file>            Print the parsed Abstract Syntax Tree (AST)");
        println!("  tokens <file>         Print tokenized output from lexical analysis");
        println!("  help                  Display help information");
        println!("  version               Display version information\n");
        println!("OPTIONS:");
        println!("  -o, --output <file>   Specify output file (default: <name>.s or <name>.exe)");
        println!("  -b, -c, --binary      Compile directly to executable (calls GCC)");
        println!("  -r, --run             Compile and run immediately");
        println!("  -S, --asm             Emit assembly output only");
        println!("  --arch <arch>         Target architecture: x64, x86, arm64 (default: x64)");
        println!("  --os <os>             Target OS: linux, windows, macos (default: auto-detected)");
        println!("  -q, --quiet           Suppress status messages and compiler banner");
        println!("  -v, --version         Show compiler version");
        println!("  -h, --help            Show this help message\n");
        println!("EXAMPLES:");
        println!("  alyac run hello.alya                 # Compile & run in one step");
        println!("  alyac build hello.alya               # Produce executable (hello.exe / hello)");
        println!("  alyac hello.alya                     # Produce assembly (hello.s)");
        println!("  alyac hello.alya -b -o my_app.exe    # Produce custom named binary");
        println!("  alyac check hello.alya               # Quick syntax validation");
        println!("  alyac ast hello.alya                 # Inspect AST hierarchy");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_args(slice: &[&str]) -> Vec<String> {
        slice.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_subcommand_run() {
        let args = to_args(&["alyac", "run", "hello.alya"]);
        let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
        assert_eq!(parsed.command, CommandKind::Run);
        assert_eq!(parsed.input_file, "hello.alya");
        assert!(parsed.output_binary);
    }

    #[test]
    fn test_subcommand_build() {
        let args = to_args(&["alyac", "build", "main.alya", "-o", "main.exe"]);
        let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
        assert_eq!(parsed.command, CommandKind::Build);
        assert_eq!(parsed.input_file, "main.alya");
        assert_eq!(parsed.output_file, Some("main.exe".into()));
        assert!(parsed.output_binary);
    }

    #[test]
    fn test_subcommand_check() {
        let args = to_args(&["alyac", "check", "code.alya"]);
        let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
        assert_eq!(parsed.command, CommandKind::Check);
        assert_eq!(parsed.input_file, "code.alya");
    }

    #[test]
    fn test_flag_run_and_quiet() {
        let args = to_args(&["alyac", "test.alya", "-r", "-q"]);
        let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
        assert_eq!(parsed.command, CommandKind::Run);
        assert!(parsed.output_binary);
        assert!(parsed.quiet);
    }

    #[test]
    fn test_target_arch_and_os() {
        let args = to_args(&["alyac", "test.alya", "--arch", "arm64", "--os", "linux"]);
        let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
        assert_eq!(parsed.arch, Architecture::ARM64);
        assert_eq!(parsed.os, OperatingSystem::Linux);
    }

    #[test]
    fn test_help_and_version() {
        assert_eq!(CliArgs::parse_from(&to_args(&["alyac"])), Ok(None));
        assert_eq!(CliArgs::parse_from(&to_args(&["alyac", "--help"])), Ok(None));
        assert_eq!(CliArgs::parse_from(&to_args(&["alyac", "help"])), Ok(None));
        assert_eq!(CliArgs::parse_from(&to_args(&["alyac", "--version"])), Ok(None));
        assert_eq!(CliArgs::parse_from(&to_args(&["alyac", "version"])), Ok(None));
    }
}
