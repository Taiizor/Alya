pub fn print_version() {
    println!(
        "alyac {} ({}-{})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH
    );
}

pub fn print_usage() {
    println!(
        "Alya Programming Language Compiler (alyac) v{}",
        env!("CARGO_PKG_VERSION")
    );
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
    println!(
        "  --arch <arch>         Target architecture: x86, x64, arm64 (default: auto-detected)"
    );
    println!("  --os <os>             Target OS: windows, linux, macos (default: auto-detected)");
    println!("  -q, --quiet           Suppress status messages and compiler banner");
    println!("  --time                Display timing for each compilation phase");
    println!("  --stats, --bench      Display detailed compilation and execution metrics");
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
