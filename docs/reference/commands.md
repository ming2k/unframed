# Interactive Commands Reference

This document provides the reference for all built-in commands supported by the Unframed interactive console (`:`) and the IPC control channel (`ufm-msg`).

Commands can be typed interactively after pressing `:`, triggered by keybindings in `~/.config/ufm/config`, executed via CLI flags (`-c`), or sent over UNIX domain sockets.

---

## 1. Command Syntax & Argument Parsing

- Command arguments are separated by whitespace.
- Quotes (single `'...'` and double `"..."`) preserve whitespace within arguments.
- Backslash `\` escapes literal quotes and delimiters.
- Multiple commands can be chained using semicolon `;` delimiters (e.g. `next; zoom actual`).

---

## 2. Command Index

| Command | Arguments | Description |
| :--- | :--- | :--- |
| `quit` | | Exit `ufm`. Aliased to `q`. |
| `close` | `[index\|all]` | Close current image, image at 1-based index, or all images. Aliased to `x`. |
| `next` | `[offset]` | Advance forward in playlist by offset items (default: 1). Aliased to `n`. |
| `prev` | `[offset]` | Step backward in playlist by offset items (default: 1). Aliased to `p`. |
| `goto` | `<index>` | Jump to 1-based index (1 = first, -1 = last). Aliased to `g`. |
| `open` | `[-r] <paths...>`| Append image paths or directories to playlist. Aliased to `o`. |
| `zoom` | `<amount\|actual>`| Zoom by logarithmic step (`pow(1.04, amount)`) or reset to 100% with `actual`. Aliased to `z`. |
| `pan` | `<dx> <dy>` | Translate image viewport by `dx` and `dy` pixels. |
| `center`| | Re-center image within current viewport geometry. Aliased to `c`. |
| `reset` | | Reset pan offset, rotation, and zoom back to default mode. Aliased to `r`. |
| `rotate`| `<to\|by> <deg>`| Rotate clockwise by degrees (`by`) or set absolute rotation (`to`). |
| `flip` | `<horizontal\|vertical>` | Mirror image across vertical or horizontal axis. |
| `scaling`| `<none\|shrink\|full\|crop\|next>` | Set viewport scaling mode or cycle to next. Aliased to `s`. |
| `upscaling`| `<linear\|nearest_neighbour\|next>` | Set sampling filter or toggle between smooth and pixelated. Aliased to `S`. |
| `fullscreen`| | Toggle Wayland window fullscreen state. Aliased to `f`. |
| `overlay`| | Toggle visibility of OSD metrics banner. Aliased to `d`. |
| `slideshow`| `<+amount\|-amount\|duration>` | Adjust slideshow timer or set interval directly. Aliased to `ss`. |
| `background`| `<checks\|hex>` | Set background to transparency checkerboard or hex color code. Aliased to `bg`. |
| `transparency`| `<checks-light\|checks-dark\|white\|black\|off\|next>` | Set image transparency backing grid or cycle presentation mode (`b` key). Aliased to `trans`. |
| `next_frame`| | Advance one frame in animated GIF or WebP. |
| `toggle_playing`| | Pause or resume animated playback. |
| `exec` | `<shell command>` | Execute an asynchronous shell command with populated `$ufm_*` variables. |
| `bind` | `<keys> <command>`| Dynamically register or override a key sequence mapping. |
| `alias` | `<alias> <command>`| Register a command shortcut alias. |

---

## 3. Shell Execution (`exec`)

Executes `<command>` inside `/bin/sh -c "<command>"` asynchronously in a detached worker thread. Populates environment variables `$ufm_current_file`, `$ufm_width`, `$ufm_height`, `$ufm_scale`, etc.

### Examples
```text
exec wl-copy "$ufm_current_file"
exec notify-send "Image" "$ufm_current_file (${ufm_width}x${ufm_height})"
```
