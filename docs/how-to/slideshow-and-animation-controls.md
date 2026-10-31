# How to Control Slideshows and Animations

This guide describes how to configure automated image slideshows, adjust presentation intervals at runtime, and control multi-frame animation playback for GIF and animated WebP files in Unframed.

---

## 1. Automated Slideshow Mode

Unframed features a built-in timer engine that automatically advances to the next image after a configurable duration.

### Launching with a Slideshow Interval
Pass the `-t` CLI option with the interval in seconds:

```bash
# Cycle through ~/Wallpapers every 5 seconds
unframed -t 5 ~/Wallpapers

# Load recursively with a 10-second slideshow timer
unframed -t 10 -r ~/Pictures
```

### Controlling the Slideshow at Runtime
You can adjust or cancel the slideshow timer while viewing:

- **Increase Interval by 1s**: Press `t` (starts slideshow at 1s if previously inactive).
- **Decrease Interval by 1s**: Press `Shift+T` (stops slideshow if the duration reaches 0).
- **Interactive Console / IPC**:
  ```text
  :slideshow +2    # Add 2 seconds to interval
  :slideshow -1    # Subtract 1 second from interval
  :slideshow 8     # Set interval directly to 8 seconds
  :slideshow 0     # Stop slideshow timer
  ```

---

## 2. Animation Playback (GIF & Animated WebP)

When an animated image format (such as multi-frame GIF or WebP) is loaded, Unframed decodes each frame along with its native delay metadata.

### Play and Pause
- Press `Space` or run `:toggle_playing` to pause and resume animation playback.
- While playing, the render loop respects each frame's native millisecond delay specification.

### Manual Frame Stepping
- When an animation is paused, press `.` (period) or execute `:next_frame` to advance exactly one frame forward.
- This allows frame-by-frame inspection of high-speed animations or sprite sheets.

### Transparency Inspection (GIF & Alpha WebP)
- Press `b` or execute `:transparency next` to cycle through image-bound backing modes:
  - **Checks (Light)**: Photoshop-style pure white + neutral light gray (`#FFFFFF` / `#CCCCCC`).
  - **Checks (Dark)**: Deep charcoal + dark steel (`#202020` / `#383838`).
  - **Solid White**: Pure white (`#FFFFFF`) for inspecting dark lines and light-theme web rendering.
  - **Solid Black**: Pure black (`#000000`) for inspecting bright graphics and dark-theme contrast.
  - **Off**: Directly transparent to window canvas background.
- Backing grids remain firmly locked to image bounds during panning, rotation, and zooming with zero animation flicker.

---

## 3. Looping and Playlist Termination

By default, reaching the end of the playlist wraps around to the first image. You can modify this behavior:

### Disabling Circular Looping
- Via CLI flag: Pass `-x` to prevent wrapping. When the end of the playlist is reached, navigation halts:
  ```bash
  unframed -x ~/Pictures/*
  ```
- Via configuration file:
  ```ini
  [options]
  loop_input = false
  ```

### Outputting Remaining Files on Exit
Pass the `-l` option to have Unframed print all unviewed paths to standard output upon exit:

```bash
unframed -l -x *.jpg > unviewed_files.txt
```

---

## Next Steps

- **[CLI Reference](../reference/cli.md)**: Complete list of launch options.
- **[Format Support Reference](../reference/format-support.md)**: Technical details on animated GIF and WebP decoding.
