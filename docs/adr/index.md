# Architectural Decision Records

This directory contains the canonical Architectural Decision Records (ADRs) governing Unframed. For authoring standards, lifecycle rules, and frontmatter schemas, consult the [ADR Standards](../dev/standards/adr.md).

---

## Architecture Decision Registry

| ID | Title | Status | Scope | Decision Summary & Primary Invariant | Date |
| :--- | :--- | :--- | :--- | :--- | :--- |
| [0001](0001-wayland-exclusive-architecture-and-x11-elimination.md) | Wayland-Exclusive Architecture and X11 Elimination | Accepted | platform/wayland, architecture/windowing | Abolish legacy X11/XCB code and libraries; require native Wayland compositor connection and xdg-shell presentation [`[INV-ARCH-01]`, `[INV-ARCH-02]`] | 2026-10-15 |
| [0002](0002-optics-graphics-stack-unification.md) | Optics Graphics Stack Unification | Accepted | substrate/optics, graphics/vulkan, ui/immediate | Unify platform shell, Vulkan 2D canvas, and immediate-mode OSD overlays under the Optics (`iris` + `flux` + `lens`) suite [`[INV-ARCH-01]`, `[INV-ARCH-04]`] | 2026-10-16 |
| [0003](0003-canonical-binary-naming-as-ufm.md) | Canonical Binary Naming as ufm | Accepted | cli/naming, cli/compatibility, ipc/socket | Establish `ufm` as primary binary with socket listeners (`ufm`, `unframed`) | 2026-10-18 |
| [0004](0004-trie-based-prefix-keybinding-engine.md) | Trie-Based Prefix Keybinding Engine | Accepted | input/binds, interaction/chords, ui/console | Replace flat keymap lookup with a stateful prefix-trie parser supporting multi-key chords (`gg`, `<Shift+G>`) and modal console isolation | 2026-10-20 |
| [0005](0005-vector-and-netpbm-first-class-decoding-pipeline.md) | Vector and Netpbm First-Class Decoding Pipeline | Accepted | media/svg, media/ppm, decoding/pipeline | Integrate `resvg` for dynamic resolution-independent SVG rasterization and a dedicated zero-dependency parser for the complete Netpbm suite (P1–P7) | 2026-10-22 |
| [0006](0006-retained-image-state-and-lazy-gpu-texture-upload.md) | Retained Image State and Lazy GPU Texture Upload | Accepted | render/pipeline, memory/caching, texture/vulkan | Separate decoded CPU pixel frames from Vulkan GPU texture allocation with lazy dirty-flagged texture synchronization [`[INV-ARCH-04]`] | 2026-10-24 |
| [0007](0007-isolated-image-transparency-grid-and-contrast-presentation.md) | Isolated Image Transparency Grid and Contrast Presentation | Accepted | render/pipeline, ui/transparency, interaction/presentation | Reject dynamic color adaptation as an anti-pattern; implement decoupled two-tier window canvas and image-bound multi-mode checkerboard presentation (`b` key cycling) | 2026-10-26 |
