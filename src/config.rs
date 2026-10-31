//! Configuration file parsing for ufm/unframed ini format.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::viewport::{ScalingMode, UpscalingMethod};

#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundMode {
    Solid(u32), // flux_color
    Checks,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub background: BackgroundMode,
    pub fullscreen: bool,
    pub width: i32,
    pub height: i32,
    pub initial_pan_x: f64,
    pub initial_pan_y: f64,
    pub list_files_at_exit: bool,
    pub loop_input: bool,
    pub overlay_enabled: bool,
    pub overlay_font: String,
    pub overlay_text: String,
    pub overlay_text_color: u32,
    pub overlay_bg_color: u32,
    pub overlay_position_bottom: bool,
    pub recursive_load: bool,
    pub scaling_mode: ScalingMode,
    pub slideshow_duration: f64,
    pub suppress_default_binds: bool,
    pub title_text: String,
    pub upscaling_method: UpscalingMethod,

    pub aliases: HashMap<String, String>,
    pub binds: Vec<(String, String)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background: BackgroundMode::Solid(0xFF000000), // black
            fullscreen: false,
            width: 1280,
            height: 720,
            initial_pan_x: 0.5,
            initial_pan_y: 0.5,
            list_files_at_exit: false,
            loop_input: true,
            overlay_enabled: false,
            overlay_font: "Monospace:24".to_string(),
            overlay_text: "[${ufm_current_index}/${ufm_file_count}] [${ufm_width}x${ufm_height}] [${ufm_scale}%] $ufm_current_file [$ufm_scaling_mode]".to_string(),
            overlay_text_color: 0xFFFFFFFF, // white
            overlay_bg_color: 0xC3000000,   // semi-transparent black
            overlay_position_bottom: false,
            recursive_load: false,
            scaling_mode: ScalingMode::Full,
            slideshow_duration: 0.0,
            suppress_default_binds: false,
            title_text: "ufm - [${ufm_current_index}/${ufm_file_count}] [${ufm_width}x${ufm_height}] [${ufm_scale}%] $ufm_current_file [$ufm_scaling_mode]".to_string(),
            upscaling_method: UpscalingMethod::Linear,

            aliases: HashMap::new(),
            binds: Vec::new(),
        }
    }
}

pub fn parse_hex_color(hex: &str, alpha: u8) -> Option<u32> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
    let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
    let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
    Some(pack_flux_color(r, g, b, alpha))
}

pub fn pack_flux_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let a_u = a as u32;
    let r_p = ((r as u32 * a_u + 127) / 255) as u8;
    let g_p = ((g as u32 * a_u + 127) / 255) as u8;
    let b_p = ((b as u32 * a_u + 127) / 255) as u8;
    (a_u << 24) | ((r_p as u32) << 16) | ((g_p as u32) << 8) | (b_p as u32)
}

pub fn find_config_file() -> Option<PathBuf> {
    if let Ok(path) = env::var("ufm_config").or_else(|_| env::var("unframed_config")) {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(p);
        }
    }

    let mut candidates = Vec::new();

    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        candidates.push(PathBuf::from(xdg.clone()).join("ufm/config"));
        candidates.push(PathBuf::from(xdg).join("unframed/config"));
    }

    if let Ok(home) = env::var("HOME") {
        let home_p = PathBuf::from(home);
        candidates.push(home_p.join(".config/ufm/config"));
        candidates.push(home_p.join(".config/unframed/config"));
        candidates.push(home_p.join(".ufm_config"));
        candidates.push(home_p.join(".unframed_config"));
    }

    candidates.push(PathBuf::from("/etc/ufm/config"));
    candidates.push(PathBuf::from("/usr/local/etc/ufm/config"));

    candidates.into_iter().find(|p| p.is_file())
}

