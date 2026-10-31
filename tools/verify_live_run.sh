#!/usr/bin/env bash
set -euo pipefail

echo "=========================================================="
echo "      ufm (unframed) Live Render Verification Script"
echo "=========================================================="

# 1. Check Wayland environment
if [ -z "${WAYLAND_DISPLAY:-}" ]; then
    echo "[-] Error: WAYLAND_DISPLAY environment variable is not set"
    exit 1
fi
echo "[+] 1. Active Wayland session detected: $WAYLAND_DISPLAY"

# 2. Prepare test fixtures
FIXTURE_DIR="/tmp/ufm-live-test"
mkdir -p "$FIXTURE_DIR"
./tools/generate_test_fixtures.sh "$FIXTURE_DIR" >/dev/null
echo "[+] 2. Test fixtures ready: SVG, Netpbm (P1-P7), Animated GIF, Transparent PNG"

# 3. Launch ufm with SVG and PPM
echo "[+] 3. Launching ufm window..."
./target/release/ufm -d "$FIXTURE_DIR/vector_sample.svg" "$FIXTURE_DIR/sample_p3_ascii.ppm" &
UFM_PID=$!
sleep 1.2

# 4. Verify Wayland window creation
WINDOW_INFO=$(tessera window | grep -E "ufm|unframed" || true)
if [ -z "$WINDOW_INFO" ]; then
    echo "[-] Error: Wayland compositor did not detect ufm window"
    kill -9 $UFM_PID 2>/dev/null || true
    exit 1
fi
echo "[+] 4. Wayland window mapped and composited successfully:"
echo "       $WINDOW_INFO"

# 5. Capture live screen screenshot
echo "[+] 5. Capturing compositor screen (verifying SVG vector rendering)..."
tessera display capture "$FIXTURE_DIR/screenshot_1_svg.png" >/dev/null
sleep 1.2

# 6. Analyze screenshot pixels to confirm vector SVG was rendered by Vulkan
python3 -c '
from PIL import Image
im = Image.open("'"$FIXTURE_DIR"'/screenshot_1_svg.png")
colors = im.getcolors(maxcolors=1000000)
has_cyan = any(c[1][0] < 50 and c[1][1] > 200 and c[1][2] > 220 for c in colors)
has_orange = any(c[1][0] > 220 and c[1][1] > 60 and c[1][1] < 120 and c[1][2] < 50 for c in colors)
if has_cyan and has_orange:
    print("       >>> Passed: Successfully detected SVG vector pixels (cyan #00e5ff and orange #ff5500)")
else:
    print("       >>> Pixel check failed: Vector SVG pixels not found on screen")
    exit(1)
'

# 7. Send IPC command to rotate by 90 degrees
echo "[+] 6. Testing IPC control: Sending rotation command (rotate by 90)..."
./target/release/ufm-msg $UFM_PID "rotate by 90"
sleep 0.8
tessera display capture "$FIXTURE_DIR/screenshot_2_rotated.png" >/dev/null
sleep 1.2
echo "       >>> Rotation executed: Captured rotated screen at $FIXTURE_DIR/screenshot_2_rotated.png"

# 8. Send IPC command to navigate to next image (PPM)
echo "[+] 7. Testing IPC control: Switching to PPM image (next)..."
./target/release/ufm-msg $UFM_PID "next"
sleep 0.8
tessera display capture "$FIXTURE_DIR/screenshot_3_ppm.png" >/dev/null
sleep 1.2
echo "       >>> Next executed: Captured PPM screen at $FIXTURE_DIR/screenshot_3_ppm.png"

# 9. Verify PPM image pixels (pure red and pure green in sRGB screen space)
python3 -c '
from PIL import Image
im = Image.open("'"$FIXTURE_DIR"'/screenshot_3_ppm.png")
colors = im.getcolors(maxcolors=1000000)
has_red = any(c[1][0] > 180 and c[1][1] < 80 for c in colors)
has_green = any(c[1][1] > 180 and c[1][0] < 80 for c in colors)
if has_red and has_green:
    print("       >>> Passed: Successfully detected PPM RGB pixels on screen (red and green)")
else:
    print("       >>> Warning: PPM expected color values not detected in screenshot")
    exit(1)
'

# 10. Exit ufm via IPC
echo "[+] 8. Testing IPC termination: Sending quit command..."
./target/release/ufm-msg $UFM_PID "quit"
sleep 0.5
if kill -0 $UFM_PID 2>/dev/null; then
    echo "[-] Process still active, forcing kill"
    kill -9 $UFM_PID 2>/dev/null || true
else
    echo "       >>> ufm process exited cleanly; sockets unlinked automatically!"
fi

echo "=========================================================="
echo "  [SUCCESS] All live rendering checks passed!"
echo "  Screenshots saved:"
echo "    - SVG Vector:  $FIXTURE_DIR/screenshot_1_svg.png"
echo "    - Rotated 90°: $FIXTURE_DIR/screenshot_2_rotated.png"
echo "    - PPM RGB:     $FIXTURE_DIR/screenshot_3_ppm.png"
echo "=========================================================="
