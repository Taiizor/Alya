use std::collections::BTreeMap;
use std::path::PathBuf;

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
    Cache {
        clean: bool,
        all: bool,
    },
    Clean {
        all: bool,
    },
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub alya_version: Option<String>,
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
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageLock {
    pub version: u32,
    pub packages: Vec<LockedPackage>,
}

#[derive(Debug, Clone)]
pub struct CachedPackageDetails {
    pub name: String,
    pub version: String,
    pub source: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub file_count: usize,
}
