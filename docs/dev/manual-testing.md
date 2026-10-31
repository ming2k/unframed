# Manual Testing & Full-Stack Verification Guide

This document defines the comprehensive manual testing procedures, exploratory testing playbooks, and release verification runbooks for **Unframed (`ufm`)**.

It ensures that developers, QA engineers, and release gatekeepers can verify end-to-end functionality, user experience ergonomics, hardware acceleration, format rendering, and fault recovery from a clean cold start without synthetic test harnesses.

---

## 1. Principles & Testing Philosophy

Manual verification complements automated test suites (`cargo test --all-targets`) by focusing on visual fidelity, Wayland compositor interaction, and cognitive ergonomics that automated runners cannot evaluate:

1. **`[INV-ARCH-01]` Pure Wayland Fail-Fast**:
   Verify that `ufm` executes seamlessly under modern Wayland compositors (Sway, Hyprland, GNOME, KDE) while rejecting pure X11 execution with clean diagnostics.
2. **`[INV-HIDPI-COMPENSATION]` High-DPI Visual Integrity**:
   Verify that fractional and integer buffer scales (1x, 1.5x, 2x) render images, background patterns, and OSD text in proper visual proportions without fractional cropping or blurriness.
3. **Format Accuracy & Vector Fidelity**:
   Verify that SVG vector paths scale smoothly, Netpbm variants (P1 ~ P7 PAM) parse accurately, and multi-frame animations (GIF / Animated WebP) advance their frames on time.
4. **Interactive Ergonomics**:
   Verify that mouse dragging, mouse-centered zooming, Vim key navigation, and the `:` command console respond instantaneously with zero latency.
5. **IPC Automation Parity**:
   Ensure external scripts driving `ufm-msg` can control running viewer windows over UNIX domain sockets reliably.

---

## 2. Environment Preparation & Sandbox Setup

### Prerequisites

- **Wayland Compositor**: Sway, Hyprland, GNOME Wayland, or KDE Plasma Wayland (`WAYLAND_DISPLAY` set).
- **GPU Drivers**: Vulkan 1.3 capable driver (RADV, ANV, or NVIDIA).
- **Terminal Emulator**: Ghostty, Foot, Alacritty, or Kitty.

### Generating Test Fixtures

Run the built-in fixture generator to populate test images:

```bash
# 1. Build release binaries
cargo build --release

# 2. Generate sample test images
./tools/generate_test_fixtures.sh /tmp/ufm-fixtures

# 3. Add binaries to PATH for testing
export PATH="$PWD/target/release:$PATH"

echo "Ready for manual testing with ufm and ufm-msg"
```

---

## 3. Step-by-Step Manual Verification Scenarios

### Suite 1: CLI Flags & Safety Gates

#### Scenario 1.1: Help and Version Banners
- **Action**:
  ```bash
  ufm --help
  ufm --version
  ```
- **Expected Outcome**:
  - Displays version `0.1.0` and complete usage options.
  - Returns exit code 0.

#### Scenario 1.2: Pure X11 Fail-Safe Guardrail
- **Action**:
  ```bash
  env -u WAYLAND_DISPLAY -u WAYLAND_SOCKET ufm /tmp/ufm-fixtures/vector_sample.svg
  ```
- **Expected Outcome**:
  - Prints: `ufm: Wayland connection required (WAYLAND_DISPLAY is not set). X11 is not supported.`
  - Exits with code 1 immediately without panicking or hanging.

---

### Suite 2: Multi-Format Image Rendering

#### Scenario 2.1: SVG Vector Rendering & Dynamic Scaling
- **Action**:
  ```bash
  ufm /tmp/ufm-fixtures/vector_sample.svg
  ```
- **Expected Outcome**:
  - Window opens with the vector image rendered crisply with gradient and antialiased text.
  - Press `+` or scroll wheel up to zoom in to 500%+.
  - Text and circles remain sharp and vector-smooth.

