# How to Configure Keybindings and Aliases

This guide explains how to customize keybindings, define multi-key chord sequences, configure custom aliases, and suppress default bindings in Unframed.

---

## 1. Locating or Creating Your Configuration File

Unframed searches for configuration files in the following order:

1. Path specified by `$unframed_config` or `$imv_config` environment variable
2. `$XDG_CONFIG_HOME/unframed/config` or `$XDG_CONFIG_HOME/imv/config`
3. `~/.config/unframed/config` or `~/.config/imv/config`
4. `/etc/imv_config`

Create the configuration directory and file:

```bash
mkdir -p ~/.config/unframed
touch ~/.config/unframed/config
```

---

## 2. Binding Keys in the `[binds]` Section

The `[binds]` section maps key sequences directly to commands.

```ini
[binds]
# Single key mapping
q = quit
x = close

# Multi-key chord sequences (prefix trie)
gg = goto 1

# Special keys enclosed in angle brackets
<Left> = prev
<Right> = next
<bracketleft> = prev
<bracketright> = next
<space> = toggle_playing

# Modifier combinations (Shift, Ctrl)
<Shift+G> = goto -1
<Ctrl+r> = rotate by 90
<Shift+plus> = zoom 1
<minus> = zoom -1
```

### Supported Key Tokens
- **Letters & Numbers**: `a`–`z`, `0`–`9`.
- **Special Keys**: `<Left>`, `<Right>`, `<Up>`, `<Down>`, `<space>`, `<period>`, `<minus>`, `<bracketleft>`, `<bracketright>`.
- **Modifiers**: Combined with `+`, such as `<Shift+G>`, `<Ctrl+r>`, `<Shift+S>`, `<Shift+T>`.

---

## 3. Suppressing Default Bindings

By default, Unframed loads standard default bindings. If you prefer to define a completely clean, custom keymap without default fallbacks, set `suppress_default_binds = true` in the `[options]` section:

```ini
[options]
suppress_default_binds = true

[binds]
q = quit
<Left> = prev
<Right> = next
```

---

## 4. Defining Command Aliases in `[aliases]`

You can create convenient aliases for frequently used commands in the `[aliases]` section:

```ini
[aliases]
q = quit
n = next
p = prev
z = zoom
fit = scaling full
pixel = upscaling nearest_neighbour
```

When you type `:fit` in the command console or run an IPC command `unframed-msg $PID fit`, Unframed automatically expands the alias to `scaling full`.

---

## 5. Modifying Keybindings Dynamically at Runtime

You can add or update keybindings on the fly without restarting Unframed, using the `:bind` command in the interactive console or via IPC:

```bash
# Map F to fullscreen dynamically
:bind f fullscreen

# Map a custom shell command to p
:bind p exec echo $imv_current_file

# Create a dynamic alias
:alias wall exec feh --bg-fill "$imv_current_file"
```

---

## Next Steps

- **[IPC Scripting & Automation](ipc-scripting-and-automation.md)**: Send commands remotely from scripts or window managers.
- **[Configuration Reference](../reference/configuration.md)**: Full specification of all available ini options.
- **[Command Reference](../reference/commands.md)**: Full list of supported commands and parameters.
