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
    Fmt,
    Test,
    Repl,
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
    pub time: bool,
    pub stats: bool,
    pub check_only: bool,
    pub bundle: bool,
    pub bundle_id: Option<String>,
    pub icon_path: Option<String>,
    pub run_args: Vec<String>,
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
            return Ok(Some(Self {
                command: CommandKind::Repl,
                input_file: String::new(),
                output_file: None,
                output_binary: false,
                arch,
                os,
                quiet: false,
                time: false,
                stats: false,
                check_only: false,
                bundle: false,
                bundle_id: None,
                icon_path: None,
                run_args: Vec::new(),
            }));
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
            "repl" => {
                command = CommandKind::Repl;
                start_idx = 2;
            }
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
            "fmt" => {
                command = CommandKind::Fmt;
                start_idx = 2;
            }
            "test" => {
                command = CommandKind::Test;
                start_idx = 2;
            }
            _ => {}
        }

        let mut input_file = None;
        let mut output_file = None;
        let mut quiet = false;
        let mut time = false;
        let mut stats = false;
        let mut check_only = false;
        let mut bundle = false;
        let mut bundle_id = None;
        let mut icon_path = None;
        let mut os_explicit = false;
        let mut arch_explicit = false;
        let mut run_args = Vec::new();
        let mut arch = if cfg!(target_arch = "aarch64") {
            Architecture::ARM64
        } else if cfg!(target_arch = "x86") {
            Architecture::X86
        } else {
            Architecture::X64
        };
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
                "--" => {
                    run_args.extend(args[i + 1..].iter().cloned());
                    break;
                }
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
                    if command == CommandKind::Fmt {
                        check_only = true;
                    } else {
                        command = CommandKind::Check;
                    }
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
                "--time" => {
                    time = true;
                }
                "--stats" | "--bench" => {
                    stats = true;
                    time = true;
                }
                "--bundle" | "--app" => {
                    bundle = true;
                    output_binary = true;
                }
                "--bundle-id" | "--identifier" => {
                    if i + 1 < args.len() {
                        bundle_id = Some(args[i + 1].clone());
                        i += 1;
                    } else {
                        return Err("Error: Missing argument for '--bundle-id'".to_string());
                    }
                }
                "--icon" => {
                    if i + 1 < args.len() {
                        icon_path = Some(args[i + 1].clone());
                        i += 1;
                    } else {
                        return Err("Error: Missing argument for '--icon'".to_string());
                    }
                }
                "--arch" => {
                    if i + 1 < args.len() {
                        arch_explicit = true;
                        arch = match args[i + 1].as_str() {
                            "x64" => Architecture::X64,
                            "x86" => Architecture::X86,
                            "arm64" => Architecture::ARM64,
                            other => {
                                return Err(format!(
                                    "Error: Unknown architecture '{}'. Supported: x86, x64, arm64",
                                    other
                                ))
                            }
                        };
                        i += 1;
                    } else {
                        return Err("Error: Missing argument for '--arch'".to_string());
                    }
                }
                "--os" => {
                    if i + 1 < args.len() {
                        os_explicit = true;
                        os = match args[i + 1].as_str() {
                            "linux" => OperatingSystem::Linux,
                            "windows" => OperatingSystem::Windows,
                            "macos" => OperatingSystem::MacOS,
                            other => {
                                return Err(format!(
                                    "Error: Unknown OS '{}'. Supported: linux, windows, macos",
                                    other
                                ))
                            }
                        };
                        i += 1;
                    } else {
                        return Err("Error: Missing argument for '--os'".to_string());
                    }
                }
                arg if !arg.starts_with('-') => {
                    if let Some(existing) = &input_file {
                        if command == CommandKind::Run {
                            run_args.push(arg.to_string());
                        } else {
                            return Err(format!(
                                "Error: Unexpected multiple input files: '{}' and '{}'",
                                existing, arg
                            ));
                        }
                    } else {
                        input_file = Some(arg.to_string());
                        if command == CommandKind::Run && i + 1 < args.len() {
                            if args[i + 1] == "--" {
                                run_args.extend(args[i + 2..].iter().cloned());
                            } else {
                                run_args.extend(args[i + 1..].iter().cloned());
                            }
                            break;
                        }
                    }
                }
                other => {
                    if command == CommandKind::Run && input_file.is_some() {
                        run_args.push(other.to_string());
                    } else {
                        return Err(format!("Error: Unknown option '{}'", other));
                    }
                }
            }
            i += 1;
        }

        let input_file = match input_file {
            Some(f) => f,
            None => {
                if matches!(command, CommandKind::Fmt | CommandKind::Test) {
                    ".".to_string()
                } else if command == CommandKind::Repl {
                    String::new()
                } else {
                    return Err("Error: No input source file specified.".to_string());
                }
            }
        };

        if bundle {
            if !os_explicit {
                os = OperatingSystem::MacOS;
            }
            if !arch_explicit && !matches!(arch, Architecture::ARM64 | Architecture::X64) {
                arch = Architecture::ARM64;
            }
        }

        Ok(Some(Self {
            command,
            input_file,
            output_file,
            output_binary,
            arch,
            os,
            quiet,
            time,
            stats,
            check_only,
            bundle,
            bundle_id,
            icon_path,
            run_args,
        }))
    }

    pub fn print_version() {
        crate::cli::help::print_version();
    }

    pub fn print_usage() {
        crate::cli::help::print_usage();
    }
}
