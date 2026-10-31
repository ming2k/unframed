//! Multi-format image loader supporting PNG, JPEG, GIF, WebP, BMP, TIFF, SVG, PPM, Farbfeld, QOI, etc.

use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::time::SystemTime;

use image::codecs::gif::GifDecoder;
use image::codecs::webp::WebPDecoder;
use image::AnimationDecoder;

use crate::ppm;
use crate::svg;

#[derive(Clone)]
pub struct FrameData {
    pub rgba: Vec<u8>,
    pub duration_ms: u32,
}

/// Normalize frame delays conforming to browser standards (HTML5/Chromium/Firefox specs):
/// Zero delay or durations <= 10ms are clamped to 100ms (10 FPS) to prevent CPU burn and strobe.
pub fn normalize_frame_delay(num: u32, denom: u32) -> u32 {
    if denom == 0 || num == 0 {
        100
    } else {
        let ms = num / denom;
        if ms <= 10 {
            100
        } else {
            ms
        }
    }
}

pub struct LoadedImage {
    pub width: u32,
    pub height: u32,
    pub frames: Vec<FrameData>,
    pub current_frame: usize,
    pub frame_timer_ms: f64,
    pub format_name: String,
    pub path: Option<String>,
    pub file_size: u64,
    pub modified: Option<SystemTime>,
}

impl LoadedImage {
    pub fn current_frame_data(&self) -> &FrameData {
        &self.frames[self.current_frame % self.frames.len()]
    }

    pub fn is_animated(&self) -> bool {
        self.frames.len() > 1
    }

    pub fn next_frame(&mut self) {
        if self.frames.len() > 1 {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.frame_timer_ms = 0.0;
        }
    }

    #[allow(dead_code)]
    pub fn prev_frame(&mut self) {
        if self.frames.len() > 1 {
            if self.current_frame == 0 {
                self.current_frame = self.frames.len() - 1;
            } else {
                self.current_frame -= 1;
            }
            self.frame_timer_ms = 0.0;
        }
    }

    /// Advance animation timer by `dt_ms`. Returns true if the frame changed.
    pub fn tick_animation(&mut self, dt_ms: f64) -> bool {
        if self.frames.len() <= 1 {
            return false;
        }

        self.frame_timer_ms += dt_ms;
        let cur_duration = self.current_frame_data().duration_ms.max(10) as f64;
        if self.frame_timer_ms >= cur_duration {
            self.frame_timer_ms -= cur_duration;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            true
        } else {
            false
        }
    }
}

/// Convert straight RGBA8 to premultiplied RGBA8 in place for correct GPU compositing.
pub fn premultiply_rgba(rgba: &mut [u8]) {
    for chunk in rgba.chunks_exact_mut(4) {
        let a = chunk[3] as u32;
        if a == 255 {
            continue;
        } else if a == 0 {
            chunk[0] = 0;
            chunk[1] = 0;
            chunk[2] = 0;
        } else {
            chunk[0] = ((chunk[0] as u32 * a + 127) / 255) as u8;
            chunk[1] = ((chunk[1] as u32 * a + 127) / 255) as u8;
            chunk[2] = ((chunk[2] as u32 * a + 127) / 255) as u8;
        }
    }
}

pub fn load_image_from_path(path_str: &str) -> Result<LoadedImage, String> {
    let path = Path::new(path_str);
    let metadata = fs::metadata(path).map_err(|e| format!("Failed to read metadata for {path_str}: {e}"))?;
    let modified = metadata.modified().ok();
    let file_size = metadata.len();

    let data = fs::read(path).map_err(|e| format!("Failed to read file {path_str}: {e}"))?;
    let mut image = load_image_from_bytes(&data, Some(path_str))?;
    image.path = Some(path_str.to_string());
    image.file_size = file_size;
    image.modified = modified;
    Ok(image)
}