#### Scenario 2.2: Netpbm Family Verification (P1 ~ P7 PAM)
- **Action**:
  ```bash
  ufm /tmp/ufm-fixtures/sample_p1_ascii.pbm \
      /tmp/ufm-fixtures/sample_p2_ascii.pgm \
      /tmp/ufm-fixtures/sample_p3_ascii.ppm \
      /tmp/ufm-fixtures/sample_p4_binary.pbm \
      /tmp/ufm-fixtures/sample_p5_binary.pgm \
      /tmp/ufm-fixtures/sample_p6_binary.ppm \
      /tmp/ufm-fixtures/sample_p7_alpha.pam
  ```
- **Expected Outcome**:
  - Cycle through all images with `Right` / `Left`.
  - P1 and P4 render high-contrast black/white pixel grids.
  - P2 and P5 render smooth grayscale gradients.
  - P3 and P6 render primary colors.
  - P7 PAM renders semi-transparent colored blocks over the background.

#### Scenario 2.3: Animated GIF & WebP Playback
- **Action**:
  ```bash
  ufm /tmp/ufm-fixtures/sample_animated.gif
  ```
- **Expected Outcome**:
  - The GIF animates continuously between frames.
  - Press **`Space`**: The animation freezes/pauses.
  - Press **`.`**: Advances exactly one frame while paused.
  - Press **`Space`** again: Resumes smooth continuous playback.

#### Scenario 2.4: Transparency & Checkerboard Background
- **Action**:
  ```bash
  ufm -b checks /tmp/ufm-fixtures/sample_transparent.png
  ```
- **Expected Outcome**:
  - Canvas background shows a two-tone gray checkerboard pattern through transparent areas.
  - Press `:`, type `background 000000`, and hit Enter: switches instantly to solid black.
  - Press `:`, type `background checks`, and hit Enter: switches back to checkerboard.

---

### Suite 3: Viewport & Transformation Interaction

#### Scenario 3.1: Mouse-Centered Logarithmic Zooming
- **Action**:
  - Move mouse cursor over the top-right corner of the image.
  - Scroll mouse wheel up.
- **Expected Outcome**:
  - Image zooms in smoothly centered exactly on the cursor location. The point under the mouse does not drift.
  - Press **`a`**: Resets immediately to 100% actual size.

#### Scenario 3.2: Left-Click Mouse Drag Panning
- **Action**:
  - Zoom into an image until it exceeds window bounds.
  - Click and hold the **left mouse button**, then drag across the screen.
- **Expected Outcome**:
  - The image tracks 1:1 with cursor displacement.
  - Boundary clamps prevent dragging the image completely out of sight.

#### Scenario 3.3: Rotation & Mirroring
- **Action**:
  - Press **`Ctrl + r`**: Image rotates 90° clockwise.
  - Press **`Ctrl + r`** three more times: Returns to original 360° orientation.
  - Press `:`, enter `flip horizontal`, hit Enter: Image mirrors horizontally.
  - Press `:`, enter `flip vertical`, hit Enter: Image mirrors vertically.
  - Press **`r`**: Resets rotation, mirroring, zoom, and pan back to default.

#### Scenario 3.4: Scaling Modes & Upscaling Filters
- **Action**:
  - Press **`s`**: Cycles through `none` → `shrink` → `full` → `crop`.
  - Press **`Shift + S`**: Toggles upscaler between `linear` (smooth) and `nearest_neighbour` (crisp pixel art).

---

### Suite 4: Navigation, Live Disk Reloading & Stdin

#### Scenario 4.1: Directory Scanning & Playlist Wrap
- **Action**:
  ```bash
  ufm -r /tmp/ufm-fixtures
  ```
- **Expected Outcome**:
  - Loads all supported images in the folder.
  - Press **`Right`** past the last image: wraps seamlessly to the first image.
  - Press **`Shift + G`**: Jumps immediately to the last image.
  - Press **`gg`**: Jumps immediately to the first image.

