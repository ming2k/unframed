# ADR 0002: Format Expansion and Native Vector Rasterization

- **Status**: Accepted
- **Date**: 2026-11-01
- **Deciders**: Engineering Team

---

## Context

`imv` had optional backend plugins for various formats via C libraries (such as `librsvg` for SVG, and custom C code for PPM). In a pure Rust environment, relying on dynamic linking to C libraries (e.g. `librsvg`, `glib`, `cairo`) introduces heavy system dependencies, complex build scripts, and cross-compilation hurdles.

---

## Decision

1. **Native Rust SVG Rasterization**:
   - Use `resvg` and `usvg` to parse and render SVG files directly into 32-bit premultiplied RGBA pixel buffers in pure Rust.
   - Decouple SVG parsing from C ABI or GLib/GObject dependencies.
2. **Dedicated Netpbm (PPM/PGM/PBM/PAM) Implementation**:
   - Provide a specialized Netpbm parser (`src/ppm.rs`) covering both ASCII (P1, P2, P3) and binary (P4, P5, P6, P7 PAM) variants with robust comment and whitespace handling.
3. **Rust Image Ecosystem**:
   - Use the `image` crate (0.25) for PNG, JPEG, GIF, WebP, TIFF, BMP, QOI, Farbfeld, TGA, ICO, HDR, and EXR.
4. **Unified Premultiplied RGBA Memory Contract**:
   - Standardize all loaded image frames into contiguous 32-bit premultiplied RGBA pixel buffers ready for direct Vulkan texture upload.

---

## Consequences

### Positive
- Zero external C dynamic library dependencies for image decoders.
- Memory safety across all format decoders (resistant to malformed image exploits).
- First-class support for animated GIF and animated WebP playback.

### Negative
- Pure Rust vector rasterization of extremely complex SVGs may have slight CPU overhead compared to native SIMD Cairo routines, though fully mitigated by caching rendered bitmaps.
