---
id: ADR-0001
title: "Wayland-Exclusive Architecture and X11 Elimination"
status: accepted
date: 2026-10-15
scope: platform/wayland, architecture/windowing
superseded_by: null
negative_knowledge: true
---

# 0001. Wayland-Exclusive Architecture and X11 Elimination

- Status: Accepted
- Date: 2026-10-15
- Deciders: Unframed Core Working Group, Platform Architects

---

## Context and Problem Statement

Classical Linux image viewers were architected during the dominance of the X Window System (X11). When modern Wayland compositors (Sway, Hyprland, GNOME Shell, KDE Plasma) emerged, viewers either operated through the XWayland compatibility emulation layer or maintained complex conditional compilation branches for X11/Xlib/XCB and Wayland protocols.

This dual-stack architecture produces significant technical friction:
1. **XWayland Performance Degradation**: Translating X11 client requests into Wayland surfaces causes buffer copying overhead, input latency, and sub-optimal frame pacing.
2. **Display Scaling Failures**: Mixed-DPI and fractional scaling under XWayland results in blurry image rasterization across high-resolution displays.
3. **Architectural Complexity**: Maintaining dual event dispatchers, disparate clipboard interfaces, and competing window positioning models bloats codebases and complicates maintenance.

We require a definitive architectural direction for Unframed's display and windowing substrate.

---

## Decision Drivers

- **Zero-Overhead Presentation**: Leverage direct Wayland client protocols (`xdg-shell`) for tearing-free, frame-paced image rendering.
- **Modern Desktop Purity**: Eliminate legacy X11 C dependencies, XCB libraries, and transitional fallback code.
- **Deterministic Deployment**: Fail fast with clear diagnostic errors if executed outside a native Wayland environment.

---

## Architectural Decisions

We establish **Unframed** as an exclusively Wayland-native application:

1. **Abolition of X11**: The codebase shall link zero X11 libraries (`libX11`, `libXext`, `libxcb`). No dual-backend abstraction layers or X11 fallback codepaths shall exist.
2. **Native Wayland Presentation**: Window lifecycle, surface allocation, buffer passing, and event pump are handled directly via Wayland protocols through the `iris` platform shell.
3. **Startup Enforcement**: If neither `WAYLAND_DISPLAY` nor `WAYLAND_SOCKET` is detected at process initialization, Unframed aborts immediately with a clear error message.

### Normative Invariants Established

- `[INV-ARCH-01] Pure Wayland Purity`: The application requires a valid Wayland environment at launch and unconditionally refuses execution in non-Wayland environments (`src/main.rs:64`).
- `[INV-ARCH-02] Zero X11 Dependency`: The workspace dependency graph contains zero references to X11 client libraries (`Cargo.toml`).

---

## Consequences

- **Positive**:
  - Eliminates thousands of lines of X11/XCB event translation and window management code.
  - Native integration with Wayland fractional scaling and high-refresh-rate compositor pacing.
  - Zero XWayland memory and latency overhead.
- **Negative & Trade-offs**:
  - Cannot be run on legacy pure X11 desktop environments without a Wayland compositor. Users on legacy X11 systems must use legacy viewers.

---

## Rejected Alternatives & Negative Knowledge

- **Dual X11/Wayland Backend Architecture**: Rejected due to high ongoing maintenance cost, conditional compilation sprawl, and the inevitable divergence between backends.
- **Running under XWayland Transparently**: Rejected because it introduces blur on fractional scaling and defeats the latency advantages of direct Wayland rendering.
