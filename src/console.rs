//! Command console prompt, line editing, history, and tab completion.

pub struct Console {
    pub active: bool,
    pub buffer: String,
    pub cursor: usize, // Character offset
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    pub completions: Vec<String>,
    completion_index: usize,
    last_tab_prefix: Option<String>,
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    pub fn new() -> Self {
        let completions = vec![
            "quit".into(),
            "close".into(),
            "next".into(),
            "prev".into(),
            "goto".into(),
            "zoom".into(),
            "pan".into(),
            "rotate".into(),
            "flip".into(),
            "open".into(),
            "fullscreen".into(),
            "overlay".into(),
            "exec".into(),
            "center".into(),
            "reset".into(),
            "next_frame".into(),
            "toggle_playing".into(),
            "scaling".into(),
            "upscaling".into(),
            "slideshow".into(),
            "background".into(),
            "transparency".into(),
            "bind".into(),
            "alias".into(),
        ];

        Self {
            active: false,
            buffer: String::new(),
            cursor: 0,
            history: Vec::new(),
            history_index: None,
            completions,
            completion_index: 0,
            last_tab_prefix: None,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.buffer.clear();
        self.cursor = 0;
        self.history_index = None;
        self.last_tab_prefix = None;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.buffer.clear();
        self.cursor = 0;
        self.history_index = None;
        self.last_tab_prefix = None;
    }

    #[allow(dead_code)]
    pub fn insert_text(&mut self, text: &str) {
        for c in text.chars() {
            self.insert_char(c);
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if c.is_control() {
            return;
        }
        let byte_pos = self.char_to_byte_index(self.cursor);
        self.buffer.insert(byte_pos, c);
        self.cursor += 1;
        self.last_tab_prefix = None;
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let byte_pos = self.char_to_byte_index(self.cursor);
            self.buffer.remove(byte_pos);
            self.last_tab_prefix = None;
        }
    }

    pub fn delete(&mut self) {
        let total_chars = self.buffer.chars().count();
        if self.cursor < total_chars {
            let byte_pos = self.char_to_byte_index(self.cursor);
            self.buffer.remove(byte_pos);
            self.last_tab_prefix = None;
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_right(&mut self) {
        let total_chars = self.buffer.chars().count();
        if self.cursor < total_chars {
            self.cursor += 1;
        }
    }

    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor = self.buffer.chars().count();
    }

    pub fn submit(&mut self) -> Option<String> {
        let cmd = self.buffer.trim().to_string();
        if !cmd.is_empty() && self.history.last() != Some(&cmd) {
            self.history.push(cmd.clone());
        }
        self.deactivate();
        if cmd.is_empty() {
            None
        } else {
            Some(cmd)
        }
    }

    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let new_idx = match self.history_index {
            None => self.history.len().saturating_sub(1),
            Some(idx) => idx.saturating_sub(1),
        };

        self.history_index = Some(new_idx);
        self.buffer = self.history[new_idx].clone();
        self.cursor = self.buffer.chars().count();
    }

    pub fn history_next(&mut self) {
        if let Some(idx) = self.history_index {
            if idx + 1 < self.history.len() {
                let new_idx = idx + 1;
                self.history_index = Some(new_idx);
                self.buffer = self.history[new_idx].clone();
                self.cursor = self.buffer.chars().count();
            } else {
                self.history_index = None;
                self.buffer.clear();
                self.cursor = 0;
            }
        }
    }

    pub fn complete(&mut self) {
        let prefix = if let Some(p) = &self.last_tab_prefix {
            p.clone()
        } else {
            let p = self.buffer[..self.char_to_byte_index(self.cursor)].to_string();
            self.last_tab_prefix = Some(p.clone());
            self.completion_index = 0;
            p
        };

        let matches: Vec<&String> = self
            .completions
            .iter()
            .filter(|c| c.starts_with(&prefix))
            .collect();

        if matches.is_empty() {
            return;
        }

        let choice = matches[self.completion_index % matches.len()];
        self.completion_index += 1;

        self.buffer = choice.clone();
        self.cursor = self.buffer.chars().count();
    }

    fn char_to_byte_index(&self, char_idx: usize) -> usize {
        self.buffer
            .char_indices()
            .nth(char_idx)
            .map(|(idx, _)| idx)
            .unwrap_or(self.buffer.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_editing() {
        let mut c = Console::new();
        c.activate();
        c.insert_text("hello");
        assert_eq!(c.buffer, "hello");
        assert_eq!(c.cursor, 5);
        c.backspace();
        assert_eq!(c.buffer, "hell");
        c.move_home();
        c.insert_char('a');
        assert_eq!(c.buffer, "ahell");
    }

    #[test]
    fn test_console_history_and_submit() {
        let mut c = Console::new();
        c.activate();
        c.insert_text("next");
        let cmd = c.submit();
        assert_eq!(cmd, Some("next".into()));
        assert!(!c.active);

        c.activate();
        c.history_prev();
        assert_eq!(c.buffer, "next");
    }

    #[test]
    fn test_console_tab_complete() {
        let mut c = Console::new();
        c.activate();
        c.insert_text("qui");
        c.complete();
        assert_eq!(c.buffer, "quit");
    }
}
