# How to Use IPC Scripting and Automation

This guide demonstrates how to control running Unframed instances externally using UNIX domain socket IPC, script automated workflows, and integrate Unframed with Wayland window managers such as Sway and Hyprland.

---

## 1. IPC Architecture Overview

Every running Unframed process creates a UNIX domain socket listener under `$XDG_RUNTIME_DIR` (or `/tmp` if the runtime directory is unset). The socket names follow the patterns:
- `$XDG_RUNTIME_DIR/unframed-$PID.sock`
- `$XDG_RUNTIME_DIR/imv-$PID.sock`
- `$XDG_RUNTIME_DIR/ufm-$PID.sock`

Because Unframed maintains dual socket listeners, all existing scripts and tools designed for `imv` work seamlessly with Unframed.

---

## 2. Using the IPC Clients

Unframed provides two identical IPC client binaries:
- `unframed-msg`: The native utility.
- `imv-msg`: Backwards-compatible binary.

### Syntax
```bash
unframed-msg <PID> <command> [arguments...]
imv-msg <PID> <command> [arguments...]
```

### Examples
```bash
PID=$(pgrep -n unframed)

# Advance to the next image
unframed-msg $PID next

# Zoom in
unframed-msg $PID zoom 2

# Rotate 90 degrees clockwise
unframed-msg $PID rotate by 90

# Toggle fullscreen mode
unframed-msg $PID fullscreen

# Dynamically load a new image into the active playlist
unframed-msg $PID open ~/Pictures/screenshot.png

# Jump directly to the first image
unframed-msg $PID goto 1
```

---

## 3. Wayland Window Manager Integration

You can bind keys in your compositor to control an active Unframed window globally.

### Sway Configuration (`~/.config/sway/config`)
```text
# Control unframed playback globally
bindsym Mod4+bracketright exec unframed-msg $(pgrep -n unframed) next
bindsym Mod4+bracketleft  exec unframed-msg $(pgrep -n unframed) prev
bindsym Mod4+r          exec unframed-msg $(pgrep -n unframed) rotate by 90
```

### Hyprland Configuration (`~/.config/hypr/hyprland.conf`)
```text
# Global bindings to navigate active viewer
bind = SUPER, bracketright, exec, unframed-msg $(pgrep -n unframed) next
bind = SUPER, bracketleft,  exec, unframed-msg $(pgrep -n unframed) prev
bind = SUPER, R,           exec, unframed-msg $(pgrep -n unframed) rotate by 90
```

---

## 4. Shell Execution with Environment Variables (`exec`)

When Unframed executes a shell command via `exec` (triggered from keybindings, the console, or IPC), it populates a rich set of environment variables describing the current state:

| Variable | Description | Example |
| :--- | :--- | :--- |
| `$imv_current_file` | Absolute or relative path to the active file | `/home/user/Pictures/photo.jpg` |
| `$imv_current_index` | 1-based index of current image in playlist | `3` |
| `$imv_file_count` | Total number of items in playlist | `42` |
| `$imv_width` | Natural pixel width of the decoded image | `3840` |
| `$imv_height` | Natural pixel height of the decoded image | `2160` |
| `$imv_scale` | Viewport scaling percentage | `100` |
| `$imv_format` | Format name detected during decoding | `PNG` |
| `$imv_scaling_mode` | Active scaling mode (`none`, `shrink`, `full`, `crop`) | `full` |
| `$imv_upscaling_method` | Active upscaler (`linear`, `nearest_neighbour`) | `linear` |
| `$imv_pid` | Process ID of the viewer instance | `12345` |
| `$unframed_pid` | Process ID of the viewer instance | `12345` |

### Script Example: Copy Current Image Path to Clipboard
In `~/.config/unframed/config`:
```ini
[binds]
y = exec wl-copy "$imv_current_file"
```

### Script Example: Set as Wallpaper
```ini
[binds]
W = exec swww img "$imv_current_file"
```

---

## Next Steps

- **[IPC Protocol Reference](../reference/ipc-protocol.md)**: Technical wire specification for socket communication.
- **[Command Reference](../reference/commands.md)**: Complete inventory of supported commands.
