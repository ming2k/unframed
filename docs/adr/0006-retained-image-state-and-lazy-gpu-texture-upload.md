---
id: ADR-0006
title: "Retained Image State and Lazy GPU Texture Upload"
status: accepted
date: 2026-10-24
scope: render/pipeline, memory/caching, texture/vulkan
superseded_by: null
negative_knowledge: true
---

# 0006. Retained Image State and Lazy GPU Texture Upload

- Status: Accepted
- Date: 2026-10-24
- Deciders: Unframed Core Working Group, Graphics Architects

---

## Context and Problem Statement

An image viewer handles diverse workloads: panning around a massive 50-megapixel photo, cycling through thousands of images in a playlist, or playing a 60-FPS animated GIF.

If pixel data is re-uploaded from CPU memory to GPU VRAM on every frame callback, PCI-e bandwidth is quickly saturated and frame drops occur during panning and zooming. Conversely, if all loaded images in a 10,000-file playlist are held as allocated Vulkan GPU textures simultaneously, GPU VRAM will be exhausted.

---

## Decision Drivers

- **Zero-Cost Pan & Zoom**: Panning, zooming, and rotating an already loaded image must consume zero GPU upload bandwidth.
- **VRAM Conservation**: Only the currently active image texture should reside in GPU VRAM; previous images in the playlist must release their GPU resources immediately.
- **Fast Animation Playback**: Multi-frame sequences must update GPU textures smoothly without re-allocating descriptor sets on every frame tick.

---

## Architectural Decisions

We implement a **Retained Frame Cache with Lazy GPU Upload** (`src/app.rs`):

1. **Retained CPU Cache (`LoadedImage`)**: Decoded RGBA pixel buffers are retained in CPU memory. Multi-frame animations hold an array of decoded frames with native delay metadata.
2. **Lazy GPU Upload via Dirty Flag (`gpu_needs_upload`)**:
   - Switching images or advancing animation frames asserts `self.gpu_needs_upload = true`.
   - The paint callback (`on_paint`) checks this flag:
     - If `true`: uploads pixel bytes to the Vulkan `flux::Image` texture and resets the flag.
     - If `false`: reuses the existing GPU texture directly with updated transformation matrices.
3. **Automatic Texture Eviction**: When a new image is loaded, any prior GPU `flux::Image` instance is dropped, releasing VRAM allocations immediately.

### Normative Invariants Established

- `[INV-ARCH-04] Retained Frame Cache & Lazy GPU Upload`: Pixel uploading is decoupled from frame drawing; GPU texture synchronization occurs strictly on dirty flags (`src/app.rs`).

---

## Consequences

- **Positive**:
  - High-framerate (144Hz+) mouse panning and zooming with zero CPU-to-GPU data transfer overhead.
  - Minimal VRAM footprint even when opening playlists containing tens of thousands of images.
- **Negative & Trade-offs**:
  - Image switching incurs an upload step on the first frame of display.

---

## Rejected Alternatives & Negative Knowledge

- **Unconditional Per-Frame Texture Re-Upload**: Rejected because copying tens of megabytes every 6.9ms (144Hz) severely degrades battery life and causes rendering stutter.
- **Pre-allocating Texture Pool for All Playlist Images**: Rejected because playlists with thousands of images would trigger out-of-memory crashes on systems with limited GPU VRAM.
