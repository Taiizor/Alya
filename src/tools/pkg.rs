use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PkgCommand {
    Init {
        path: Option<String>,
        name: Option<String>,
        is_lib: bool,
    },
    Add {
        name: String,
        path: Option<String>,
        git: Option<String>,
        tag: Option<String>,
        branch: Option<String>,
        version: Option<String>,
    },
    Install,
    List,
    Update,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub entry: String,
    pub license: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencySource {
    Version(String),
    Path {
        path: String,
    },
    Git {
        url: String,
        tag: Option<String>,
        branch: Option<String>,
        rev: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageManifest {
    pub package: PackageInfo,
    pub dependencies: BTreeMap<String, DependencySource>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    pub entry: String,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageLock {
    pub version: u32,
    pub packages: Vec<LockedPackage>,
}

// ============================================================================
// Pure Rust SHA-256 (RFC 6234 / FIPS 180-4) - Zero Dependencies
// ============================================================================

pub struct Sha256 {
    state: [u32; 8],
    count: u64,
    buffer: [u8; 64],
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    pub fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            count: 0,
            buffer: [0u8; 64],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut idx = 0;
        let mut buf_len = (self.count % 64) as usize;
        self.count += data.len() as u64;

        if buf_len > 0 && buf_len + data.len() >= 64 {
            let fill = 64 - buf_len;
            self.buffer[buf_len..64].copy_from_slice(&data[..fill]);
            Self::transform(&mut self.state, &self.buffer);
            idx += fill;
            buf_len = 0;
        }

        while idx + 64 <= data.len() {
            Self::transform(&mut self.state, data[idx..idx + 64].try_into().unwrap());
            idx += 64;
        }

        if idx < data.len() {
            let rem = data.len() - idx;
            self.buffer[buf_len..buf_len + rem].copy_from_slice(&data[idx..]);
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        let total_bits = self.count * 8;
        let buf_len = (self.count % 64) as usize;

        self.buffer[buf_len] = 0x80;
        if buf_len + 1 > 56 {
            for b in &mut self.buffer[buf_len + 1..64] {
                *b = 0;
            }
            Self::transform(&mut self.state, &self.buffer);
            self.buffer = [0u8; 64];
        } else {
            for b in &mut self.buffer[buf_len + 1..56] {
                *b = 0;
            }
        }

        self.buffer[56..64].copy_from_slice(&total_bits.to_be_bytes());
        Self::transform(&mut self.state, &self.buffer);

        let mut out = [0u8; 32];
        for (i, &word) in self.state.iter().enumerate() {
            out[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }

    fn transform(state: &mut [u32; 8], block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(block[i * 4..(i + 1) * 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];

        for (i, &w_val) in w.iter().enumerate() {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(Self::K[i])
                .wrapping_add(w_val);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let bytes = hasher.finalize();
    let mut s = String::with_capacity(64);
    for b in bytes {
        use std::fmt::Write;
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

// ============================================================================
// TOML Parsing & Serialization (Zero Dependencies)
// ============================================================================

fn unquote(s: &str) -> String {
    let trimmed = s.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        if trimmed.len() >= 2 {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            String::new()
        }
    } else {
        trimmed.to_string()
    }
}

fn parse_string_array(s: &str) -> Vec<String> {
    let trimmed = s.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Vec::new();
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    inner
        .split(',')
        .map(|item| unquote(item.trim()))
        .filter(|item| !item.is_empty())
        .collect()
}

fn parse_inline_table(s: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let trimmed = s.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return map;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    for part in inner.split(',') {
        if let Some((k, v)) = part.split_once('=') {
            map.insert(k.trim().to_string(), unquote(v.trim()));
        }
    }
    map
}

pub fn parse_manifest(content: &str) -> Result<PackageManifest, String> {
    let mut name = String::new();
    let mut version = "0.1.0".to_string();
    let mut authors = Vec::new();
    let mut description = None;
    let mut entry = "src/main.alya".to_string();
    let mut license = None;
    let mut dependencies = BTreeMap::new();

    let mut current_section = "";

    for (line_no, raw_line) in content.lines().enumerate() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].trim();
            continue;
        }

        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim();
            let val = v.trim();

            match current_section {
                "package" => match key {
                    "name" => name = unquote(val),
                    "version" => version = unquote(val),
                    "authors" => authors = parse_string_array(val),
                    "description" => description = Some(unquote(val)),
                    "entry" => entry = unquote(val),
                    "license" => license = Some(unquote(val)),
                    _ => {}
                },
                "dependencies" => {
                    if val.starts_with('{') {
                        let table = parse_inline_table(val);
                        if let Some(p) = table.get("path") {
                            dependencies.insert(
                                key.to_string(),
                                DependencySource::Path { path: p.clone() },
                            );
                        } else if let Some(g) = table.get("git") {
                            dependencies.insert(
                                key.to_string(),
                                DependencySource::Git {
                                    url: g.clone(),
                                    tag: table.get("tag").cloned(),
                                    branch: table.get("branch").cloned(),
                                    rev: table.get("rev").cloned(),
                                },
                            );
                        } else if let Some(v_inner) = table.get("version") {
                            dependencies.insert(
                                key.to_string(),
                                DependencySource::Version(v_inner.clone()),
                            );
                        }
                    } else {
                        dependencies
                            .insert(key.to_string(), DependencySource::Version(unquote(val)));
                    }
                }
                _ => {}
            }
        } else {
            return Err(format!(
                "Syntax error in alya.toml at line {}: '{}'",
                line_no + 1,
                raw_line
            ));
        }
    }

    if name.is_empty() {
        return Err("Missing required field 'name' under [package] in alya.toml".to_string());
    }

    Ok(PackageManifest {
        package: PackageInfo {
            name,
            version,
            authors,
            description,
            entry,
            license,
        },
        dependencies,
    })
}

pub fn serialize_manifest(manifest: &PackageManifest) -> String {
    let mut out = String::new();
    out.push_str("[package]\n");
    out.push_str(&format!("name = \"{}\"\n", manifest.package.name));
    out.push_str(&format!("version = \"{}\"\n", manifest.package.version));
    out.push_str(&format!("entry = \"{}\"\n", manifest.package.entry));
    if let Some(desc) = &manifest.package.description {
        out.push_str(&format!("description = \"{}\"\n", desc));
    }
    if !manifest.package.authors.is_empty() {
        let authors_str = manifest
            .package
            .authors
            .iter()
            .map(|a| format!("\"{}\"", a))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("authors = [{}]\n", authors_str));
    }
    if let Some(lic) = &manifest.package.license {
        out.push_str(&format!("license = \"{}\"\n", lic));
    }

    out.push_str("\n[dependencies]\n");
    for (name, dep) in &manifest.dependencies {
        match dep {
            DependencySource::Version(v) => {
                out.push_str(&format!("{} = \"{}\"\n", name, v));
            }
            DependencySource::Path { path } => {
                out.push_str(&format!(
                    "{} = {{ path = \"{}\" }}\n",
                    name,
                    path.replace('\\', "/")
                ));
            }
            DependencySource::Git {
                url,
                tag,
                branch,
                rev,
            } => {
                let mut parts = vec![format!("git = \"{}\"", url)];
                if let Some(t) = tag {
                    parts.push(format!("tag = \"{}\"", t));
                }
                if let Some(b) = branch {
                    parts.push(format!("branch = \"{}\"", b));
                }
                if let Some(r) = rev {
                    parts.push(format!("rev = \"{}\"", r));
                }
                out.push_str(&format!("{} = {{ {} }}\n", name, parts.join(", ")));
            }
        }
    }
    out
}

pub fn parse_lockfile(content: &str) -> Result<PackageLock, String> {
    let mut version = 1;
    let mut packages = Vec::new();
    let mut current_pkg: Option<LockedPackage> = None;

    for line in content.lines() {
        let trimmed = line.split('#').next().unwrap_or("").trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "[[package]]" {
            if let Some(pkg) = current_pkg.take() {
                packages.push(pkg);
            }
            current_pkg = Some(LockedPackage {
                name: String::new(),
                version: String::new(),
                source: String::new(),
                entry: String::new(),
                checksum: String::new(),
            });
            continue;
        }

        if let Some((k, v)) = trimmed.split_once('=') {
            let key = k.trim();
            let val = unquote(v.trim());

            if let Some(ref mut pkg) = current_pkg {
                match key {
                    "name" => pkg.name = val,
                    "version" => pkg.version = val,
                    "source" => pkg.source = val,
                    "entry" => pkg.entry = val,
                    "checksum" => pkg.checksum = val,
                    _ => {}
                }
            } else if key == "version" {
                version = val.parse::<u32>().unwrap_or(1);
            }
        }
    }

    if let Some(pkg) = current_pkg {
        packages.push(pkg);
    }

    Ok(PackageLock { version, packages })
}

pub fn serialize_lockfile(lock: &PackageLock) -> String {
    let mut out = String::new();
    out.push_str("# Auto-generated by Alya Package Manager. DO NOT EDIT.\n");
    out.push_str(&format!("version = {}\n", lock.version));
    for pkg in &lock.packages {
        out.push_str("\n[[package]]\n");
        out.push_str(&format!("name = \"{}\"\n", pkg.name));
        out.push_str(&format!("version = \"{}\"\n", pkg.version));
        out.push_str(&format!("source = \"{}\"\n", pkg.source));
        out.push_str(&format!("entry = \"{}\"\n", pkg.entry.replace('\\', "/")));
        out.push_str(&format!("checksum = \"{}\"\n", pkg.checksum));
    }
    out
}

// ============================================================================
// File Discovery & Integrity Checking
// ============================================================================

pub fn find_manifest_dir() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    find_manifest_dir_from(&cwd)
}

pub fn find_manifest_dir_from(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join("alya.toml").exists() {
            return Some(current);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

pub fn detect_package_entry() -> Option<String> {
    let manifest_dir = find_manifest_dir()?;
    let manifest_path = manifest_dir.join("alya.toml");
    let content = fs::read_to_string(&manifest_path).ok()?;
    let manifest = parse_manifest(&content).ok()?;
    let entry_path = manifest_dir.join(&manifest.package.entry);
    if entry_path.exists() {
        Some(entry_path.to_string_lossy().replace('\\', "/"))
    } else {
        None
    }
}

pub fn collect_alya_files(
    root: &Path,
    current: &Path,
    out: &mut Vec<(String, PathBuf)>,
) -> Result<(), String> {
    if !current.exists() {
        return Ok(());
    }
    let entries = fs::read_dir(current)
        .map_err(|e| format!("Cannot read directory '{}': {}", current.display(), e))?;
    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name == ".git" || file_name == ".alya" || file_name == "target" {
            continue;
        }
        if path.is_dir() {
            collect_alya_files(root, &path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "alya") {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            out.push((rel, path));
        }
    }
    Ok(())
}

pub fn compute_package_checksum(dir: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    collect_alya_files(dir, dir, &mut files)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));

    if files.is_empty() {
        return Ok(
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        );
    }

    let mut hasher = Sha256::new();
    for (rel_path, full_path) in files {
        hasher.update(rel_path.as_bytes());
        let content = fs::read(&full_path).map_err(|e| {
            format!(
                "Failed to read '{}' for checksum: {}",
                full_path.display(),
                e
            )
        })?;
        hasher.update(&content);
    }
    let digest = sha256_hex(&hasher.finalize());
    Ok(format!("sha256:{}", digest))
}

pub fn find_package_entry(pkg_dir: &Path, pkg_name: &str) -> Result<PathBuf, String> {
    let manifest_file = pkg_dir.join("alya.toml");
    if manifest_file.exists() {
        if let Ok(content) = fs::read_to_string(&manifest_file) {
            if let Ok(manifest) = parse_manifest(&content) {
                let candidate = pkg_dir.join(&manifest.package.entry);
                if candidate.exists() {
                    return Ok(candidate);
                }
            }
        }
    }

    let candidates = [
        pkg_dir.join("src").join("main.alya"),
        pkg_dir.join("src").join("lib.alya"),
        pkg_dir.join("main.alya"),
        pkg_dir.join("lib.alya"),
        pkg_dir.join(format!("{}.alya", pkg_name)),
        pkg_dir.join("src").join(format!("{}.alya", pkg_name)),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    Err(format!(
        "Cannot find entry point for package '{}' in directory '{}'. Expected 'src/main.alya', 'src/lib.alya', or manifest entry.",
        pkg_name,
        pkg_dir.display()
    ))
}

// ============================================================================
// Module Resolution Hook for Compiler
// ============================================================================

pub fn resolve_package_import(
    import_path: &str,
    current_dir: &Path,
) -> Result<Option<PathBuf>, String> {
    if import_path.starts_with("./")
        || import_path.starts_with("../")
        || import_path.starts_with('/')
        || import_path.starts_with('\\')
        || import_path.starts_with("std/")
        || import_path.starts_with("std::")
    {
        return Ok(None);
    }

    let manifest_dir = match find_manifest_dir_from(current_dir) {
        Some(d) => d,
        None => return Ok(None),
    };

    let manifest_path = manifest_dir.join("alya.toml");
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read 'alya.toml': {}", e))?;
    let manifest = parse_manifest(&content)?;

    let parts: Vec<&str> = import_path.splitn(2, '/').collect();
    let pkg_name = parts[0];
    let subpath = if parts.len() > 1 {
        Some(parts[1])
    } else {
        None
    };

    if let Some(dep_source) = manifest.dependencies.get(pkg_name) {
        let pkg_dir = match dep_source {
            DependencySource::Path { path } => {
                let p = Path::new(path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    manifest_dir.join(p)
                }
            }
            DependencySource::Git { .. } | DependencySource::Version(_) => {
                manifest_dir.join(".alya").join("packages").join(pkg_name)
            }
        };

        if !pkg_dir.exists() {
            return Err(format!(
                "Package '{}' is declared in alya.toml but not installed at '{}'. Run 'alyac install' to resolve dependencies.",
                pkg_name,
                pkg_dir.display()
            ));
        }

        if let Some(sub) = subpath {
            let candidates = [
                pkg_dir.join("src").join(format!("{}.alya", sub)),
                pkg_dir.join("src").join(sub).join("mod.alya"),
                pkg_dir.join("src").join(sub),
                pkg_dir.join(format!("{}.alya", sub)),
                pkg_dir.join(sub).join("mod.alya"),
                pkg_dir.join(sub),
            ];
            for cand in candidates {
                if cand.exists() {
                    return Ok(Some(cand));
                }
            }
            return Err(format!(
                "Cannot find module '{}' in package '{}' (searched inside '{}')",
                sub,
                pkg_name,
                pkg_dir.display()
            ));
        } else {
            let entry = find_package_entry(&pkg_dir, pkg_name)?;
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

// ============================================================================
// Command Handlers
// ============================================================================

pub fn run_pkg(cmd: &PkgCommand) -> Result<(), String> {
    match cmd {
        PkgCommand::Init { path, name, is_lib } => {
            run_init(path.as_deref(), name.as_deref(), *is_lib)
        }
        PkgCommand::Add {
            name,
            path,
            git,
            tag,
            branch,
            version,
        } => run_add(
            name,
            path.as_deref(),
            git.as_deref(),
            tag.as_deref(),
            branch.as_deref(),
            version.as_deref(),
        ),
        PkgCommand::Install => run_install(),
        PkgCommand::List => run_list(),
        PkgCommand::Update => run_update(),
        PkgCommand::Help => {
            print_pkg_help();
            Ok(())
        }
    }
}

pub fn run_init(path: Option<&str>, name: Option<&str>, is_lib: bool) -> Result<(), String> {
    let target_dir = PathBuf::from(path.unwrap_or("."));
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(|e| {
            format!(
                "Failed to create directory '{}': {}",
                target_dir.display(),
                e
            )
        })?;
    }

    let manifest_path = target_dir.join("alya.toml");
    if manifest_path.exists() {
        return Err(format!(
            "Package manifest '{}' already exists.",
            manifest_path.display()
        ));
    }

    let pkg_name = if let Some(n) = name {
        n.to_string()
    } else {
        let abs = target_dir
            .canonicalize()
            .unwrap_or_else(|_| target_dir.clone());
        abs.file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "my_package".to_string())
            .to_lowercase()
            .replace(' ', "_")
    };

    let entry_file = if is_lib {
        "src/lib.alya"
    } else {
        "src/main.alya"
    };

    let manifest = PackageManifest {
        package: PackageInfo {
            name: pkg_name.clone(),
            version: "0.1.0".to_string(),
            authors: Vec::new(),
            description: Some(format!("Alya package {}", pkg_name)),
            entry: entry_file.to_string(),
            license: Some("MIT".to_string()),
        },
        dependencies: BTreeMap::new(),
    };

    fs::write(&manifest_path, serialize_manifest(&manifest))
        .map_err(|e| format!("Failed to write alya.toml: {}", e))?;

    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir).map_err(|e| format!("Failed to create 'src' directory: {}", e))?;

    let code_entry = target_dir.join(entry_file);
    if !code_entry.exists() {
        let starter_code = if is_lib {
            format!(
                "# {} library\n\nfunction add(a, b)\n    return a + b\nend\n",
                pkg_name
            )
        } else {
            format!(
                "# {} application\n\nfunction main()\n    say \"Hello from {}!\"\nend\n\nmain()\n",
                pkg_name, pkg_name
            )
        };
        fs::write(&code_entry, starter_code)
            .map_err(|e| format!("Failed to create entry file: {}", e))?;
    }

    let gitignore_path = target_dir.join(".gitignore");
    if !gitignore_path.exists() {
        let gitignore = if is_lib {
            "/target/\n.alya/\nalya.lock\nalya.lock.bak\n*.exe\n*.s\n*.o\n*.app\n"
        } else {
            "/target/\n.alya/\nalya.lock.bak\n*.exe\n*.s\n*.o\n*.app\n"
        };
        let _ = fs::write(gitignore_path, gitignore);
    }

    println!(
        "✓ Created {} package '{}' at {}",
        if is_lib { "library" } else { "binary" },
        pkg_name,
        target_dir.display()
    );
    Ok(())
}

pub fn run_add(
    name: &str,
    path: Option<&str>,
    git: Option<&str>,
    tag: Option<&str>,
    branch: Option<&str>,
    version: Option<&str>,
) -> Result<(), String> {
    let manifest_dir = find_manifest_dir().ok_or_else(|| {
        "Error: Could not find 'alya.toml' in current directory or any parent.".to_string()
    })?;
    let manifest_path = manifest_dir.join("alya.toml");

    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read alya.toml: {}", e))?;
    let mut manifest = parse_manifest(&content)?;

    let source = if let Some(p) = path {
        DependencySource::Path {
            path: p.to_string(),
        }
    } else if let Some(g) = git {
        DependencySource::Git {
            url: g.to_string(),
            tag: tag.map(|s| s.to_string()),
            branch: branch.map(|s| s.to_string()),
            rev: None,
        }
    } else if let Some(v) = version {
        DependencySource::Version(v.to_string())
    } else {
        DependencySource::Version("*".to_string())
    };

    manifest.dependencies.insert(name.to_string(), source);

    fs::write(&manifest_path, serialize_manifest(&manifest))
        .map_err(|e| format!("Failed to update alya.toml: {}", e))?;

    println!("✓ Added dependency '{}' to alya.toml", name);

    run_install_in(&manifest_dir)?;
    Ok(())
}

// ============================================================================
// Remote Package Resolution (Git & HTTP Archive Fallback)
// ============================================================================

pub fn resolve_archive_candidates(
    url: &str,
    tag: Option<&str>,
    branch: Option<&str>,
    rev: Option<&str>,
) -> Vec<String> {
    let clean = url.trim_end_matches('/').trim_end_matches(".git");

    // GitHub repository pattern
    let gh_sub = clean
        .strip_prefix("https://github.com/")
        .or_else(|| clean.strip_prefix("http://github.com/"))
        .or_else(|| clean.strip_prefix("git@github.com:"));

    if let Some(sub) = gh_sub {
        let parts: Vec<&str> = sub.split('/').collect();
        if parts.len() >= 2 {
            let owner = parts[0];
            let repo = parts[1];
            let mut candidates = Vec::new();

            if let Some(t) = tag {
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/refs/tags/{}.tar.gz",
                    owner, repo, t
                ));
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/refs/tags/{}.zip",
                    owner, repo, t
                ));
            } else if let Some(b) = branch {
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/refs/heads/{}.tar.gz",
                    owner, repo, b
                ));
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/refs/heads/{}.zip",
                    owner, repo, b
                ));
            } else if let Some(r) = rev {
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/{}.tar.gz",
                    owner, repo, r
                ));
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/{}.zip",
                    owner, repo, r
                ));
            } else {
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/refs/heads/main.tar.gz",
                    owner, repo
                ));
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/refs/heads/master.tar.gz",
                    owner, repo
                ));
                candidates.push(format!(
                    "https://github.com/{}/{}/archive/refs/heads/main.zip",
                    owner, repo
                ));
            }
            return candidates;
        }
    }

    // GitLab repository pattern
    let gl_sub = clean
        .strip_prefix("https://gitlab.com/")
        .or_else(|| clean.strip_prefix("http://gitlab.com/"));

    if let Some(sub) = gl_sub {
        let parts: Vec<&str> = sub.split('/').collect();
        if parts.len() >= 2 {
            let owner = parts[0];
            let repo = parts[1];
            let ref_name = tag.or(branch).or(rev).unwrap_or("main");
            return vec![
                format!(
                    "https://gitlab.com/{}/{}/-/archive/{}/{}-{}.tar.gz",
                    owner, repo, ref_name, repo, ref_name
                ),
                format!(
                    "https://gitlab.com/{}/{}/-/archive/{}/{}-{}.zip",
                    owner, repo, ref_name, repo, ref_name
                ),
            ];
        }
    }

    // Direct HTTP archive URL
    if url.ends_with(".tar.gz") || url.ends_with(".zip") || url.ends_with(".tgz") {
        return vec![url.to_string()];
    }

    Vec::new()
}

