import { spawnSync } from "child_process";
import { performance } from "perf_hooks";
import * as fs from "fs";
import * as path from "path";

interface BenchConfig {
    name: string;
    displayName: string;
    workload: string;
    expected: string;
    alyaSrc: string;
    cSrc: string;
    pySrc: string;
    jsSrc: string;
}

const BENCHMARKS: BenchConfig[] = [
    {
        name: "Recursive Fibonacci (n=30)",
        displayName: "**Recursive Fibonacci**",
        workload: "`fib(30)` (~2.69M calls)",
        expected: "832040",
        alyaSrc: "benchmarks/cross_lang/fibonacci.alya",
        cSrc: "benchmarks/cross_lang/fibonacci.c",
        pySrc: "benchmarks/cross_lang/fibonacci.py",
        jsSrc: "benchmarks/cross_lang/fibonacci.js"
    },
    {
        name: "Mandelbrot Fractal (200x100x200)",
        displayName: "**Mandelbrot Fractal**",
        workload: "200×100 grid, 200 iters",
        expected: "767273",
        alyaSrc: "benchmarks/cross_lang/mandelbrot.alya",
        cSrc: "benchmarks/cross_lang/mandelbrot.c",
        pySrc: "benchmarks/cross_lang/mandelbrot.py",
        jsSrc: "benchmarks/cross_lang/mandelbrot.js"
    },
    {
        name: "Sieve of Eratosthenes (50,000)",
        displayName: "**Sieve of Eratosthenes**",
        workload: "Primes under 50,000",
        expected: "5133",
        alyaSrc: "benchmarks/cross_lang/sieve.alya",
        cSrc: "benchmarks/cross_lang/sieve.c",
        pySrc: "benchmarks/cross_lang/sieve.py",
        jsSrc: "benchmarks/cross_lang/sieve.js"
    },
    {
        name: "FNV-1a String Hash (50,000 iters)",
        displayName: "**FNV-1a String Hash**",
        workload: "50,000 hash calculations",
        expected: "1736110778",
        alyaSrc: "benchmarks/cross_lang/str_hash.alya",
        cSrc: "benchmarks/cross_lang/str_hash.c",
        pySrc: "benchmarks/cross_lang/str_hash.py",
        jsSrc: "benchmarks/cross_lang/str_hash.js"
    }
];

function getPythonCmd(): string {
    const candidates = process.platform === "win32" ? ["python", "py", "python3"] : ["python3", "python"];
    for (const cmd of candidates) {
        try {
            const r = spawnSync(cmd, ["--version"], { encoding: "utf-8" });
            if (r.status === 0) return cmd;
        } catch {}
    }
    return "python";
}

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

function getTestEnvironment(pyCmd: string): { os: string; gcc: string; bun: string; python: string; alya: string } {
    let osName = "Windows 11 Pro x64";
    if (process.platform === "win32") {
        osName = "Windows 11 Pro x64";
    } else if (process.platform === "darwin") {
        osName = `macOS (${process.arch})`;
    } else {
        osName = "Linux x86_64";
        try {
            if (fs.existsSync("/etc/os-release")) {
                const content = fs.readFileSync("/etc/os-release", "utf-8");
                const match = content.match(/PRETTY_NAME="([^"]+)"/);
                if (match) osName = `${match[1]} (${process.arch})`;
            }
        } catch {}
    }

    let gccVer = "GCC (-O2 optimization)";
    try {
        const res = spawnSync("gcc", ["--version"], { encoding: "utf-8" });
        const firstLine = (res.stdout || "").split("\n")[0].trim();
        if (firstLine) {
            gccVer = `${firstLine} (\`-O2\` optimization)`;
        }
    } catch {}

    let bunVer = "Bun (JavaScriptCore JIT)";
    try {
        const res = spawnSync("bun", ["--version"], { encoding: "utf-8" });
        const ver = (res.stdout || "").trim();
        if (ver) bunVer = `Bun ${ver} (JavaScriptCore JIT)`;
    } catch {}

    let pyVer = "Python 3.12 (64-bit)";
    try {
        const res = spawnSync(pyCmd, ["--version"], { encoding: "utf-8" });
        const ver = (res.stdout || res.stderr || "").trim();
        if (ver) pyVer = ver;
    } catch {}

    let alyaVer = "0.0.3";
    try {
        const cargo = fs.readFileSync(path.resolve("Cargo.toml"), "utf-8");
        const match = cargo.match(/version\s*=\s*"([^"]+)"/);
        if (match) alyaVer = match[1];
    } catch {}

    return { os: osName, gcc: gccVer, bun: bunVer, python: pyVer, alya: alyaVer };
}

function runCompilerBenchmarks(): string[] {
    console.log("Running compiler throughput benchmarks (`cargo bench --bench compiler_bench`)...");
    const res = spawnSync("cargo", ["bench", "--bench", "compiler_bench"], { encoding: "utf-8" });
    const output = (res.stdout || "") + "\n" + (res.stderr || "");
    const rows: string[] = [];

    const lines = output.split("\n");
    for (const line of lines) {
        if (!line.includes("|") || line.includes("Benchmark Stage") || line.includes("---+---")) {
            continue;
        }
        const parts = line.split("|").map(p => p.trim());
        if (parts.length >= 6) {
            const stage = parts[0];
            const iters = parts[1];
            const avg = parts[2];
            const min = parts[3];
            const max = parts[4];
            const throughput = parts[5] || "";
            if (stage && iters && avg && min && max) {
                rows.push(`| **\`${stage}\`** | ${iters} | \`${avg}\` | \`${min}\` | \`${max}\` | **${throughput}** |`);
            }
        }
    }
    return rows;
}

