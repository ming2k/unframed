# Subsystems & Code Topology

This document describes the module breakdown and internal boundaries of `ufm`.

---

## 1. Directory Structure

```text
src/
├── lib.rs              # Library exports and run() CLI driver
├── main.rs             # Default ufm binary entrypoint
├── app.rs              # App state, on_build and on_paint callbacks
├── binds.rs            # Trie prefix tree for keyboard chords and shortcuts
├── commands.rs         # Command word splitting and default aliases
├── config.rs           # INI configuration parser ([options], [aliases], [binds])
├── console.rs          # Interactive console prompt, editing, and tab completion
├── image_loader.rs     # Multi-format decoding and multi-frame animation manager
├── ipc.rs              # UNIX domain socket server and thread-safe wakeups
├── navigator.rs        # Playlist manager and directory traversal
├── ppm.rs              # Dedicated Netpbm (P1-P7) parser
├── svg.rs              # resvg/usvg vector rasterizer
├── viewport.rs         # Viewport scaling modes, rotation, pan, and zoom math
└── bin/
    ├── ufm_msg.rs      # Native ufm-msg IPC tool
    ├── unframed.rs     # unframed alias binary
    └── unframed_msg.rs # unframed-msg alias tool
```

---

## 2. In-Memory Data Flow

```text
[Disk File / Stdin Stream]
           │
           ▼
[image_loader::load_image_from_*] ──> [LoadedImage { frames, duration_ms, format }]
                                                      │
                                                      ▼ (Texture Upload)
[iris::Application::run] ──> [on_paint] ──> [flux::Image (Vulkan VkImage)]
                                                      │
                                                      ▼
                                           [flux_canvas::draw_image_sampled]
```

1. Files or piped data are loaded into memory and converted to standard 32-bit premultiplied RGBA.
2. In `on_paint`, textures are uploaded lazily to Vulkan `flux::Image` when the frame index or current image changes.
3. Affine matrices in `viewport.rs` calculate exact subpixel offsets for `flux::CanvasCommands`.