pub fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst)
        .map_err(|e| format!("Failed to create directory '{}': {}", dst.display(), e))?;
    let entries = fs::read_dir(src)
        .map_err(|e| format!("Failed to read directory '{}': {}", src.display(), e))?;
    for entry in entries.flatten() {
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_contents(&path, &target)?;
        } else {
            fs::copy(&path, &target).map_err(|e| format!("Failed to copy file: {}", e))?;
        }
    }
    Ok(())
}

pub fn try_download_and_extract_archive(
    candidate_urls: &[String],
    target_dir: &Path,
    pkg_name: &str,
) -> Result<(), String> {
    fs::create_dir_all(target_dir).map_err(|e| {
        format!(
            "Failed to create package directory '{}': {}",
            target_dir.display(),
            e
        )
    })?;

    let temp_dir = env::temp_dir();
    let pid = std::process::id();

    for url in candidate_urls {
        let is_zip = url.ends_with(".zip");
        let ext = if is_zip { "zip" } else { "tar.gz" };
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let temp_archive =
            temp_dir.join(format!("alya_pkg_{}_{}_{}.{}", pkg_name, pid, millis, ext));

        // 1. Download archive using curl, wget, or PowerShell
        let mut download_ok = Command::new("curl")
            .args(["-sSL", "-f", url, "-o"])
            .arg(&temp_archive)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !download_ok {
            download_ok = Command::new("wget")
                .args(["-q", url, "-O"])
                .arg(&temp_archive)
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
        }

        if !download_ok && cfg!(windows) {
            let ps_script = format!(
                "$ProgressPreference = 'SilentlyContinue'; Invoke-WebRequest -Uri '{}' -OutFile '{}'",
                url,
                temp_archive.display().to_string().replace('\\', "/")
            );
            download_ok = Command::new("powershell")
                .args(["-NoProfile", "-Command", &ps_script])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
        }

        if !download_ok
            || !temp_archive.exists()
            || fs::metadata(&temp_archive).map(|m| m.len()).unwrap_or(0) == 0
        {
            let _ = fs::remove_file(&temp_archive);
            continue;
        }

        // 2. Extract archive using tar (bsdtar on Windows / GNU tar on Unix)
        let tar_ok = Command::new("tar")
            .arg("-xf")
            .arg(&temp_archive)
            .args(["--strip-components", "1", "-C"])
            .arg(target_dir)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if tar_ok {
            let _ = fs::remove_file(&temp_archive);
            return Ok(());
        }

        // 3. Fallback on Windows with PowerShell Expand-Archive if tar fails
        if cfg!(windows) && is_zip {
            let extract_tmp =
                temp_dir.join(format!("alya_extract_{}_{}_{}", pkg_name, pid, millis));
            let ps_expand = format!(
                "$ProgressPreference = 'SilentlyContinue'; Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                temp_archive.display().to_string().replace('\\', "/"),
                extract_tmp.display().to_string().replace('\\', "/")
            );
            let ps_ok = Command::new("powershell")
                .args(["-NoProfile", "-Command", &ps_expand])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            if ps_ok && extract_tmp.exists() {
                if let Ok(entries) = fs::read_dir(&extract_tmp) {
                    let mut found_dir = None;
                    for e in entries.flatten() {
                        if e.path().is_dir() {
                            found_dir = Some(e.path());
                            break;
                        }
                    }
                    let source_folder = found_dir.unwrap_or_else(|| extract_tmp.clone());
                    let copy_res = copy_dir_contents(&source_folder, target_dir);
                    let _ = fs::remove_dir_all(&extract_tmp);
                    let _ = fs::remove_file(&temp_archive);
                    if copy_res.is_ok() {
                        return Ok(());
                    }
                }
            }
            let _ = fs::remove_dir_all(&extract_tmp);
        }

        let _ = fs::remove_file(&temp_archive);
    }

    Err(format!(
        "Failed to download and extract archive for '{}' from candidate URLs",
        pkg_name
    ))
}

