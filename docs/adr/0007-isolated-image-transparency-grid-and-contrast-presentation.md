---
id: ADR-0007
title: "Isolated Image Transparency Grid and Contrast Presentation"
status: accepted
date: 2026-10-26
scope: render/pipeline, ui/transparency, interaction/presentation
superseded_by: null
negative_knowledge: true
---

# 0007. Isolated Image Transparency Grid and Contrast Presentation

- Status: Accepted
- Date: 2026-10-26
- Deciders: Unframed Core Working Group, Graphics Architects

---

## Context and Problem Statement

When viewing transparent graphics (such as 1-bit palette transparent GIFs, animated WebP sequences, alpha-channel PNGs, and vector SVGs), viewing software must provide a backing surface against which transparent pixels are rendered.

Historically, naive implementations adopt one of two flawed approaches:
1. **Dynamic / Adaptive Background Color**: Attempting to scan pixel colors and calculate a contrasting background color.
2. **Global Full-Window Checkerboard**: Flooring the entire application window with high-contrast checkerboard squares.

Both approaches degrade viewer usability:
- Adaptive backgrounds produce severe visual flickering during animated GIF playback (inter-frame color shifts cause background strobe), distort human color perception through simultaneous contrast, and fail completely on graphics containing both pure black and pure white elements.
- Full-window checkerboards cause extreme visual noise and cognitive fatigue when viewing small images in a large window.

---

## Decision Drivers

- **Zero Flicker Animation**: Animated GIF and WebP playback must remain rock-solid without strobe or frame-to-frame background shifts.
- **Visual Orthogonality**: The window frame canvas and the image's transparency backing must be decoupled.
- **Universal Industry Standards**: Conformance to the time-tested Photoshop/GIMP/Figma checkerboard standard for unambiguous transparency indication.
- **Inspector Usability**: Fast single-key toggling between light checkers, dark checkers, solid white, and solid black to evaluate alpha matte fringing.

---

## Architectural Decisions

We reject dynamic color adaptation as an anti-pattern and implement **Isolated Image-Bound Transparency Presentation** (`src/app.rs`, `src/config.rs`):

### 1. Two-Tier Canvas Architecture
- **Window Canvas Background (`BackgroundMode`)**: Fills the entire application window bounds (`win_w x win_h`). Defaults to solid neutral black (`0xFF000000`) for high-immersion viewing.
- **Image Transparency Backing (`TransparencyGrid`)**: Renders strictly **within the local bounding box of the active image** (`(0.0, 0.0)` to `(scaled_w, scaled_h)`).

### 2. Coordinate Transformation Alignment
The transparency grid is drawn inside the image's local transformation scope (`canvas.translate(cx, cy); canvas.rotate(...); canvas.scale(...)`). Consequently:
- Checkerboard tiles move, zoom, and rotate in 1:1 mathematical alignment with the image quad.
- Window gutters outside the image bounds remain pure, uncluttered window canvas background.
- Fixed 16px screen-space tile proportions avoid moiré or giant blocks across any viewport scale.

### 3. Multi-Mode Inspection Pipeline (`TransparencyGrid`)
The image backing supports 5 discrete, cyclable inspection modes:
1. `ChecksLight`: Classic Photoshop-style light checkerboard (`#FFFFFF` + `#CCCCCC`).
2. `ChecksDark`: Modern dark-mode checkerboard (`#202020` + `#383838`).
3. `SolidWhite`: Pure white backing (`#FFFFFF`) for inspecting dark line-art and light-theme web asset rendering.
4. `SolidBlack`: Pure black backing (`#000000`) for inspecting bright graphics and dark-theme web asset rendering.
5. `Off`: Complete pass-through directly to the window canvas background.

### 4. Direct Keybinding & IPC Control
- Default keybinding: `b` executes `transparency next` to cycle presentation modes instantly during navigation and playback.
- Interactive Command: `:transparency <checks-light|checks-dark|white|black|off|next>` (aliased to `:trans`).
- OSD Telemetry: Exported via environment variable `$ufm_transparency` for real-time status overlays.

---

## Consequences

### Positive
- **No Strobe or Flickering**: Animated GIFs and WebP sequences play with zero background brightness fluctuation.
- **Precise Edge Inspection**: Users can instantly detect dirty matte halos and transparency fringing by toggling with `b`.
- **Clean Aesthetic**: Clean window margins without full-screen checkerboard clutter.

### Negative / Trade-Offs
- Requires an additional drawing pass prior to drawing the GPU image when transparency backing is enabled (fully mitigated by simple clipped fills).
