# Testing Strategy & Execution

This document outlines the testing strategy, test suites, and quality gates for `ufm`.

---

## 1. Running Test Suites

Run all unit tests and integration tests:

```bash
cargo test --all-targets
```

Run tests with verbose stdout output:

```bash
cargo test --all-targets -- --nocapture
```

Run static analysis and lints:

```bash
cargo clippy --all-targets
```

---

## 2. Test Coverage Breakdown

The test suite contains **34 automated tests** covering every critical subsystem:

### Format & Decoder Tests
- `test_ppm_all_variants`: Full Netpbm verification across P1 (ASCII PBM), P2 (ASCII PGM), P3 (ASCII PPM), P4 (Binary PBM), P5 (Binary PGM), P6 (Binary PPM), and P7 (PAM with alpha).
- `test_svg_rendering_and_dimensions`: Validates SVG parsing, intrinsic viewBox calculation, and scaled rasterization.
- `test_gif_animation_loading_and_tick`: Verifies multi-frame GIF decoding, timeline accumulator, and frame transitions.
- `test_load_png_bytes` & `test_test_images_from_imv`: Regression tests loading real PNG, JPEG, TIFF, and WebP sample images from `../imv/test/data`.

### Viewport & Transformation Tests
- `test_viewport_scaling_modes`: Verifies `full`, `shrink`, `crop`, and `none` aspect ratio calculations.
- `test_viewport_zoom`: Validates mouse-anchored and keyboard-centered logarithmic zoom math.
- `test_viewport_rotation_flip`: Tests clockwise 90-degree rotations, horizontal mirroring, and vertical flipping.

### Navigation & Playlist Tests
- `test_navigator_rel_navigation`: Tests next/prev with wrapped looping.
- `test_navigator_no_loop`: Tests boundary clamping when looping is disabled.
- `test_navigator_abs_navigation`: Tests 1-based and negative index addressing.
- `test_navigator_recursive_and_image_detection`: Validates recursive directory scanning and file extension filters.

### Keyboard & Command Engine Tests
- `test_parse_key_sequence`: Tests splitting multi-key chords like `gg`, `<Shift+G>`, and `<Ctrl+a>p`.
- `test_binds_single_and_multi_key`: Validates Trie prefix tree matching and pending actions.
- `test_binds_multiple_commands`: Tests semicolon-delimited compound command sequences.
- `test_app_full_command_suite`: End-to-end command dispatching covering `open`, `pan`, `zoom`, `rotate`, `flip`, `scaling`, `slideshow`, `background`, and `alias`.

### IPC Communication Tests
- `test_ipc_roundtrip`: Spawns a real UNIX domain socket listener, connects via client stream, and verifies command delivery across threads.
