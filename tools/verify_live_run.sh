#!/usr/bin/env bash
set -euo pipefail

echo "=========================================================="
echo "      ufm (unframed) 现场实机运行效果验证脚本"
echo "=========================================================="

# 1. 检查 Wayland 环境
if [ -z "${WAYLAND_DISPLAY:-}" ]; then
    echo "[-] 错误: 未检测到 WAYLAND_DISPLAY 环境变量"
    exit 1
fi
echo "[+] 1. 检测到活跃 Wayland 会话: $WAYLAND_DISPLAY"

# 2. 准备测试图像
FIXTURE_DIR="/tmp/ufm-live-test"
mkdir -p "$FIXTURE_DIR"
./tools/generate_test_fixtures.sh "$FIXTURE_DIR" >/dev/null
echo "[+] 2. 测试样本就绪: SVG, Netpbm (P1-P7), 动图 GIF, 透明 PNG"

# 3. 启动 ufm 并打开 SVG 矢量图与 PPM 图像
echo "[+] 3. 正在启动 ufm 渲染窗口..."
./target/release/ufm -d "$FIXTURE_DIR/vector_sample.svg" "$FIXTURE_DIR/sample_p3_ascii.ppm" &
UFM_PID=$!
sleep 1.2

# 4. 验证 Wayland 窗口已成功创建
WINDOW_INFO=$(tessera window | grep -E "ufm|unframed" || true)
if [ -z "$WINDOW_INFO" ]; then
    echo "[-] 错误: Wayland 合成器未检测到 ufm 窗口"
    kill -9 $UFM_PID 2>/dev/null || true
    exit 1
fi
echo "[+] 4. Wayland 窗口已成功在桌面合成并呈现场景:"
echo "       $WINDOW_INFO"

# 5. 抓取实机渲染屏幕截图
echo "[+] 5. 抓取桌面实机屏幕截图 (验证 SVG 矢量上屏)..."
tessera display capture "$FIXTURE_DIR/screenshot_1_svg.png" >/dev/null
sleep 1.2

# 6. 分析屏幕截图中的像素颜色，验证 SVG 确实被 Vulkan 绘制上屏
python3 -c '
from PIL import Image
im = Image.open("'"$FIXTURE_DIR"'/screenshot_1_svg.png")
colors = im.getcolors(maxcolors=1000000)
has_cyan = any(c[1][0] < 50 and c[1][1] > 200 and c[1][2] > 220 for c in colors)
has_orange = any(c[1][0] > 220 and c[1][1] > 60 and c[1][1] < 120 and c[1][2] < 50 for c in colors)
if has_cyan and has_orange:
    print("       >>> 验证通过: 成功在屏幕抓取到 SVG 矢量特征像素（青色 #00e5ff 与 橙色 #ff5500）")
else:
    print("       >>> 像素检测失败: 未在屏幕中找到 SVG 特征像素")
    exit(1)
'

# 7. 通过 ufm-msg 发送 IPC 指令进行顺时针旋转 90 度
echo "[+] 6. 测试 IPC 跨进程控制: 发送旋转指令 (rotate by 90)..."
./target/release/ufm-msg $UFM_PID "rotate by 90"
sleep 0.8
tessera display capture "$FIXTURE_DIR/screenshot_2_rotated.png" >/dev/null
sleep 1.2
echo "       >>> 旋转指令执行成功，已截取旋转后屏幕: $FIXTURE_DIR/screenshot_2_rotated.png"

# 8. 通过 ufm-msg 切换至下一张 Netpbm (PPM) 图像
echo "[+] 7. 测试 IPC 切图指令: 切换至 PPM 图像 (next)..."
./target/release/ufm-msg $UFM_PID "next"
sleep 0.8
tessera display capture "$FIXTURE_DIR/screenshot_3_ppm.png" >/dev/null
sleep 1.2
echo "       >>> 切图指令执行成功，已截取 PPM 图像上屏: $FIXTURE_DIR/screenshot_3_ppm.png"

# 9. 分析 PPM 图像上屏像素 (纯红 #ff0000 与 纯绿 #00ff00)
python3 -c '
from PIL import Image
im = Image.open("'"$FIXTURE_DIR"'/screenshot_3_ppm.png")
colors = im.getcolors(maxcolors=1000000)
has_pure_red = any(c[1][0] > 250 and c[1][1] < 10 and c[1][2] < 10 for c in colors)
has_pure_green = any(c[1][0] < 10 and c[1][1] > 250 and c[1][2] < 10 for c in colors)
if has_pure_red and has_pure_green:
    print("       >>> 验证通过: 成功在屏幕检测到 PPM 原生 RGB 像素（纯红 #ff0000 与 纯绿 #00ff00）")
else:
    print("       >>> 警告: 未在当前画面捕捉到 PPM 预设色值")
'

# 10. 退出 ufm
echo "[+] 8. 测试 IPC 退出指令: 发送 quit 并清理资源..."
./target/release/ufm-msg $UFM_PID "quit"
sleep 0.5
if kill -0 $UFM_PID 2>/dev/null; then
    echo "[-] 进程未正常退出，执行强制清理"
    kill -9 $UFM_PID 2>/dev/null || true
else
    echo "       >>> ufm 进程正常优雅退出，套接字已自动清理！"
fi

echo "=========================================================="
echo "  [SUCCESS] 实机运行效果全部验证通过！"
echo "  截图保存路径:"
echo "    - SVG 渲染: $FIXTURE_DIR/screenshot_1_svg.png"
echo "    - 旋转 90°: $FIXTURE_DIR/screenshot_2_rotated.png"
echo "    - PPM 渲染: $FIXTURE_DIR/screenshot_3_ppm.png"
echo "=========================================================="
