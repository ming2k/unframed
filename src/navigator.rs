//! Navigator manages the playlist of image paths, directory traversal, and navigation.

use std::fs;
use std::path::{Path, PathBuf};

pub struct Navigator {
    paths: Vec<String>,
    current_index: usize,
    pub loop_input: bool,
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            current_index: 0,
            loop_input: true,
        }
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    pub fn index(&self) -> usize {
        self.current_index
    }

    pub fn selection(&self) -> Option<&str> {
        if self.current_index < self.paths.len() {
            Some(&self.paths[self.current_index])
        } else {
            None
        }
    }

    pub fn all_paths(&self) -> &[String] {
        &self.paths
    }

    pub fn add(&mut self, path_str: &str, recursive: bool) {
        if path_str == "-" {
            self.paths.push("-".to_string());
            return;
        }

        let path = Path::new(path_str);
        if path.is_dir() {
            let mut found = Vec::new();
            scan_dir(path, recursive, &mut found);
            found.sort();
            for f in found {
                let s = f.to_string_lossy().to_string();
                if !self.paths.contains(&s) {
                    self.paths.push(s);
                }
            }
        } else {
            let s = path_str.to_string();
            if !self.paths.contains(&s) {
                self.paths.push(s);
            }
        }
    }

    pub fn select_rel(&mut self, offset: isize) -> Option<&str> {
        if self.paths.is_empty() {
            return None;
        }

        let len = self.paths.len() as isize;
        let mut new_idx = self.current_index as isize + offset;

        if self.loop_input {
            new_idx = ((new_idx % len) + len) % len;
        } else {
            new_idx = new_idx.clamp(0, len - 1);
        }

        self.current_index = new_idx as usize;
        self.selection()
    }

    pub fn select_abs(&mut self, index: isize) -> Option<&str> {
        if self.paths.is_empty() {
            return None;
        }

        let len = self.paths.len() as isize;
        let mut target = if index < 0 {
            len + index
        } else if index > 0 {
            index - 1 // 1-based index to 0-based
        } else {
            0
        };

        target = target.clamp(0, len - 1);
        self.current_index = target as usize;
        self.selection()
    }

    pub fn remove_at(&mut self, index: usize) -> Option<&str> {
        if index >= self.paths.len() {
            return None;
        }

        self.paths.remove(index);
        if self.paths.is_empty() {
            self.current_index = 0;
            None
        } else {
            if self.current_index >= self.paths.len() {
                self.current_index = self.paths.len() - 1;
            }
            self.selection()
        }
    }

    pub fn remove_current(&mut self) -> Option<&str> {
        self.remove_at(self.current_index)
    }

    pub fn remove_all(&mut self) {
        self.paths.clear();
        self.current_index = 0;
    }

    #[allow(dead_code)]
    pub fn remove_path(&mut self, path: &str) {
        if let Some(pos) = self.paths.iter().position(|p| p == path) {
            self.remove_at(pos);
        }
    }

    pub fn find_path(&self, path: &str) -> Option<usize> {
        self.paths.iter().position(|p| p == path)
    }
}

fn scan_dir(dir: &Path, recursive: bool, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            if recursive {
                scan_dir(&p, true, out);
            }
        } else if is_supported_image_extension(&p) {
            out.push(p);
        }
    }
}

pub fn is_supported_image_extension(p: &Path) -> bool {
    let Some(ext) = p.extension().and_then(|s| s.to_str()) else {
        return false;
    };
    let ext_lower = ext.to_ascii_lowercase();
    matches!(
        ext_lower.as_str(),
        "png"
            | "jpg"
            | "jpeg"
            | "jpe"
            | "jfif"
            | "gif"
            | "webp"
            | "bmp"
            | "dib"
            | "tiff"
            | "tif"
            | "svg"
            | "svgz"
            | "ppm"
            | "pgm"
            | "pbm"
            | "pnm"
            | "pam"
            | "qoi"
            | "ff"
            | "farbfeld"
            | "tga"
            | "ico"
            | "hdr"
            | "exr"
            | "avif"
            | "heic"
            | "heif"
            | "jxl"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigator_rel_navigation() {
        let mut nav = Navigator::new();
        nav.paths = vec!["a.png".into(), "b.jpg".into(), "c.svg".into()];
        assert_eq!(nav.selection(), Some("a.png"));
        assert_eq!(nav.select_rel(1), Some("b.jpg"));
        assert_eq!(nav.select_rel(1), Some("c.svg"));
        assert_eq!(nav.select_rel(1), Some("a.png")); // wrapped
        assert_eq!(nav.select_rel(-1), Some("c.svg")); // wrapped backwards
    }

    #[test]
    fn test_navigator_no_loop() {
        let mut nav = Navigator::new();
        nav.loop_input = false;
        nav.paths = vec!["a.png".into(), "b.jpg".into()];
        assert_eq!(nav.select_rel(1), Some("b.jpg"));
        assert_eq!(nav.select_rel(1), Some("b.jpg")); // clamped
        assert_eq!(nav.select_rel(-1), Some("a.png"));
        assert_eq!(nav.select_rel(-1), Some("a.png")); // clamped
    }

    #[test]
    fn test_navigator_abs_navigation() {
        let mut nav = Navigator::new();
        nav.paths = vec!["1.png".into(), "2.png".into(), "3.png".into()];
        assert_eq!(nav.select_abs(1), Some("1.png"));
        assert_eq!(nav.select_abs(3), Some("3.png"));
        assert_eq!(nav.select_abs(-1), Some("3.png")); // last
        assert_eq!(nav.select_abs(-2), Some("2.png")); // second to last
    }

    #[test]
    fn test_navigator_remove() {
        let mut nav = Navigator::new();
        nav.paths = vec!["a.png".into(), "b.png".into()];
        nav.current_index = 1;
        assert_eq!(nav.remove_current(), Some("a.png"));
        assert_eq!(nav.len(), 1);
        assert_eq!(nav.remove_current(), None);
        assert_eq!(nav.len(), 0);
    }
}
