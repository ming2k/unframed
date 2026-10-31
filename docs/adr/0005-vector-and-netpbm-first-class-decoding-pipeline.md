---
id: ADR-0005
title: "Vector and Netpbm First-Class Decoding Pipeline"
status: accepted
date: 2026-10-22
scope: media/svg, media/ppm, decoding/pipeline
superseded_by: null
negative_knowledge: true
---

# 0005. Vector and Netpbm First-Class Decoding Pipeline

- Status: Accepted
- Date: 2026-10-22
- Deciders: Unframed Core Working Group, Media Pipeline

---

## Context and Problem Statement

Most desktop image viewers treat vector graphics (SVG) and Netpbm images (PBM, PGM, PPM, PAM) as secondary formats or rely on external runtime dependencies (e.g., `librsvg`, `netpbm-tools`) that are heavy, slow, or inconsistent across Linux distributions.

In modern workflows, SVG is omnipresent for UI design and icon assets, requiring crisp rendering at arbitrary zoom scales. Similarly, Netpbm formats remain the standard exchange medium for scientific visualization, compiler output, and computer vision pipelines, requiring robust parsing of both ASCII and binary variants.

---

## Decision Drivers

- **Zero External C Library Dependencies**: Media decoding must be self-contained in Rust without dynamic linking against `librsvg` or `libnetpbm`.
- **First-Class Vector Scalability**: SVG rendering must produce crisp vector output without pixelation.
- **Complete Netpbm Suite Coverage**: Native, zero-dependency support for P1 through P7 formats, with robust comment and whitespace parsing.

---

## Architectural Decisions

We establish a first-class media decoding pipeline in Unframed:

1. **SVG via `resvg` / `usvg` (`src/svg.rs`)**:
   - Integrates `resvg` directly in Rust.
   - Extracts viewBox and intrinsic dimensions; dynamically rasterizes vectors to RGBA target pixel buffers.
2. **Dedicated Zero-Dependency Netpbm Engine (`src/ppm.rs`)**:
   - Implements a purpose-built streaming parser covering all 7 Netpbm formats:
     - P1 (ASCII PBM), P2 (ASCII PGM), P3 (ASCII PPM).
     - P4 (Binary PBM), P5 (Binary PGM), P6 (Binary PPM).
     - P7 (Binary PAM with arbitrary tuple types).
   - Fully supports embedded `#` comments and non-standard whitespace delimiters.
3. **Animated Media Support**:
   - Dedicated multi-frame extraction for GIF and WebP via `image::codecs::{gif, webp}`, preserving exact per-frame millisecond delays.

---

## Consequences

- **Positive**:
  - Zero system library dependencies on `librsvg` or Netpbm C utilities.
  - Complete format parity across all Linux distributions.
  - Seamless handling of scientific Netpbm files and modern vector assets.
- **Negative & Trade-offs**:
  - `resvg` and fontdb add to the compilation footprint of the Unframed binary.

---

## Rejected Alternatives & Negative Knowledge

- **Shelling out to `convert` or `rsvg-convert`**: Spawning external CLI processes to convert SVGs to temporary PNGs was rejected due to process creation overhead, disk I/O latency, and inability to handle interactive zoom scaling.
