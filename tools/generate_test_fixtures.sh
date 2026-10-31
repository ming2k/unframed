#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="${1:-/tmp/ufm-fixtures}"
mkdir -p "$TARGET_DIR"

echo "Generating test fixtures in: $TARGET_DIR"

# 1. Vector SVG (with vector shapes, gradient, transparency) - 800x600
cat << 'EOF' > "$TARGET_DIR/vector_sample.svg"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 300" width="800" height="600">
  <defs>
    <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#ff5500;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#8800ff;stop-opacity:1" />
    </linearGradient>
  </defs>
  <rect width="100%" height="100%" fill="none" />
  <circle cx="200" cy="150" r="120" fill="url(#grad)" />
  <rect x="100" y="80" width="200" height="140" rx="20" fill="#00e5ff" opacity="0.85" />
  <text x="200" y="160" font-size="28" font-family="sans-serif" font-weight="bold" fill="#ffffff" text-anchor="middle">ufm Wayland</text>
</svg>
EOF

# 2. Netpbm P3 (ASCII PPM - 200x150 with pure red top half and pure green bottom half)
python3 -c '
w, h = 200, 150
with open("'"$TARGET_DIR"'/sample_p3_ascii.ppm", "w") as f:
    f.write(f"P3\n# 200x150 RGB color block\n{w} {h}\n255\n")
    for y in range(h):
        for x in range(w):
            if y < h // 2:
                f.write("255 0 0 ")
            else:
                f.write("0 255 0 ")
        f.write("\n")
'

# 3. Netpbm P6 (Binary PPM - 200x150 pure blue)
python3 -c '
w, h = 200, 150
header = f"P6\n{w} {h}\n255\n".encode("ascii")
data = bytearray(header)
for _ in range(w * h):
    data.extend([0, 0, 255])
with open("'"$TARGET_DIR"'/sample_p6_binary.ppm", "wb") as f:
    f.write(data)
'

# 4. Netpbm P1 (ASCII PBM - 20x20 Checkerboard)
python3 -c '
w, h = 20, 20
with open("'"$TARGET_DIR"'/sample_p1_ascii.pbm", "w") as f:
    f.write(f"P1\n# 20x20 checkerboard\n{w} {h}\n")
    for y in range(h):
        for x in range(w):
            f.write(f"{(x // 4 + y // 4) % 2} ")
        f.write("\n")
'

# 5. Netpbm P2 (ASCII PGM - 64x64 Grayscale ramp)
python3 -c '
w, h = 64, 64
with open("'"$TARGET_DIR"'/sample_p2_ascii.pgm", "w") as f:
    f.write(f"P2\n{w} {h}\n255\n")
    for y in range(h):
        for x in range(w):
            f.write(f"{(x * 255) // w} ")
        f.write("\n")
'

# 6. Netpbm P4 (Binary PBM)
printf 'P4\n8 2\n\xaa\x55' > "$TARGET_DIR/sample_p4_binary.pbm"

# 7. Netpbm P5 (Binary PGM - 64x64 Grayscale)
python3 -c '
w, h = 64, 64
header = f"P5\n{w} {h}\n255\n".encode("ascii")
data = bytearray(header)
for y in range(h):
    for x in range(w):
        data.append((y * 255) // h)
with open("'"$TARGET_DIR"'/sample_p5_binary.pgm", "wb") as f:
    f.write(data)
'

# 8. Netpbm P7 (Binary PAM with Alpha channel - 100x100)
python3 -c '
w, h = 100, 100
header = f"P7\nWIDTH {w}\nHEIGHT {h}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n".encode("ascii")
data = bytearray(header)
for y in range(h):
    for x in range(w):
        data.extend([255, 255, 0, 200]) # Yellow with 200 alpha
with open("'"$TARGET_DIR"'/sample_p7_alpha.pam", "wb") as f:
    f.write(data)
'

# 9. Minimal Transparent PNG
printf '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x02\x00\x00\x00\x02\x08\x06\x00\x00\x00v\xb7\xcb\xa3\x00\x00\x00\x1aIDATx\x9cc\xf8\xcf\xc0\x00\x02#\x18\x80A\x0c\xc0 \x06\x06\xa6\x01\x009\xe0\x03\xb1\xc6\xee\xd9\xc0\x00\x00\x00\x00IEND\xaeB`\x82' > "$TARGET_DIR/sample_transparent.png"

# 10. Minimal Animated GIF (2 frames, 50ms delay)
printf '\x47\x49\x46\x38\x39\x61\x02\x00\x02\x00\xf0\x00\x00\xff\x00\x00\x00\x00\xff\x21\xf9\x04\x00\x05\x00\x00\x00\x2c\x00\x00\x00\x00\x02\x00\x02\x00\x00\x02\x02\x44\x01\x00\x21\xf9\x04\x00\x05\x00\x00\x00\x2c\x00\x00\x00\x00\x02\x00\x02\x00\x00\x02\x02\x4c\x01\x00\x3b' > "$TARGET_DIR/sample_animated.gif"

chmod -R a+r "$TARGET_DIR"
echo "Generated fixtures in $TARGET_DIR:"
ls -lh "$TARGET_DIR"
