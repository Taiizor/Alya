# Alya {{VERSION}}

Alya is an expressive, compiled, multi-paradigm systems programming language designed for clarity, performance, and simplicity.

## 🚀 What's Changed

{{CHANGELOG_COMMITS}}

## 📦 Pre-built Binaries & Checksums

| Platform | Architecture | Package | SHA-256 Checksum |
|:---|:---|:---|:---|
| <img src="https://svgl.app/library/linux.svg" width="16" height="16" valign="middle" alt="Linux" /> **Linux** | `x86_64` | [alyac-{{VERSION}}-x86_64-linux.tar.gz](https://github.com/{{REPO}}/releases/download/{{VERSION}}/alyac-{{VERSION}}-x86_64-linux.tar.gz) ([sha256](https://github.com/{{REPO}}/releases/download/{{VERSION}}/alyac-{{VERSION}}-x86_64-linux.tar.gz.sha256)) | `{{LINUX_SHA}}` |
| <picture><source media="(prefers-color-scheme: dark)" srcset="https://svgl.app/library/apple_dark.svg"><source media="(prefers-color-scheme: light)" srcset="https://svgl.app/library/apple.svg"><img src="https://svgl.app/library/apple.svg" width="16" height="16" valign="middle" alt="macOS" /></picture> **macOS** | Apple Silicon (`arm64`) | [alyac-{{VERSION}}-arm64-macos.tar.gz](https://github.com/{{REPO}}/releases/download/{{VERSION}}/alyac-{{VERSION}}-arm64-macos.tar.gz) ([sha256](https://github.com/{{REPO}}/releases/download/{{VERSION}}/alyac-{{VERSION}}-arm64-macos.tar.gz.sha256)) | `{{MAC_ARM_SHA}}` |
| <picture><source media="(prefers-color-scheme: dark)" srcset="https://svgl.app/library/apple_dark.svg"><source media="(prefers-color-scheme: light)" srcset="https://svgl.app/library/apple.svg"><img src="https://svgl.app/library/apple.svg" width="16" height="16" valign="middle" alt="macOS" /></picture> **macOS** | Intel (`x86_64`) | [alyac-{{VERSION}}-x86_64-macos.tar.gz](https://github.com/{{REPO}}/releases/download/{{VERSION}}/alyac-{{VERSION}}-x86_64-macos.tar.gz) ([sha256](https://github.com/{{REPO}}/releases/download/{{VERSION}}/alyac-{{VERSION}}-x86_64-macos.tar.gz.sha256)) | `{{MAC_X64_SHA}}` |
| <img src="https://svgl.app/library/windows.svg" width="16" height="16" valign="middle" alt="Windows" /> **Windows** | `x86_64` | [alyac-{{VERSION}}-x86_64-windows.zip](https://github.com/{{REPO}}/releases/download/{{VERSION}}/alyac-{{VERSION}}-x86_64-windows.zip) ([sha256](https://github.com/{{REPO}}/releases/download/{{VERSION}}/alyac-{{VERSION}}-x86_64-windows.zip.sha256)) | `{{WIN_SHA}}` |

---

## ⚡ Quick Start

### Linux / macOS
```bash
# 1. Extract the archive
tar -xzf alyac-{{VERSION}}-<platform>.tar.gz
cd alyac-{{VERSION}}-<platform>

# 2. Check compiler version
./alyac --version

# 3. Run an Alya program directly
./alyac run main.alya

# 4. Or compile to a native binary
./alyac build main.alya
```

### Windows (PowerShell)
```powershell
# 1. Extract the archive
Expand-Archive alyac-{{VERSION}}-x86_64-windows.zip
cd alyac-{{VERSION}}-x86_64-windows

# 2. Check compiler version
.\alyac.exe --version

# 3. Run an Alya program directly
.\alyac.exe run main.alya
```

---

## 🔒 Checksum Verification

```bash
# Linux / macOS
shasum -a 256 -c alyac-{{VERSION}}-<platform>.tar.gz.sha256

# Windows (PowerShell)
(Get-FileHash alyac-{{VERSION}}-x86_64-windows.zip -Algorithm SHA256).Hash.ToLower()
```

---

## 🔗 Useful Links

- **Documentation**: [https://github.com/{{REPO}}#readme](https://github.com/{{REPO}}#readme)
- **Standard Library**: [std/](https://github.com/{{REPO}}/tree/{{VERSION}}/std)
- **Examples**: [examples/](https://github.com/{{REPO}}/tree/{{VERSION}}/examples)
- **Issue Tracker**: [GitHub Issues](https://github.com/{{REPO}}/issues)

---

{{FULL_CHANGELOG}}
