use std::hint::black_box;
use std::time::{Duration, Instant};

use alya::codegen::analysis::ProgramInference;
use alya::codegen::{generate, Architecture, OperatingSystem};
use alya::lexer::Lexer;
use alya::parser::Parser;

struct BenchStat {
    name: &'static str,
    iterations: usize,
    avg: Duration,
    min: Duration,
    max: Duration,
    throughput: String,
}

fn run_bench<F, R>(
    name: &'static str,
    target_duration: Duration,
    mut f: F,
    throughput_calc: impl Fn(usize, Duration) -> String,
) -> BenchStat
where
    F: FnMut() -> R,
{
    // Warmup
    let warmup_end = Instant::now() + Duration::from_millis(50);
    while Instant::now() < warmup_end {
        black_box(f());
    }

    // Measurement
    let mut times = Vec::new();
    let start_all = Instant::now();
    let mut total_iters = 0;

    while start_all.elapsed() < target_duration || total_iters < 10 {
        let t0 = Instant::now();
        black_box(f());
        let el = t0.elapsed();
        times.push(el);
        total_iters += 1;
    }

    let total: Duration = times.iter().copied().sum();
    let avg = total / (times.len() as u32);
    let min = *times.iter().min().unwrap();
    let max = *times.iter().max().unwrap();
    let throughput = throughput_calc(times.len(), total);

    BenchStat {
        name,
        iterations: times.len(),
        avg,
        min,
        max,
        throughput,
    }
}

fn sample_large_source() -> String {
    let mut src = String::new();
    src.push_str("# Synthetic Alya source file for compiler benchmarking\n");
    src.push_str("struct Vector3\n    x\n    y\n    z\nend\n\n");
    src.push_str("struct Matrix\n    m00\n    m01\n    m10\n    m11\nend\n\n");

    for i in 0..50 {
        src.push_str(&format!(
            "function calculate_block_{i}(a, b, c)\n    let total = a * 2 + b * 3 - c / 4\n    let acc = 0\n    let j = 0\n    while j < 40\n        acc = acc + j * total\n        if acc > 1000\n            acc = acc % 997\n        else\n            acc = acc + 1\n        end\n        j = j + 1\n    end\n    return acc + {i}\nend\n\nfunction transform_vector_{i}(v)\n    let rx = v.x * {i} + v.y\n    let ry = v.y * {i} - v.z\n    let rz = v.z * {i} + v.x\n    return rx + ry + rz\nend\n\n"
        ));
    }

    src.push_str(
        "function benchmark_entry()\n    let sum = 0\n    let i = 0\n    while i < 50\n        sum = sum + calculate_block_0(i, i + 1, i + 2)\n        i = i + 1\n    end\n    say \"Benchmark completed: sum is\"\n    say sum\n    return sum\nend\n\nbenchmark_entry()\n",
    );

    src
}

fn main() {
    println!("==========================================================================");
    println!("               ALYA COMPILER BENCHMARK SUITE                             ");
    println!("==========================================================================");
    println!();

    let source = sample_large_source();
    let source_bytes = source.len();
    let source_lines = source.lines().count();

    println!(
        "Benchmark Workload: {} lines, {:.2} KB synthetic program",
        source_lines,
        source_bytes as f64 / 1024.0
    );
    println!("--------------------------------------------------------------------------");

    // 1. Lexer Benchmark
    let lex_stat = run_bench(
        "Lexer::tokenize",
        Duration::from_millis(400),
        || {
            let mut lexer = Lexer::new(&source);
            lexer.tokenize().unwrap()
        },
        |iters, dur| {
            let total_bytes = (source_bytes * iters) as f64;
            let secs = dur.as_secs_f64();
            let mb_per_sec = (total_bytes / (1024.0 * 1024.0)) / secs;
            format!("{:.1} MB/s", mb_per_sec)
        },
    );

    // Pre-tokenize for Parser
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();
    let token_count = tokens.len();

    // 2. Parser Benchmark
    let parse_stat = run_bench(
        "Parser::parse",
        Duration::from_millis(400),
        || {
            let mut parser = Parser::new(tokens.clone());
            parser.parse().unwrap()
        },
        |iters, dur| {
            let total_lines_processed = (source_lines * iters) as f64;
            let secs = dur.as_secs_f64();
            let lines_per_sec = total_lines_processed / secs;
            format!("{:.0} lines/s", lines_per_sec)
        },
    );

    // Pre-parse for Codegen & Analysis
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    // 3. ProgramInference Benchmark
    let infer_stat = run_bench(
        "ProgramInference::analyze",
        Duration::from_millis(400),
        || ProgramInference::analyze(&program),
        |iters, dur| {
            let secs = dur.as_secs_f64();
            let ops_per_sec = (iters as f64) / secs;
            format!("{:.0} ops/s", ops_per_sec)
        },
    );

    // 4. Codegen x64 Benchmark
    let asm = generate(&program, Architecture::X64, OperatingSystem::Windows);
    let sample_out_len = asm.len();
    let sample_out_lines = asm.lines().count();

    let codegen_stat = run_bench(
        "CodeGen::generate (x64)",
        Duration::from_millis(400),
        || generate(&program, Architecture::X64, OperatingSystem::Windows),
        |iters, dur| {
            let total_lines = (sample_out_lines * iters) as f64;
            let secs = dur.as_secs_f64();
            let lines_per_sec = total_lines / secs;
            format!("{:.0} asm lines/s", lines_per_sec)
        },
    );

    // 5. Full Pipeline Benchmark (Lex -> Parse -> Codegen)
    let full_stat = run_bench(
        "Full Frontend Pipeline",
        Duration::from_millis(500),
        || {
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let prog = parser.parse().unwrap();
            generate(&prog, Architecture::X64, OperatingSystem::Windows)
        },
        |iters, dur| {
            let secs = dur.as_secs_f64();
            let ops_per_sec = (iters as f64) / secs;
            format!("{:.1} files/s", ops_per_sec)
        },
    );

    // Display Table
    let results = [lex_stat, parse_stat, infer_stat, codegen_stat, full_stat];

    println!(
        "{:<28} | {:>8} | {:>10} | {:>10} | {:>10} | {:>18}",
        "Benchmark Stage", "Iters", "Avg", "Min", "Max", "Throughput"
    );
    println!(
        "{:-<28}-+-{:-<8}-+-{:-<10}-+-{:-<10}-+-{:-<10}-+-{:-<18}",
        "", "", "", "", "", ""
    );

    for r in &results {
        println!(
            "{:<28} | {:>8} | {:>10.2?} | {:>10.2?} | {:>10.2?} | {:>18}",
            r.name, r.iterations, r.avg, r.min, r.max, r.throughput
        );
    }

    println!("--------------------------------------------------------------------------");
    println!(
        "Tokens: {}, Generated ASM: {} lines ({:.1} KB)",
        token_count,
        sample_out_lines,
        sample_out_len as f64 / 1024.0
    );
    println!("All compiler benchmarks finished successfully.");
    println!("==========================================================================");
}
