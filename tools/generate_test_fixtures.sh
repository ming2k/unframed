#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="${1:-/tmp/ufm-fixtures}"
mkdir -p "$TARGET_DIR"

echo "Generating test fixtures in: $TARGET_DIR"

# 1. Vector SVG (with vector shapes, gradient, transparency)
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

# 2. Netpbm P1 (ASCII PBM - Monochrome)
cat << 'EOF' > "$TARGET_DIR/sample_p1_ascii.pbm"
P1
# 6x6 Checkerboard
6 6
1 0 1 0 1 0
0 1 0 1 0 1
1 0 1 0 1 0
0 1 0 1 0 1
1 0 1 0 1 0
0 1 0 1 0 1
EOF

# 3. Netpbm P2 (ASCII PGM - Grayscale)
cat << 'EOF' > "$TARGET_DIR/sample_p2_ascii.pgm"
P2
# 4x4 Grayscale ramp
4 4
255
0   64  128 255
64  128 255 128
128 255 128 64
255 128 64  0
EOF

# 4. Netpbm P3 (ASCII PPM - RGB)
cat << 'EOF' > "$TARGET_DIR/sample_p3_ascii.ppm"
P3
# 3x2 RGB colors
3 2
255
255 0 0    0 255 0    0 0 255
255 255 0  0 255 255  255 0 255
EOF

# 5. Netpbm P4 (Binary PBM)
printf 'P4\n8 2\n\xaa\x55' > "$TARGET_DIR/sample_p4_binary.pbm"

# 6. Netpbm P5 (Binary PGM)
printf 'P5\n4 2\n255\n\x00\x55\xaa\xff\xff\xaa\x55\x00' > "$TARGET_DIR/sample_p5_binary.pgm"

# 7. Netpbm P6 (Binary PPM)
printf 'P6\n2 2\n255\n\xff\x00\x00\x00\xff\x00\x00\x00\xff\xff\xff\xff' > "$TARGET_DIR/sample_p6_binary.ppm"

# 8. Netpbm P7 (Binary PAM with Alpha channel)
printf 'P7\nWIDTH 2\nHEIGHT 2\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n\xff\x00\x00\xff\x00\xff\x00\x80\x00\x00\xff\x40\xff\xff\x00\xff' > "$TARGET_DIR/sample_p7_alpha.pam"

# 9. Minimal Transparent PNG
printf '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x02\x00\x00\x00\x02\x08\x06\x00\x00\x00v\xb7\xcb\xa3\x00\x00\x00\x1aIDATx\x9cc\xf8\xcf\xc0\x00\x02#\x18\x80A\x0c\xc0 \x06\x06\xa6\x01\x009\xe0\x03\xb1\xc6\xee\xd9\xc0\x00\x00\x00\x00IEND\xaeB`\x82' > "$TARGET_DIR/sample_transparent.png"

# 10. Minimal Animated GIF (2 frames, 50ms delay)
printf '\x47\x49\x46\x38\x39\x61\x02\x00\x02\x00\xf0\x00\x00\xff\x00\x00\x00\x00\xff\x21\xf9\x04\x00\x05\x00\x00\x00\x2c\x00\x00\x00\x00\x02\x00\x02\x00\x00\x02\x02\x44\x01\x00\x21\xf9\x04\x00\x05\x00\x00\x00\x2c\x00\x00\x00\x00\x02\x00\x02\x00\x00\x02\x02\x4c\x01\x00\x3b' > "$TARGET_DIR/sample_animated.gif"

chmod -R a+r "$TARGET_DIR"
echo "Generated fixtures in $TARGET_DIR:"
ls -lh "$TARGET_DIR"
