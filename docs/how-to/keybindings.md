# How-to: Custom Keybindings & Chords

`ufm` features a Trie-based prefix tree for keyboard shortcuts, allowing single-key bindings, modifier chords, and multi-key sequences.

---

## 1. Configuration Location

Keybindings can be configured in `~/.config/unframed/config` or `~/.config/imv/config`:

```ini
[options]
# Set to true to disable default bindings and specify everything from scratch
suppress_default_binds = false

[binds]
# Your custom bindings here
```

---

## 2. Syntax Rules

### Single Keys
```ini
q = quit
x = close
f = fullscreen
d = overlay
```

### Special Keys (Wrapped in `<>`)
```ini
<Left> = prev
<Right> = next
<Up> = zoom 1
<Down> = zoom -1
<space> = toggle_playing
<period> = next_frame
<bracketleft> = prev
<bracketright> = next
```

### Modifiers
Prefix the key name with `Ctrl+`, `Shift+`, or `Alt+`:
```ini
<Ctrl+r> = rotate by 90
<Shift+G> = goto -1
<Shift+plus> = zoom 1
<Shift+T> = slideshow -1
```

### Multi-Key Sequences
Simply write the sequence directly:
```ini
# Press 'g' twice to go to the first image
gg = goto 1

# Press Ctrl+a followed by p
<Ctrl+a>p = exec echo "Prefix command executed"
```

### Compound Commands (Separated by Semicolon)
```ini
# Delete the current image and close it
<Shift+X> = exec rm "$imv_current_file"; close
```

---

## 3. Dynamic Binding at Runtime

You can also test and apply bindings on the fly from the interactive console:
1. Press **`:`** to open the prompt.
2. Enter:
   ```text
   bind <Ctrl+q> quit
   ```
3. Press **Enter**. The binding is instantly active!