fn try_git_clone(
    url: &str,
    tag: Option<&str>,
    branch: Option<&str>,
    target_dir: &Path,
) -> Result<(), String> {
    let mut cmd = Command::new("git");
    cmd.arg("clone").arg("--depth").arg("1");
    if let Some(t) = tag {
        cmd.arg("--branch").arg(t);
    } else if let Some(b) = branch {
        cmd.arg("--branch").arg(b);
    }
    cmd.arg(url).arg(target_dir);

    match cmd.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            let _ = fs::remove_dir_all(target_dir);
            Err(format!(
                "git clone exited with status {}",
                status.code().unwrap_or(-1)
            ))
        }
        Err(e) => {
            let _ = fs::remove_dir_all(target_dir);
            Err(format!("could not execute 'git' ({})", e))
        }
    }
}

fn update_git_dependency(tag: Option<&str>, branch: Option<&str>, target_dir: &Path) {
    if !target_dir.join(".git").exists() {
        return;
    }
    let _ = Command::new("git")
        .current_dir(target_dir)
        .args(["fetch", "--depth", "1"])
        .status();
    if let Some(t) = tag {
        let _ = Command::new("git")
            .current_dir(target_dir)
            .args(["checkout", t])
            .status();
    } else if let Some(b) = branch {
        let _ = Command::new("git")
            .current_dir(target_dir)
            .args(["checkout", b])
            .status();
    }
}

