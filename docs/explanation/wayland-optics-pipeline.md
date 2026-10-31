# Explanation: Wayland & Optics Pipeline

This document explains the frame execution model, display pacing, and HiDPI handling in `ufm`.

---

## 1. The Per-Frame Life Cycle

Each frame driven by `iris` executes in two strictly ordered phases:

```text
[Input Events Accumulated from Wayland compositor]
                      │
                      ▼
            Phase 1: on_build()
      - Drain IPC channel commands
      - Process keyboard shortcuts & console input
      - Handle mouse drag (pan) and wheel (zoom)
      - Check file modification timestamps
      - Tick GIF/WebP animation timers
      - Render UI Chrome via lens (Overlay & Console)
      - Decide display cadence:
          * If needs_redraw || is_animating: request_animation_frame()
          * Else: paint_mark_static()
                      │
                      ▼
            Phase 2: on_paint()
      - Apply HiDPI scale factor: canvas.scale(scale, scale)
      - Draw background (Solid or Checks)
      - Transform canvas (translate center, scale, mirror, rotate)
      - Draw image with selected Sampler (Linear or Nearest)
      - Restore canvas
                      │
                      ▼
[GPU Submit -> Present to Wayland Compositor (wp_presentation / xdg_surface)]
```

---

## 2. Low-Power Idle vs Active Animation

Traditional image viewers either poll continuously (burning GPU power) or block on socket events (dropping animation frames).

`ufm` combines `optics`'s two-tier cadence model:
- **Low-Power Idle**: When an image is loaded and static, `ufm` signals `iris::paint_mark_static()`. The backend skips the entire acquire → clear → paint → present cycle, achieving 0% GPU utilization while idle.
- **Active Animation**: When playing an animated GIF, transitioning in a slideshow, or responding to mouse drags, `iris::request_animation_frame()` sustains the active compositor refresh rate (e.g. 144Hz / 240Hz).

---

## 3. HiDPI & Buffer Scale Handling

Wayland compositors negotiate buffer scaling via `wl_surface.preferred_buffer_scale`.

- Window dimensions and mouse inputs are in **logical coordinates**.
- Vulkan swapchain framebuffers operate in **physical device pixels**.

In `on_paint()`, `ufm` wraps drawing in:
```rust
let scale = paint_host.scale();
canvas.save();
canvas.scale(scale, scale);
// Draw in logical coordinates (0.0 to win_w, win_h)
canvas.restore();
```
This guarantees that viewport centering, mouse panning, and checkerboard tiles remain identical in logical visual proportion regardless of whether the output is a 1x 1080p monitor or a 2x 4K HiDPI laptop display.
