# unframed

An ultra-fast, modern, Wayland-native image viewer written in Rust, powered by the **Optics** (`iris` + `flux` + `lens`) graphics stack and designed as an evolution of **`imv`**.

---

## Highlights

- **Pure Wayland Architecture**: Tailored exclusively for modern Wayland desktop compositors (Sway, Hyprland, GNOME Wayland, KDE Plasma Wayland, River, etc.). X11 is completely dropped in favor of zero-overhead native Wayland protocols and Vulkan dynamic rendering via `optics`.
- **Powered by Optics Graphics Stack**:
  - **`iris`**: Direct Wayland platform shell, xdg-shell integration, display-driven event pacing, and window state management.
  - **`flux`**: High-performance Vulkan 2D canvas foundation for hardware-accelerated image drawing, sub-pixel transformations, and linear / nearest-neighbour sampling.
  - **`lens`**: Immediate-mode UI system rendering crisp OSD overlays and the command console.
- **Expanded Format Support**:
  - **SVG**: Crisp vector rendering via `resvg` / `usvg` with arbitrary scaling.
  - **PPM / PNM**: Full Netpbm family support including P1/P4 (PBM), P2/P5 (PGM), P3/P6 (PPM), and P7 (PAM).
  - **Standard Raster**: PNG, JPEG, WebP, TIFF, BMP, QOI, Farbfeld, TGA, ICO, HDR, EXR.
  - **Animations**: Multi-frame animated GIF and animated WebP playback with frame delays and pause/play controls.
- **Flexible Viewport & Navigation**:
  - Scaling modes: `full` (fit to window), `shrink` (scale down only), `crop` (fill window), `none` (1:1 actual pixels).
  - Upscaling filters: `linear` (smooth bilinear) or `nearest_neighbour` (crisp pixel art).
  - Mouse-centered zooming and left-click pan dragging.
  - Clockwise rotation (`rotate by 90`) and horizontal / vertical mirroring (`flip`).
- **Interactive Console & Scriptable IPC**:
  - Interactive `:` command console with history navigation, line editing, and tab completion.
  - Full IPC over UNIX domain sockets compatible with `imv-msg` (`$XDG_RUNTIME_DIR/imv-$PID.sock` and `unframed-$PID.sock`).
  - Companion CLI tools: `unframed-msg` and `imv-msg`.
- **Keyboard Sequences & Configuration**:
  - Trie-based keybinding engine supporting multi-key chords (`gg`, `<Shift+G>`, `<Ctrl+r>`, `<bracketleft>`, etc.).
  - Full ini configuration compatibility with `imv` (`[options]`, `[aliases]`, `[binds]`).

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
- `unframed`: Main image viewer
- `unframed-msg`: IPC client utility
- `imv-msg`: Backward-compatible IPC client utility

---

## Usage Examples

```bash
# Open one or more images
unframed photo.jpg logo.svg graphic.ppm

# Open a directory recursively
unframed -r ~/Pictures

# Open via pipe / stdin
find . -type f -name "*.svg" | unframed

# Stream an image directly from curl
curl -sL https://example.com/sample.png | unframed -

# Open fullscreen with chequered background
unframed -f -b checks sample.png

# Slideshow mode: cycle images every 5 seconds
unframed -t 5 ~/Wallpapers
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

- `quit`: Exit `unframed`.
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
- `exec <shell command>`: Execute shell command with `$imv_*` and `$unframed_*` environment variables.
- `bind <keys> <commands>`: Add or update a keybinding dynamically.
- `alias <name> <command>`: Add a command alias.

---

## IPC Scripting

`unframed` listens on `$XDG_RUNTIME_DIR/unframed-$PID.sock` and `$XDG_RUNTIME_DIR/imv-$PID.sock`.

You can control a running instance with `unframed-msg` or `imv-msg`:

```bash
# Next image
unframed-msg $PID next

# Zoom in
unframed-msg $PID zoom 2

# Rotate 90 degrees
unframed-msg $PID rotate by 90

# Open new files
unframed-msg $PID open ~/Pictures/new.svg
```

---

## Configuration

`unframed` searches for configuration in:
1. `$unframed_config` or `$imv_config`
2. `$XDG_CONFIG_HOME/unframed/config` or `$XDG_CONFIG_HOME/imv/config`
3. `~/.config/unframed/config` or `~/.config/imv/config`
4. `/etc/imv_config`

See [`files/config`](files/config) for the reference configuration file.

---

## License

MIT License.