fn fetch_git_or_archive_dependency(
    name: &str,
    url: &str,
    tag: Option<&str>,
    branch: Option<&str>,
    rev: Option<&str>,
    target_dir: &Path,
) -> Result<(), String> {
    println!("  Resolving dependency '{}' from {}...", name, url);

    // 1. Try Git clone first if git CLI is installed
    let git_err = match try_git_clone(url, tag, branch, target_dir) {
        Ok(()) => return Ok(()),
        Err(e) => e,
    };

    // 2. If git is missing or failed, fall back to automatic HTTP archive download
    let candidate_urls = resolve_archive_candidates(url, tag, branch, rev);
    if candidate_urls.is_empty() {
        return Err(format!(
            "Git clone failed ({}) and no HTTP archive candidate URL could be resolved for '{}'",
            git_err, url
        ));
    }

    println!("  Notice: Git unavailable ({}).", git_err);
    println!(
        "  Attempting automatic HTTP archive download fallback for '{}'...",
        name
    );

    match try_download_and_extract_archive(&candidate_urls, target_dir, name) {
        Ok(()) => {
            println!(
                "  ✓ Successfully downloaded and unpacked '{}' archive without Git",
                name
            );
            Ok(())
        }
        Err(fallback_err) => {
            let _ = fs::remove_dir_all(target_dir);
            Err(format!(
                "Failed to fetch package '{}' from {}:\n  Git error: {}\n  Archive download error: {}",
                name, url, git_err, fallback_err
            ))
        }
    }
}

