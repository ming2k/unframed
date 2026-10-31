# Configuration File Reference

`ufm` reads ini-style configuration from the first location found:
1. `$ufm_config` or `$imv_config`
2. `$XDG_CONFIG_HOME/unframed/config` or `$XDG_CONFIG_HOME/imv/config`
3. `~/.config/unframed/config` or `~/.config/imv/config`
4. `/etc/imv_config`

---

## `[options]` Section

| Option | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `background` | string | `000000` | Hex RGB color or `checks`. |
| `fullscreen` | bool | `false` | Start in fullscreen. |
| `width` | int | `1280` | Initial window width. |
| `height` | int | `720` | Initial window height. |
| `initial_pan` | `float float` | `50 50` | Default pan center percentages (0-100). |
| `list_files_at_exit`| bool | `false` | Output remaining files on exit. |
| `loop_input` | bool | `true` | Loop back when reaching end of playlist. |
| `overlay` | bool | `false` | Start with OSD overlay enabled. |
| `overlay_text` | string | `[...]` | Format template with environment variables. |
| `overlay_text_color`| hex | `ffffff` | RGB color of overlay text. |
| `overlay_background_color`| hex | `000000` | RGB color of overlay box. |
| `overlay_position_bottom`| bool | `false` | Place overlay at bottom instead of top. |
| `recursively` | bool | `false` | Scan folders recursively by default. |
| `scaling_mode` | string | `full` | `none`, `shrink`, `full`, or `crop`. |
| `slideshow_duration`| float | `0.0` | Default slideshow seconds (0 = off). |
| `suppress_default_binds`| bool | `false` | If true, built-in bindings are cleared. |
| `title_text` | string | `[...]` | Window title template. |
| `upscaling_method` | string | `linear` | `linear` or `nearest_neighbour`. |

---

## `[aliases]` Section
```ini
[aliases]
x = close
n = next
p = prev
```

---

## `[binds]` Section
```ini
[binds]
q = quit
<Left> = prev
<Right> = next
gg = goto 1
<Shift+G> = goto -1
```
