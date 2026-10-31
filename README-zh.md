# unframed (`ufm`)

[English](README.md) | [简体中文](README-zh.md)

一个现代、超高性能、纯 Wayland 原生的 Rust 图像查看器。底层基于 **Optics** (`iris` + `flux` + `lens`) 图形栈构建，作为经典工具 **`imv`** 的内存安全与现代架构进化版。

---

## 核心特性

- **纯 Wayland 架构**：专为 Sway, Hyprland, GNOME Wayland, KDE Plasma Wayland, River 等现代 Wayland 合成器打造。彻底抛弃 X11 与 GLX，采用零损耗原生 Wayland 协议与 Vulkan 动态渲染。
- **基于 Optics 图形技术栈**：
  - **`iris`**：原生 Wayland 平台 Shell 接入、xdg-shell 协议交互、按需动态帧步调（display pacing）与系统级交互（DND 拖拽打开、剪贴板支持）。
  - **`flux`**：硬件加速 Vulkan 2D Canvas 基础，支持亚像素级平移、对数缩放、顺时针旋转、双轴镜像，并提供双线性插值（`linear`）与像素风最近邻（`nearest_neighbour`）无损采样。
  - **`lens`**：即时模式 UI 引擎，呈现高清晰度 OSD 状态浮层与交互式控制台。
- **全方位扩展的图像格式支持**：
  - **SVG 矢量图**：内置 `resvg` / `usvg` 光栅化引擎，支持任意比例超清晰矢量渲染。
  - **Netpbm 协议簇**：完整支持 P1/P4（PBM 单色位图）、P2/P5（PGM 灰度图）、P3/P6（PPM 彩色图）以及 P7（PAM 任意深度格式）。
  - **标准与现代光栅格式**：PNG, JPEG, WebP, TIFF, BMP, QOI, Farbfeld, TGA, ICO, HDR, EXR。
  - **动图时间轴回放**：多帧 GIF 与 Animated WebP 动画播放，精确支持帧延时、空格键暂停/播放与单步跳帧。
- **强大视口与浏览控制**：
  - 缩放模式：`full`（适应窗口）、`shrink`（仅缩小）、`crop`（填满裁剪）、`none`（1:1 实际物理像素）。
  - 以鼠标指针为不动点的对数放大/缩小、左键拖拽平移。
  - 支持顺时针旋转（`rotate by 90`）与水平/垂直轴镜像（`flip`）。
- **命令行控制台与 IPC 脚本化**：
  - 按 `:` 随时唤出内置交互式控制台，支持行编辑、历史翻阅与 Tab 自动补全。
  - 兼容 `imv-msg` 的 UNIX 域套接字通信机制（`$XDG_RUNTIME_DIR/ufm-$PID.sock` 与 `imv-$PID.sock`）。
  - 配套命令行工具：`ufm-msg` 与 `imv-msg`。
- **按键前缀树序列与配置系统**：
  - Trie 前缀树按键解析器，原生支持多键组合（如 `gg` 跳转第一张，`<Shift+G>` 跳转最后一张，`<Ctrl+r>` 旋转等）。
  - 兼容 `imv` 的 INI 配置文件语法（`[options]`, `[aliases]`, `[binds]`）。

---

## 编译与安装

### 环境前置要求

- Rust 工具链（1.75+）
- Vulkan 显卡驱动（RADV, ANV, 或 NVIDIA Vulkan ICD）
- Wayland 桌面环境（环境变量包含 `WAYLAND_DISPLAY`）
- 本地构建好的 `../optics`（或系统已安装 Optics 动态库）

### 构建

```bash
cargo build --release
```

编译产物位于 `target/release/`：
- **`ufm`**：主图像查看器二进制（兼具别名 `unframed`）
- **`ufm-msg`**：跨进程 IPC 控制工具
- **`imv-msg`**：向下兼容原版脚本的 IPC 控制工具

---

## 使用范例

```bash
# 打开单个或多个图片
ufm photo.jpg logo.svg graphic.ppm

# 递归浏览整个目录
ufm -r ~/Pictures

# 从管道中接收路径列表
find . -type f -name "*.svg" | ufm

# 配合 curl 从网络流直接流式查看图片
curl -sL https://example.com/sample.png | ufm -

# 以全屏和棋盘透明背景启动
ufm -f -b checks sample.png

# 启动幻灯片轮播模式（每 5 秒自动切图）
ufm -t 5 ~/Wallpapers
```

---

## 默认键位映射 (Keybindings)