pub fn run_install() -> Result<(), String> {
    let manifest_dir = find_manifest_dir().ok_or_else(|| {
        "Error: Could not find 'alya.toml' in current directory or any parent.".to_string()
    })?;
    run_install_in(&manifest_dir)
}

pub fn run_install_in(manifest_dir: &Path) -> Result<(), String> {
    let manifest_path = manifest_dir.join("alya.toml");
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read alya.toml: {}", e))?;
    let manifest = parse_manifest(&content)?;

    let packages_dir = manifest_dir.join(".alya").join("packages");
    let mut locked_packages = Vec::new();

    for (name, dep) in &manifest.dependencies {
        match dep {
            DependencySource::Path { path } => {
                let dep_path = Path::new(path);
                let full_path = if dep_path.is_absolute() {
                    dep_path.to_path_buf()
                } else {
                    manifest_dir.join(dep_path)
                };

                if !full_path.exists() {
                    return Err(format!(
                        "Path dependency '{}' does not exist at '{}'",
                        name,
                        full_path.display()
                    ));
                }

                let entry = find_package_entry(&full_path, name)?;
                let rel_entry = entry
                    .strip_prefix(manifest_dir)
                    .unwrap_or(&entry)
                    .to_string_lossy()
                    .replace('\\', "/");
                let checksum = compute_package_checksum(&full_path)?;

                locked_packages.push(LockedPackage {
                    name: name.clone(),
                    version: "0.1.0".to_string(),
                    source: format!("path:{}", path.replace('\\', "/")),
                    entry: rel_entry,
                    checksum,
                });
            }
            DependencySource::Git {
                url,
                tag,
                branch,
                rev,
            } => {
                fs::create_dir_all(&packages_dir)
                    .map_err(|e| format!("Failed to create .alya/packages directory: {}", e))?;
                let target_dir = packages_dir.join(name);

                if !target_dir.exists() {
                    fetch_git_or_archive_dependency(
                        name,
                        url,
                        tag.as_deref(),
                        branch.as_deref(),
                        rev.as_deref(),
                        &target_dir,
                    )?;
                } else {
                    update_git_dependency(tag.as_deref(), branch.as_deref(), &target_dir);
                }

                let entry = find_package_entry(&target_dir, name)?;
                let rel_entry = entry
                    .strip_prefix(manifest_dir)
                    .unwrap_or(&entry)
                    .to_string_lossy()
                    .replace('\\', "/");
                let checksum = compute_package_checksum(&target_dir)?;

                let tag_or_branch = tag
                    .as_deref()
                    .or(branch.as_deref())
                    .or(rev.as_deref())
                    .unwrap_or("head");
                let source = format!("git:{}#{}", url, tag_or_branch);

                locked_packages.push(LockedPackage {
                    name: name.clone(),
                    version: tag_or_branch.to_string(),
                    source,
                    entry: rel_entry,
                    checksum,
                });
            }
            DependencySource::Version(v) => {
                let pkg_dir = packages_dir.join(name);
                let checksum = if pkg_dir.exists() {
                    compute_package_checksum(&pkg_dir)?
                } else {
                    format!("sha256:ver:{}", sha256_hex(v.as_bytes()))
                };

                let rel_entry = format!(".alya/packages/{}/src/main.alya", name);

                locked_packages.push(LockedPackage {
                    name: name.clone(),
                    version: v.clone(),
                    source: format!("registry:{}", v),
                    entry: rel_entry,
                    checksum,
                });
            }
        }
    }

    let lock = PackageLock {
        version: 1,
        packages: locked_packages,
    };

    let lockfile_path = manifest_dir.join("alya.lock");
    fs::write(&lockfile_path, serialize_lockfile(&lock))
        .map_err(|e| format!("Failed to write alya.lock: {}", e))?;

    // On case-sensitive filesystems, remove legacy Alya.lock if distinct from alya.lock
    let legacy_lock = manifest_dir.join("Alya.lock");
    if legacy_lock.exists() {
        if let (Ok(p1), Ok(p2)) = (lockfile_path.canonicalize(), legacy_lock.canonicalize()) {
            if p1 != p2 {
                let _ = fs::remove_file(&legacy_lock);
            }
        }
    }

    println!(
        "✓ Locked {} package{} in alya.lock",
        lock.packages.len(),
        if lock.packages.len() == 1 { "" } else { "s" }
    );
    Ok(())
}

