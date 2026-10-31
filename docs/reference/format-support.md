# Format Support Reference

This document details the image, vector, and animation formats supported by Unframed, including decoding implementations, color representation, and limitations.

---

## 1. Format Matrix

| Format | Extensions | Decoder Engine | Animated | Max Bit Depth |
| :--- | :--- | :--- | :--- | :--- |
| **SVG** | `.svg`, `.svgz` | `resvg` / `usvg` | No | Arbitrary Vector |
| **PPM / PNM** | `.pbm`, `.pgm`, `.ppm`, `.pnm`, `.pam` | Custom parser (`src/ppm.rs`) | No | 1, 8, 16-bit |
| **PNG** | `.png` | `image::codecs::png` | No | 8, 16-bit RGBA |
| **JPEG** | `.jpg`, `.jpeg` | `image::codecs::jpeg` | No | 8-bit YCbCr/RGB |
| **GIF** | `.gif` | `image::codecs::gif` | Yes (multi-frame) | 8-bit Palette |
| **WebP** | `.webp` | `image::codecs::webp` | Yes (multi-frame) | 8-bit RGBA |
| **BMP** | `.bmp` | `image::codecs::bmp` | No | 1, 4, 8, 24, 32-bit |
| **TIFF** | `.tif`, `.tiff` | `image::codecs::tiff` | No | 8, 16, 32-bit |
| **QOI** | `.qoi` | `image::codecs::qoi` | No | 8-bit RGBA |
| **Farbfeld**| `.ff` | `image::codecs::farbfeld` | No | 16-bit RGBA |
| **TGA** | `.tga` | `image::codecs::tga` | No | 8, 15, 16, 24, 32-bit|
| **ICO** | `.ico` | `image::codecs::ico` | No | 8-bit RGBA |
| **HDR** | `.hdr` | `image::codecs::hdr` | No | 32-bit Float |

---

## 2. Vector Graphics (SVG)

Unframed provides first-class SVG support powered by `resvg`:
- **Vector Fidelity**: SVG documents are dynamically rasterized into RGBA pixel buffers matching the natural viewBox or target window bounds.
- **Standards Compliance**: Supports SVG 1.1 Full, SVG Tiny, CSS styling, gradients, clipping paths, embedded fonts, and embedded bitmap elements.
- **Arbitrary Resolution**: Scaling vector graphics does not introduce bilinear blur or jagged raster artifacts.

---

## 3. Netpbm Suite (`src/ppm.rs`)

Unframed features a dedicated, zero-dependency Netpbm parser that supports all standard magic numbers:

| Magic | Format | Type | Channels | Representation |
| :--- | :--- | :--- | :--- | :--- |
| `P1` | PBM | ASCII | 1-bit | `1` = black, `0` = white |
| `P2` | PGM | ASCII | Grayscale | Decimal numbers up to `maxval` |
| `P3` | PPM | ASCII | RGB | Decimal triplets `R G B` |
| `P4` | PBM | Binary | 1-bit | Bit-packed bytes (MSB first) |
| `P5` | PGM | Binary | Grayscale | 8-bit or 16-bit raw bytes |
| `P6` | PPM | Binary | RGB | 8-bit or 16-bit interleaved `RGB` |
| `P7` | PAM | Binary | Any | Arbitrary channels (`BLACKANDWHITE`, `GRAYSCALE`, `RGB`, `RGB_ALPHA`) |

### Comment & Whitespace Handling
The parser conforms strictly to POSIX Netpbm semantics:
- Comments beginning with `#` and continuing to the next newline are stripped at any point in the header sequence.
- Arbitrary mixtures of spaces, tabs, carriage returns, and newlines are valid token delimiters.

---

## 4. Multi-Frame Animated Images

### GIF (`image::codecs::gif`)
- Frame streams are unpacked into individual RGBA frames on load.
- Frame delay metadata (`delay_ms`) is preserved and clamped to a minimum duration of 10ms to prevent CPU saturation from zero-delay timing loops.
- Supports disposal methods and transparency indices.

### WebP Animations (`image::codecs::webp`)
- Multi-frame animated WebP sequences are unpacked into discrete frames.
- Interframe timing is respected during playback.

---

## 5. Color Spaces & GPU Presentation

All decoded pixel data is normalized to **8-bit RGBA (Premultiplied)** format before being uploaded to GPU memory via `flux::Image`:
- Linear color packing ensures hardware-accelerated Vulkan sampling accurately blends translucent edges with background fills.
- Non-standard channels (e.g. 1-bit monochrome or 16-bit grayscale) are automatically mapped to 32-bit RGBA during the decoding step.