impl Config {
    pub fn load_from_file(&mut self, path: &Path) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file {}: {e}", path.display()))?;
        self.parse_ini(&content);
        Ok(())
    }

    pub fn parse_ini(&mut self, content: &str) {
        let mut section = "";

        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].trim();
                continue;
            }

            let Some((key, val)) = line.split_once('=') else {
                continue;
            };

            let key = key.trim();
            let val = val.trim();

            match section {
                "options" => self.parse_option(key, val),
                "aliases" => {
                    self.aliases.insert(key.to_string(), val.to_string());
                }
                "binds" => {
                    self.binds.push((key.to_string(), val.to_string()));
                }
                _ => {}
            }
        }
    }

    fn parse_option(&mut self, key: &str, val: &str) {
        match key {
            "background" => {
                if val.eq_ignore_ascii_case("checks") {
                    self.background = BackgroundMode::Checks;
                } else if let Some(color) = parse_hex_color(val, 255) {
                    self.background = BackgroundMode::Solid(color);
                }
            }
            "fullscreen" => {
                self.fullscreen = parse_bool(val).unwrap_or(self.fullscreen);
            }
            "width" => {
                if let Ok(w) = val.parse::<i32>() {
                    self.width = w;
                }
            }
            "height" => {
                if let Ok(h) = val.parse::<i32>() {
                    self.height = h;
                }
            }
            "initial_pan" => {
                let parts: Vec<&str> = val.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                        self.initial_pan_x = x / 100.0;
                        self.initial_pan_y = y / 100.0;
                    }
                }
            }
            "list_files_at_exit" => {
                self.list_files_at_exit = parse_bool(val).unwrap_or(self.list_files_at_exit);
            }
            "loop_input" => {
                self.loop_input = parse_bool(val).unwrap_or(self.loop_input);
            }
            "overlay" => {
                self.overlay_enabled = parse_bool(val).unwrap_or(self.overlay_enabled);
            }
            "overlay_font" => {
                self.overlay_font = val.to_string();
            }
            "overlay_text" => {
                self.overlay_text = val.to_string();
            }
            "overlay_text_color" => {
                if let Some(c) = parse_hex_color(val, 255) {
                    self.overlay_text_color = c;
                }
            }
            "overlay_background_color" => {
                if let Some(c) = parse_hex_color(val, 200) {
                    self.overlay_bg_color = c;
                }
            }
            "overlay_position_bottom" => {
                self.overlay_position_bottom =
                    parse_bool(val).unwrap_or(self.overlay_position_bottom);
            }
            "recursively" => {
                self.recursive_load = parse_bool(val).unwrap_or(self.recursive_load);
            }
            "scaling_mode" => {
                if let Some(mode) = ScalingMode::parse(val) {
                    self.scaling_mode = mode;
                }
            }
            "slideshow_duration" => {
                if let Ok(d) = val.parse::<f64>() {
                    self.slideshow_duration = d;
                }
            }
            "suppress_default_binds" => {
                self.suppress_default_binds =
                    parse_bool(val).unwrap_or(self.suppress_default_binds);
            }
            "title_text" => {
                self.title_text = val.to_string();
            }
            "upscaling_method" => {
                if let Some(method) = UpscalingMethod::parse(val) {
                    self.upscaling_method = method;
                }
            }
            _ => {}
        }
    }
}

fn parse_bool(s: &str) -> Option<bool> {
    match s.to_ascii_lowercase().as_str() {
        "true" | "yes" | "1" | "on" => Some(true),
        "false" | "no" | "0" | "off" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ini_basic() {
        let ini = r#"
        # Comment line
        [options]
        background = checks
        scaling_mode = crop
        slideshow_duration = 3.5
        overlay = true

        [aliases]
        x = close
        n = next

        [binds]
        q = quit
        <Left> = prev
        "#;
        let mut cfg = Config::default();
        cfg.parse_ini(ini);

        assert_eq!(cfg.background, BackgroundMode::Checks);
        assert_eq!(cfg.scaling_mode, ScalingMode::Crop);
        assert_eq!(cfg.slideshow_duration, 3.5);
        assert!(cfg.overlay_enabled);
        assert_eq!(cfg.aliases.get("x").map(|s| s.as_str()), Some("close"));
        assert_eq!(cfg.binds.len(), 2);
    }
}