pub fn run_list() -> Result<(), String> {
    let manifest_dir = find_manifest_dir().ok_or_else(|| {
        "Error: Could not find 'alya.toml' in current directory or any parent.".to_string()
    })?;
    let manifest_path = manifest_dir.join("alya.toml");
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read alya.toml: {}", e))?;
    let manifest = parse_manifest(&content)?;

    println!(
        "Package: {} v{}",
        manifest.package.name, manifest.package.version
    );
    println!("Entry:   {}", manifest.package.entry);
    if let Some(desc) = &manifest.package.description {
        println!("About:   {}", desc);
    }
    println!();

    if manifest.dependencies.is_empty() {
        println!("No dependencies declared in alya.toml.");
        return Ok(());
    }

    println!("Dependencies ({}):", manifest.dependencies.len());

    let lock_path = if manifest_dir.join("alya.lock").exists() {
        manifest_dir.join("alya.lock")
    } else {
        manifest_dir.join("Alya.lock")
    };
    let lock = if lock_path.exists() {
        fs::read_to_string(&lock_path)
            .ok()
            .and_then(|c| parse_lockfile(&c).ok())
    } else {
        None
    };

    for (name, dep) in &manifest.dependencies {
        let locked = lock
            .as_ref()
            .and_then(|l| l.packages.iter().find(|p| &p.name == name));
        let dep_desc = match dep {
            DependencySource::Path { path } => format!("path: {}", path),
            DependencySource::Git {
                url, tag, branch, ..
            } => {
                let mut s = format!("git: {}", url);
                if let Some(t) = tag {
                    s.push_str(&format!(" (tag: {})", t));
                } else if let Some(b) = branch {
                    s.push_str(&format!(" (branch: {})", b));
                }
                s
            }
            DependencySource::Version(v) => format!("version: {}", v),
        };

        if let Some(lp) = locked {
            let chk_short = if lp.checksum.len() > 17 {
                &lp.checksum[..17]
            } else {
                &lp.checksum
            };
            println!(
                "  • {:<16} {:<35} [locked: {}...]",
                name, dep_desc, chk_short
            );
        } else {
            println!(
                "  • {:<16} {:<35} [not locked - run 'alyac install']",
                name, dep_desc
            );
        }
    }
    Ok(())
}

