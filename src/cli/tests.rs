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
    let args = to_args(&["alyac"]);
    let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
    assert_eq!(parsed.command, CommandKind::Repl);

    assert_eq!(
        CliArgs::parse_from(&to_args(&["alyac", "--help"])),
        Ok(None)
    );
    assert_eq!(CliArgs::parse_from(&to_args(&["alyac", "help"])), Ok(None));
    assert_eq!(
        CliArgs::parse_from(&to_args(&["alyac", "--version"])),
        Ok(None)
    );
    assert_eq!(
        CliArgs::parse_from(&to_args(&["alyac", "version"])),
        Ok(None)
    );
}

#[test]
fn test_subcommand_repl() {
    let args = to_args(&["alyac", "repl"]);
    let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
    assert_eq!(parsed.command, CommandKind::Repl);
}

#[test]
fn test_run_with_double_dash_args() {
    let args = to_args(&[
        "alyac",
        "run",
        "examples/cli_args.alya",
        "--",
        "foo",
        "bar",
        "baz",
    ]);
    let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
    assert_eq!(parsed.command, CommandKind::Run);
    assert_eq!(parsed.input_file, "examples/cli_args.alya");
    assert_eq!(parsed.run_args, vec!["foo", "bar", "baz"]);
}

#[test]
fn test_run_with_trailing_args_without_dash() {
    let args = to_args(&[
        "alyac",
        "run",
        "examples/cli_args.alya",
        "foo",
        "bar",
        "baz",
    ]);
    let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
    assert_eq!(parsed.command, CommandKind::Run);
    assert_eq!(parsed.input_file, "examples/cli_args.alya");
    assert_eq!(parsed.run_args, vec!["foo", "bar", "baz"]);
}

#[test]
fn test_time_and_stats_flags() {
    let args = to_args(&["alyac", "build", "main.alya", "--time"]);
    let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
    assert!(parsed.time);
    assert!(!parsed.stats);

    let args_stats = to_args(&["alyac", "main.alya", "--stats"]);
    let parsed_stats = CliArgs::parse_from(&args_stats).unwrap().unwrap();
    assert!(parsed_stats.time);
    assert!(parsed_stats.stats);

    let args_bench = to_args(&["alyac", "main.alya", "--bench"]);
    let parsed_bench = CliArgs::parse_from(&args_bench).unwrap().unwrap();
    assert!(parsed_bench.time);
    assert!(parsed_bench.stats);
}

#[test]
fn test_run_with_program_flags_without_double_dash() {
    let args = to_args(&[
        "alyac",
        "run",
        "apps/http_server/main.alya",
        "--port",
        "8080",
    ]);
    let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
    assert_eq!(parsed.command, CommandKind::Run);
    assert_eq!(parsed.input_file, "apps/http_server/main.alya");
    assert_eq!(parsed.run_args, vec!["--port", "8080"]);
}

#[test]
fn test_bundle_flags() {
    let args = to_args(&["alyac", "build", "game.alya", "--bundle"]);
    let parsed = CliArgs::parse_from(&args).unwrap().unwrap();
    assert!(parsed.bundle);
    assert!(parsed.output_binary);
    assert_eq!(parsed.os, OperatingSystem::MacOS);

    let args_arm = to_args(&["alyac", "build", "game.alya", "--bundle", "--arch", "arm64"]);
    let parsed_arm = CliArgs::parse_from(&args_arm).unwrap().unwrap();
    assert!(parsed_arm.bundle);
    assert_eq!(parsed_arm.os, OperatingSystem::MacOS);
    assert_eq!(parsed_arm.arch, Architecture::ARM64);

    let args_custom = to_args(&[
        "alyac",
        "build",
        "game.alya",
        "--app",
        "--bundle-id",
        "com.mycompany.game",
        "--icon",
        "my_icon.icns",
        "--arch",
        "x64",
    ]);
    let parsed_custom = CliArgs::parse_from(&args_custom).unwrap().unwrap();
    assert!(parsed_custom.bundle);
    assert_eq!(parsed_custom.bundle_id, Some("com.mycompany.game".into()));
    assert_eq!(parsed_custom.icon_path, Some("my_icon.icns".into()));
    assert_eq!(parsed_custom.os, OperatingSystem::MacOS);
    assert_eq!(parsed_custom.arch, Architecture::X64);
}
