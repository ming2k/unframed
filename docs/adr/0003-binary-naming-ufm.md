# ADR 0003: Canonical Binary Naming as `ufm`

- **Status**: Accepted
- **Date**: 2026-11-01
- **Deciders**: Engineering Team

---

## Context

The repository is named `unframed`, reflecting the philosophy of minimalist, borderless, pure image presentation.

However, in Unix command-line environments and tiling window managers (Sway, Hyprland, i3, River), users invoke image viewers dozens or hundreds of times per day:
- Typing `unframed image.png` has a 8-character burden and higher cognitive friction.
- Iconic Unix CLI tools consistently adopt concise, memorable 2-3 letter names:
  - `ripgrep` -> `rg`
  - `fd-find` -> `fd`
  - `simple x image viewer` -> `sxiv`
  - `image viewer` -> `imv`
  - `movie player` -> `mpv`

---

## Decision

1. **Adopt `ufm` as the Primary Binary Name**:
   - Derived directly from **u**n**f**ra**m**ed.
   - 3 characters, ergonomic to type in terminals and shell aliases.
2. **IPC Companion as `ufm-msg`**:
   - The IPC CLI tool is named `ufm-msg`, perfectly mirroring `imv-msg`.
3. **Dual Alias Compatibility**:
   - The Cargo manifest registers `ufm` as the default run target (`default-run = "ufm"`).
   - Additional binary targets `unframed`, `unframed-msg`, and `imv-msg` are compiled and distributed in parallel, ensuring existing shell scripts and workflow automation continue working without modification.

---

## Consequences

### Positive
- Ergonomic, fast terminal invocation (`ufm <file>`).
- Consistent IPC tool naming (`ufm-msg`).
- Complete backward compatibility with existing `imv` and `unframed` invocations.

### Negative
- Users familiar only with the repository name `unframed` need clear documentation pointing to `ufm`.
