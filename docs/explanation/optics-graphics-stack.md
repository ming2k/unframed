# The Optics Graphics Stack: Pure Wayland & Vulkan Dynamic Rendering

This article explains the technical rationale behind choosing the **Optics** stack (`iris` + `flux` + `lens`) as Unframed's rendering and platform foundation, and why legacy X11 abstractions were completely omitted.

---

## 1. The Cost of Legacy X11 Abstractions

Desktop Linux image viewers historically relied on X11 or dual-stack frameworks (SDL2, GTK, Qt) that maintained backwards compatibility with the X Window System. In modern Wayland environments, this architectural debt incurs significant penalties:

1. **XWayland Overhead & Memory Amplification**: Running under XWayland introduces an unnecessary translation layer where X11 protocol requests are mapped to Wayland surfaces, increasing memory consumption and introducing input latency.
2. **Display Tearing & Uncontrolled Pacing**: The X11 core protocol lacks modern frame pacing primitives, leading to visual tearing or reliance on synchronous XSync extensions that stall the CPU.
3. **Multi-DPI Scaling Fragility**: Mixed-DPI multi-monitor setups (e.g. 4K laptop display paired with 1440p desktop monitors) frequently cause blurred bitmap scaling under X11 bridges.
4. **Binary Bloat & Library Churn**: Retained-mode toolkits bundle Megabytes of layout engines, accessibility trees, and widget themes that have no bearing on a high-performance image viewer.

By shedding all X11 code and dependencies ([ADR-0001](../adr/0001-wayland-exclusive-architecture-and-x11-elimination.md)), Unframed targets modern Wayland compositors (Sway, Hyprland, GNOME Wayland, KDE Plasma Wayland, River) directly.

---

## 2. The Three Pillars of Optics

Unframed unifies its platform shell, GPU canvas, and UI presentation under the **Optics** suite ([ADR-0002](../adr/0002-optics-graphics-stack-unification.md)):

```text
┌────────────────────────────────────────────────────────────────────────┐
│                              Application                               │
│                                unframed                                │
└───────────────────┬────────────────────────────────┬───────────────────┘
                    │                                │
                    ▼                                ▼
┌───────────────────────────────────────┐ ┌──────────────────────────────┐
│                 iris                  │ │             lens             │
│   Pure Wayland Shell & Event Loop     │ │   Immediate-Mode HUD & OSD   │
└───────────────────┬───────────────────┘ └──────────────┬───────────────┘
                    │                                    │
                    └─────────────────┬──────────────────┘
                                      ▼
┌────────────────────────────────────────────────────────────────────────┐
│                                 flux                                   │
│            Vulkan 2D Dynamic Rendering Canvas & Samplers               │
└─────────────────────────────────────┬──────────────────────────────────┘
                                      ▼
┌────────────────────────────────────────────────────────────────────────┐
│                         Hardware / Compositor                          │
│             Vulkan Driver (RADV / ANV / NVIDIA) & Wayland DRM          │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 3. `iris`: Direct Wayland Shell & Frame Pacing

`iris` acts as the native Wayland client substrate:
- **`xdg-shell` Integration**: Negotiates window states (`xdg_toplevel`), handles compositor resize requests, manages maximize/minimize states, and controls fullscreen transitions without external helper daemons.
- **Display-Driven Event Loop**: Instead of busy-waiting or arbitrary `sleep()` loops, `iris` utilizes Wayland frame callbacks (`wl_surface.frame`). The compositor signals when the next hardware vertical blank (VSync) is approaching, allowing Unframed to render frames at the exact display refresh rate (60 Hz, 120 Hz, 144 Hz, 240 Hz+).
- **Zero-Copy Surface Passing**: Surfaces are committed directly to compositor backbuffers via Wayland DRM / DMABUF mechanics.

---

## 4. `flux`: Hardware-Accelerated Vulkan 2D Canvas

Rather than using legacy OpenGL with its global state machine and driver heuristics, `flux` builds directly upon Vulkan:
- **Vulkan Dynamic Rendering**: `flux` leverages modern Vulkan dynamic rendering (`VK_KHR_dynamic_rendering`). This completely bypasses traditional `VkRenderPass` and `VkFramebuffer` allocation boilerplate, enabling direct rendering into swapchain attachments with minimal overhead.
- **Dynamic Texture Management**: Decoded images are uploaded into GPU device-local memory as `flux::Image` textures. Memory layouts are optimized for parallel texture cache reads during sub-pixel scaling and rotation.
- **Dedicated Sampler Descriptors**:
  - `Linear` Sampler: Bilinear filtering configured with clamp-to-edge addressing.
  - `Nearest` Sampler: Point sampling for pixel art and icon analysis.

---

## 5. `lens`: Immediate-Mode UI Simplicity

Traditional retained-mode GUI toolkits maintain complex scene graphs, layout solvers, and dirty-rectangle invalidation trees. For an image viewer, this introduces synchronization bugs where UI state diverges from image state.

`lens` resolves this via the immediate-mode paradigm:
- **Single-Pass Evaluation**: Every frame, the OSD overlay and console bar are evaluated on-demand from current application state.
- **Vector Antialiased Text**: Glyphs are rasterized and blended directly on top of the image canvas with sub-pixel precision.
- **Zero Layout Jitter**: Text elements compute their bounding boxes dynamically, ensuring HUD elements never glitch, flicker, or lag behind image pans and resizes.
