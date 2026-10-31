# unframed (`ufm`)

An ultra-fast, modern, Wayland-native image viewer written in Rust and powered by the **Optics** (`iris` + `flux` + `lens`) graphics stack.

---

## Highlights

- **Pure Wayland Architecture**: Tailored exclusively for modern Wayland desktop compositors (Sway, Hyprland, GNOME Wayland, KDE Plasma Wayland, River, etc.). Built from scratch without legacy X11 baggage, featuring zero-overhead native Wayland protocols and Vulkan dynamic rendering via `optics`.
- **Powered by Optics Graphics Stack**:
  - **`iris`**: Direct Wayland platform shell, xdg-shell integration, display-driven event pacing, and window state management.
  - **`flux`**: High-performance Vulkan 2D canvas foundation for hardware-accelerated image drawing, sub-pixel transformations, and linear / nearest-neighbour sampling.
  - **`lens`**: Immediate-mode UI system rendering crisp OSD overlays and the command console.
- **Comprehensive Format Support**:
  - **SVG**: Crisp vector rendering via `resvg` / `usvg` with arbitrary scaling.
  - **Netpbm Family**: Full Netpbm support including P1/P4 (PBM), P2/P5 (PGM), P3/P6 (PPM), and P7 (PAM).
  - **Standard Raster**: PNG, JPEG, WebP, TIFF, BMP, QOI, Farbfeld, TGA, ICO, HDR, EXR.
  - **Animations**: Multi-frame animated GIF and animated WebP playback with frame delays and pause/play controls.
  - **Transparency & Alpha Presentation**: Isolated Photoshop-style white/gray checkerboard, dark checkerboard, solid white, and solid black backings; press `b` to cycle modes with zero playback flicker.
- **Flexible Viewport & Navigation**:
  - Scaling modes: `full` (fit to window), `shrink` (scale down only), `crop` (fill window), `none` (1:1 actual pixels).
  - Upscaling filters: `linear` (smooth bilinear) or `nearest_neighbour` (crisp pixel art).
  - Mouse-centered zooming and left-click pan dragging.
  - Clockwise rotation (`rotate by 90`) and horizontal / vertical mirroring (`flip`).
- **Interactive Console & Scriptable IPC**:
  - Interactive `:` command console with history navigation, line editing, and tab completion.
  - Full IPC over UNIX domain sockets (`$XDG_RUNTIME_DIR/ufm-$PID.sock`).
  - Companion CLI tool: `ufm-msg`.
- **Keyboard Sequences & Configuration**:
  - Trie-based keybinding engine supporting multi-key chords (`gg`, `<Shift+G>`, `<Ctrl+r>`, `<bracketleft>`, etc.).
  - Flexible INI configuration (`[options]`, `[aliases]`, `[binds]`).

---

## Installation & Building

### Prerequisites

Ensure you have:
- A working Rust toolchain (1.75+)
- Vulkan graphics drivers (RADV, ANV, or NVIDIA Vulkan ICD)
- Wayland compositor (`WAYLAND_DISPLAY` set)
- Optics libraries compiled in `../optics` (or installed system-wide)

### Build with Cargo

```bash
cargo build --release
```

Binaries will be placed in `target/release/`:
- **`ufm`**: Primary image viewer binary (aliased as `unframed`)
- **`ufm-msg`**: IPC client utility (aliased as `unframed-msg`)

---

## Usage Examples

```bash
# Open one or more images
ufm photo.jpg logo.svg graphic.ppm

# Open a directory recursively
ufm -r ~/Pictures

# Open via pipe / stdin
find . -type f -name "*.svg" | ufm

# Stream an image directly from curl
curl -sL https://example.com/sample.png | ufm -

# Open fullscreen with chequered background
ufm -f -b checks sample.png

# Slideshow mode: cycle images every 5 seconds
ufm -t 5 ~/Wallpapers
```

---

## Keybindings (Default)

