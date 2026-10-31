# Configuration File Reference

This document specifies the format, sections, syntax, and options supported by Unframed's configuration file.

---

## 1. File Location & Precedence

Unframed resolves its configuration file by checking the following paths in order of priority:

1. `$unframed_config` environment variable
2. `$imv_config` environment variable
3. `$XDG_CONFIG_HOME/unframed/config` (or `~/.config/unframed/config`)
4. `$XDG_CONFIG_HOME/imv/config` (or `~/.config/imv/config`)
5. `/etc/imv_config`

If no configuration file is found, built-in defaults are used.

---

## 2. INI Format Structure

The configuration file follows standard INI syntax with three valid sections:
- `[options]`: System settings, appearance, overlay styling, and initial geometry.
- `[aliases]`: Custom command shorthands.
- `[binds]`: Keybindings and multi-key chord mappings.

Lines starting with `#` or `;` are treated as comments and ignored.

---

## 3. `[options]` Reference

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `background` | String | `000000` | Canvas background. Either `checks` for a transparency checkerboard or a 6-digit hex color (`RRGGBB`). |
| `fullscreen` | Boolean | `false` | When `true`, Unframed opens directly in fullscreen mode. |
| `width` | Integer | `1280` | Initial window pixel width. |
| `height` | Integer | `720` | Initial window pixel height. |
| `initial_pan` | Float pair | `50 50` | Initial image centering percentage (`0 0` = top-left, `50 50` = center). |
| `list_files_at_exit`| Boolean | `false` | When `true`, prints remaining unviewed paths to stdout on exit. |
| `loop_input` | Boolean | `true` | When `true`, reaching the end of the playlist wraps to the beginning. |
| `overlay` | Boolean | `false` | Initial visibility of the OSD overlay HUD. |
| `overlay_font` | String | `"Monospace:24"` | Font description for the overlay (family and size). |
| `overlay_text` | String | Template | Text template displayed inside the OSD HUD banner. |
| `overlay_text_color`| Hex | `FFFFFF` | Foreground text color in 6-digit RGB or 8-digit ARGB hex. |
| `overlay_bg_color` | Hex | `C3000000` | Background banner color in 6-digit RGB or 8-digit ARGB hex. |
| `overlay_position_bottom` | Boolean | `false` | When `true`, positions OSD at bottom; when `false`, at top. |
| `recursive_load` | Boolean | `false` | Recursively load recognized images inside directories by default. |
| `scaling_mode` | String | `full` | Default scaling algorithm: `none`, `shrink`, `full`, or `crop`. |
| `slideshow_duration`| Float | `0.0` | Initial slideshow interval in seconds (`0.0` disables timer). |
| `suppress_default_binds` | Boolean | `false` | When `true`, Unframed clears all default keybindings. |
| `title_text` | String | Template | Format string used to update the Wayland window title. |
| `upscaling_method` | String | `linear` | Upscaling filter: `linear` (smooth) or `nearest_neighbour` (pixelated). |

---

## 4. `[aliases]` Reference

The `[aliases]` section maps alias names to target commands. Arguments passed to the alias at runtime are appended to the target command.

```ini
[aliases]
q = quit
n = next
p = prev
fit = scaling full
pixel = upscaling nearest_neighbour
```

---

## 5. `[binds]` Reference

The `[binds]` section maps single characters, bracketed special keys, or chorded sequences to command strings.

```ini
[binds]
# Standard keys
q = quit
x = close

# Arrow keys and brackets
<Left> = prev
<Right> = next
<bracketleft> = prev
<bracketright> = next

# Vim-style navigation chords
gg = goto 1
<Shift+G> = goto -1

# Transformations
<Ctrl+r> = rotate by 90
c = center
r = reset

# Shell command execution
p = exec echo "$imv_current_file"
```

---

## 6. Color Encoding

Colors can be specified as:
- **6-digit hex**: `RRGGBB` (e.g. `000000` for black, `FFFFFF` for white, `FF5500` for orange). Full opacity (alpha = 255) is applied automatically.
- **8-digit hex**: `AARRGGBB` (e.g. `80000000` for 50% translucent black, `C3000000` for 76% opaque black).
- **Special keyword**: `checks` renders an alternating two-tone gray checkerboard pattern representing transparency.
