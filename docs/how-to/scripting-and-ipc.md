# How-to: Tiling Window Managers & IPC Scripting

`ufm` provides a UNIX domain socket server on every instance, allowing external scripts and window managers to automate image viewing.

---

## 1. IPC Socket Endpoints

Each running instance of `ufm` creates sockets at:
- `$XDG_RUNTIME_DIR/ufm-$PID.sock`
- `$XDG_RUNTIME_DIR/imv-$PID.sock` (backward compatibility)
- Or `/tmp/ufm-$PID.sock` if `$XDG_RUNTIME_DIR` is not set.

---

## 2. Using `ufm-msg` and `imv-msg`

The companion CLI tool connects to the running instance and issues commands:

```bash
# Start ufm in the background
ufm ~/Pictures &
PID=$!

# Tell that instance to go to the next image
ufm-msg $PID next

# Zoom in
ufm-msg $PID zoom 2

# Rotate by 90 degrees
ufm-msg $PID rotate by 90

# Open an additional directory
ufm-msg $PID open ~/Downloads
```

---

## 3. Shell Pipeline Workflows

### Selection Pipeline
Use `ufm` to inspect and filter a list of photos, outputting only the remaining files on exit:

```bash
selected_photos=$(ufm -l ~/Photos)
echo "Selected photos:"
echo "$selected_photos"
```

### Stdin Streaming
Pipe images directly from standard utilities into `ufm`:

```bash
# Find all SVGs and browse them
find /usr/share/icons -name "*.svg" | ufm

# Download and view directly without saving to disk
curl -sL https://rust-lang.org/static/images/rust-logo-blk.svg | ufm -
```

---

## 4. Environment Variables Available to `exec`

When using the `exec` command or defining a shortcut, `ufm` injects the following environment variables:

| Variable | Description |
| :--- | :--- |
| `$imv_pid` / `$unframed_pid` | PID of the current instance |
| `$imv_current_file` | File path of the currently displayed image |
| `$imv_current_index` | 1-based index in the playlist |
| `$imv_file_count` | Total number of items in the playlist |
| `$imv_width` | Width of the image in pixels |
| `$imv_height` | Height of the image in pixels |
| `$imv_scale` | Current zoom percentage (e.g. 100) |
| `$imv_scaling_mode` | `full`, `shrink`, `crop`, or `none` |
| `$imv_format` | Format name (e.g. `SVG`, `PPM`, `PNG`) |