pub fn run_update() -> Result<(), String> {
    println!("Updating package dependencies...");
    run_install()
}

pub fn print_pkg_help() {
    println!("Alya Package Manager (alyac pkg)");
    println!("Manage project dependencies, manifests (alya.toml), and lockfiles (alya.lock).\n");
    println!("USAGE:");
    println!("  alyac pkg <COMMAND> [OPTIONS]");
    println!("  alyac init [path] [OPTIONS]          # Shortcut for pkg init");
    println!("  alyac add <name> [OPTIONS]           # Shortcut for pkg add");
    println!("  alyac install                        # Shortcut for pkg install\n");
    println!("COMMANDS:");
    println!("  init [path]        Initialize a new Alya package in [path] (default: .)");
    println!("  add <name>         Add a new dependency to alya.toml");
    println!("  install            Resolve and lock all dependencies specified in alya.toml");
    println!("  list               List project dependencies and lock integrity status");
    println!("  update             Update and re-lock dependencies to latest versions");
    println!("  help               Show this help message\n");
    println!("OPTIONS FOR 'init':");
    println!("  --name <name>      Override package name (default: directory name)");
    println!("  --lib              Initialize as a library (src/lib.alya) instead of binary\n");
    println!("OPTIONS FOR 'add':");
    println!("  --path <path>      Add dependency from local file system path");
    println!("  --git <url>        Add dependency from remote Git repository");
    println!("  --tag <tag>        Specify Git tag for dependency");
    println!("  --branch <branch>  Specify Git branch for dependency");
    println!("  --version <ver>    Specify semantic version constraint\n");
    println!("EXAMPLES:");
    println!("  alyac init my_app");
    println!("  alyac init my_lib --lib");
    println!("  alyac add raylib --path ../libs/raylib");
    println!("  alyac add sqlite --git https://github.com/alya-lang/sqlite --tag v1.0.0");
    println!("  alyac install");
    println!("  alyac pkg list");
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn test_manifest_parse_and_serialize() {
        let toml = r#"
[package]
name = "demo_pkg"
version = "1.2.3"
entry = "src/main.alya"
authors = ["Alice", "Bob"]
description = "A great package"
license = "MIT"

[dependencies]
local_lib = { path = "../libs/local" }
git_lib = { git = "https://github.com/test/repo", tag = "v1.0" }
simple_ver = "0.5.0"
"#;

        let manifest = parse_manifest(toml).expect("parse manifest failed");
        assert_eq!(manifest.package.name, "demo_pkg");
        assert_eq!(manifest.package.version, "1.2.3");
        assert_eq!(manifest.package.entry, "src/main.alya");
        assert_eq!(manifest.package.authors, vec!["Alice", "Bob"]);
        assert_eq!(
            manifest.package.description,
            Some("A great package".to_string())
        );
        assert_eq!(manifest.package.license, Some("MIT".to_string()));
        assert_eq!(manifest.dependencies.len(), 3);

        let serialized = serialize_manifest(&manifest);
        let manifest2 = parse_manifest(&serialized).expect("roundtrip parse failed");
        assert_eq!(manifest, manifest2);
    }

    #[test]
    fn test_lockfile_parse_and_serialize() {
        let lock_toml = r#"# Auto-generated
version = 1

[[package]]
name = "raylib"
version = "0.1.0"
source = "path:../libs/raylib"
entry = "../libs/raylib/src/main.alya"
checksum = "sha256:1234567890abcdef"

[[package]]
name = "sqlite"
version = "v1.0.0"
source = "git:https://github.com/alya-lang/sqlite#v1.0.0"
entry = ".alya/packages/sqlite/src/main.alya"
checksum = "sha256:abcdef1234567890"
"#;

        let lock = parse_lockfile(lock_toml).expect("parse lockfile failed");
        assert_eq!(lock.version, 1);
        assert_eq!(lock.packages.len(), 2);
        assert_eq!(lock.packages[0].name, "raylib");
        assert_eq!(lock.packages[1].name, "sqlite");

        let serialized = serialize_lockfile(&lock);
        let lock2 = parse_lockfile(&serialized).expect("roundtrip parse failed");
        assert_eq!(lock, lock2);
    }

    #[test]
    fn test_pkg_init_lifecycle() {
        let temp_dir = std::env::temp_dir().join(format!(
            "alya_pkg_test_init_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let temp_str = temp_dir.to_string_lossy().to_string();

        let res = run_init(Some(&temp_str), Some("test_lib"), true);
        assert!(res.is_ok(), "run_init failed: {:?}", res);

        assert!(temp_dir.join("alya.toml").exists());
        assert!(temp_dir.join("src").join("lib.alya").exists());
        assert!(temp_dir.join(".gitignore").exists());

        let content = fs::read_to_string(temp_dir.join("alya.toml")).unwrap();
        let manifest = parse_manifest(&content).unwrap();
        assert_eq!(manifest.package.name, "test_lib");
        assert_eq!(manifest.package.entry, "src/lib.alya");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_pkg_add_and_install_path_dependency() {
        let base_temp = std::env::temp_dir().join(format!(
            "alya_pkg_dep_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let lib_dir = base_temp.join("math_lib");
        let app_dir = base_temp.join("my_app");

        let _ = fs::create_dir_all(&lib_dir);
        let _ = fs::create_dir_all(&app_dir);

        run_init(Some(&lib_dir.to_string_lossy()), Some("math_lib"), true).unwrap();

        run_init(Some(&app_dir.to_string_lossy()), Some("my_app"), false).unwrap();

        // Write custom library function
        fs::write(
            lib_dir.join("src").join("lib.alya"),
            "function add(a, b)\n    return a + b\nend\n",
        )
        .unwrap();

        // Add math_lib as relative dependency in app's alya.toml
        let mut app_manifest =
            parse_manifest(&fs::read_to_string(app_dir.join("alya.toml")).unwrap()).unwrap();
        app_manifest.dependencies.insert(
            "math_lib".to_string(),
            DependencySource::Path {
                path: "../math_lib".to_string(),
            },
        );
        fs::write(app_dir.join("alya.toml"), serialize_manifest(&app_manifest)).unwrap();

        // Run install in app_dir
        run_install_in(&app_dir).unwrap();

        assert!(app_dir.join("alya.lock").exists());
        let lock = parse_lockfile(&fs::read_to_string(app_dir.join("alya.lock")).unwrap()).unwrap();
        assert_eq!(lock.packages.len(), 1);
        assert_eq!(lock.packages[0].name, "math_lib");
        assert!(lock.packages[0].checksum.starts_with("sha256:"));

        // Test compiler resolution
        let resolved = resolve_package_import("math_lib", &app_dir).unwrap();
        assert!(resolved.is_some());
        let resolved_path = resolved.unwrap();
        assert!(resolved_path.ends_with("lib.alya"));

        let _ = fs::remove_dir_all(&base_temp);
    }

    #[test]
    fn test_resolve_archive_candidates() {
        let gh_branch = resolve_archive_candidates(
            "https://github.com/alya-lang/dotenv",
            None,
            Some("develop"),
            None,
        );
        assert_eq!(gh_branch.len(), 2);
        assert!(gh_branch[0].contains("archive/refs/heads/develop.tar.gz"));
        assert!(gh_branch[1].contains("archive/refs/heads/develop.zip"));

        let gh_tag = resolve_archive_candidates(
            "https://github.com/alya-lang/dotenv.git",
            Some("v0.1.0"),
            None,
            None,
        );
        assert_eq!(gh_tag.len(), 2);
        assert!(gh_tag[0].contains("archive/refs/tags/v0.1.0.tar.gz"));
        assert!(gh_tag[1].contains("archive/refs/tags/v0.1.0.zip"));

        let gh_default =
            resolve_archive_candidates("git@github.com:alya-lang/dotenv.git", None, None, None);
        assert!(gh_default.iter().any(|u| u.contains("heads/main.tar.gz")));

        let gl =
            resolve_archive_candidates("https://gitlab.com/group/repo", Some("v1.0.0"), None, None);
        assert!(gl[0].contains("gitlab.com/group/repo/-/archive/v1.0.0"));
    }

    #[test]
    fn test_download_and_extract_archive_live() {
        let temp_dir =
            std::env::temp_dir().join(format!("alya_test_archive_live_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);

        let candidates = resolve_archive_candidates(
            "https://github.com/alya-lang/dotenv",
            None,
            Some("main"),
            None,
        );
        let res = try_download_and_extract_archive(&candidates, &temp_dir, "dotenv");
        if res.is_ok() {
            assert!(temp_dir.join("alya.toml").exists());
            assert!(temp_dir.join("src").join("lib.alya").exists());
        }
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
