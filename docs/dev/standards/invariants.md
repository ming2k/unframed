# Core System Invariants

This document catalogs the authoritative system invariants enforced across Unframed (`ufm`).

---

## Invariant Catalog

| Invariant ID | Name | Description | Verification Method |
| :--- | :--- | :--- | :--- |
| `[INV-ARCH-01]` | **Pure Wayland Guarantee** | The binary requires Wayland and will not execute under pure X11 without a compositor display socket. | Checked on startup in `src/main.rs`. |
| `[INV-ARCH-02]` | **Optics Graphics Substrate** | Windowing, drawing, and UI chrome flow strictly through `iris`, `flux`, and `lens`. | Crate dependency gates in `Cargo.toml`. |
| `[INV-ARCH-03]` | **Premultiplied GPU Alpha** | Textures uploaded to `flux::Image` must have premultiplied alpha applied. | Verified in `image_loader::premultiply_rgba`. |
| `[INV-ARCH-04]` | **HiDPI Buffer Scale Compensation** | `flux_canvas` drawing operations are scaled by `paint_host.scale()`. | Checked in `on_paint` callback. |
| `[INV-ARCH-05]` | **Socket IPC Contract** | The IPC listener binds `ufm-$PID.sock` for external tooling. | Verified in `tests/integration_tests.rs`. |
| `[INV-ARCH-06]` | **Vector Resolution Independence** | SVG files are rendered on demand with full antialiasing. | Unit tested in `tests/integration_tests.rs`. |
| `[INV-ARCH-07]` | **Netpbm Completeness** | P1 through P7 PAM formats are parsed with whitespace and comment resilience. | Unit tested in `tests/integration_tests.rs`. |
