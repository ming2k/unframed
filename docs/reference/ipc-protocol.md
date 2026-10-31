# IPC Protocol Reference

This document outlines the inter-process communication (IPC) protocol supported by Unframed (`ufm`).

---

## 1. Transport Layer

Unframed establishes a stream-oriented UNIX domain socket (`AF_UNIX`, `SOCK_STREAM`) bound at startup.

### Socket Location Precedence

Sockets are resolved and created in the following order:
1. `$XDG_RUNTIME_DIR/ufm-$PID.sock`
2. Fallback: `/tmp/ufm-$PID.sock`

---

## 2. Framing & Protocol Semantics

- **Encoding**: UTF-8 plain text.
- **Framing**: Line-delimited (`\n`).
- **Direction**: Unidirectional command ingestion. Clients write a command line and may close immediately.
- **Thread Safety**: Sockets are serviced by worker threads. Each received command posts an event directly to the Wayland main loop via `iris::post_to_main_thread` and requests an immediate animation frame.

```text
┌──────────────────────┐                     ┌───────────────────────────┐
│     ufm-msg (CLI)    │                     │        ufm (Host)         │
└──────────┬───────────┘                     └─────────────┬─────────────┘
           │                                               │
           │  connect(AF_UNIX, "ufm-$PID.sock")           │
           ├──────────────────────────────────────────────>│
           │                                               │
           │  write("next 1\n")                            │
           ├──────────────────────────────────────────────>│
           │                                               │ [Worker Thread]
           │  close()                                      │  parse line
           ├──────────────────────────────────────────────>│  post_to_main_thread
           │                                               │
           ▼                                               ▼ [Main Loop]
                                                             execute_command("next 1")
                                                             request_animation_frame()
```

---

## 3. Environment Variable Injection Contract

Commands executed via the `exec` command spawn `/bin/sh -c "<cmd>"` with the following environment variables:

| Variable Name | Type | Description |
| :--- | :--- | :--- |
| `ufm_pid` | String (integer) | Process ID of the viewer. |
| `ufm_current_file` | String | Path to the active image, or empty string if no image loaded. |
| `ufm_format` | String | Detected format name (e.g., `PNG`, `SVG`, `PPM`, `GIF`). |
| `ufm_scaling_mode` | String | Current mode: `none`, `shrink`, `full`, or `crop`. |
| `ufm_upscaling_method`| String | Current method: `linear` or `nearest_neighbour`. |
| `ufm_loading` | String (`0` or `1`)| `0` if image is loaded, `1` if empty. |
| `ufm_current_index` | String (integer) | 1-based index of current item in playlist (`0` if empty). |
| `ufm_file_count` | String (integer) | Total count of items in playlist. |
| `ufm_width` | String (integer) | Width in pixels of original image (`0` if empty). |
| `ufm_height` | String (integer) | Height in pixels of original image (`0` if empty). |
| `ufm_scale` | String (integer) | Zoom scale as a percentage (`100` = actual size). |
| `ufm_slideshow_duration`| String (float) | Current slideshow interval in seconds. |
| `ufm_slideshow_elapsed` | String (float) | Seconds elapsed since last image cycle. |
