#!/usr/bin/env python3
"""
Alya Brand Assets Builder
Regenerates PNG (transparent RGBA) and multi-resolution ICO files from SVGs.

Requirements:
  - Python 3.8+ with Pillow (PIL)
  - Google Chrome or Microsoft Edge (for headless vector rasterization)

Usage:
  python assets/brand/build.py
"""

import os
import sys
import shutil
import subprocess
from pathlib import Path
from PIL import Image

SCRIPT_DIR = Path(__file__).resolve().parent
ICONS_DIR = SCRIPT_DIR / "icons"
LOGOS_DIR = SCRIPT_DIR / "logos"

# Multi-resolution sizes for Windows / Web ICO containers
ICO_SIZES = [
    (16, 16),
    (24, 24),
    (32, 32),
    (48, 48),
    (64, 64),
    (128, 128),
    (256, 256),
]

def find_browser():
    """Finds Google Chrome or Microsoft Edge executable."""
    candidates = [
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        shutil.which("chrome"),
        shutil.which("google-chrome"),
        shutil.which("chromium"),
        shutil.which("msedge"),
    ]
    for candidate in candidates:
        if candidate and os.path.exists(candidate):
            return str(candidate)
    return None

def render_svg_to_png(browser_path, svg_path, png_path, width, height):
    """Renders an SVG to PNG with full RGBA transparency using headless Chrome/Edge."""
    html_tmp = png_path.with_suffix(".tmp.html")
    svg_uri = svg_path.as_posix()
    
    html_content = f"""<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  html, body {{
    margin: 0; padding: 0;
    background: transparent !important;
    width: {width}px; height: {height}px;
    overflow: hidden;
  }}
  img {{
    display: block;
    width: {width}px; height: {height}px;
  }}
</style>
</head>
<body>
  <img src="file:///{svg_uri}" />
</body>
</html>"""

    html_tmp.write_text(html_content, encoding="utf-8")
    
    cmd = [
        browser_path,
        "--headless=new",
        "--hide-scrollbars",
        "--default-background-color=00000000",
        f"--window-size={width},{height}",
        f"--screenshot={png_path}",
        f"file:///{html_tmp.as_posix()}",
    ]
    subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    
    if html_tmp.exists():
        html_tmp.unlink()

def make_square_fit(img):
    """Centers non-square images inside a square transparent canvas to prevent distortion in ICO."""
    w, h = img.size
    max_dim = max(w, h)
    square = Image.new("RGBA", (max_dim, max_dim), (0, 0, 0, 0))
    offset = ((max_dim - w) // 2, (max_dim - h) // 2)
    square.paste(img, offset)
    return square

def generate_ico(png_path, ico_path):
    """Generates a multi-resolution ICO file from PNG with aspect-ratio preservation."""
    src = Image.open(png_path).convert("RGBA")
    square = make_square_fit(src)
    square.save(ico_path, format="ICO", sizes=ICO_SIZES)

def main():
    browser = find_browser()
    if not browser:
        print("Error: Google Chrome or Microsoft Edge not found. Please install one of them to rasterize SVGs.")
        sys.exit(1)
        
    print(f"Using browser: {browser}\n")

    targets = [
        # (svg_path, png_path, ico_path, width, height)
        (ICONS_DIR / "alya-file-dark.svg", ICONS_DIR / "alya-file-dark.png", ICONS_DIR / "alya-file-dark.ico", 424, 512),
        (ICONS_DIR / "alya-file-light.svg", ICONS_DIR / "alya-file-light.png", ICONS_DIR / "alya-file-light.ico", 424, 512),
        (LOGOS_DIR / "alya-icon-dark.svg", LOGOS_DIR / "alya-icon-dark.png", LOGOS_DIR / "alya-icon-dark.ico", 485, 512),
        (LOGOS_DIR / "alya-icon-light.svg", LOGOS_DIR / "alya-icon-light.png", LOGOS_DIR / "alya-icon-light.ico", 485, 512),
    ]

    for svg_path, png_path, ico_path, width, height in targets:
        if not svg_path.exists():
            print(f"Skipping missing: {svg_path.name}")
            continue

        print(f"Building {svg_path.stem}:")
        render_svg_to_png(browser, svg_path, png_path, width, height)
        print(f"  -> PNG rendered: {png_path.name} ({width}x{height})")
        
        generate_ico(png_path, ico_path)
        print(f"  -> ICO created:  {ico_path.name} (7 embedded resolutions)")

    print("\nAll brand assets successfully built!")

if __name__ == "__main__":
    main()
