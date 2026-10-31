# Keybinding Engine & Modal Input Routing

This article explains the design and operational mechanics of Unframed's input routing pipeline, covering prefix-trie chord resolution, token parsing, and modal console isolation.

---

## 1. Why a Prefix Trie?

Traditional image viewers and desktop utilities often implement keybindings using a flat hash map (`HashMap<KeyCode, Command>`). While straightforward, a flat lookup table has critical limitations:
- It cannot support multi-key sequences or "chords" (such as vim-style `gg` to jump to the first image).
- It cannot gracefully differentiate between an incomplete chord prefix (e.g. the first `g` in `gg`) and an invalid key press.

Unframed implements a stateful **Prefix Trie** (`src/binds.rs`):

```text
                     [ Root Trie Node ]
                     /       │        \
                    /        │         \
             'q'   /    'g'  │   '<Left>' \
                  ▼          ▼             ▼
              [ "quit" ]   [ Node ]    [ "prev" ]
                             │
                        'g'  │
                             ▼
                        [ "goto 1" ]
```

### 1.1 Trie Node Evaluation
When a key event is received from the Wayland input pipeline, the engine navigates the trie:
1. **`Match(command)`**: The key sequence exactly matches a terminal node in the trie. The bound command is returned for execution, and the trie cursor resets to the root.
2. **`Pending`**: The key matches an intermediate branch node that has child transitions. The engine buffers the key and awaits the next input event without executing an action.
3. **`Mismatch`**: The key does not match any valid path from the current node. The engine resets its traversal cursor to root.

This ensures multi-key chords execute deterministically without artificial polling timers or race conditions.

---

## 2. Token Grammar & Modifier Encoding

Key definitions in configuration files and commands are tokenized into a normalized string representation:

### 2.1 Grammar Rules
- **Literal Characters**: Single ASCII printable characters represent themselves directly (e.g., `q`, `x`, `j`, `k`, `+`, `-`).
- **Bracketed Tokens**: Special keys and non-printable keys are enclosed in angle brackets `<...>` (e.g., `<Left>`, `<Right>`, `<Up>`, `<Down>`, `<space>`, `<bracketleft>`).
- **Modifier Precedence**: Modifiers (`Shift+`, `Ctrl+`, `Alt+`) are prefixed inside bracketed tokens:
  ```text
  <Shift+G>
  <Ctrl+r>
  <Shift+plus>
  <Shift+T>
  ```

### 2.2 Platform Translation
The input pipeline receives raw key codes and bitmask modifiers from `lens::key` and `lens::mods`. The helper function `lens_key_to_string` normalizes these into the canonical bracketed tokens, guaranteeing that configuration keys match physical keystrokes regardless of underlying Wayland scancode shifts.

---

## 3. Modal Isolation: Normal Viewing vs. Command Console

Unframed maintains two distinct input states:

```text
┌────────────────────────┐           ':' Key           ┌────────────────────────┐
│      Normal Mode       │  ────────────────────────►  │      Console Mode      │
│  - Trie chord dispatch │                             │  - Raw text insertion  │
│  - Pan & zoom dragging │  ◄────────────────────────  │  - History recall      │
│  - Slideshow timers    │        Enter / Escape       │  - Tab completion      │
└────────────────────────┘                             └────────────────────────┘
```

1. **Normal Mode**: Keystrokes are directed exclusively to the prefix-trie binding engine. Mouse clicks and drags manipulate the viewport.
2. **Console Mode (Activated by `:`)**:
   - The interactive command bar opens at the bottom of the window.
   - All standard navigation keybindings are suspended.
   - Keystrokes are routed directly to the `Console` buffer:
     - Printable text appends to the edit line.
     - `Backspace` and `Delete` perform character erasure.
     - `Left`, `Right`, `Home`, and `End` move the cursor.
     - `Up` and `Down` cycle through previous command history.
     - `Tab` invokes lexical prefix completion against built-in commands and aliases.
   - Pressing `Enter` executes the command and returns immediately to Normal Mode.
   - Pressing `Escape` discards the input line and returns immediately to Normal Mode.

This strict modal separation eliminates accidental navigation during command input while preserving the instant responsiveness expected of a lightweight terminal-grade viewer.
