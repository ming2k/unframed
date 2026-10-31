# Getting Started with Unframed

This tutorial guides you through building **Unframed**, opening your first images, navigating playlists, manipulating the viewport, and using the interactive command console.

---

## 1. Prerequisites

Before building Unframed, verify that your environment meets these requirements:

1. **Rust Toolchain**: Rust 1.75 or later with Cargo (`rustc --version`).
2. **Wayland Compositor**: Unframed is a pure Wayland application. An active Wayland session is required (`echo $WAYLAND_DISPLAY` should print e.g. `wayland-0`). X11 / XWayland is intentionally unsupported.
3. **Vulkan Drivers**: An installed Vulkan driver ICD (`vulkaninfo` or driver packages such as `mesa-vulkan-drivers`, `vulkan-radeon`, `vulkan-intel`, or NVIDIA Vulkan).
4. **Optics Graphics Stack**: The sibling repository `../optics` containing `iris`, `flux`, and `lens`.

---

## 2. Building Unframed

Clone the repository and compile in release mode:

```bash
cargo build --release
```

Upon successful compilation, the following binaries will be present in `target/release/`:
- `unframed`: The primary image viewer application.
- `unframed-msg`: The native IPC control utility.
- `ufm-msg`: Companion IPC tool for controlling `ufm` from scripts.

---

## 3. Opening Images

### Opening Single and Multiple Files
Launch Unframed with one or more image files:

```bash
./target/release/unframed photo.jpg logo.svg graphic.ppm
```

### Opening Directories Recursively
Use the `-r` flag to traverse directory trees and load all detected images:

```bash
./target/release/unframed -r ~/Pictures
```

### Piping from Stdin
You can pipe a list of file paths directly to Unframed:

```bash
find ~/Wallpapers -type f -name "*.png" | ./target/release/unframed
```

Alternatively, stream raw image bytes directly using the `-` argument:

```bash
curl -sL https://example.com/sample.png | ./target/release/unframed -
```

---

## 4. Basic Navigation

Once Unframed is displaying an image, use the default keybindings to navigate through the loaded playlist:

- **Next Image**: Press `]` or the `Right Arrow` key.
- **Previous Image**: Press `[` or the `Left Arrow` key.
- **First Image**: Press `gg` (the vim-style chord).
- **Last Image**: Press `Shift+G`.
- **Close Current Image**: Press `x` (advances to next image).

---

## 5. Viewport Manipulation

Unframed gives you full hardware-accelerated control over viewing transformations:

- **Zoom In / Out**: Press `+` / `-`, `i` / `o`, or use the mouse scroll wheel. Mouse wheel zooming automatically centers on the cursor location.
- **Actual Size (100%)**: Press `a` to inspect pixels at 1:1 resolution.
- **Reset Viewport**: Press `r` to restore the default scaling and center the image.
- **Pan View**: Press `h`, `j`, `k`, `l` (left, down, up, right) or click and drag with the left mouse button.
- **Rotate**: Press `Ctrl+r` to rotate 90 degrees clockwise.
- **Cycle Scaling Mode**: Press `s` to cycle between:
  - `none`: 1:1 pixel rendering.
  - `shrink`: Scales down large images to fit the window, leaves small images unscaled.
  - `full`: Scales images up or down to fit inside the window boundaries.
  - `crop`: Scales images to fill the entire window, cropping overflowing margins.
- **Upscaling Filter**: Press `Shift+S` to toggle between `linear` (smooth bilinear interpolation) and `nearest_neighbour` (crisp pixelated rendering, ideal for pixel art).

---

## 6. Using the Interactive Command Console

Unframed includes a built-in interactive console:

1. Press `:` at any time while viewing an image. A command prompt will appear at the bottom of the window.
2. Type a command, such as:
   ```text
   scaling crop
   ```
3. Press `Tab` to trigger auto-completion for available commands and aliases.
4. Use the `Up` and `Down` arrow keys to browse through command history.
5. Press `Enter` to execute the command, or `Escape` to close the console without executing.

### Toggling the OSD Overlay
Press `d` or execute `:overlay` in the console to toggle the On-Screen Display. The overlay displays the current playlist index, image dimensions, zoom percentage, and filename.

---

## 7. Exiting

Press `q` or execute `:quit` to close Unframed cleanly.

---

## Where to Go Next

- **[How-To Guides](../how-to/index.md)**: Configure custom keybindings, script Unframed over UNIX domain sockets, and customize overlay themes.
- **[CLI Reference](../reference/cli.md)**: Explore all command-line flags and parameters.
- **[Command Reference](../reference/commands.md)**: Complete list of built-in console commands.
