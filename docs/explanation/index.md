# Explanation

Discursive documentation that explains how and why Unframed is designed the way it is. Read these articles to understand architectural rationale, coordinate spaces, and engineering trade-offs, rather than to accomplish a specific task.

---

## Architectural Articles

| Article | Explains |
| :--- | :--- |
| [Architecture Overview](architecture-overview.md) | High-level system structure, the Optics platform stack, and the separation of image decoding from GPU presentation. |
| [Viewport Transforms & Coordinate Spaces](viewport-transforms-and-scaling.md) | The mathematical pipeline behind scaling modes, rotation, flipping, and mouse-centered zoom pinning. |
| [Keybinding Engine](keybinding-engine.md) | The trie-based prefix parser, vim-style chord resolution, and the modal switch to console input. |
| [The Optics Graphics Stack](optics-graphics-stack.md) | Why pure Wayland via `iris`, Vulkan dynamic rendering via `flux`, and immediate UI via `lens` eliminate X11 legacy baggage. |
| [Wayland & Optics Pipeline](wayland-optics-pipeline.md) | Frame lifecycle (`on_build` / `on_paint`), display pacing, low-power idle gating, and HiDPI buffer scaling. |

For the formal records of architectural decisions, consult the [Architectural Decision Records](../adr/index.md).
