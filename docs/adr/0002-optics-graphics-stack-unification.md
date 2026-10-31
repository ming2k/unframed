---
id: ADR-0002
title: "Optics Graphics Stack Unification"
status: accepted
date: 2026-10-16
scope: substrate/optics, graphics/vulkan, ui/immediate
superseded_by: null
negative_knowledge: true
---

# 0002. Optics Graphics Stack Unification

- Status: Accepted
- Date: 2026-10-16
- Deciders: Unframed Core Working Group, Graphics Architects

---

## Context and Problem Statement

Image viewers require three fundamental graphics capabilities:
1. Windowing and event dispatching.
2. 2D graphics rendering and texture scaling on the GPU.
3. UI overlays for metadata (OSD) and command input.

In traditional Linux software, these responsibilities are fragmented across disparate libraries (e.g. GLFW/SDL2 for windowing, raw OpenGL/EGL for rendering, and FreeType/Cairo or Dear ImGui for UI). This fragmentation introduces complex synchronization between window events, GL contexts, and font caches, along with significant runtime bloat.

We require a unified, coherent graphics substrate tailored for modern high-performance Rust applications.

---

## Decision Drivers

- **Cohesion and Modularity**: A single graphics ecosystem providing windowing, Vulkan rendering, and immediate-mode UI with shared design contracts.
- **Hardware Acceleration without Legacy Overhead**: Modern Vulkan dynamic rendering without legacy OpenGL state machine overhead or bloated render pass allocations.
- **Immediate-Mode UI Simplicity**: Render OSD banners and command prompts with sub-pixel antialiased text and zero retained widget state.

---

## Architectural Decisions

We adopt the **Optics** graphics stack (`iris` + `flux` + `lens`) as Unframed's sole windowing and rendering foundation:

1. **`iris` Substrate**: Serves as the native Wayland window shell, managing `xdg-shell` interactions, display-driven frame pacing (via `wl_surface.frame`), and input event delivery.
2. **`flux` Substrate**: Provides the Vulkan 2D canvas foundation. Manages texture images (`flux::Image`), linear and nearest-neighbour hardware samplers, and dynamic rendering pipelines.
3. **`lens` Substrate**: Handles immediate-mode UI rendering. Directly renders the OSD overlay HUD, status information, and interactive command console bar using crisp antialiased vector fonts.

---

## Consequences

- **Positive**:
  - Unified lifecycle across window creation, input handling, GPU rasterization, and overlay drawing.
  - Sub-millisecond differential redraws and native high-refresh-rate pacing (60/120/144 Hz+).
  - Clean immediate-mode UI eliminates layout jitter and synchronization bugs between image transforms and overlay text.
- **Negative & Trade-offs**:
  - Requires Vulkan-capable graphics hardware and drivers; systems lacking Vulkan ICDs cannot execute Unframed.

---

## Rejected Alternatives & Negative Knowledge

- **OpenGL / EGL via `glutin` + `glow`**: Rejected due to OpenGL's global state machine complexities, legacy context threading quirks, and lack of modern dynamic rendering primitives.
- **Retained GUI Frameworks (GTK4 / Qt6 / Iced)**: Rejected because their heavy widget hierarchies, background layout solvers, and multi-megabyte dependencies introduce excessive overhead for a lightweight image viewer.