#### Scenario 4.2: Live Disk Modification Reload
- **Action**:
  - Open an image: `ufm /tmp/ufm-fixtures/sample_p3_ascii.ppm`
  - In another terminal, modify the file:
    ```bash
    echo -e "P3\n1 1\n255\n255 255 0" > /tmp/ufm-fixtures/sample_p3_ascii.ppm
    ```
- **Expected Outcome**:
  - Within 0.5s, `ufm` detects the changed disk timestamp and reloads the canvas with the updated image without restarting.

#### Scenario 4.3: Stdin Image Stream (`-`)
- **Action**:
  ```bash
  cat /tmp/ufm-fixtures/vector_sample.svg | ufm -
  ```
- **Expected Outcome**:
  - Loads the image directly from the standard input byte stream.

---

### Suite 5: Interactive Console (`:`) & Trie Keybinding

#### Scenario 5.1: Command Prompt & Line Editing
- **Action**:
  - Press **`:`**: The console prompt opens at the bottom with `:`.
  - Type `zoom 2`, then press **Left arrow**, **Backspace**, and edit text.
  - Hit **Enter**: Command executes and console closes.
  - Press **`:`** then **Up arrow**: Recalls `zoom 2` from command history.
  - Press **Escape**: Closes console without executing.

#### Scenario 5.2: Tab Completion
- **Action**:
  - Press **`:`**, type `scal`, then press **Tab**.
- **Expected Outcome**:
  - Autocompletes to `scaling`.

#### Scenario 5.3: OSD Overlay Display
- **Action**:
  - Press **`d`**: OSD banner appears showing index, dimensions, zoom scale, file path, and scaling mode.
  - Press **`d`** again: OSD banner hides.

---

### Suite 6: Inter-Process Communication (IPC via `ufm-msg`)

#### Scenario 6.1: Remote Command Control
- **Action**:
  1. Start viewer in background:
     ```bash
     ufm /tmp/ufm-fixtures &
     VIEWER_PID=$!
     ```
  2. Send commands from another terminal:
     ```bash
     ufm-msg $VIEWER_PID next
     ufm-msg $VIEWER_PID zoom 2
     ufm-msg $VIEWER_PID rotate by 90
     ufm-msg $VIEWER_PID quit
     ```
- **Expected Outcome**:
  - The viewer advances, zooms, rotates, and exits cleanly.
  - Sockets under `$XDG_RUNTIME_DIR/ufm-$VIEWER_PID.sock` are unlinked on exit.

---

### Suite 7: Wayland Desktop Integration & Drag-and-Drop (DND)

#### Scenario 7.1: Wayland Drag-and-Drop
- **Action**:
  - Launch an empty viewer: `ufm`
  - Open your Wayland file manager (Nautilus, Dolphin, Thunar, or PCManFM-Qt).
  - Drag an image file into the `ufm` window and release.
- **Expected Outcome**:
  - `ufm` accepts the drop payload, parses the `file://` URI, and displays the image immediately.

#### Scenario 7.2: Fullscreen Toggle
- **Action**:
  - Press **`f`**: Window toggles to native Wayland fullscreen.
  - Press **`f`** again: Returns to floating/tiled layout.

---

## 4. Verification Sign-Off Matrix

| Category | Suite | Test Scenarios | Status | Gatekeeper |
| :--- | :--- | :--- | :---: | :---: |
| **CLI & Safety** | Suite 1 | Flags, Version, Wayland Guard | Pass | QA / Dev |
| **Formats** | Suite 2 | SVG, Netpbm (P1-P7), GIF, WebP | Pass | QA / Dev |
| **Viewport** | Suite 3 | Zoom, Pan, Rotation, Flip, HiDPI | Pass | QA / Dev |
| **Navigation** | Suite 4 | Directory, Playlist, Live Reload | Pass | QA / Dev |
| **Console & Keys** | Suite 5 | Prompt, Trie Binds, Tab Complete | Pass | QA / Dev |
| **IPC** | Suite 6 | UNIX Sockets, `ufm-msg` Control | Pass | QA / Dev |
| **Desktop** | Suite 7 | Wayland DND, Fullscreen, XDG | Pass | QA / Dev |
