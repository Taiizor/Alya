import { spawnSync } from "child_process";
import { performance } from "perf_hooks";
import * as fs from "fs";
import * as path from "path";

interface BenchConfig {
    name: string;
    expected: string;
    alyaSrc: string;
    cSrc: string;
    pySrc: string;
    jsSrc: string;
}

const BENCHMARKS: BenchConfig[] = [
    {
        name: "Recursive Fibonacci (n=30)",
        expected: "832040",
        alyaSrc: "benchmarks/cross_lang/fibonacci.alya",
        cSrc: "benchmarks/cross_lang/fibonacci.c",
        pySrc: "benchmarks/cross_lang/fibonacci.py",
        jsSrc: "benchmarks/cross_lang/fibonacci.js"
    },
    {
        name: "Mandelbrot Fractal (200x100x200)",
        expected: "767273",
        alyaSrc: "benchmarks/cross_lang/mandelbrot.alya",
        cSrc: "benchmarks/cross_lang/mandelbrot.c",
        pySrc: "benchmarks/cross_lang/mandelbrot.py",
        jsSrc: "benchmarks/cross_lang/mandelbrot.js"
    },
    {
        name: "Sieve of Eratosthenes (50,000)",
        expected: "5133",
        alyaSrc: "benchmarks/cross_lang/sieve.alya",
        cSrc: "benchmarks/cross_lang/sieve.c",
        pySrc: "benchmarks/cross_lang/sieve.py",
        jsSrc: "benchmarks/cross_lang/sieve.js"
    },
    {
        name: "FNV-1a String Hash (50,000 iters)",
        expected: "1736110778",
        alyaSrc: "benchmarks/cross_lang/str_hash.alya",
        cSrc: "benchmarks/cross_lang/str_hash.c",
        pySrc: "benchmarks/cross_lang/str_hash.py",
        jsSrc: "benchmarks/cross_lang/str_hash.js"
    }
];

function runCommand(cmd: string, args: string[]): { durationMs: number; output: string; status: number } {
    const t0 = performance.now();
    const res = spawnSync(cmd, args, { encoding: "utf-8" });
    const durationMs = performance.now() - t0;
    const output = (res.stdout || "").trim();
    return { durationMs, output, status: res.status ?? -1 };
}

function measureMedian(cmd: string, args: string[], iters = 5): { median: number; min: number; max: number; output: string } {
    // Warmup
    runCommand(cmd, args);

    const times: number[] = [];
    let lastOutput = "";

    for (let i = 0; i < iters; i++) {
        const { durationMs, output, status } = runCommand(cmd, args);
        if (status !== 0) {
            console.error(`Command failed: ${cmd} ${args.join(" ")}`);
        }
        times.push(durationMs);
        lastOutput = output;
    }

    times.sort((a, b) => a - b);
    const median = times[Math.floor(times.length / 2)];
    const min = times[0];
    const max = times[times.length - 1];

    return { median, min, max, output: lastOutput };
}

async function main() {
    console.log("=========================================================================================");
    console.log("             CROSS-LANGUAGE PERFORMANCE BENCHMARK SUITE                                ");
    console.log("             Alya vs C (GCC -O2) vs Bun (JavaScript JIT) vs Python 3.12                 ");
    console.log("=========================================================================================\n");

    const alyaCompiler = path.resolve("target/release/alyac.exe");
    if (!fs.existsSync(alyaCompiler)) {
        console.log("Compiling Alya compiler in release mode...");
        spawnSync("cargo", ["build", "--release"], { stdio: "inherit" });
    }

    const rows: any[] = [];

    for (const b of BENCHMARKS) {
        process.stdout.write(`Benchmarking ${b.name}... `);

        // 1. Compile Alya to binary
        const alyaExe = path.resolve(b.alyaSrc.replace(".alya", "_alya.exe"));
        const alyaBuild = spawnSync(alyaCompiler, ["build", b.alyaSrc, "-o", alyaExe], { encoding: "utf-8" });
        if (alyaBuild.status !== 0) {
            console.error(`\nFailed to compile ${b.alyaSrc}:\n${alyaBuild.stderr}`);
            continue;
        }

        // 2. Compile C with GCC -O2
        const cExe = path.resolve(b.cSrc.replace(".c", "_c.exe"));
        const cBuild = spawnSync("gcc", ["-O2", b.cSrc, "-o", cExe], { encoding: "utf-8" });
        if (cBuild.status !== 0) {
            console.error(`\nFailed to compile ${b.cSrc}:\n${cBuild.stderr}`);
            continue;
        }

        // Run C
        const cRes = measureMedian(cExe, []);
        // Run Alya
        const alyaRes = measureMedian(alyaExe, []);
        // Run Bun (JS)
        const bunRes = measureMedian("bun", ["run", b.jsSrc]);
        // Run Python
        const pyRes = measureMedian("python", [b.pySrc]);

        // Cleanup binaries
        try { fs.unlinkSync(alyaExe); } catch {}
        try { fs.unlinkSync(cExe); } catch {}

        // Validation
        const allMatch = [cRes.output, alyaRes.output, bunRes.output, pyRes.output].every(o => o.includes(b.expected));
        if (!allMatch) {
            console.warn(`\n[WARNING] Output mismatch for ${b.name}! Expected ${b.expected}`);
            console.warn(`C: ${cRes.output} | Alya: ${alyaRes.output} | Bun: ${bunRes.output} | Py: ${pyRes.output}`);
        }

        const alyaVsC = (alyaRes.median / cRes.median).toFixed(1) + "x";
        const alyaVsPy = (pyRes.median / alyaRes.median).toFixed(1) + "x faster";

        rows.push({
            name: b.name,
            c: `${cRes.median.toFixed(1)} ms`,
            alya: `${alyaRes.median.toFixed(1)} ms`,
            bun: `${bunRes.median.toFixed(1)} ms`,
            py: `${pyRes.median.toFixed(1)} ms`,
            vsC: alyaVsC,
            vsPy: alyaVsPy
        });

        console.log("Done.");
    }

    console.log("\n=========================================================================================");
    console.log("                               BENCHMARK RESULTS (Median of 5 runs)                      ");
    console.log("=========================================================================================");
    console.log(
        "| Benchmark                         | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | vs C (Ratio) | vs Python (Speedup) |"
    );
    console.log(
        "|:----------------------------------|------------:|--------------:|-------------:|------------:|-------------:|--------------------:|"
    );

    for (const r of rows) {
        console.log(
            `| ${r.name.padEnd(33)} | ${r.c.padStart(11)} | ${r.alya.padStart(13)} | ${r.bun.padStart(12)} | ${r.py.padStart(11)} | ${r.vsC.padStart(12)} | ${r.vsPy.padStart(19)} |`
        );
    }
    console.log("=========================================================================================\n");
}

main();