pub fn load_image_from_bytes(data: &[u8], hint_path: Option<&str>) -> Result<LoadedImage, String> {
    if data.is_empty() {
        return Err("Empty image data".to_string());
    }

    let is_svg_path = hint_path.map(|p| p.ends_with(".svg") || p.ends_with(".SVG")).unwrap_or(false);
    if is_svg_path || svg::is_svg(data) {
        if let Some(svg_img) = svg::render_svg(data, None) {
            return Ok(LoadedImage {
                width: svg_img.width,
                height: svg_img.height,
                frames: vec![FrameData {
                    rgba: svg_img.rgba,
                    duration_ms: 0,
                }],
                current_frame: 0,
                frame_timer_ms: 0.0,
                format_name: "SVG".to_string(),
                path: hint_path.map(|s| s.to_string()),
                file_size: data.len() as u64,
                modified: None,
            });
        }
    }

    // Check PPM / PGM / PBM / PAM
    if data.len() >= 2 && data[0] == b'P' && (b'1'..=b'7').contains(&data[1]) {
        if let Some(ppm_img) = ppm::parse_pnm(data) {
            return Ok(LoadedImage {
                width: ppm_img.width,
                height: ppm_img.height,
                frames: vec![FrameData {
                    rgba: ppm_img.rgba,
                    duration_ms: 0,
                }],
                current_frame: 0,
                frame_timer_ms: 0.0,
                format_name: match data[1] {
                    b'1' | b'4' => "PBM",
                    b'2' | b'5' => "PGM",
                    b'3' | b'6' => "PPM",
                    b'7' => "PAM",
                    _ => "PNM",
                }.to_string(),
                path: hint_path.map(|s| s.to_string()),
                file_size: data.len() as u64,
                modified: None,
            });
        }
    }

    // Check GIF for animations
    if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
        if let Ok(decoder) = GifDecoder::new(Cursor::new(data)) {
            if let Ok(raw_frames) = decoder.into_frames().collect_frames() {
                if !raw_frames.is_empty() {
                    let first = &raw_frames[0];
                    let (width, height) = (first.buffer().width(), first.buffer().height());
                    let mut frames = Vec::with_capacity(raw_frames.len());

                    for f in raw_frames {
                        let (num, denom) = f.delay().numer_denom_ms();
                        let delay_ms = normalize_frame_delay(num, denom);
                        let mut rgba = f.into_buffer().into_raw();
                        premultiply_rgba(&mut rgba);
                        frames.push(FrameData {
                            rgba,
                            duration_ms: delay_ms,
                        });
                    }

                    return Ok(LoadedImage {
                        width,
                        height,
                        frames,
                        current_frame: 0,
                        frame_timer_ms: 0.0,
                        format_name: "GIF".to_string(),
                        path: hint_path.map(|s| s.to_string()),
                        file_size: data.len() as u64,
                        modified: None,
                    });
                }
            }
        }
    }

    // Check WebP for animations
    if data.len() >= 12 && data.starts_with(b"RIFF") && &data[8..12] == b"WEBP" {
        if let Ok(decoder) = WebPDecoder::new(Cursor::new(data)) {
            if decoder.has_animation() {
                if let Ok(raw_frames) = decoder.into_frames().collect_frames() {
                    if !raw_frames.is_empty() {
                        let first = &raw_frames[0];
                        let (width, height) = (first.buffer().width(), first.buffer().height());
                        let mut frames = Vec::with_capacity(raw_frames.len());

                        for f in raw_frames {
                            let (num, denom) = f.delay().numer_denom_ms();
                            let delay_ms = normalize_frame_delay(num, denom);
                            let mut rgba = f.into_buffer().into_raw();
                            premultiply_rgba(&mut rgba);
                            frames.push(FrameData {
                                rgba,
                                duration_ms: delay_ms,
                            });
                        }

                        return Ok(LoadedImage {
                            width,
                            height,
                            frames,
                            current_frame: 0,
                            frame_timer_ms: 0.0,
                            format_name: "WebP (animated)".to_string(),
                            path: hint_path.map(|s| s.to_string()),
                            file_size: data.len() as u64,
                            modified: None,
                        });
                    }
                }
            }
        }
    }

    // Standard static decoder via `image` crate (handles PNG, JPEG, BMP, TIFF, WebP static, QOI, Farbfeld, etc.)
    let dynamic_img = image::load_from_memory(data)
        .map_err(|e| format!("Image decode error: {e}"))?;

    let format_name = image::guess_format(data)
        .map(|f| format!("{f:?}"))
        .unwrap_or_else(|_| "Unknown".to_string());

    let (width, height) = (dynamic_img.width(), dynamic_img.height());
    let mut rgba = dynamic_img.to_rgba8().into_raw();
    premultiply_rgba(&mut rgba);

    Ok(LoadedImage {
        width,
        height,
        frames: vec![FrameData {
            rgba,
            duration_ms: 0,
        }],
        current_frame: 0,
        frame_timer_ms: 0.0,
        format_name,
        path: hint_path.map(|s| s.to_string()),
        file_size: data.len() as u64,
        modified: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_png_bytes() {
        // Minimal 1x1 red PNG
        let png_bytes = [
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d,
            0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xde, 0x00, 0x00, 0x00,
            0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8, 0xcf, 0xc0, 0x00,
            0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xdd, 0x8d, 0xb0, 0x00, 0x00, 0x00,
            0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
        ];
        let img = load_image_from_bytes(&png_bytes, None).expect("load png");
        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
        assert_eq!(img.frames.len(), 1);
        assert_eq!(&img.frames[0].rgba[0..3], &[255, 0, 0]);
    }

    #[test]
    fn test_load_svg_bytes() {
        let svg_data = br#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="20">
            <rect width="10" height="20" fill="blue"/>
        </svg>"#;
        let img = load_image_from_bytes(svg_data, Some("test.svg")).expect("load svg");
        assert_eq!(img.width, 10);
        assert_eq!(img.height, 20);
        assert_eq!(img.format_name, "SVG");
    }

    #[test]
    fn test_load_ppm_bytes() {
        let ppm_data = b"P6\n1 1\n255\n\x00\xFF\x00";
        let img = load_image_from_bytes(ppm_data, None).expect("load ppm");
        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
        assert_eq!(&img.frames[0].rgba[0..4], &[0, 255, 0, 255]);
    }

    #[test]
    fn test_sample_image_decoding() {
        // 1x1 24-bit RGB BMP image
        let bmp_data = [
            0x42, 0x4d, 0x3a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00,
            0x28, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x13, 0x0b, 0x00, 0x00,
            0x13, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xff, 0x00,
        ];
        let img = load_image_from_bytes(&bmp_data, Some("test.bmp")).expect("load bmp");
        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
        assert_eq!(&img.frames[0].rgba[0..3], &[255, 0, 0]);
    }

    #[test]
    fn test_normalize_frame_delay() {
        // Zero delay fallback to 100ms
        assert_eq!(normalize_frame_delay(0, 1), 100);
        assert_eq!(normalize_frame_delay(10, 0), 100);
        // <= 10ms clamped to 100ms
        assert_eq!(normalize_frame_delay(5, 1), 100);
        assert_eq!(normalize_frame_delay(10, 1), 100);
        // > 10ms preserved exactly
        assert_eq!(normalize_frame_delay(11, 1), 11);
        assert_eq!(normalize_frame_delay(50, 1), 50);
        assert_eq!(normalize_frame_delay(200, 2), 100);
    }
}
