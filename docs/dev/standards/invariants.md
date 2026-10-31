# Invariant & Constraint Governance

This document codifies the definition, categorization, and lifecycle of architectural invariants in Unframed. It establishes a strict policy against pseudo-invariant tag proliferation, ensuring that invariants remain high-value architectural anchors rather than ceremonial comments.

---

## 1. Core Principle: Mechanisms Over Exhortations

In software engineering, an invariant is a condition that must unconditionally hold across all valid states of a system. Historically, projects suffer from "invariant inflation" when mundane UI styles, layout heuristics, or ephemeral flags are labeled as invariants without verification mechanics.

**The Golden Rule**:
> **If a rule has neither a compiler-level proof nor an automated test assertion, it is NOT an invariant — it is merely an aspiration.**

---

## 2. Constraint Admissibility Matrix

Every architectural and operational rule maps deterministically into one of four categories:

| Category | Definition & Scope | Identifier Policy | Mandatory Enforcement Mechanism |
| :--- | :--- | :--- | :--- |
| **System Invariant** | Non-negotiable structural red-lines with cross-boundary blast radius (e.g., protocol purity, thread isolation, driver constraints). | Formally ratified in ADRs (`[INV-ARCH-*]`) | 1. Automated integration regression tests (`tests/*`)<br>2. Compiler typestates & dependency boundaries<br>3. Environment startup assertions |
| **Domain Contract** | State-machine transitions and behavioral rules within a single module (e.g., `Viewport`, `Navigator`, `Console`). | **NO global `INV-` tag**.<br>Expressed as named types. | 1. Typed `enum` variants & private field encapsulation<br>2. Unit tests (`#[test]` in module) |
| **Design Specification** | Visual styling, layout margins, chequered box sizes, default fonts. | **NO `INV-` tag**.<br>Expressed as design constants. | 1. Config defaults and constants<br>2. Direct assertions |
| **Implementation Note** | Transient workarounds, parser edge-case guards, endianness conversions. | **NO `INV-` tag**.<br>Standard code comments. | Direct regression unit test |

---

## 3. The 3-Condition Gate for System Invariants

A rule may be elevated to a **System Invariant** if and only if it satisfies all three criteria:

1. **High Blast Radius**: Violation results in runtime crashes, unrecoverable display corruption, socket leaks, or architectural regression.
2. **Durable Architecture**: It represents a foundational pillar that will not become obsolete due to routine feature iterations or localized redesigns.
3. **Mechanically Enforced**: It specifies an automated enforcement mechanism (compiler type check, dependency rule, or deterministic automated test).

---

## 4. Ratified System Invariants in Unframed

| Identifier | Invariant Statement | Enforcement Mechanism |
| :--- | :--- | :--- |
| `[INV-ARCH-01]` | **Pure Wayland Purity**: The process strictly refuses to launch in non-Wayland environments (`WAYLAND_DISPLAY` or `WAYLAND_SOCKET` missing). | Startup assertion in `src/main.rs:64` |
| `[INV-ARCH-02]` | **Zero X11 Dependency**: The workspace links zero X11 client libraries (e.g., `libX11`, `xcb`) in `Cargo.toml`. | Cargo dependency graph verification |
| `[INV-ARCH-03]` | **Isolated IPC Worker**: UNIX domain socket listeners execute in detached background worker threads and communicate with `App` exclusively via bounded mpsc channels. | Thread isolation in `src/ipc.rs` and channel in `src/app.rs` |
| `[INV-ARCH-04]` | **Retained Frame Cache & Lazy GPU Upload**: CPU-decoded pixel buffers are retained in memory; GPU `flux::Image` textures are lazily staged and uploaded only when the dirty flag is set. | Typestate flag `gpu_needs_upload` in `src/app.rs` |
| `[INV-ARCH-05]` | **Dual-Socket Imv Parity**: The IPC listener binds both `unframed-$PID.sock` and legacy `imv-$PID.sock` to ensure complete backwards compatibility with existing tooling. | Unit test and bind loop in `src/ipc.rs` |
| `[INV-ARCH-06]` | **RAII Socket Cleanup**: Socket files in `$XDG_RUNTIME_DIR` or `/tmp` are unlinked prior to binding and on graceful shutdown. | Stale socket removal in `src/ipc.rs` |