| Key | Action |
| :--- | :--- |
| `q` | Quit |
| `x` | Close current image |
| `Right` / `]` | Next image |
| `Left` / `[` | Previous image |
| `gg` | Jump to first image |
| `Shift+G` | Jump to last image |
| `j` / `k` / `h` / `l` | Pan view down / up / left / right |
| `Up` / `+` / `i` / Wheel Up | Zoom in (centers on mouse cursor if using mouse) |
| `Down` / `-` / `o` / Wheel Down | Zoom out |
| `a` | Actual size (100% zoom) |
| `r` | Reset viewport and scaling |
| `c` | Center image in window |
| `Ctrl+r` | Rotate clockwise by 90° |
| `s` | Cycle scaling mode (`none` → `shrink` → `full` → `crop`) |
| `Shift+S` | Toggle upscaling (`linear` ↔ `nearest_neighbour`) |
| `f` | Toggle fullscreen |
| `d` | Toggle OSD overlay display |
| `.` / `period` | Step to next animation frame |
| `Space` | Toggle animation playback (pause / play) |
| `t` | Increase slideshow timer by 1s (starts if inactive) |
| `Shift+T` | Decrease slideshow timer by 1s (stops if ≤ 0) |
| `:` | Open command console prompt |

---

## Commands

Press `:` at any time to open the interactive command console. Supported commands include:

- `quit`: Exit `ufm`.
- `next [offset]`: Go forward by `offset` images (default: 1).
- `prev [offset]`: Go backward by `offset` images (default: 1).
- `goto <index>`: Jump to 1-based index (e.g. `1` for first, `-1` for last).
- `zoom <amount|actual>`: Zoom by amount or reset to 100%.
- `pan <dx> <dy>`: Move viewport by `dx` and `dy`.
- `rotate <to|by> <angle>`: Rotate image clockwise.
- `flip <horizontal|vertical>`: Mirror image.
- `open [-r] <paths...>`: Append paths or directory images to navigator.
- `close [index|all]`: Close current image, specific index, or all.
- `fullscreen`: Toggle fullscreen.
- `overlay`: Toggle OSD overlay visibility.
- `scaling <none|shrink|full|crop|next>`: Configure scaling mode.
- `upscaling <linear|nearest_neighbour|next>`: Configure upscaling filter.
- `slideshow <+amount|-amount|duration>`: Adjust or set slideshow duration.
- `background <checks|hex>`: Change background appearance.
- `center`: Re-center image.
- `reset`: Reset viewport transforms.
- `exec <shell command>`: Execute shell command with `$ufm_*` environment variables.
- `bind <keys> <commands>`: Add or update a keybinding dynamically.
- `alias <name> <command>`: Add a command alias.

---

## IPC Scripting

`ufm` listens on `$XDG_RUNTIME_DIR/ufm-$PID.sock`.

You can control a running instance with `ufm-msg`:

```bash
# Next image
ufm-msg $PID next

# Zoom in
ufm-msg $PID zoom 2

# Rotate 90 degrees
ufm-msg $PID rotate by 90

# Open new files
ufm-msg $PID open ~/Pictures/new.svg
```

---

## Detailed Documentation Suite

Refer to the in-depth documentation following the Diátaxis model:
- [Getting Started Tutorial](docs/tutorials/getting-started.md)
- [How-to: Custom Keybindings & Chords](docs/how-to/keybindings.md)
- [How-to: Tiling WM & IPC Scripting](docs/how-to/scripting-and-ipc.md)
- [Reference: CLI Parameters](docs/reference/cli.md)
- [Reference: Interactive Commands](docs/reference/commands.md)
- [Reference: Configuration File](docs/reference/configuration.md)
- [Reference: Supported Formats](docs/reference/supported-formats.md)
- [Explanation: Architecture Overview](docs/explanation/architecture-overview.md)
- [Explanation: Wayland & Optics Pipeline](docs/explanation/wayland-optics-pipeline.md)
- [Architecture Decision Records (ADRs)](docs/adr/0001-wayland-only-architecture-with-optics.md)
- [Development & Subsystems Topology](docs/dev/subsystems.md)
- [Development & Testing Strategy](docs/dev/testing.md)
- [Manual Testing & Verification Runbook](docs/dev/manual-testing.md)

---

## License

MIT License.