function updateReadme(scoreboardRows: string[], compilerRows?: string[], pyCmd: string = "python") {
    const readmePath = path.resolve("benchmarks/README.md");
    if (!fs.existsSync(readmePath)) {
        console.error(`Cannot find ${readmePath}`);
        return;
    }
    let content = fs.readFileSync(readmePath, "utf-8");

    // 1. Update Test Environment
    const env = getTestEnvironment(pyCmd);
    const envBlock = `### Test Environment\n* **Operating System:** ${env.os}\n* **C Compiler:** ${env.gcc}\n* **JavaScript Engine:** ${env.bun}\n* **Python Runtime:** ${env.python}\n* **Alya Version:** ${env.alya} (Compiled with \`alyac build\` in Release mode)\n* **Measurement Methodology:** 1 warmup run, followed by 5 timed runs. Median execution time reported.`;
    content = content.replace(/### Test Environment[\s\S]*?(?=\r?\n\r?\n---)/, envBlock);

    // 2. Update Scoreboard Table
    const tableHeader = "| Benchmark | Target Workload | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs C | Alya vs Python | Alya vs Bun |\n| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |";
    const newTable = `### Benchmark Scoreboard\n\n${tableHeader}\n${scoreboardRows.join("\n")}`;
    content = content.replace(/### Benchmark Scoreboard[\s\S]*?(?=\r?\n\r?\n---)/, newTable);

    // 3. Update Compiler Throughput Table (if available)
    if (compilerRows && compilerRows.length > 0) {
        const compilerHeader = "| Benchmark Stage | Iterations | Average Time | Min Time | Max Time | Measured Throughput |\n| :--- | :---: | :---: | :---: | :---: | :---: |";
        const newCompilerTable = `${compilerHeader}\n${compilerRows.join("\n")}`;
        content = content.replace(/\| Benchmark Stage \| Iterations \| Average Time \|[\s\S]*?(?=\r?\n\r?\n---)/, newCompilerTable);
    }

    fs.writeFileSync(readmePath, content, "utf-8");
    console.log(`[INFO] Successfully updated ${readmePath} with latest benchmark results.`);
}

async function main() {
    const shouldUpdateReadme = process.argv.includes("--update-readme");

    console.log("=========================================================================================");
    console.log("             CROSS-LANGUAGE PERFORMANCE BENCHMARK SUITE                                ");
    console.log("             Alya vs C (GCC -O2) vs Bun (JavaScript JIT) vs Python 3.12                 ");
    console.log("=========================================================================================\n");

    const exeExt = process.platform === "win32" ? ".exe" : "";
    const alyaCompiler = path.resolve(`target/release/alyac${exeExt}`);

    if (!fs.existsSync(alyaCompiler)) {
        console.log("Compiling Alya compiler in release mode...");
        const buildRes = spawnSync("cargo", ["build", "--release"], { stdio: "inherit" });
        if (buildRes.status !== 0) {
            console.error("Failed to build Alya compiler in release mode.");
            process.exit(1);
        }
    }

    const pyCmd = getPythonCmd();
    const rows: any[] = [];
    const markdownRows: string[] = [];

    for (const b of BENCHMARKS) {
        process.stdout.write(`Benchmarking ${b.name}... `);

        // 1. Compile Alya to binary
        const alyaExe = path.resolve(b.alyaSrc.replace(".alya", `_alya${exeExt}`));
        const alyaBuild = spawnSync(alyaCompiler, ["build", b.alyaSrc, "-o", alyaExe], { encoding: "utf-8" });
        if (alyaBuild.status !== 0) {
            console.error(`\nFailed to compile ${b.alyaSrc}:\n${alyaBuild.stderr}`);
            continue;
        }

        // 2. Compile C with GCC -O2
        const cExe = path.resolve(b.cSrc.replace(".c", `_c${exeExt}`));
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
        const pyRes = measureMedian(pyCmd, [b.pySrc]);

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
        const alyaVsBun = alyaRes.median <= bunRes.median
            ? `**${(bunRes.median / alyaRes.median).toFixed(1)}x faster**`
            : `\`${(alyaRes.median / bunRes.median).toFixed(1)}x slower\``;

        rows.push({
            name: b.name,
            c: `${cRes.median.toFixed(1)} ms`,
            alya: `${alyaRes.median.toFixed(1)} ms`,
            bun: `${bunRes.median.toFixed(1)} ms`,
            py: `${pyRes.median.toFixed(1)} ms`,
            vsC: alyaVsC,
            vsPy: alyaVsPy
        });

        markdownRows.push(
            `| ${b.displayName} | ${b.workload} | \`${cRes.median.toFixed(1)} ms\` | **\`${alyaRes.median.toFixed(1)} ms\`** | \`${bunRes.median.toFixed(1)} ms\` | \`${pyRes.median.toFixed(1)} ms\` | **${alyaVsC}** | **${alyaVsPy}** | ${alyaVsBun} |`
        );

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

    if (shouldUpdateReadme) {
        let compilerRows: string[] | undefined;
        try {
            compilerRows = runCompilerBenchmarks();
        } catch (e) {
            console.warn("Could not run compiler benchmarks:", e);
        }
        updateReadme(markdownRows, compilerRows, pyCmd);
    }
}

main();
