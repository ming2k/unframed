# ufm Hands-On Manual Testing & Verification Runbook

This runbook provides **practical, step-by-step hands-on testing procedures** for **Unframed (`ufm`)**. It avoids abstract testing checklists and instead provides concrete, executable shell commands, exact keypress interactions, and observable visual expectations for developers, QA engineers, and everyday users.

---

## 0. Preparation (Ready in 30 Seconds)

Before starting hands-on testing, build the release binary and generate sample test images:

```bash
# 1. Build release binaries
cargo build --release

# 2. Generate test image fixtures (SVG, Netpbm P1~P7, Animated GIF, Transparent PNG)
./tools/generate_test_fixtures.sh /tmp/ufm-fixtures

# 3. Add binaries to current PATH
export PATH="$PWD/target/release:$PATH"
```

---

## Practice 1: Launch, OSD Status Bar & Vector SVG Zooming

### Action Steps
1. In your terminal, run:
   ```bash
   ufm -d /tmp/ufm-fixtures/vector_sample.svg
   ```
2. **Visual Observation**:
   - A window appears with a black background displaying an SVG vector graphic with an orange-to-purple gradient circle, a cyan rounded rectangle, and crisp white text.
   - A semi-transparent black OSD status banner appears at the top:
     `[1/1] [800x600] [100%] ...vector_sample.svg [full]`.
3. **Interactive Testing**:
   - Hover your mouse cursor over the text or circle edge and **scroll mouse wheel up**:
     - *Observation*: The image zooms in rapidly past 500%. Inspect the edges—**there is zero pixelation or blurriness; vector paths remain perfectly crisp and sharp**! Notice that zoom anchors precisely on your mouse pointer.
   - Press **`a`** (Actual size):
     - *Observation*: Resets instantly to 100% 1:1 original physical dimensions.
   - Press **`d`** (Display overlay):
     - *Observation*: The top OSD status bar hides; press **`d`** again to toggle it back on.
   - Press **`q`**: The window closes immediately and returns to your shell prompt.

---

## Practice 2: Multi-Image Browsing, Netpbm (P1~P7) & Vim Sequences

### Action Steps
1. Launch with all Netpbm format samples:
   ```bash
   ufm /tmp/ufm-fixtures/sample_p*
   ```
2. **Interactive Testing**:
   - Press **`Right`** or **`]`**:
     - *Observation*: Smoothly navigates to the next image (cycles through P1 1-bit bitmap, P2 grayscale ramp, P3 RGB colors, P4 binary bitmap, P5 binary grayscale, P6 binary RGB, and P7 PAM with alpha transparency).
   - Press **`Left`** or **`[`**:
     - *Observation*: Steps backward through the playlist. Wrapping past the first image cycles cleanly to the last item.
   - Press **`gg`** (Double-tap 'g' in rapid succession):
     - *Observation*: Jumps instantly to the very first image in the playlist!
   - Press **`Shift + G`**:
     - *Observation*: Jumps instantly to the last image in the playlist!
   - Press **`x`**:
     - *Observation*: Closes and removes the active image from the playlist, automatically advancing to the next remaining file.
   - Press **`q`** to exit.

---

## Practice 3: Animation Timeline, Pausing & Single-Frame Stepping

### Action Steps
1. Launch with the multi-frame animated GIF:
   ```bash
   ufm /tmp/ufm-fixtures/sample_animated.gif
   ```
2. **Visual Observation**:
   - The graphic alternates continuously between red and green frames at the configured 50ms timeline interval.
3. **Interactive Testing**:
   - Press **`Space`**:
     - *Observation*: The animation freezes instantly! The backend enters low-power idle mode immediately.
   - Press **`.`** (Period):
     - *Observation*: Advances forward by exactly one single frame while paused. Press **`.`** repeatedly to step frame-by-frame.
   - Press **`Space`** again:
     - *Observation*: Resumes continuous animated playback.
   - Press **`q`** to exit.

---

## Practice 4: Mouse Pan Dragging, Rotation & Checkerboard Background

### Action Steps
1. Launch with a transparent PNG on a checkerboard background:
   ```bash
   ufm -b checks /tmp/ufm-fixtures/sample_transparent.png
   ```
2. **Visual Observation**:
   - Transparent regions show a clean alternating two-tone gray checkerboard pattern.
