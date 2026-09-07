use super::args::{CliArgs, CommandKind};
use crate::codegen::{Architecture, OperatingSystem};

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
