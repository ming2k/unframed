# Explanation: Architecture Overview

`ufm` is constructed as a modern, memory-safe, hardware-accelerated image viewing engine written in Rust on top of the **Optics** graphics foundation.

---

## 1. Why Rewrite `imv` in Rust?

`imv` is a beloved, minimalist image viewer for tiling window managers. However, its original C implementation accumulated significant legacy complexity:
- Manual memory management across asynchronous decoders, bitmap pools, and format structs.
- Two divergent window backend implementations: `wl_window.c` (Wayland) and `x11_window.c` (X11).
- Direct Cairo and GLX rendering, which suffers from tearing or software rasterization overhead on modern compositors.
- Fragile string parsing and C pointer juggling for IPC and key bindings.

Rewriting the project in Rust provides:
- **Compile-Time Memory Safety**: Eliminates use-after-free, double-free, and data races.
- **Modern Hardware Acceleration**: Direct Vulkan 2D canvas drawing via `flux`.
- **Pure Wayland Focus**: Zero lines of X11 legacy code.
- **Robust Extensibility**: Rust's ecosystem provides production-grade decoders (`resvg`, `image`) without fragile external C library dependencies.

---

## 2. Optics Subsystem Demarcation

`ufm` integrates three orthogonal libraries from `optics`:

```text
               ┌───────────────────────────────┐
               │         ufm Runtime           │
               └───────────────┬───────────────┘
                               │
       ┌───────────────────────┼───────────────────────┐
       ▼                       ▼                       ▼
┌──────────────┐       ┌──────────────┐       ┌─────────────────┐
│     iris     │       │     flux     │       │      lens       │
│ Platform Shell│      │ Vulkan Canvas│       │ Immediate-Mode  │
│ (Wayland,    │       │ (2D Drawing, │       │ UI Chrome       │
│  Event Loop) │       │  Samplers)   │       │ (OSD, Console)  │
└──────────────┘       └──────────────┘       └─────────────────┘
```

1. **`iris` (Platform Shell)**:
   - Owns the native Wayland window (`xdg_toplevel`).
   - Dispatches the per-frame event loop.
   - Merges pointer motion, dragging, wheel, and keyboard inputs into an atomic per-frame `Input` snapshot.
2. **`flux` (Rendering Engine)**:
   - Vulkan 2D Canvas pipeline.
   - Fast GPU texture upload via `Image::from_bytes`.
   - Affine transformation matrices (rotation, translation, scaling) executed on the GPU.
3. **`lens` (UI Chrome)**:
   - Evaluates immediate-mode widgets inside `build`.
   - Renders the OSD metrics banner and the command console overlay.
