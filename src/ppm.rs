//! PPM / PGM / PBM / PAM image parser.
//!
//! Supports:
//! - P1: ASCII 1-bit bitmap (PBM)
//! - P2: ASCII greyscale (PGM)
//! - P3: ASCII RGB (PPM)
//! - P4: Binary 1-bit bitmap (PBM)
//! - P5: Binary greyscale (PGM)
//! - P6: Binary RGB (PPM)
//! - P7: Binary PAM (Portable Arbitrary Map)

pub struct PpmImage {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Parse PPM/PGM/PBM/PAM data from memory into 32-bit RGBA pixels.
pub fn parse_pnm(data: &[u8]) -> Option<PpmImage> {
    if data.len() < 3 || data[0] != b'P' {
        return None;
    }

    let magic = data[1];
    match magic {
        b'1' => parse_p1(data),
        b'2' => parse_p2(data),
        b'3' => parse_p3(data),
        b'4' => parse_p4(data),
        b'5' => parse_p5(data),
        b'6' => parse_p6(data),
        b'7' => parse_pam(data),
        _ => None,
    }
}

fn skip_whitespace_and_comments(data: &[u8], mut pos: usize) -> usize {
    while pos < data.len() {
        let b = data[pos];
        if b.is_ascii_whitespace() {
            pos += 1;
        } else if b == b'#' {
            pos += 1;
            while pos < data.len() && data[pos] != b'\n' {
                pos += 1;
            }
        } else {
            break;
        }
    }
    pos
}

fn read_uint(data: &[u8], pos: &mut usize) -> Option<u32> {
    *pos = skip_whitespace_and_comments(data, *pos);
    if *pos >= data.len() || !data[*pos].is_ascii_digit() {
        return None;
    }
    let mut val: u32 = 0;
    while *pos < data.len() && data[*pos].is_ascii_digit() {
        val = val.checked_mul(10)?.checked_add((data[*pos] - b'0') as u32)?;
        *pos += 1;
    }
    Some(val)
}

/// P6: Binary PPM (24-bit RGB, or 48-bit RGB if maxval > 255)
fn parse_p6(data: &[u8]) -> Option<PpmImage> {
    let mut pos = 2;
    let width = read_uint(data, &mut pos)?;
    let height = read_uint(data, &mut pos)?;
    let maxval = read_uint(data, &mut pos)?;

    if width == 0 || height == 0 || maxval == 0 {
        return None;
    }

    // A single whitespace character separates header from binary data
    if pos >= data.len() || !data[pos].is_ascii_whitespace() {
        return None;
    }
    pos += 1;

    let total_pixels = (width as usize).checked_mul(height as usize)?;
    let mut rgba = Vec::with_capacity(total_pixels * 4);

    if maxval <= 255 {
        let expected_bytes = total_pixels.checked_mul(3)?;
        if data.len() - pos < expected_bytes {
            return None;
        }
        let src = &data[pos..pos + expected_bytes];
        for chunk in src.chunks_exact(3) {
            let r = if maxval == 255 { chunk[0] } else { ((chunk[0] as u32 * 255) / maxval) as u8 };
            let g = if maxval == 255 { chunk[1] } else { ((chunk[1] as u32 * 255) / maxval) as u8 };
            let b = if maxval == 255 { chunk[2] } else { ((chunk[2] as u32 * 255) / maxval) as u8 };
            rgba.extend_from_slice(&[r, g, b, 255]);
        }
    } else {
        // 16-bit per channel big-endian
        let expected_bytes = total_pixels.checked_mul(6)?;
        if data.len() - pos < expected_bytes {
            return None;
        }
        let src = &data[pos..pos + expected_bytes];
        for chunk in src.chunks_exact(6) {
            let r16 = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
            let g16 = u16::from_be_bytes([chunk[2], chunk[3]]) as u32;
            let b16 = u16::from_be_bytes([chunk[4], chunk[5]]) as u32;
            let r = ((r16 * 255) / maxval) as u8;
            let g = ((g16 * 255) / maxval) as u8;
            let b = ((b16 * 255) / maxval) as u8;
            rgba.extend_from_slice(&[r, g, b, 255]);
        }
    }

    Some(PpmImage { width, height, rgba })
}

/// P3: ASCII PPM (RGB text numbers)
fn parse_p3(data: &[u8]) -> Option<PpmImage> {
    let mut pos = 2;
    let width = read_uint(data, &mut pos)?;
    let height = read_uint(data, &mut pos)?;
    let maxval = read_uint(data, &mut pos)?;

    if width == 0 || height == 0 || maxval == 0 {
        return None;
    }

    let total_pixels = (width as usize).checked_mul(height as usize)?;
    let mut rgba = Vec::with_capacity(total_pixels * 4);

    for _ in 0..total_pixels {
        let r_val = read_uint(data, &mut pos)?;
        let g_val = read_uint(data, &mut pos)?;
        let b_val = read_uint(data, &mut pos)?;

        let r = ((r_val.min(maxval) * 255) / maxval) as u8;
        let g = ((g_val.min(maxval) * 255) / maxval) as u8;
        let b = ((b_val.min(maxval) * 255) / maxval) as u8;
        rgba.extend_from_slice(&[r, g, b, 255]);
    }

    Some(PpmImage { width, height, rgba })
}

/// P5: Binary PGM (Greyscale)
fn parse_p5(data: &[u8]) -> Option<PpmImage> {
    let mut pos = 2;
    let width = read_uint(data, &mut pos)?;
    let height = read_uint(data, &mut pos)?;
    let maxval = read_uint(data, &mut pos)?;

    if width == 0 || height == 0 || maxval == 0 {
        return None;
    }

    if pos >= data.len() || !data[pos].is_ascii_whitespace() {
        return None;
    }
    pos += 1;

    let total_pixels = (width as usize).checked_mul(height as usize)?;
    let mut rgba = Vec::with_capacity(total_pixels * 4);

    if maxval <= 255 {
        if data.len() - pos < total_pixels {
            return None;
        }
        for &byte in &data[pos..pos + total_pixels] {
            let v = if maxval == 255 { byte } else { ((byte as u32 * 255) / maxval) as u8 };
            rgba.extend_from_slice(&[v, v, v, 255]);
        }
    } else {
        let expected_bytes = total_pixels.checked_mul(2)?;
        if data.len() - pos < expected_bytes {
            return None;
        }
        let src = &data[pos..pos + expected_bytes];
        for chunk in src.chunks_exact(2) {
            let v16 = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
            let v = ((v16 * 255) / maxval) as u8;
            rgba.extend_from_slice(&[v, v, v, 255]);
        }
    }

    Some(PpmImage { width, height, rgba })
}

/// P2: ASCII PGM (Greyscale)
fn parse_p2(data: &[u8]) -> Option<PpmImage> {
    let mut pos = 2;
    let width = read_uint(data, &mut pos)?;
    let height = read_uint(data, &mut pos)?;
    let maxval = read_uint(data, &mut pos)?;

    if width == 0 || height == 0 || maxval == 0 {
        return None;
    }

    let total_pixels = (width as usize).checked_mul(height as usize)?;
    let mut rgba = Vec::with_capacity(total_pixels * 4);

    for _ in 0..total_pixels {
        let val = read_uint(data, &mut pos)?;
        let v = ((val.min(maxval) * 255) / maxval) as u8;
        rgba.extend_from_slice(&[v, v, v, 255]);
    }

    Some(PpmImage { width, height, rgba })
}

/// P4: Binary PBM (1 bit per pixel, 1 = black, 0 = white)
fn parse_p4(data: &[u8]) -> Option<PpmImage> {
    let mut pos = 2;
    let width = read_uint(data, &mut pos)?;
    let height = read_uint(data, &mut pos)?;

    if width == 0 || height == 0 {
        return None;
    }

    if pos >= data.len() || !data[pos].is_ascii_whitespace() {
        return None;
    }
    pos += 1;

    let row_bytes = (width as usize).div_ceil(8);
    let total_bytes = row_bytes.checked_mul(height as usize)?;
    if data.len() - pos < total_bytes {
        return None;
    }

    let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
    for y in 0..height as usize {
        let row_start = pos + y * row_bytes;
        for x in 0..width as usize {
            let byte_idx = row_start + (x / 8);
            let bit_idx = 7 - (x % 8);
            let is_black = ((data[byte_idx] >> bit_idx) & 1) == 1;
            let v = if is_black { 0 } else { 255 };
            rgba.extend_from_slice(&[v, v, v, 255]);
        }
    }

    Some(PpmImage { width, height, rgba })
}

/// P1: ASCII PBM (0 = white, 1 = black)
fn parse_p1(data: &[u8]) -> Option<PpmImage> {
    let mut pos = 2;
    let width = read_uint(data, &mut pos)?;
    let height = read_uint(data, &mut pos)?;

    if width == 0 || height == 0 {
        return None;
    }

    let total_pixels = (width as usize).checked_mul(height as usize)?;
    let mut rgba = Vec::with_capacity(total_pixels * 4);

    for _ in 0..total_pixels {
        let bit = read_uint(data, &mut pos)?;
        let v = if bit == 1 { 0 } else { 255 };
        rgba.extend_from_slice(&[v, v, v, 255]);
    }

    Some(PpmImage { width, height, rgba })
}

/// P7: Binary PAM (Portable Arbitrary Map)
fn parse_pam(data: &[u8]) -> Option<PpmImage> {
    let end_header = data.windows(7).position(|w| w == b"ENDHDR\n")?;
    let header_str = std::str::from_utf8(&data[..end_header]).ok()?;
    let data_start = end_header + 7;

    let mut width = 0u32;
    let mut height = 0u32;
    let mut depth = 0u32;
    let mut maxval = 0u32;

    for line in header_str.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            match parts[0] {
                "WIDTH" => width = parts[1].parse().ok()?,
                "HEIGHT" => height = parts[1].parse().ok()?,
                "DEPTH" => depth = parts[1].parse().ok()?,
                "MAXVAL" => maxval = parts[1].parse().ok()?,
                _ => {}
            }
        }
    }

    if width == 0 || height == 0 || depth == 0 || maxval == 0 {
        return None;
    }

    let total_pixels = (width as usize).checked_mul(height as usize)?;
    let mut rgba = Vec::with_capacity(total_pixels * 4);
    let bytes_per_sample = if maxval > 255 { 2 } else { 1 };
    let bytes_per_pixel = depth as usize * bytes_per_sample;
    let expected_bytes = total_pixels.checked_mul(bytes_per_pixel)?;

    if data.len() - data_start < expected_bytes {
        return None;
    }

    let src = &data[data_start..data_start + expected_bytes];
    for chunk in src.chunks_exact(bytes_per_pixel) {
        let mut r = 0u8;
        let mut g = 0u8;
        let mut b = 0u8;
        let mut a = 255u8;

        let sample = |i: usize| -> u8 {
            if bytes_per_sample == 1 {
                ((chunk[i] as u32 * 255) / maxval) as u8
            } else {
                let v16 = u16::from_be_bytes([chunk[i * 2], chunk[i * 2 + 1]]) as u32;
                ((v16 * 255) / maxval) as u8
            }
        };

        match depth {
            1 => {
                let grey = sample(0);
                r = grey;
                g = grey;
                b = grey;
            }
            2 => {
                let grey = sample(0);
                r = grey;
                g = grey;
                b = grey;
                a = sample(1);
            }
            3 => {
                r = sample(0);
                g = sample(1);
                b = sample(2);
            }
            4 => {
                r = sample(0);
                g = sample(1);
                b = sample(2);
                a = sample(3);
            }
            _ => {}
        }
        rgba.extend_from_slice(&[r, g, b, a]);
    }

    Some(PpmImage { width, height, rgba })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p6_binary_ppm() {
        let data = b"P6\n# a test comment\n2 2\n255\n\xFF\x00\x00\x00\xFF\x00\x00\x00\xFF\xFF\xFF\xFF";
        let img = parse_pnm(data).expect("parse p6");
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 2);
        assert_eq!(img.rgba.len(), 16);
        // Red
        assert_eq!(&img.rgba[0..4], &[255, 0, 0, 255]);
        // Green
        assert_eq!(&img.rgba[4..8], &[0, 255, 0, 255]);
        // Blue
        assert_eq!(&img.rgba[8..12], &[0, 0, 255, 255]);
        // White
        assert_eq!(&img.rgba[12..16], &[255, 255, 255, 255]);
    }

    #[test]
    fn test_p3_ascii_ppm() {
        let data = b"P3\n2 1\n255\n255 128 64 0 0 0\n";
        let img = parse_pnm(data).expect("parse p3");
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 1);
        assert_eq!(&img.rgba[0..4], &[255, 128, 64, 255]);
        assert_eq!(&img.rgba[4..8], &[0, 0, 0, 255]);
    }

    #[test]
    fn test_p5_binary_pgm() {
        let data = b"P5\n2 1\n255\n\x00\xFF";
        let img = parse_pnm(data).expect("parse p5");
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 1);
        assert_eq!(&img.rgba[0..4], &[0, 0, 0, 255]);
        assert_eq!(&img.rgba[4..8], &[255, 255, 255, 255]);
    }

    #[test]
    fn test_p4_binary_pbm() {
        let data = b"P4\n8 1\n\x81"; // 10000001 in binary -> pixel 0 black, pixels 1..6 white, pixel 7 black
        let img = parse_pnm(data).expect("parse p4");
        assert_eq!(img.width, 8);
        assert_eq!(img.height, 1);
        assert_eq!(&img.rgba[0..4], &[0, 0, 0, 255]); // black
        assert_eq!(&img.rgba[4..8], &[255, 255, 255, 255]); // white
        assert_eq!(&img.rgba[28..32], &[0, 0, 0, 255]); // black
    }
}
