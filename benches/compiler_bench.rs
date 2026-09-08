use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use alya::codegen::analysis::ProgramInference;
use alya::codegen::{generate, Architecture, OperatingSystem};
use alya::lexer::Lexer;
use alya::parser::Parser;

// ============================================================================
// Memory Tracking Allocator (Tracking total allocated bytes & alloc counts)
// ============================================================================

struct TrackingAllocator;

static ALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCATED_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            ALLOCATED_BYTES.fetch_add(new_size - layout.size(), Ordering::Relaxed);
        }
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        System.realloc(ptr, layout, new_size)
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn reset_alloc() {
    ALLOCATED_BYTES.store(0, Ordering::SeqCst);
    ALLOC_COUNT.store(0, Ordering::SeqCst);
}

fn get_alloc() -> (usize, usize) {
    (
        ALLOCATED_BYTES.load(Ordering::SeqCst),
        ALLOC_COUNT.load(Ordering::SeqCst),
    )
}

// ============================================================================
// Benchmark Statistics & Formatting
// ============================================================================

pub struct BenchStat {
    pub name: &'static str,
    pub iterations: usize,
    pub mean: Duration,
    pub error: Duration,
    pub std_dev: Duration,
    pub min: Duration,
    pub max: Duration,
    pub allocated_bytes: usize,
    pub alloc_count: usize,
    pub throughput: String,
}

/// Two-sided Student's t-value for 99.9% confidence interval based on degrees of freedom (N - 1)
fn student_t_999(n: usize) -> f64 {
    match n {
        1..=2 => 31.599,
        3 => 12.924,
        4 => 8.610,
        5 => 6.869,
        6 => 5.959,
        7 => 5.408,
        8 => 5.041,
        9 => 4.781,
        10 => 4.587,
        11..=15 => 4.140,
        16..=20 => 3.883,
        21..=30 => 3.646,
        31..=60 => 3.460,
        _ => 3.291,
    }
}

pub fn format_duration(dur: Duration) -> String {
    let nanos = dur.as_nanos();
    if nanos < 1_000 {
        format!("{:.2} ns", nanos as f64)
    } else if nanos < 1_000_000 {
        format!("{:.2} µs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2} ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2} s", dur.as_secs_f64())
    }
}

