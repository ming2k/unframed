# How-to: Customizing Overlay and Status Display

Unframed provides a customizable on-screen display (OSD) overlay rendered directly above the canvas using `lens`.

---

## 1. Overlay Configuration Options

In `~/.config/ufm/config`, customize the `[options]` section:

```ini
[options]
# Whether to start with overlay visible
overlay = true

# Position: true for bottom, false for top (default)
overlay_position_bottom = false

# Colors in 6-digit (RRGGBB) or 8-digit (AARRGGBB) hex
overlay_text_color = FFFFFF
overlay_bg_color = C3000000

# Template string with variables
overlay_text = [${ufm_current_index}/${ufm_file_count}] [${ufm_width}x${ufm_height}] [${ufm_scale}%] $ufm_current_file [$ufm_scaling_mode]
```

---

## 2. Available Template Variables

The following variables are dynamically expanded in `overlay_text` and `title_text`:

| Variable | Description | Example |
| :--- | :--- | :--- |
| `${ufm_current_index}` | 1-based index of the currently displayed image | `3` |
| `${ufm_file_count}` | Total number of items in the current playlist | `42` |
| `${ufm_width}` | Original pixel width of the loaded image | `3840` |
| `${ufm_height}` | Original pixel height of the loaded image | `2160` |
| `${ufm_scale}` | Current zoom percentage | `100` |
| `$ufm_current_file` | File path of the currently displayed image | `/home/user/Pictures/photo.jpg` |
| `$ufm_scaling_mode` | Active scaling mode (`none`, `shrink`, `full`, `crop`) | `full` |
| `$ufm_upscaling_method`| Active upscaling filter (`linear`, `nearest_neighbour`) | `linear` |
| `$ufm_format` | Format detected during decoding (`PNG`, `SVG`, `PPM`, etc.) | `SVG` |
| `$ufm_slideshow_duration` | Slideshow interval in seconds | `5.0` |
| `$ufm_slideshow_elapsed` | Elapsed seconds since last slideshow cycle | `2.3` |
| `$ufm_pid` | Process ID of this instance | `12345` |

Both `${VAR}` and `$VAR` notations are supported. Standard environment variables (e.g. `$USER`, `$HOME`) can also be included.

---

## 3. Toggling Overlay at Runtime

- Press **`d`** to toggle OSD visibility at any time.
- Or open the `:` console and execute:
  ```text
  :overlay
  ```