| 键位 | 功能描述 |
| :--- | :--- |
| `q` | 退出程序 |
| `x` | 关闭当前图片 |
| `Right` / `]` | 下一张图片 |
| `Left` / `[` | 上一张图片 |
| `gg` | 跳转到第一张 |
| `Shift+G` | 跳转到最后一张 |
| `j` / `k` / `h` / `l` | 向下 / 向上 / 向左 / 向右平移视图 |
| `Up` / `+` / `i` / 滚轮向上 | 放大视图（以鼠标指针或画面中心为原点） |
| `Down` / `-` / `o` / 滚轮向下 | 缩小视图 |
| `a` | 实际尺寸（100% 原始比例） |
| `r` | 重置缩放、平移与旋转 |
| `c` | 将图片居中对齐 |
| `Ctrl+r` | 顺时针旋转 90 度 |
| `s` | 轮换缩放模式 (`none` → `shrink` → `full` → `crop`) |
| `Shift+S` | 切换插值采样算法 (`linear` ↔ `nearest_neighbour`) |
| `f` | 切换全屏状态 |
| `d` | 切换 OSD 状态浮层显隐 |
| `.` | 动图跳至下一帧 |
| `Space` | 动图暂停 / 播放切换 |
| `t` | 幻灯片间隔增加 1 秒（未开启时则启动轮播） |
| `Shift+T` | 幻灯片间隔减少 1 秒（≤0 时停止轮播） |
| `:` | 打开底部交互式命令行控制台 |

---

## 交互式命令 (Commands)

在运行过程中按 `:` 即可呼出底部控制台输入指令。常用命令如下：

- `quit` (别名 `q`): 退出查看器
- `next [offset]` (别名 `n`): 向后翻动 offset 张图片（默认 1）
- `prev [offset]` (别名 `p`): 向前翻动 offset 张图片（默认 1）
- `goto <index>` (别名 `g`): 跳转到指定的 1-based 索引（`1` 为第一张，`-1` 为最后一张）
- `zoom <amount|actual>` (别名 `z`): 缩放指定对数步长或恢复 100%
- `pan <dx> <dy>`: 按像素平移
- `rotate <to|by> <angle>`: 顺时针旋转角度
- `flip <horizontal|vertical>`: 镜像翻转
- `open [-r] <paths...>` (别名 `o`): 动态向播放列表追加图片或目录
- `close [index|all]`: 关闭当前、指定索引或全部图片
- `fullscreen` (别名 `f`): 全屏切换
- `overlay` (别名 `d`): OSD 浮层显隐
- `scaling <none|shrink|full|crop|next>` (别名 `s`): 设定缩放模式
- `upscaling <linear|nearest_neighbour|next>` (别名 `S`): 设定放大滤波方式
- `slideshow <+amount|-amount|duration>` (别名 `ss`): 控制幻灯片轮播
- `background <checks|hex>` (别名 `bg`): 切换背景样式
- `center` (别名 `c`): 重新居中
- `reset` (别名 `r`): 重置所有视口变换
- `exec <shell command>`: 执行 Shell 脚本（暴露 `$imv_*` 环境变量）
- `bind <keys> <commands>`: 动态注册快捷键绑定
- `alias <name> <command>`: 动态声明命令别名

---

## 进程间通信 (IPC)

`ufm` 监听 `$XDG_RUNTIME_DIR/ufm-$PID.sock` 与 `$XDG_RUNTIME_DIR/imv-$PID.sock`。

可以通过配套的 `ufm-msg` 或 `imv-msg` 命令行工具在外部自动化控制：

```bash
# 控制下一张
ufm-msg $PID next

# 缩放 2 倍
ufm-msg $PID zoom 2

# 旋转 90 度
ufm-msg $PID rotate by 90

# 动态载入新图片
ufm-msg $PID open ~/Pictures/new.svg
```

---

## 配置文件

配置文件搜索优先级依次为：
1. `$ufm_config` 或 `$imv_config`
2. `$XDG_CONFIG_HOME/unframed/config` 或 `$XDG_CONFIG_HOME/imv/config`
3. `~/.config/unframed/config` 或 `~/.config/imv/config`
4. `/etc/imv_config`

完整配置文件参考见 [`files/config`](files/config)。

---

## 详细架构文档

关于深入的技术原理、架构决策记录与开发文档，请参阅：
- [快速入门指南](docs/tutorials/getting-started.md)
- [快捷键与操作指南](docs/how-to/keybindings.md)
- [IPC 与平铺窗口管理器脚本集成](docs/how-to/scripting-and-ipc.md)
- [命令行参考 (CLI)](docs/reference/cli.md)
- [交互式指令全集 (Commands)](docs/reference/commands.md)
- [配置项与样式参考 (Configuration)](docs/reference/configuration.md)
- [图像支持格式参考 (Formats)](docs/reference/supported-formats.md)
- [架构深度概览 (Explanation)](docs/explanation/architecture-overview.md)
- [Wayland + Optics 渲染管线 (Explanation)](docs/explanation/wayland-optics-pipeline.md)
- [架构决策记录 (ADR)](docs/adr/0001-wayland-only-architecture-with-optics.md)
- [子系统拓扑 (Dev)](docs/dev/subsystems.md)
- [测试验证指南 (Dev)](docs/dev/testing.md)

---

## 许可证

MIT License.
