# Command-Line Interface (CLI) Reference

This document provides the authoritative reference for the command-line flags, options, and input handling for the `unframed` binary.

---

## 1. Synopsis

```bash
unframed [options] [paths...]
unframed [options] -
find . -name "*.png" | unframed [options]
```

---

## 2. Options Reference

| Option | Argument | Description | Default |
| :--- | :--- | :--- | :--- |
| `-h`, `--help` | None | Print command summary and exit immediately. | — |
| `-v`, `--version` | None | Print version string and exit immediately. | `0.1.0` |
| `-b` | `<background>` | Set canvas background: a 6-digit hex color (`RRGGBB` or `#RRGGBB`) or the literal string `checks` for transparency grid. | `000000` (black) |
| `-B`, `--transparency` | `<mode>` | Set image transparency presentation: `checks-light`, `checks-dark`, `white`, `black`, or `off`. | `checks-light` |
| `-c` | `<command>` | Command to execute immediately after launch. Can be repeated multiple times. | None |
| `-C` | `<command>` | Command to execute whenever the active image changes. Can be repeated multiple times. | None |
| `-d` | None | Enable OSD overlay visibility at launch. | `false` |
| `-f` | None | Launch window directly in fullscreen mode. | `false` |
| `-i` | `<id>` | Configure the Wayland `app_id` exposed via `xdg_toplevel`. | `ai.opencode.unframed` |
| `-l` | None | Print unviewed paths remaining in playlist to `stdout` upon exit. | `false` |
| `-n` | `<path\|index>` | Launch with the specified 1-based playlist index or matching file path selected. | First file |
| `-r` | None | Recursively scan subdirectories for recognized image files. | `false` |
| `-s` | `<mode>` | Initial scaling mode: `none`, `shrink`, `full`, or `crop`. | `full` |
| `-t` | `<seconds>` | Launch in slideshow mode, advancing images after `<seconds>`. | `0.0` (disabled) |
| `-u` | `<method>` | Set upscaling interpolation filter: `linear` or `nearest_neighbour`. | `linear` |
| `-w` | `<title>` | Set initial window title string. | Template string |
| `-x` | None | Disable playlist wrapping (halt at start or end of list). | Looping enabled |
| `-V` | None | Enable verbose logging output to stderr. | `false` |
| `-W` | `<width>` | Initial window pixel width. | `1280` |
| `-H` | `<height>` | Initial window pixel height. | `720` |

---

## 3. Positional Arguments & Directory Traversal

- **File Paths**: Unframed accepts multiple image paths on the command line:
  ```bash
  unframed photo1.jpg photo2.png diagram.svg
  ```
- **Directories**: When directory paths are provided, Unframed scans them for recognized image extensions and magic bytes:
  ```bash
  unframed ~/Pictures
  ```
  Pass `-r` to traverse subdirectories recursively.

---

## 4. Standard Input (Stdin) Modes

Unframed provides two distinct modes for reading from standard input:

### 4.1 Path List Ingestion (Piped File List)
When no positional arguments are given and `stdin` is connected to a pipe or redirection, Unframed reads paths line-by-line:
```bash
find ~/Wallpapers -type f | unframed
```
Empty lines and paths that do not exist on disk are filtered out automatically.

### 4.2 Raw Image Byte Streaming (`-`)
When the literal argument `-` is supplied, Unframed reads the entire stream from `stdin` into memory and decodes it as an image:
```bash
curl -sL https://example.com/logo.svg | unframed -
cat render.ppm | unframed -
```

---

## 5. Exit Status

| Exit Code | Meaning |
| :--- | :--- |
| `0` | Clean, intentional termination (e.g. user pressed `q` or executed `:quit`). |
| `1` | Fatal error, including launch in non-Wayland environment (`WAYLAND_DISPLAY` missing), invalid command arguments, or failure to open window shell. |
