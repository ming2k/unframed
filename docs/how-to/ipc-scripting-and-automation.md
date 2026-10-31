# How-to: IPC Scripting & Automation

Unframed (`ufm`) exposes a UNIX domain socket server allowing external shell scripts, tiling window managers, and status bars to control running instances.

---

## 1. IPC Socket Discovery

When an instance starts, it listens on:
- `$XDG_RUNTIME_DIR/ufm-$PID.sock`
- Or `/tmp/ufm-$PID.sock` if `$XDG_RUNTIME_DIR` is unset.

---

## 2. Using `ufm-msg`

The companion CLI binary `ufm-msg` connects to a target instance by process ID:

```bash
ufm-msg <PID> <command> [arguments...]
```

### Examples

```bash
# Advance to next image
ufm-msg $PID next

# Zoom in
ufm-msg $PID zoom 1

# Pan down by 100 pixels
ufm-msg $PID pan 0 100

# Rotate image clockwise
ufm-msg $PID rotate by 90

# Open new files or directories
ufm-msg $PID open ~/Pictures/Wallpapers

# Close the currently focused image
ufm-msg $PID close
```

---

## 3. Shell Script Integration

### Tiling Window Manager Action Scripts
Bind keys in Sway (`~/.config/sway/config`) or Hyprland (`~/.config/hypr/hyprland.conf`) to control the focused viewer instance:

```bash
# Extract PID of the focused ufm window and advance image
PID=$(pgrep -n ufm)
ufm-msg $PID next
```

### Wallpaper Setter via `exec`
Bind a key inside `~/.config/ufm/config` to set the current image as your desktop wallpaper:

```ini
[binds]
W = exec swww img "$ufm_current_file"
y = exec wl-copy "$ufm_current_file"
```

---

## 4. Environment Variables Available to `exec`

Commands executed via `exec` have access to the complete set of viewer state variables:

| Variable | Description | Example |
| :--- | :--- | :--- |
| `$ufm_current_file` | Absolute or relative path to the active file | `/home/user/Pictures/photo.jpg` |
| `$ufm_current_index` | 1-based index of current image in playlist | `3` |
| `$ufm_file_count` | Total number of items in playlist | `42` |
| `$ufm_width` | Natural pixel width of the decoded image | `3840` |
| `$ufm_height` | Natural pixel height of the decoded image | `2160` |
| `$ufm_scale` | Viewport scaling percentage | `100` |
| `$ufm_format` | Format name detected during decoding | `PNG` |
| `$ufm_scaling_mode` | Active scaling mode (`none`, `shrink`, `full`, `crop`) | `full` |
| `$ufm_upscaling_method` | Active upscaler (`linear`, `nearest_neighbour`) | `linear` |
| `$ufm_pid` | Process ID of the viewer instance | `12345` |