pub fn format_bytes(bytes: usize) -> String {
    if bytes == 0 {
        "-".to_string()
    } else if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
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
    let warmup_end = Instant::now() + Duration::from_millis(60);
    while Instant::now() < warmup_end {
        black_box(f());
    }

    // Timed measurements
    let mut times = Vec::new();
    let mut total_allocated_bytes = 0usize;
    let mut total_alloc_count = 0usize;
    let start_all = Instant::now();
    let mut total_iters = 0;

    while start_all.elapsed() < target_duration || total_iters < 10 {
        reset_alloc();
        let t0 = Instant::now();
        black_box(f());
        let el = t0.elapsed();
        let (bytes, count) = get_alloc();
        times.push(el);
        total_allocated_bytes += bytes;
        total_alloc_count += count;
        total_iters += 1;
    }

    let n = times.len();
    let total_nanos: u128 = times.iter().map(|t| t.as_nanos()).sum();
    let mean_nanos = total_nanos as f64 / n as f64;
    let mean = Duration::from_nanos(mean_nanos as u64);

    let variance = if n > 1 {
        let sum_sq_diff: f64 = times
            .iter()
            .map(|t| {
                let diff = t.as_nanos() as f64 - mean_nanos;
                diff * diff
            })
            .sum();
        sum_sq_diff / (n - 1) as f64
    } else {
        0.0
    };
    let std_dev_nanos = variance.sqrt();
    let std_dev = Duration::from_nanos(std_dev_nanos as u64);

    let t_val = student_t_999(n);
    let error_nanos = t_val * (std_dev_nanos / (n as f64).sqrt());
    let error = Duration::from_nanos(error_nanos as u64);

    let min = *times.iter().min().unwrap();
    let max = *times.iter().max().unwrap();
    let total_duration: Duration = times.iter().copied().sum();
    let throughput = throughput_calc(n, total_duration);

    let allocated_per_op = total_allocated_bytes / n;
    let alloc_count_per_op = total_alloc_count / n;

    BenchStat {
        name,
        iterations: n,
        mean,
        error,
        std_dev,
        min,
        max,
        allocated_bytes: allocated_per_op,
        alloc_count: alloc_count_per_op,
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
    let os_name = match std::env::consts::OS {
        "windows" => "Windows",
        "linux" => "Linux",
        "macos" => "macOS",
        other => other,
    };
    let arch = std::env::consts::ARCH;
    let logical_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let alya_ver = env!("CARGO_PKG_VERSION");

    let source = sample_large_source();
    let source_bytes = source.len();
    let source_lines = source.lines().count();

    println!("// * Summary *\n");
    println!("Alya Compiler Benchmark v{alya_ver}, {os_name} ({arch})");
    if let Ok(proc_id) = std::env::var("PROCESSOR_IDENTIFIER") {
        println!(
            "Processor: {}, {} logical cores",
            proc_id.trim(),
            logical_cores
        );
    } else {
        println!("Processor: {} logical cores", logical_cores);
    }
    println!("Toolchain: rustc 1.75+ (stable), Profile: Release (opt-level=3, LTO=true)");
    println!(
        "Workload : {} lines, {:.2} KB synthetic program (50+ functions, structs, control flow)\n",
        source_lines,
        source_bytes as f64 / 1024.0
    );

    println!("IterationCount=10+  WarmupDuration=60ms  TargetDuration=400ms\n");

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

    let baseline_mean_nanos = lex_stat.mean.as_nanos() as f64;
    let baseline_alloc = lex_stat.allocated_bytes.max(1) as f64;

    let results = [lex_stat, parse_stat, infer_stat, codegen_stat, full_stat];

    // BenchmarkDotNet Summary Table
    println!(
        "| {:<26} | {:>6} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10} | {:>6} | {:>10} | {:>11} | {:>19} |",
        "Benchmark Stage", "Iters", "Mean", "Error", "StdDev", "Min", "Max", "Ratio", "Allocated", "Alloc Ratio", "Throughput"
    );
    println!(
        "|:{:-<26}-|-{:-<6}:|-{:-<10}:|-{:-<10}:|-{:-<10}:|-{:-<10}:|-{:-<10}:|-{:-<6}:|-{:-<10}:|-{:-<11}:|-{:-<19}:|",
        "", "", "", "", "", "", "", "", "", "", ""
    );

    for r in &results {
        let ratio = (r.mean.as_nanos() as f64) / baseline_mean_nanos;
        let alloc_ratio = (r.allocated_bytes as f64) / baseline_alloc;

        println!(
            "| {:<26} | {:>6} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10} | {:>6.2} | {:>10} | {:>11.2} | {:>19} |",
            r.name,
            r.iterations,
            format_duration(r.mean),
            format_duration(r.error),
            format_duration(r.std_dev),
            format_duration(r.min),
            format_duration(r.max),
            ratio,
            format_bytes(r.allocated_bytes),
            alloc_ratio,
            r.throughput
        );
    }

    println!("\n// * Legends *");
    println!("  Mean        : Arithmetic mean of all measurements");
    println!("  Error       : Half of 99.9% confidence interval");
    println!("  StdDev      : Standard deviation of all measurements");
    println!("  Min / Max   : Minimum and maximum recorded execution time");
    println!("  Ratio       : Mean time ratio relative to baseline (Lexer::tokenize)");
    println!("  Allocated   : Allocated heap memory per single operation (1 KB = 1024 B)");
    println!("  Alloc Ratio : Allocated memory ratio relative to baseline");
    println!("  Throughput  : Processed workload units per second\n");

    println!(
        "Synthetic Workload Metrics: {} tokens, {} lines generated ASM ({:.1} KB)",
        token_count,
        sample_out_lines,
        sample_out_len as f64 / 1024.0
    );
}
