# ADR 0001: Wayland-Only Architecture Powered by Optics

- **Status**: Accepted
- **Date**: 2026-11-01
- **Deciders**: Engineering Team

---

## Context

The original `imv` project supported both X11 and Wayland via dual platform window backends (`x11_window.c` and `wl_window.c`), paired with OpenGL, Cairo, and Pango.

Over time:
1. X11 codebase required separate event loops, xcb / xkbcommon-x11 dependencies, and manual GLX context management.
2. Modern tiling desktop environments (Sway, Hyprland, River, Cosmic) run purely on Wayland.
3. The `../optics` stack provides a modern, high-performance C23/Rust foundation built exclusively for Wayland on Linux, featuring Vulkan 1.3 dynamic rendering (`flux`), native Wayland shell (`iris`), and immediate-mode UI (`lens`).

---

## Decision

1. **Drop X11 Entirely**: The Rust rewrite (`unframed` / `ufm`) will solely support Wayland on Linux. All X11, GLX, XCB, and Cairo dependencies are permanently removed.
2. **Optics as the Foundation**:
   - `iris` provides windowing, xdg-shell, clipboard, and the frame event loop.
   - `flux` provides the hardware-accelerated Vulkan 2D canvas.
   - `lens` provides the UI layout and widget layer for OSD overlays and command prompt.
3. **Fail-Fast Environment Check**: If launched without `WAYLAND_DISPLAY` or `WAYLAND_SOCKET`, the binary halts immediately with an informative diagnostic message.

---

## Consequences

### Positive
- Zero X11 legacy code or dual-backend abstractions.
- High-efficiency Vulkan 1.3 rendering with display pacing and low-power frame skipping.
- Drastically simpler codebase, reducing maintenance burden and vulnerability surface.

### Negative
- Cannot run on legacy pure X11 desktop sessions without Xwayland.
