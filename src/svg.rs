//! SVG rendering support via resvg and usvg.

pub struct SvgImage {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Detect if raw bytes start like an SVG file (XML tag or <svg).
pub fn is_svg(data: &[u8]) -> bool {
    let limit = data.len().min(1024);
    if let Ok(s) = std::str::from_utf8(&data[..limit]) {
        let s_lower = s.to_ascii_lowercase();
        s_lower.contains("<svg")
    } else {
        false
    }
}

/// Parse and render SVG data to RGBA pixels.
///
/// An optional target size can be specified; if None, the SVG's intrinsic dimensions are used.
pub fn render_svg(data: &[u8], target_size: Option<(u32, u32)>) -> Option<SvgImage> {
    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(data, &opt).ok()?;

    let intrinsic = tree.size().to_int_size();
    let (mut width, mut height) = (intrinsic.width(), intrinsic.height());

    if width == 0 || height == 0 {
        width = 800;
        height = 600;
    }

    let transform = match target_size {
        Some((tw, th)) if tw > 0 && th > 0 => {
            let sx = tw as f32 / width as f32;
            let sy = th as f32 / height as f32;
            width = tw;
            height = th;
            resvg::tiny_skia::Transform::from_scale(sx, sy)
        }
        _ => resvg::tiny_skia::Transform::default(),
    };

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)?;
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Some(SvgImage {
        width,
        height,
        rgba: pixmap.take(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_svg() {
        assert!(is_svg(b"<?xml version=\"1.0\"?><svg viewBox=\"0 0 10 10\"></svg>"));
        assert!(is_svg(b"  \n<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>"));
        assert!(!is_svg(b"\x89PNG\r\n\x1a\n\x00\x00\x00"));
    }

    #[test]
    fn test_render_svg_basic() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="32">
            <rect width="64" height="32" fill="#ff0000"/>
        </svg>"##;
        let img = render_svg(svg, None).expect("render svg");
        assert_eq!(img.width, 64);
        assert_eq!(img.height, 32);
        assert_eq!(img.rgba.len(), 64 * 32 * 4);
        // Red color in RGBA
        assert_eq!(&img.rgba[0..4], &[255, 0, 0, 255]);
    }

    #[test]
    fn test_render_svg_scaled() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="50" height="50">
            <circle cx="25" cy="25" r="25" fill="#00ff00"/>
        </svg>"##;
        let img = render_svg(svg, Some((100, 100))).expect("render scaled svg");
        assert_eq!(img.width, 100);
        assert_eq!(img.height, 100);
        assert_eq!(img.rgba.len(), 100 * 100 * 4);
    }
}
