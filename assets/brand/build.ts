#!/usr/bin/env bun
/**
 * Alya Brand Assets Builder (powered by Bun & Rust resvg)
 *
 * Fully standalone, zero-browser asset pipeline:
 *  - Renders vector SVGs to transparent RGBA PNGs using Rust's resvg engine
 *  - Generates multi-resolution Windows ICO containers (16px to 256px) directly from vector math
 *
 * Usage:
 *   bun assets/brand/build.ts
 */

import { Resvg } from "@resvg/resvg-js";
import { readFileSync, writeFileSync, existsSync } from "fs";
import { join, dirname } from "path";

const SCRIPT_DIR = dirname(new URL(import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"));
const ICONS_DIR = join(SCRIPT_DIR, "icons");
const LOGOS_DIR = join(SCRIPT_DIR, "logos");

const ICO_SIZES = [16, 24, 32, 48, 64, 128, 256];

interface AssetConfig {
  svgPath: string;
  pngPath: string;
  icoPath: string;
  width: number;
  height: number;
  squareViewBox: string;
}

/**
 * Packs multiple PNG image buffers into a standard Windows ICO binary container.
 */
function buildIco(images: { size: number; data: Buffer }[]): Buffer {
  const count = images.length;
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0); // Reserved (must be 0)
  header.writeUInt16LE(1, 2); // Image type: 1 = ICO
  header.writeUInt16LE(count, 4); // Number of images

  let offset = 6 + count * 16;
  const dirEntries: Buffer[] = [];
  const imageBuffers: Buffer[] = [];

  for (const img of images) {
    const entry = Buffer.alloc(16);
    entry.writeUInt8(img.size >= 256 ? 0 : img.size, 0); // Width (0 means 256)
    entry.writeUInt8(img.size >= 256 ? 0 : img.size, 1); // Height (0 means 256)
    entry.writeUInt8(0, 2); // Color count (0 = no palette)
    entry.writeUInt8(0, 3); // Reserved (must be 0)
    entry.writeUInt16LE(1, 4); // Color planes
    entry.writeUInt16LE(32, 6); // Bits per pixel (32-bit RGBA)
    entry.writeUInt32LE(img.data.length, 8); // Size of image data in bytes
    entry.writeUInt32LE(offset, 12); // Offset to image data from start of file

    offset += img.data.length;
    dirEntries.push(entry);
    imageBuffers.push(img.data);
  }

  return Buffer.concat([header, ...dirEntries, ...imageBuffers]);
}

/**
 * Renders an SVG file to a PNG buffer using native resvg.
 */
function renderPng(svgSource: string, height: number): Buffer {
  const resvg = new Resvg(svgSource, {
    fitTo: { mode: "height", value: height },
  });
  return Buffer.from(resvg.render().asPng());
}

/**
 * Renders an SVG file directly into an ICO container at 7 standard resolutions.
 */
function renderIco(svgSource: string, squareViewBox: string): Buffer {
  // Center graphics inside square viewBox for aspect-ratio preservation without distortion
  const squareSvg = svgSource.replace(/viewBox="[^"]+"/, `viewBox="${squareViewBox}"`);

  const frames = ICO_SIZES.map((size) => {
    const resvg = new Resvg(squareSvg, {
      fitTo: { mode: "width", value: size },
    });
    const data = Buffer.from(resvg.render().asPng());
    return { size, data };
  });

  return buildIco(frames);
}

function main() {
  console.log(`[Alya Brand Builder] Running on Bun v${Bun.version} with Rust resvg\n`);

  const assets: AssetConfig[] = [
    {
      svgPath: join(ICONS_DIR, "alya-file-dark.svg"),
      pngPath: join(ICONS_DIR, "alya-file-dark.png"),
      icoPath: join(ICONS_DIR, "alya-file-dark.ico"),
      width: 424,
      height: 512,
      squareViewBox: "24 34 464 464", // Centered card in 464x464
    },
    {
      svgPath: join(ICONS_DIR, "alya-file-light.svg"),
      pngPath: join(ICONS_DIR, "alya-file-light.png"),
      icoPath: join(ICONS_DIR, "alya-file-light.ico"),
      width: 424,
      height: 512,
      squareViewBox: "24 34 464 464",
    },
    {
      svgPath: join(LOGOS_DIR, "alya-icon-dark.svg"),
      pngPath: join(LOGOS_DIR, "alya-icon-dark.png"),
      icoPath: join(LOGOS_DIR, "alya-icon-dark.ico"),
      width: 485,
      height: 512,
      squareViewBox: "131.5 97 249 249", // Centered prism emblem in 249x249
    },
    {
      svgPath: join(LOGOS_DIR, "alya-icon-light.svg"),
      pngPath: join(LOGOS_DIR, "alya-icon-light.png"),
      icoPath: join(LOGOS_DIR, "alya-icon-light.ico"),
      width: 485,
      height: 512,
      squareViewBox: "131.5 97 249 249",
    },
    {
      svgPath: join(ICONS_DIR, "alyac-dark.svg"),
      pngPath: join(ICONS_DIR, "alyac-dark.png"),
      icoPath: join(ICONS_DIR, "alyac-dark.ico"),
      width: 512,
      height: 512,
      squareViewBox: "0 0 512 512",
    },
    {
      svgPath: join(ICONS_DIR, "alyac-light.svg"),
      pngPath: join(ICONS_DIR, "alyac-light.png"),
      icoPath: join(ICONS_DIR, "alyac-light.ico"),
      width: 512,
      height: 512,
      squareViewBox: "0 0 512 512",
    },
  ];

  const startTime = performance.now();

  for (const asset of assets) {
    if (!existsSync(asset.svgPath)) {
      console.warn(`Skipping missing: ${asset.svgPath}`);
      continue;
    }

    const svgSource = readFileSync(asset.svgPath, "utf-8");
    const name = asset.svgPath.split(/[/\\]/).pop();

    console.log(`Building ${name}:`);

    // 1. Render PNG
    const pngBuffer = renderPng(svgSource, asset.height);
    writeFileSync(asset.pngPath, pngBuffer);
    console.log(`  -> PNG: ${asset.pngPath.split(/[/\\]/).pop()} (${asset.width}x${asset.height}, ${pngBuffer.length} bytes)`);

    // 2. Render ICO (multi-res vector rasterization)
    const icoBuffer = renderIco(svgSource, asset.squareViewBox);
    writeFileSync(asset.icoPath, icoBuffer);
    console.log(`  -> ICO: ${asset.icoPath.split(/[/\\]/).pop()} (7 resolutions: 16px..256px, ${icoBuffer.length} bytes)`);
  }

  const duration = (performance.now() - startTime).toFixed(1);
  console.log(`\nAll brand assets successfully generated in ${duration}ms! (Zero browser overhead)`);
}

main();
