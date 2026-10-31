# IPC Protocol Reference

This document describes the technical wire contract, socket layout, framing conventions, and concurrency model of Unframed's Inter-Process Communication (IPC) subsystem.

---

## 1. Socket Topology & Discovery

When Unframed launches, it binds to UNIX domain streaming sockets (`AF_UNIX`, `SOCK_STREAM`) to accept control commands from external processes.

### 1.1 Candidate Directories
Unframed searches candidate socket directories in the following order:
1. `$XDG_RUNTIME_DIR` (standard Wayland user runtime directory)
2. `/tmp` (fallback if `XDG_RUNTIME_DIR` is unset)

### 1.2 Socket Naming Patterns
Within the active runtime directory, Unframed binds dual sockets to ensure complete backwards compatibility with existing `imv` integrations:
- `unframed-$PID.sock`: The primary native socket.
- `imv-$PID.sock`: The backwards-compatibility alias socket.
- `ufm-$PID.sock`: The compact socket identifier.

Prior to binding, Unframed safely unlinks any stale socket files left behind by ungraceful terminations (`[INV-ARCH-06]`).

---

## 2. Wire Framing & Protocol Format

The IPC protocol uses a lightweight, newline-delimited ASCII/UTF-8 wire framing:

```text
┌───────────────────────────────────────────────┐
│              Client Command Line              │
│  <command> [arg1] [arg2] ... [\r]\n           │
└───────────────────────────────────────────────┘
```

- **Transport**: Standard stream-oriented UNIX domain socket.
- **Framing**: Each command line must terminate with `\n` (LF) or `\r\n` (CRLF).
- **Encoding**: UTF-8.
- **Payload**: Plain text matching any command accepted by the interactive console (`next`, `zoom 2`, `scaling crop`, etc.).
- **Response**: The socket connection is unidirectional command-oriented; commands are executed asynchronously on the viewer's main thread.

---

## 3. Concurrency & Event Loop Integration

Unframed maintains an isolated multi-threaded listener architecture (`[INV-ARCH-03]`):

```text
┌──────────────────────┐         ┌──────────────────────┐
│  unframed-msg (CLI)  │         │  imv-msg / WM Script │
└──────────┬───────────┘         └──────────┬───────────┘
           │                                │
           ▼                                ▼
┌───────────────────────────────────────────────────────┐
│     Background Worker Threads (UnixListener Loop)     │
└──────────────────────────┬────────────────────────────┘
                           │ std::sync::mpsc::channel
                           ▼
┌───────────────────────────────────────────────────────┐
│   Main Thread Frame Loop (App::on_build via try_recv)  │
│   - Executes command                                  │
│   - Triggers redraw                                   │
└───────────────────────────────────────────────────────┘
```

1. **Listener Thread Pool**: Each bound socket runs a detached worker thread blocking on `UnixListener::accept()`.
2. **Non-Blocking Channel**: Accepted commands are pushed into an unbounded `std::sync::mpsc::Sender<String>` channel.
3. **Frame Pacing**: At the beginning of each frame in `App::on_build`, the main thread drains the receiver channel with `try_recv()` and dispatches commands before evaluating layout and rendering.

---

## 4. Environment Variables Contract for `exec`

Commands executed via the `exec` verb inherit the standard system environment plus the following runtime state variables:

| Variable | Type | Description |
| :--- | :--- | :--- |
| `imv_pid` | String (integer) | Process ID of the viewer. |
| `unframed_pid` | String (integer) | Process ID of the viewer. |
| `imv_current_file` | String | Path to the active image, or empty string if no image loaded. |
| `imv_format` | String | Detected format name (e.g., `PNG`, `SVG`, `PPM`, `GIF`). |
| `imv_scaling_mode` | String | Current mode: `none`, `shrink`, `full`, or `crop`. |
| `imv_upscaling_method`| String | Current method: `linear` or `nearest_neighbour`. |
| `imv_loading` | String (`0` or `1`)| `0` if image is loaded, `1` if empty. |
| `imv_current_index` | String (integer) | 1-based index of current item in playlist (`0` if empty). |
| `imv_file_count` | String (integer) | Total count of items in playlist. |
| `imv_width` | String (integer) | Width in pixels of original image (`0` if empty). |
| `imv_height` | String (integer) | Height in pixels of original image (`0` if empty). |
| `imv_scale` | String (integer) | Zoom scale as a percentage (`100` = actual size). |
| `imv_slideshow_duration`| String (float) | Current slideshow interval in seconds. |
| `imv_slideshow_elapsed` | String (float) | Seconds elapsed since last image cycle. |
