---
id: ADR-0004
title: "Trie-Based Prefix Keybinding Engine"
status: accepted
date: 2026-10-20
scope: input/binds, interaction/chords, ui/console
superseded_by: null
negative_knowledge: true
---

# 0004. Trie-Based Prefix Keybinding Engine

- Status: Accepted
- Date: 2026-10-20
- Deciders: Unframed Core Working Group, Input Architecture

---

## Context and Problem Statement

Image viewing requires versatile keyboard navigation, ranging from single-key shortcuts (`q`, `f`, `d`) to complex vim-style chords (`gg` to jump to first, `<Shift+G>` for last) and modifier combinations (`<Ctrl+r>`, `<Shift+S>`).

A naive keybinding system based on a flat hash map (`HashMap<Key, Command>`) cannot represent multi-key sequences without introducing fragile state flags, timeouts, or race conditions. Furthermore, opening an interactive console (`:`) requires cleanly diverting raw keystrokes away from navigation into text buffer editing.

---

## Decision Drivers

- **Deterministic Chord Parsing**: Multi-key sequences must resolve cleanly without artificial delay timers.
- **Prefix Isolation**: Partial sequences must not prematurely trigger leaf actions or drop subsequent keystrokes.
- **Clean Modal Demarcation**: Strict separation between normal navigation mode and interactive command editing mode.

---

## Architectural Decisions

We implement a stateful **Prefix Trie** for keybinding resolution (`src/binds.rs`):

1. **Prefix Trie Architecture**: Keybindings are organized as a tree where edges represent normalized key tokens (`q`, `g`, `<Left>`, `<Ctrl+r>`). Each node holds an optional command action.
2. **Evaluation States**:
   - `Match(command)`: Leaf reached; executes command and resets cursor.
   - `Pending`: Intermediate node reached; holds state waiting for subsequent keystroke.
   - `Mismatch`: Key does not branch from current state; resets cursor to root.
3. **Modal Console Separation**: Pressing `:` shifts input focus into the `Console` state machine. Standard navigation chords are suspended until the console is closed via `Enter` or `Escape`.

---

## Consequences

- **Positive**:
  - Full support for multi-key chords (`gg`, `dd`, custom multi-key binds) with zero race conditions.
  - Clean separation of concerns between navigation input and command line editing.
- **Negative & Trade-offs**:
  - Trie nodes require memory allocation for child branches (mitigated by small keymap cardinality).

---

## Rejected Alternatives & Negative Knowledge

- **Timer-Based Chord Buffering**: Buffering keys and waiting 300ms before dispatching was rejected because it introduces perceptible input latency and degrades responsiveness.
