# How to Customize the OSD Overlay and Window Title

This guide details how to customize the On-Screen Display (OSD) overlay and window title bar in Unframed, including text templates, variable interpolation, typography, and color schemes.

---

## 1. Overview of the OSD Overlay

Unframed renders a crisp OSD overlay using the immediate-mode UI system `lens`. The overlay can be toggled on or off at runtime by pressing `d` or executing the `:overlay` command.

The overlay text supports template substitution, allowing you to project runtime image properties, indices, and viewport metrics onto the canvas.

---

## 2. Template Variables

The following variables can be embedded inside `overlay_text` and `title_text`:

| Variable | Description |
| :--- | :--- |
| `${imv_current_index}` | 1-based index of the currently displayed image |
| `${imv_file_count}` | Total number of items in the current playlist |
| `${imv_width}` | Original pixel width of the loaded image |
| `${imv_height}` | Original pixel height of the loaded image |
| `${imv_scale}` | Current zoom percentage (e.g. `100`, `250`) |
| `$imv_current_file` | File path or name of the currently displayed image |
| `$imv_scaling_mode` | Active scaling mode (`none`, `shrink`, `full`, `crop`) |
| `$imv_upscaling_method`| Active upscaling filter (`linear`, `nearest_neighbour`) |
| `$imv_format` | Format detected during decoding (e.g., `PNG`, `SVG`, `PPM`) |

---

## 3. Configuring the Overlay in `config`

Add or edit the `[options]` section in `~/.config/unframed/config`:

```ini
[options]
# Initial visibility of the overlay on startup
overlay = true

# Font family and point size
overlay_font = "Monospace:24"

# Custom template string
overlay_text = [${imv_current_index}/${imv_file_count}] $imv_current_file (${imv_width}x${imv_height}) - ${imv_scale}%

# Text color in 6-digit RRGGBB or 8-digit AARRGGBB hex format
overlay_text_color = FFFFFF

# Background banner color (e.g., 50% translucent black)
overlay_bg_color = 80000000

# Position: false for top edge (default), true for bottom edge
overlay_position_bottom = false
```

---

## 4. Customizing the Window Title Bar

In Wayland compositors that show window title bars or task switchers, Unframed dynamically updates the window title based on `title_text`:

```ini
[options]
title_text = "unframed: $imv_current_file [${imv_width}x${imv_height}]"
```

You can also override the initial title string at launch using the `-w` command-line flag:

```bash
unframed -w "Project Gallery" -r ~/Pictures
```

---

## 5. Style Examples

### Minimal HUD
Displays only the filename and dimensions in the bottom corner:
```ini
[options]
overlay = true
overlay_position_bottom = true
overlay_font = "Sans:18"
overlay_text = "$imv_current_file (${imv_width}x${imv_height})"
overlay_bg_color = 00000000
overlay_text_color = FFFFFF
```

### Full Metadata Banner
Displays complete scaling, index, and format diagnostics:
```ini
[options]
overlay = true
overlay_font = "Monospace:20"
overlay_text = "[#${imv_current_index}/${imv_file_count}] ${imv_width}x${imv_height}px | Scale: ${imv_scale}% | Mode: $imv_scaling_mode ($imv_upscaling_method) | File: $imv_current_file"
overlay_bg_color = D0181818
overlay_text_color = 00FFAA
```

---

## Next Steps

- **[Configuration Reference](../reference/configuration.md)**: Full list of configuration keys.
- **[Command Reference](../reference/commands.md)**: Runtime command syntax.
