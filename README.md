# PhotoCurse

> Pixels, layers and PSDs, cursed into shape. Native, fast, offline.

PhotoCurse is a desktop image editor written entirely in Rust. It runs natively on Windows,
macOS, Linux and FreeBSD, and the same code runs in the browser as WebAssembly. There is no
account, no cloud and no subscription: your files stay on your machine.

**Contact:** Discord **execurse2**

---

## Contents

1. [Why PhotoCurse](#why-photocurse)
2. [Quick start](#quick-start)
3. [What it can do](#what-it-can-do)
4. [Toolbox](#toolbox)
5. [Files it understands](#files-it-understands)
6. [Scripting and automation](#scripting-and-automation)
7. [How it is built](#how-it-is-built)
8. [Project status](#project-status)
9. [License](#license)

---

## Why PhotoCurse

| | |
|---|---|
| **Native** | A real desktop app on the GPU (wgpu: Vulkan, Metal, DX12, WebGPU), no browser shell around it. |
| **Non-destructive** | Adjustment layers, masks, smart objects and smart filters keep every edit reversible. |
| **Layered files** | Opens and saves layered PSD/PSB files, plus many other formats. |
| **Scriptable** | Every menu item is a command you can also run from a CLI, a JSON socket or an MCP server. |
| **Own look** | A crimson accent and soft, rounded controls, with several dark and light themes to pick from. |

## Quick start

You need a recent stable Rust toolchain.

```sh
cargo run --release -p photocraft            # start the editor
cargo run --release -p photocraft -- my.psd  # start it with a file
cargo test --workspace                       # run the tests
```

Contributors and AI agents should read [`AGENTS.md`](AGENTS.md) and the [`docs/`](docs/) folder first.

## What it can do

**Layers**
Groups, clipping masks, pixel and vector masks, solid/gradient/pattern fill layers, smart objects,
27 blend modes, opacity and fill, locks, colour labels, merge and flatten.

**Adjustments**
16 adjustment types, as layers or applied straight to pixels: Curves, Levels, Vibrance,
Hue/Saturation, Black & White, Channel Mixer, Gradient Map, Photo Filter, Selective Color,
Color Lookup and more.

**Layer effects**
Shadows, glows, bevel, satin, stroke and colour/gradient/pattern overlays, live on any layer,
type layers included.

**Selections**
Marquees, lassos, Magic Wand, Quick Selection, Object Selection, Select Subject and Select and
Mask. The smart selection runs locally.

**Painting**
A brush engine with shape dynamics, scattering, texture, dual brush, colour dynamics, wet edges
and smoothing, driven by pen pressure, tilt and rotation.

**Type and vectors**
Point and paragraph text edited on the canvas; shape layers, pens and paths with gradient fills
and dashed strokes.

**Filters**
Blurs, sharpening, noise reduction, distortions, stylize and render filters, each with a live
preview on the canvas.

**Colour**
RGB, Grayscale, CMYK and Lab at 8, 16 and 32 bits per channel, with ICC colour management, soft
proofing and gamut warnings.

## Toolbox — 67 tools

Move · Artboard · Rectangular and Elliptical Marquee · Single Row Marquee · Single Column Marquee · Lasso · Polygonal Lasso · Magnetic Lasso · Object Selection · Quick Selection · Magic Wand · Crop · Perspective Crop · Slice · Slice Select · Frame · Eyedropper · Color Sampler · Ruler · Note · Count · Remove · Spot Healing Brush · Healing Brush · Patch · Content-Aware Move · Red Eye · Brush · Pencil · Mixer Brush · Color Replacement · Clone Stamp · Pattern Stamp · History Brush · Eraser · Background Eraser · Magic Eraser · Gradient · Paint Bucket · Blur · Sharpen · Smudge · Dodge · Burn · Sponge · Pen · Freeform Pen · Curvature Pen · Add Anchor Point · Delete Anchor Point · Convert Point · Horizontal and Vertical Type · Horizontal and Vertical Type Mask · Path Selection · Direct Selection · Rectangle · Ellipse · Triangle · Polygon · Line · Custom Shape · Hand · Rotate View · Zoom

## Files it understands

| Kind | Formats |
|---|---|
| Layered, read and write | PSD, PSB, layered TIFF, OpenRaster, native `.pcraft` |
| Layered, read only | Paint.NET PDN, Krita KRA, Affinity documents |
| Vector | SVG (opens as shape layers) |
| Flat images | PNG, JPEG, WebP, GIF, BMP, TGA, ICO, QOI, PNM |
| High dynamic range | OpenEXR, Radiance HDR, FITS |

Details per format: [codec capability matrix](crates/codecs/README.md#capability-matrix-default-build).

## Scripting and automation

The editor has a single command registry with more than 500 commands. Whatever you click in the
menus can also be run:

- from the **command-line tool** (open, edit, export, batch-process folders),
- over a **JSON control channel** to the running app (loopback only, authenticated),
- by an AI agent through the built-in **MCP server**.

Protocol reference: [`docs/control-protocol.md`](docs/control-protocol.md).

## How it is built

- An engine with a pure-data document model; the UI on top is thin and only dispatches commands.
- A CPU compositor as the reference, and a GPU compositor tested against it.
- Sparse 256×256 copy-on-write tiles, so undo is cheap and huge canvases stay light.
- Heavy work runs per tile on all CPU cores.
- Over 1,700 automated tests, including file round trips and multi-bit-depth checks.

## Project status

**Alpha.** Most features exist in some form, but expect rough edges. Don't use it as your only
tool for important work yet. Bugs and ideas: message **execurse2** on Discord.

## License

MIT OR Apache-2.0, at your option. Third-party assets keep their own licenses, listed in
[`ATTRIBUTION.md`](ATTRIBUTION.md).

<sub>Adobe and Photoshop are trademarks of Adobe Inc. PhotoCurse is an independent project, not affiliated with Adobe.</sub>
