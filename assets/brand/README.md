# Alya Brand Assets

Official visual identity assets, file system icons, and emblems for the **Alya** programming language.

These assets are engineered for editor icon themes (VS Code, JetBrains), operating system file associations (Windows Explorer, macOS Finder), application binaries (`alyac`), documentation, and websites.

---

## Directory Structure

```text
assets/brand/
├── icons/                        # File system icons for .alya source files
│   ├── alya-file-dark.svg        # Vector file icon (Dark theme)
│   ├── alya-file-dark.png        # 424×512 transparent PNG (Dark theme)
│   ├── alya-file-dark.ico        # Multi-resolution Windows ICO (Dark theme)
│   ├── alya-file-light.svg       # Vector file icon (Light theme)
│   ├── alya-file-light.png       # 424×512 transparent PNG (Light theme)
│   └── alya-file-light.ico       # Multi-resolution Windows ICO (Light theme)
├── logos/                        # Standalone Alya monogram emblems
│   ├── alya-icon-dark.svg        # Vector 3D prism emblem (Dark theme)
│   ├── alya-icon-dark.png        # 485×512 transparent PNG (Dark theme)
│   ├── alya-icon-dark.ico        # Multi-resolution Windows ICO (Dark theme)
│   ├── alya-icon-light.svg       # Vector 3D prism emblem (Light theme)
│   ├── alya-icon-light.png       # 485×512 transparent PNG (Light theme)
│   └── alya-icon-light.ico       # Multi-resolution Windows ICO (Light theme)
├── build.py                      # One-command asset regenerator script
└── README.md                     # This documentation
```

---

## Asset Specifications

### 1. File System Icons (`icons/`)
- **Silhouette**: Modern document sheet with a precision 45° dog-ear fold, delicate crease lighting, and realistic elevation drop shadow.
- **Watermark**: Faint syntax lines and micro-hardware trace lines inside the document sheet.
- **Badge**: Bottom `.ALYA` pill with a pulse status indicator and 100% vector-drawn letterforms (zero external font dependencies).
- **Native Dimensions**: **`424 × 512 px`** (Height: 512px, Width: 424px, tightly bounded with no empty side gaps).

### 2. Standalone Emblems (`logos/`)
- **Motif**: 3D chiseled Delta Prism forming the letter **A**, featuring:
  - **Left Wing**: Ascending Electric Amethyst & Violet gradient (`#9333ea` → `#c084fc`).
  - **Right Wing**: Descending Vivid Sky & Azure Cyan gradient (`#0284c7` → `#38bdf8`).
  - **Crossbar**: Hypersonic forward energy chevron with a glowing white core spark, symbolizing Alya's native assembly codegen and near-C execution speed.
  - **Summit**: Precision diamond crystal crown.
- **Native Dimensions**: **`485 × 512 px`** (Height: 512px, Width: 485px, tightly bounded to the glyph).

### 3. Windows Multi-Resolution Containers (`*.ico`)
Every `.ico` file contains 7 embedded resolutions with **32-bit RGBA transparency**:
- `16×16 px` — Taskbar small, File Explorer Details / List view
- `24×24 px` — Start menu, high-DPI taskbar
- `32×32 px` — Desktop icon, File Explorer Tiles / Medium view
- `48×48 px` — File Explorer Large icon view
- `64×64 px` — High-DPI displays / Explorer Extra Large
- `128×128 px` — Touch displays and high-DPI scaling
- `256×256 px` — Ultra high-resolution (PNG-compressed inside ICO)

*Note: In all `.ico` files, non-square graphics are centered within a square canvas using transparent margins to guarantee zero stretching or distortion by the Windows Shell.*

---

## Brand Color Palette

| Name | Hex Code | Purpose |
| :--- | :--- | :--- |
| **Amethyst Purple** | `#c084fc` / `#9333ea` / `#581c87` | Left wing facet, compiler frontend identity |
| **Sky Cyan** | `#38bdf8` / `#0ea5e9` / `#0284c7` | Right wing facet, native assembly speed |
| **Hyper Core** | `#ffffff` | Center energy spark & diamond crown |
| **Obsidian Navy** | `#141928` → `#080c16` | Dark document surface background |
| **Pearl White** | `#ffffff` → `#f1f5f9` | Light document surface background |
| **Dark Slate** | `#0f172a` | High-contrast typography on light theme |

---

## Regenerating Assets

### Option A: One-Command Build Script (Recommended)

Run the included `build.py` script to rasterize all SVGs to transparent PNGs and package multi-resolution ICOs automatically:

```bash
python assets/brand/build.py
```

*Requirements: Python 3.8+ with Pillow (`pip install Pillow`), and Google Chrome or Microsoft Edge installed.*

---

### Option B: Manual CLI Rasterization (PowerShell / Windows)

To render an SVG directly into a transparent PNG using headless Chrome or Edge:

```powershell
Start-Process -FilePath "C:\Program Files\Google\Chrome\Application\chrome.exe" -ArgumentList @(
  "--headless=new",
  "--hide-scrollbars",
  "--default-background-color=00000000",
  "--window-size=424,512",
  "--screenshot=$PWD\assets\brand\icons\alya-file-dark.png",
  "file:///$($PWD.Path.Replace('\','/'))/assets/brand/icons/alya-file-dark.svg"
) -Wait -NoNewWindow
```

---

### Option C: Manual Python ICO Generation

To generate multi-resolution `.ico` files from any PNG:

```python
from PIL import Image

def png_to_ico(png_path, ico_path):
    img = Image.open(png_path).convert("RGBA")
    w, h = img.size
    max_dim = max(w, h)
    square = Image.new("RGBA", (max_dim, max_dim), (0, 0, 0, 0))
    square.paste(img, ((max_dim - w) // 2, (max_dim - h) // 2))
    square.save(
        ico_path,
        format="ICO",
        sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    )

png_to_ico("assets/brand/icons/alya-file-dark.png", "assets/brand/icons/alya-file-dark.ico")
```

---

## Integration & Usage

### 1. VS Code Icon Theme

In your extension's `package.json`:

```json
{
  "contributes": {
    "iconThemes": [
      {
        "id": "alya-icons",
        "label": "Alya Icon Theme",
        "path": "./icons/icon-theme.json"
      }
    ]
  }
}
```

In `icon-theme.json`:

```json
{
  "fileExtensions": {
    "alya": "_alya_file"
  },
  "iconDefinitions": {
    "_alya_file": {
      "iconPath": "./assets/brand/icons/alya-file-dark.svg"
    },
    "_alya_file_light": {
      "iconPath": "./assets/brand/icons/alya-file-light.svg"
    }
  },
  "light": {
    "fileExtensions": {
      "alya": "_alya_file_light"
    }
  }
}
```

### 2. Windows Executable Icon (`alyac.exe`)

When compiling `alyac` with Rust, embed `alya-icon-dark.ico` as the binary application icon in `build.rs` using `winres`:

```rust
// Cargo.toml: [build-dependencies] winres = "0.1"
fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/brand/logos/alya-icon-dark.ico");
        res.compile().unwrap();
    }
}
```

### 3. Web Favicon

```html
<link rel="icon" type="image/svg+xml" href="assets/brand/logos/alya-icon-dark.svg" />
<link rel="icon" type="image/x-icon" href="assets/brand/logos/alya-icon-dark.ico" />
```
