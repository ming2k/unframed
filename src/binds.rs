//! Key sequence bindings using a Trie-based prefix tree.

use lens::key;
use lens::mods;

#[derive(Debug, Clone, Default)]
struct BindNode {
    key: String,
    commands: Vec<String>,
    children: Vec<BindNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindAction {
    None,
    Pending,
    Execute(Vec<String>),
}

pub struct Binds {
    root: BindNode,
    accumulated: Vec<String>,
}

impl Default for Binds {
    fn default() -> Self {
        Self::new()
    }
}

impl Binds {
    pub fn new() -> Self {
        Self {
            root: BindNode::default(),
            accumulated: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.root = BindNode::default();
        self.accumulated.clear();
    }

    #[allow(dead_code)]
    pub fn clear_input(&mut self) {
        self.accumulated.clear();
    }

    /// Add a key sequence binding. `key_seq` can be "q", "gg", "<Left>", "<Shift+G>", "<Ctrl+r>", etc.
    /// `command` can be multiple commands separated by semicolon.
    pub fn add(&mut self, key_seq: &str, command: &str) {
        let keys = parse_key_sequence(key_seq);
        if keys.is_empty() {
            return;
        }

        let mut node = &mut self.root;
        for k in &keys {
            let child_idx = if let Some(idx) = node.children.iter().position(|c| c.key == *k) {
                idx
            } else {
                node.children.push(BindNode {
                    key: k.clone(),
                    commands: Vec::new(),
                    children: Vec::new(),
                });
                node.children.len() - 1
            };
            node = &mut node.children[child_idx];
        }

        for cmd in split_semicolon_commands(command) {
            let trimmed = cmd.trim();
            if !trimmed.is_empty() {
                node.commands.push(trimmed.to_string());
            }
        }
    }

    /// Handle a single key event string like "q", "g", "<Left>", "<Ctrl+r>".
    pub fn handle_key(&mut self, key_str: &str) -> BindAction {
        if key_str == "Escape" && !self.accumulated.is_empty() {
            self.accumulated.clear();
            return BindAction::None;
        }

        self.accumulated.push(key_str.to_string());

        // Traverse trie
        let mut node = &self.root;
        for k in &self.accumulated {
            if let Some(child) = node.children.iter().find(|c| c.key == *k) {
                node = child;
            } else {
                // Not a match, reset
                self.accumulated.clear();
                return BindAction::None;
            }
        }

        if !node.commands.is_empty() {
            let cmds = node.commands.clone();
            self.accumulated.clear();
            BindAction::Execute(cmds)
        } else if !node.children.is_empty() {
            BindAction::Pending
        } else {
            self.accumulated.clear();
            BindAction::None
        }
    }

    pub fn load_defaults(&mut self) {
        self.add("q", "quit");
        self.add("<Left>", "prev");
        self.add("[", "prev");
        self.add("<bracketleft>", "prev");
        self.add("<Right>", "next");
        self.add("]", "next");
        self.add("<bracketright>", "next");
        self.add("gg", "goto 1");
        self.add("<Shift+G>", "goto -1");
        self.add("j", "pan 0 -50");
        self.add("k", "pan 0 50");
        self.add("h", "pan 50 0");
        self.add("l", "pan -50 0");
        self.add("<Up>", "zoom 1");
        self.add("<Shift+plus>", "zoom 1");
        self.add("+", "zoom 1");
        self.add("=", "zoom 1");
        self.add("i", "zoom 1");
        self.add("<Down>", "zoom -1");
        self.add("<minus>", "zoom -1");
        self.add("-", "zoom -1");
        self.add("o", "zoom -1");
        self.add("<Ctrl+r>", "rotate by 90");
        self.add("x", "close");
        self.add("f", "fullscreen");
        self.add("d", "overlay");
        self.add("p", "exec echo $ufm_current_file");
        self.add("c", "center");
        self.add("s", "scaling next");
        self.add("<Shift+S>", "upscaling next");
        self.add("a", "zoom actual");
        self.add("r", "reset");
        self.add("<period>", "next_frame");
        self.add(".", "next_frame");
        self.add("<space>", "toggle_playing");
        self.add(" ", "toggle_playing");
        self.add("t", "slideshow +1");
        self.add("<Shift+T>", "slideshow -1");
        self.add("b", "transparency next");
    }
}

/// Parse a key sequence like "gg", "<Ctrl+r>", "<Shift+G>a" into constituent key representations.
pub fn parse_key_sequence(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '<' {
            if let Some(close_idx) = chars[i..].iter().position(|&c| c == '>') {
                let end = i + close_idx + 1;
                let token: String = chars[i..end].iter().collect();
                parts.push(token);
                i = end;
                continue;
            }
        }
        parts.push(chars[i].to_string());
        i += 1;
    }

