//! Command parsing and word splitting utilities.

use std::collections::HashMap;

pub fn default_aliases() -> HashMap<String, String> {
    let mut aliases = HashMap::new();
    aliases.insert("q".into(), "quit".into());
    aliases.insert("n".into(), "next".into());
    aliases.insert("p".into(), "prev".into());
    aliases.insert("g".into(), "goto".into());
    aliases.insert("z".into(), "zoom".into());
    aliases.insert("o".into(), "open".into());
    aliases.insert("f".into(), "fullscreen".into());
    aliases.insert("d".into(), "overlay".into());
    aliases.insert("c".into(), "center".into());
    aliases.insert("r".into(), "reset".into());
    aliases.insert("s".into(), "scaling".into());
    aliases.insert("S".into(), "upscaling".into());
    aliases.insert("ss".into(), "slideshow".into());
    aliases.insert("bg".into(), "background".into());
    aliases
}

pub fn split_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
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
            escape = true;
            continue;
        }

        if c == '\'' && !in_double {
            in_single = !in_single;
        } else if c == '"' && !in_single {
            in_double = !in_double;
        } else if c.is_whitespace() && !in_single && !in_double {
            if !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words() {
        let words = split_words("pan 50 -50 \"a quoted string\" 'single quotes'");
        assert_eq!(
            words,
            vec!["pan", "50", "-50", "a quoted string", "single quotes"]
        );
    }
}
