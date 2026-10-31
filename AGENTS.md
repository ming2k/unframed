# Agent Directives & Architecture Topology

This document specifies the operational guardrails, mental model, and invariant constraints for AI agents and human engineers developing and maintaining **unframed (`ufm`)**.

---

## 1. Operational Guardrails

- **Prohibited Git Commands**: NEVER execute destructive commands such as `git reset --hard` (unless explicitly instructed), `git branch -D`, or force pushes.
- **Process Termination**: Any spawned processes (testing loops, benchmarks) MUST be finite and self-terminating. Never leave hanging IPC daemon loops.
- **Strict Verification Gate**: Always verify code changes with `cargo clippy --all-targets` and `cargo test --all-targets` before declaring any task complete.

---

## 2. Core Mental Model & Subsystem Boundaries

`ufm` is an ultra-fast, modern, Wayland-native image viewer built on top of the **Optics** (`iris` + `flux` + `lens`) graphics stack, designed as a memory-safe, Rust-native evolution of `imv`.

```text
Application Entry (ufm / unframed)
  ├── Platform Shell (iris)
  │     ├── Native Wayland (xdg-shell, display pacing, dynamic event loop)
  │     └── Input Routing (keyboard, mouse drag, cursor, scroll)
  ├── 2D Drawing Canvas (flux)
  │     ├── Vulkan GPU Canvas (hardware-accelerated blit, subpixel affine transforms)
  │     ├── Samplers (Linear bilinear, NearestNeighbour pixel art)
  │     └── Device Management & Image Texture Allocation
  ├── Immediate-Mode UI (lens)
  │     ├── OSD Overlay Banner (format, zoom, dimensions, index)
  │     └── Interactive Command Console (: prompt, line editing, tab completion)
  ├── Image Processing Subsystem
  │     ├── Format Router & Decoder (image crate)
  │     ├── Vector Rasterizer (resvg / usvg)
  │     ├── Netpbm Decoder (P1 ~ P7 PAM)
  │     └── Animated Multi-Frame Engine (GIF / Animated WebP timeline)
  ├── Navigation & Viewport Subsystem
  │     ├── Playlist Navigator (directory traversal, loop, history)
  │     └── Viewport Math (aspect fit, shrink, crop, center, pan, rotate, flip)
  └── Inter-Process Communication
        ├── UNIX Domain Socket Listener (ufm-$PID.sock, imv-$PID.sock)
        └── Companion Tools (ufm-msg, imv-msg)
```

---

## 3. Invariants & Architectural Rules

1. **[INV-WAYLAND-ONLY] No X11 Residue**: The codebase MUST NOT link, depend on, or fallback to X11, GLX, XCB, or Cairo. Execution requires `WAYLAND_DISPLAY` or `WAYLAND_SOCKET`. If missing, the binary exits cleanly with a diagnostic message.
2. **[INV-HIDPI-COMPENSATION] Physical-Logical Decoupling**: Viewport coordinates, window sizes, and mouse cursors are calculated in logical pixels. Drawing into `flux_canvas` MUST apply `paint_host.scale()` compensation (`canvas.scale(s, s)`) so rendering remains crisp and properly dimensioned on HiDPI panels.
3. **[INV-POWER-EFFICIENCY] Static State Gating**: Only request frames via `iris::request_animation_frame()` when an animation is actively ticking, slideshow is advancing, or `needs_redraw` is true. When static, signal `iris::paint_mark_static()` to save CPU/GPU cycles.
4. **[INV-IPC-PARITY] Backward Compatibility**: Socket names MUST include both `ufm-$PID.sock` and `imv-$PID.sock`. The CLI MUST supply both `ufm-msg` and `imv-msg` so existing tiling window manager scripts (Sway, Hyprland, i3) work without modifications.
5. **[INV-PREMULTIPLIED-ALPHA] Premultiplied GPU Blending**: All pixel buffers uploaded to `flux::Image` MUST have premultiplied alpha applied prior to or during texture creation for accurate `SRC_OVER` Porter-Duff compositing.

---

## 4. Documentation Standards

All repository documentation adheres to the **Diátaxis framework**:
- `docs/tutorials/`: Learning-oriented guides for new users.
- `docs/how-to/`: Task-oriented guides solving specific problems.
- `docs/reference/`: Information-oriented technical descriptions (CLI, commands, config, formats).
- `docs/explanation/`: Understanding-oriented architectural discussions.
- `docs/adr/`: Architectural Decision Records documenting immutable decision history.
- `docs/dev/`: Developer, contributor, and subsystem specifications.
