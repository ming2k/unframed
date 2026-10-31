---
status: "accepted"
date: "2026-10-18"
decision-makers: "Engineering Team"
consulted: "Users, Desktop Integrators"
informed: "All Contributors"
title: "Canonical Binary Naming as ufm"
tags: ["cli/naming", "cli/ergonomics", "ipc/socket"]
relates-to: []
supersedes: []
superseded-by: []
---

# 0003. Canonical Binary Naming as ufm

## Context

The repository is named `unframed`, expressing the design philosophy of an unadorned, borderless image viewer focused purely on the visual content.

In command-line environments and Wayland tiling window managers (Sway, Hyprland, River), users launch image viewers frequently throughout the day:
- Typing `unframed image.png` involves 8 characters and unnecessary friction.
- Canonical Unix utilities consistently adopt concise, memorable 2-3 letter commands (`rg` for ripgrep, `fd` for fd-find, `mpv` for movie player).

We must establish an ergonomic, concise primary command while preserving access via the full name `unframed`.

## Decision Drivers

- **Ergonomics**: Minimize keystroke overhead for daily terminal and launcher usage.
- **Brand Lineage**: Direct abbreviation from **u**n**f**ra**m**ed -> `ufm`.
- **IPC Protocol Consistency**: Symmetrical naming between the viewer (`ufm`) and its IPC control tool (`ufm-msg`).

## Considered Options

1. Keep `unframed` exclusively as the binary name.
2. Adopt `ufm` as the canonical binary name, with `unframed` as parallel binary.
3. Rename repository and project to `ufm`.

## Decision

Adopt **`ufm`** as the canonical binary name, while maintaining `unframed` as an alias:

1. **Cargo Configuration**:
   - `Cargo.toml` sets `default-run = "ufm"`.
   - Primary binary target: `[[bin]] name = "ufm" path = "src/main.rs"`.
   - Secondary binary target: `[[bin]] name = "unframed" path = "src/bin/unframed.rs"`.
   - IPC binary target: `[[bin]] name = "ufm-msg" path = "src/bin/ufm_msg.rs"`.

2. **IPC Sockets**:
   - Every viewer instance binds `ufm-$PID.sock` and `unframed-$PID.sock` under `$XDG_RUNTIME_DIR`.

3. **Configuration**:
   - Discovers configuration in `$ufm_config`, `$unframed_config`, `~/.config/ufm/config`, and `~/.config/unframed/config`.

## Consequences

### Positive

- Typing `ufm <file>` is fast, intuitive, and ergonomic.
- IPC command `ufm-msg <pid> <cmd>` is symmetrical.
- Full compatibility for users who invoke `unframed`.

### Negative

- Documentation must clearly explain that `ufm` is the primary executable for `unframed`.
