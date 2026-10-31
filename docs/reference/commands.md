# Command Reference

This document provides the reference for all built-in commands supported by the Unframed interactive console (`:`) and the IPC control channel (`unframed-msg` / `imv-msg`).

---

## 1. Command Syntax & Argument Parsing

Commands are parsed according to standard shell word-splitting rules (`src/commands.rs`):
- Arguments separated by whitespace.
- Single quotes (`'...'`) and double quotes (`"..."`) preserve whitespace.
- Backslash (`\`) escapes literal characters.

---

## 2. Built-in Commands

| Command | Arguments | Summary |
| :--- | :--- | :--- |
| `quit` | None | Exit Unframed and terminate the window process. |
| `close` | `[index \| all]` | Close the current image, a specific 1-based index, or all open images. |
| `next` | `[offset]` | Advance forward by `offset` images (default: `1`). |
| `prev` | `[offset]` | Advance backward by `offset` images (default: `1`). |
| `goto` | `<index>` | Jump to 1-based index (`1` = first image, `-1` = last image). |
| `zoom` | `<amount \| actual>` | Multiply zoom factor by `amount`, or reset zoom to 100% (`actual`). |
| `pan` | `<dx> <dy>` | Translate the viewport by `<dx>` horizontal and `<dy>` vertical pixels. |
| `rotate` | `<to \| by> <angle>` | Rotate image: `to <angle>` sets absolute orientation, `by <angle>` adds delta. |
| `flip` | `<horizontal \| vertical>` | Mirror image across the specified axis. |
| `open` | `[-r] <paths...>` | Load additional files or directories into playlist (`-r` for recursive scan). |
| `fullscreen` | None | Toggle between windowed and fullscreen display. |
| `overlay` | None | Toggle visibility of the OSD overlay. |
| `center` | None | Recenter the image within the window boundaries. |
| `reset` | None | Reset viewport scale, rotation, mirroring, and panning to default state. |
| `next_frame` | None | Advance exactly one frame in multi-frame animated images (GIF/WebP). |
| `toggle_playing` | None | Pause or resume multi-frame animation playback. |
| `scaling` | `[none \| shrink \| full \| crop \| next]` | Set scaling mode, or cycle to next mode if omitted or `next`. |
| `upscaling` | `[linear \| nearest_neighbour \| next]` | Set upscaling filter, or cycle to next if omitted or `next`. |
| `slideshow` | `<+val \| -val \| duration>` | Increase (`+N`), decrease (`-N`), or set exact slideshow duration in seconds. |
| `background` | `<checks \| hex>` | Set background to transparency checkerboard (`checks`) or 6-digit hex color. |
| `bind` | `<keys> <command>` | Bind a key sequence or chord dynamically at runtime. |
| `alias` | `<name> <command>` | Register a command alias dynamically at runtime. |
| `exec` | `<shell command>` | Execute an asynchronous shell command with populated `$imv_*` variables. |

---

## 3. Command Details & Examples

### `quit`
Exits the application immediately, closing the Wayland window and cleaning up IPC sockets.
```text
quit
```

### `close [index | all]`
Closes images in the playlist.
```text
close        # Closes current image, advances to next
close 3      # Closes the 3rd image in playlist
close all    # Empties playlist completely
```

### `next` and `prev`
Navigates the playlist relatively.
```text
next         # Next image (+1)
next 5       # Skip forward 5 images
prev         # Previous image (-1)
prev 2       # Skip back 2 images
```

### `goto <index>`
Jumps directly to a specific 1-based playlist index.
```text
goto 1       # Jump to first image
goto 10      # Jump to 10th image
goto -1      # Jump to last image in playlist
```

### `zoom <amount | actual>`
Scales the current view.
```text
zoom 2       # Zoom in by factor of 2
zoom -1      # Zoom out
zoom actual  # Reset zoom factor to 100%
```

### `pan <dx> <dy>`
Offsets the viewing center.
```text
pan 50 -50   # Pan 50 pixels right, 50 pixels up
```

### `rotate <to | by> <angle>`
Rotates the rendered image. Angle is parsed in degrees and normalized to multiples of 90.
```text
rotate by 90    # Rotate 90 degrees clockwise
rotate to 180   # Orient image to exactly 180 degrees
```

### `flip <horizontal | vertical>`
Toggles mirroring along the chosen axis.
```text
flip horizontal
flip vertical
```

### `open [-r] <paths...>`
Appends files to the running session.
```text
open ~/Pictures/photo.jpg
open -r ~/Wallpapers
```

### `scaling [mode]`
Configures the viewport fit algorithm:
- `none`: 1:1 natural pixel scaling.
- `shrink`: Only downscale if the image exceeds window dimensions.
- `full`: Scale image to fit within window bounds (maintaining aspect ratio).
- `crop`: Scale image to cover entire window, cropping overflowing borders.
- `next`: Cycle to the next mode in sequence.
```text
scaling full
scaling next
```

### `upscaling [method]`
Configures the Vulkan texture sampling filter:
- `linear`: Bilinear hardware filtering for smooth photographic scaling.
- `nearest_neighbour`: Point sampling for sharp pixel art.
- `next`: Toggle between linear and nearest neighbour.
```text
upscaling nearest_neighbour
upscaling next
```

### `exec <command>`
Executes `<command>` inside `/bin/sh -c "<command>"` asynchronously in a detached worker thread. Populates environment variables `$imv_current_file`, `$imv_width`, `$imv_height`, `$imv_scale`, etc.
```text
exec wl-copy "$imv_current_file"
exec notify-send "Image" "$imv_current_file (${imv_width}x${imv_height})"
```

---

## 4. Default Aliases

Unframed initializes the following default aliases on startup (`src/commands.rs`):

| Alias | Target Command |
| :--- | :--- |
| `q` | `quit` |
| `n` | `next` |
| `p` | `prev` |
| `g` | `goto` |
| `z` | `zoom` |
| `o` | `open` |
| `f` | `fullscreen` |
| `d` | `overlay` |
| `c` | `center` |
| `r` | `reset` |
| `s` | `scaling` |
| `S` | `upscaling` |
| `ss` | `slideshow` |
| `bg` | `background` |
