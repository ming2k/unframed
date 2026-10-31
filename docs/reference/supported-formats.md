# Supported Image Formats Reference

`ufm` features comprehensive format coverage via dedicated high-performance decoders and rasterizers:

---

## 1. Vector Formats

| Format | Extension | Engine | Capabilities |
| :--- | :--- | :--- | :--- |
| **SVG** | `.svg`, `.svgz` | `resvg` / `usvg` | Crisp resolution-independent rasterization, antialiasing, CSS styling, gradient fills. |

---

## 2. Netpbm Family (PPM / PGM / PBM / PAM)

| Magic | Format | Type | Description |
| :--- | :--- | :--- | :--- |
| **P1** | PBM | ASCII | 1-bit monochrome bitmap (`0` white, `1` black). |
| **P2** | PGM | ASCII | Grayscale up to arbitrary maxval. |
| **P3** | PPM | ASCII | 24-bit / 48-bit RGB color. |
| **P4** | PBM | Binary | 1-bit packed monochrome. |
| **P5** | PGM | Binary | Grayscale 8-bit or 16-bit big-endian. |
| **P6** | PPM | Binary | RGB 8-bit or 16-bit big-endian. |
| **P7** | PAM | Binary | Arbitrary channels (Grayscale, RGB, RGB+Alpha). |

---

## 3. Animated Formats

| Format | Extension | Capabilities |
| :--- | :--- | :--- |
| **GIF** | `.gif` | Multi-frame playback, per-frame delay timing, loop scheduling, pause/resume. |
| **Animated WebP** | `.webp` | Lossy & lossless animation frames, timeline playback. |

---

## 4. Standard & High Dynamic Range Raster Formats

| Format | Extension | Description |
| :--- | :--- | :--- |
| **PNG** | `.png` | 8-bit, 16-bit, grayscale, palette, and truecolor with alpha. |
| **JPEG** | `.jpg`, `.jpeg`, `.jfif` | Baseline and progressive JPEG. |
| **WebP** | `.webp` | VP8 lossy and VP8L lossless static images. |
| **TIFF** | `.tiff`, `.tif` | Uncompressed, LZW, PackBits, Deflate. |
| **BMP** | `.bmp`, `.dib` | Windows Bitmap format. |
| **QOI** | `.qoi` | Quite OK Image format. |
| **Farbfeld** | `.ff` | Suckless 16-bit RGBA image format. |
| **TGA** | `.tga` | Truevision Targa. |
| **ICO** | `.ico` | Windows Icon resource files. |
| **HDR** | `.hdr` | Radiance RGBE high dynamic range. |
| **EXR** | `.exr` | OpenEXR image data. |