    parts
}

/// Convert an input key code and modifier bitmask into a canonical key representation string.
pub fn lens_key_to_string(code: i32, modifier_mask: u32) -> Option<String> {
    let ctrl = (modifier_mask & mods::CTRL) != 0;
    let shift = (modifier_mask & mods::SHIFT) != 0;
    let alt = (modifier_mask & mods::ALT) != 0;
    let super_mod = (modifier_mask & mods::SUPER) != 0;

    let base_name: Option<String> = match code {
        key::ESCAPE => Some("Escape".to_string()),
        key::TAB => Some("Tab".to_string()),
        key::RETURN => Some("Return".to_string()),
        key::BACKSPACE => Some("BackSpace".to_string()),
        key::DELETE => Some("Delete".to_string()),
        key::LEFT => Some("Left".to_string()),
        key::RIGHT => Some("Right".to_string()),
        key::UP => Some("Up".to_string()),
        key::DOWN => Some("Down".to_string()),
        key::HOME => Some("Home".to_string()),
        key::END => Some("End".to_string()),
        k if (key::F1..=key::F12).contains(&k) => Some(format!("F{}", k - key::F1 + 1)),
        c if (32..=126).contains(&c) => {
            let ch = (c as u8) as char;
            match ch {
                ' ' => Some("space".to_string()),
                '.' => Some("period".to_string()),
                '-' => Some("minus".to_string()),
                '+' => Some("plus".to_string()),
                '=' if shift => Some("plus".to_string()),
                '[' => Some("bracketleft".to_string()),
                ']' => Some("bracketright".to_string()),
                'a'..='z' => {
                    if shift {
                        Some(ch.to_ascii_uppercase().to_string())
                    } else {
                        Some(ch.to_string())
                    }
                }
                _ => Some(ch.to_string()),
            }
        }
        _ => None,
    };

    let name = base_name?;

    // If no modifiers
    if !ctrl && !alt && !shift && !super_mod {
        if name.len() == 1 && name != " " {
            return Some(name);
        } else {
            return Some(format!("<{name}>"));
        }
    }

    // With modifiers, build <Mod1+Mod2+Key>
    let mut mod_parts = Vec::new();
    if ctrl {
        mod_parts.push("Ctrl");
    }
    if super_mod {
        mod_parts.push("Meta");
    }
    if alt {
        mod_parts.push("Alt");
    }
    if shift {
        mod_parts.push("Shift");
    }

    let joined = mod_parts.join("+");
    Some(format!("<{joined}+{name}>"))
}

pub fn split_semicolon_commands(s: &str) -> Vec<String> {
    let mut cmds = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut escape = false;

    for c in s.chars() {
        if escape {
            current.push(c);
            escape = false;
            continue;
        }

        if c == '\\' {
            current.push(c);
            escape = true;
            continue;
        }

        if c == '\'' && !in_double {
            in_single = !in_single;
            current.push(c);
        } else if c == '"' && !in_single {
            in_double = !in_double;
            current.push(c);
        } else if c == ';' && !in_single && !in_double {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                cmds.push(trimmed);
            }
            current.clear();
        } else {
            current.push(c);
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        cmds.push(trimmed);
    }

    cmds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_sequence() {
        assert_eq!(parse_key_sequence("q"), vec!["q"]);
        assert_eq!(parse_key_sequence("gg"), vec!["g", "g"]);
        assert_eq!(parse_key_sequence("<Left>"), vec!["<Left>"]);
        assert_eq!(parse_key_sequence("<Ctrl+r>"), vec!["<Ctrl+r>"]);
        assert_eq!(parse_key_sequence("<Ctrl+a>p"), vec!["<Ctrl+a>", "p"]);
    }

    #[test]
    fn test_binds_single_and_multi_key() {
        let mut binds = Binds::new();
        binds.add("q", "quit");
        binds.add("gg", "goto 1");
        binds.add("<Shift+G>", "goto -1");

        assert_eq!(binds.handle_key("q"), BindAction::Execute(vec!["quit".into()]));
        assert_eq!(binds.handle_key("g"), BindAction::Pending);
        assert_eq!(binds.handle_key("g"), BindAction::Execute(vec!["goto 1".into()]));
        assert_eq!(binds.handle_key("<Shift+G>"), BindAction::Execute(vec!["goto -1".into()]));
    }

    #[test]
    fn test_binds_multiple_commands() {
        let mut binds = Binds::new();
        binds.add("<Shift+X>", "exec rm \"$ufm_current_file\"; close");
        assert_eq!(
            binds.handle_key("<Shift+X>"),
            BindAction::Execute(vec![
                "exec rm \"$ufm_current_file\"".into(),
                "close".into()
            ])
        );
    }
}