3. **Interactive Testing**:
   - Press **`+`** repeatedly to zoom in significantly.
   - **Click and hold the left mouse button, then drag across the window**:
     - *Observation*: The image tracks 1:1 with cursor displacement. Boundary damping prevents dragging the image out of sight.
   - Press **`Ctrl + r`**:
     - *Observation*: Rotates clockwise by 90 degrees. Pressing 4 times completes a 360-degree rotation.
   - Press **`r`** (Reset):
     - *Observation*: Instantly resets rotation to 0, centers the viewport, and rescales to window bounds.
   - Press **`s`**:
     - *Observation*: Cycles scaling modes: `none` (100%) -> `shrink` (scale down only) -> `full` (aspect fit) -> `crop` (fill window).
   - Press **`Shift + S`**:
     - *Observation*: Toggles sampling filter between `linear` (smooth bilinear interpolation) and `nearest_neighbour` (crisp pixel art).
   - Press **`q`** to exit.

---

## Practice 5: Interactive Command Console (`:`) & Tab Completion

### Action Steps
1. Launch an image:
   ```bash
   ufm /tmp/ufm-fixtures/vector_sample.svg
   ```
2. **Interactive Testing**:
   - Press **`:`** (On US keyboards, press `Shift + ;`):
     - *Observation*: A sleek dark command prompt bar opens at the bottom displaying `: `.
   - Type `scal`, then press **`Tab`**:
     - *Observation*: The input automatically autocompletes to `scaling`!
   - Complete the command with ` crop` and press **`Enter`**:
     - *Observation*: The console closes, and the image instantly expands to fill the entire window in crop mode!
   - Press **`:`** again and press **`Up` arrow**:
     - *Observation*: Recalls the previous command `scaling crop` from command history.
   - Press **`Escape`**:
     - *Observation*: The console dismisses cleanly without executing.
   - Press **`q`** to exit.

---

## Practice 6: Multi-Terminal IPC Remote Control (`ufm-msg`)

This exercises the scriptable automation pipeline used by tiling window managers (Sway, Hyprland).

### Action Steps
1. In **Terminal 1**, start the viewer in background and capture its PID:
   ```bash
   ufm /tmp/ufm-fixtures &
   UFM_PID=$!
   echo "Viewer is running with PID: $UFM_PID"
   ```
2. In **Terminal 2**, control the viewer remotely:
   ```bash
   # 1. Remotely zoom in 2x
   ufm-msg $UFM_PID "zoom 2"

   # 2. Remotely rotate clockwise by 90 degrees
   ufm-msg $UFM_PID "rotate by 90"

   # 3. Remotely advance to next image
   ufm-msg $UFM_PID "next"

   # 4. Remotely append a new folder to playlist
   ufm-msg $UFM_PID "open /tmp/ufm-fixtures"

   # 5. Remotely terminate viewer
   ufm-msg $UFM_PID "quit"
   ```
3. **Visual Observation**:
   - The window in Terminal 1 mirrors each command in real-time without latency, closing cleanly upon receiving `quit`. The socket at `$XDG_RUNTIME_DIR/ufm-$UFM_PID.sock` is automatically deleted.

---

## Practice 7: Live Disk Modification Hot Reloading

Tests automatic filesystem monitoring and in-place reload.

### Action Steps
1. In **Terminal 1**, open a text-based PPM file:
   ```bash
   ufm /tmp/ufm-fixtures/sample_p3_ascii.ppm
   ```
2. In **Terminal 2**, overwrite the file with solid yellow pixels:
   ```bash
   python3 -c '
   with open("/tmp/ufm-fixtures/sample_p3_ascii.ppm", "w") as f:
       f.write("P3\n100 100\n255\n" + "255 255 0 " * 10000)
   '
   ```
3. **Visual Observation**:
   - Within 0.5 seconds, without restarting or pressing refresh, **the open viewer window automatically detects the modification and refreshes the canvas to solid yellow**!

---

## Practice 8: Direct Piped Streams & Zero-Disk Image Viewing

Tests pipeline integration and standard input streaming.

### Action Steps
1. **Piped file paths**:
   ```bash
   find /tmp/ufm-fixtures -name "*.ppm" | ufm
   ```
   *Observation*: The viewer opens and constructs a playlist directly from piped filenames.

2. **Piped binary image stream (`-`)**:
   ```bash
   cat /tmp/ufm-fixtures/vector_sample.svg | ufm -
   ```
   *Observation*: Decodes and presents the image entirely from standard input memory without creating temporary files on disk.

---

## Practice 9: Wayland Drag-and-Drop (DND) Ingestion

### Action Steps
1. Launch an empty viewer:
   ```bash
   ufm
   ```
   *Observation*: A clean black window opens awaiting input.
2. Open your system's Wayland file manager (Nautilus, Dolphin, PCManFM-Qt, or Thunar).
3. Drag any image file from the file manager and drop it into the `ufm` window.
4. **Visual Observation**:
   - `ufm` consumes the Wayland DND URI payload and renders the dropped image immediately.

---

## Turnkey Automated Live Verification Script

To run an automated test that launches `ufm`, captures real compositor screenshots, and verifies pixel values:

```bash
./tools/verify_live_run.sh
```
