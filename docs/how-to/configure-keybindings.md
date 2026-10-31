# How-to: Configure Custom Keybindings

This guide explains how to define shortcuts, multi-key chords, and command bindings in Unframed (`ufm`).

---

## 1. Locating the Configuration File

Unframed looks for configuration files at:

1. Path specified by `$ufm_config` or `$unframed_config` environment variable
2. `$XDG_CONFIG_HOME/ufm/config` (or `~/.config/ufm/config`)
3. `~/.ufm_config`
4. `/etc/ufm/config`

Create `~/.config/ufm/config` if it does not yet exist.

---

## 2. Binding Syntax

Keybindings are configured inside the `[binds]` section:

```ini
[binds]
# <KeySpec> = <Command> [; <AdditionalCommand>...]
```

### Single Character Keys
```ini
[binds]
q = quit
c = center
r = reset
a = zoom actual
```

### Special Keys Wrapped in `<>`
```ini
[binds]
<Left> = prev
<Right> = next
<Up> = zoom 1
<Down> = zoom -1
<space> = toggle_playing
<period> = next_frame
<bracketleft> = prev
<bracketright> = next
```

### Modifier Chords
Combine keys with `Ctrl+`, `Shift+`, or `Alt+`:
```ini
[binds]
<Ctrl+r> = rotate by 90
<Shift+G> = goto -1
<Shift+plus> = zoom 1
<Shift+T> = slideshow -1
```

### Multi-Key Sequences (Trie Matching)
Unframed matches multi-key sequences using an internal prefix trie:
```ini
[binds]
# Press 'g' twice to go to the first image in playlist
gg = goto 1

# Two-key chords
<Ctrl+x>c = close all
```

### Compound Command Chaining
Separate commands with a semicolon `;`:
```ini
[binds]
# Move to next image, zoom to actual size, and center
n = next; zoom actual; center
```

---

## 3. Disabling Default Keybindings

To build an entirely custom key mapping from scratch, disable built-in binds in `[options]`:

```ini
[options]
suppress_default_binds = true

[binds]
q = quit
h = pan -50 0
j = pan 0 -50
k = pan 0 50
l = pan 50 0
```

---

## 4. Testing Keybindings Interactively

You can test bindings dynamically in a running instance via the `:` prompt:

```text
:bind p exec echo $ufm_current_file
:alias wall exec swww img "$ufm_current_file"
```
