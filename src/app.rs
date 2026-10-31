//! Core application state, event dispatching, and rendering coordination.

use std::collections::HashMap;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

use flux::{AddressMode, Filter, Format, Image, Sampler, SamplerDesc};
use iris::{Frame, Input, PaintHost};
use lens::{key, mods, Color};

use crate::binds::{lens_key_to_string, BindAction, Binds};
use crate::commands::{default_aliases, split_words};
use crate::config::{BackgroundMode, Config, TransparencyGrid};
use crate::console::Console;
use crate::image_loader::{load_image_from_bytes, load_image_from_path, LoadedImage};
use crate::ipc::IpcServer;
use crate::navigator::Navigator;
use crate::viewport::{ScalingMode, UpscalingMethod, Viewport, ZoomSource};

fn draw_checkerboard(
    canvas: &flux::CanvasCommands<'_>,
    w: f32,
    h: f32,
    tile_size: f32,
    c1: u32,
    c2: u32,
    visible_bounds: Option<(f32, f32, f32, f32)>,
) {
    let (min_x, min_y, max_x, max_y) = match visible_bounds {
        Some((x0, y0, x1, y1)) => {
            let start_x = x0.max(0.0);
            let start_y = y0.max(0.0);
            let end_x = x1.min(w);
            let end_y = y1.min(h);
            if start_x >= end_x || start_y >= end_y {
                return;
            }
            (start_x, start_y, end_x, end_y)
        }
        None => (0.0, 0.0, w, h),
    };

    // 1. Fill base tone across visible sub-rectangle
    canvas.fill_rect(min_x, min_y, max_x - min_x, max_y - min_y, c1);

    // 2. Compute tile row/col indices anchored to image origin (0, 0)
    let row_start = (min_y / tile_size).floor() as i32;
    let row_end = (max_y / tile_size).ceil() as i32;
    let col_start = (min_x / tile_size).floor() as i32;
    let col_end = (max_x / tile_size).ceil() as i32;

    for r in row_start..row_end {
        let box_y = (r as f32) * tile_size;
        let draw_y = box_y.max(min_y);
        let draw_h = ((box_y + tile_size).min(max_y) - draw_y).max(0.0);
        if draw_h <= 0.0 {
            continue;
        }

        for c in col_start..col_end {
            if (r + c).rem_euclid(2) == 1 {
                let box_x = (c as f32) * tile_size;
                let draw_x = box_x.max(min_x);
                let draw_w = ((box_x + tile_size).min(max_x) - draw_x).max(0.0);
                if draw_w > 0.0 {
                    canvas.fill_rect(draw_x, draw_y, draw_w, draw_h, c2);
                }
            }
        }
    }
}

/// Compute the visible Axis-Aligned Bounding Box (AABB) of the screen viewport
/// in the image's local coordinate system after inverse transformation.
fn compute_local_viewport_aabb(
    win_w: f32,
    win_h: f32,
    cx: f32,
    cy: f32,
    scaled_w: f32,
    scaled_h: f32,
    rotation_deg: f64,
    mirrored: bool,
) -> (f32, f32, f32, f32) {
    let mirror_factor = if mirrored { -1.0f32 } else { 1.0f32 };
    let rad = (rotation_deg * std::f64::consts::PI / 180.0) as f32;
    let cos_t = rad.cos();
    let sin_t = rad.sin();

    // Map screen point (sx, sy) -> local image point (lx, ly)
    let screen_to_local = |sx: f32, sy: f32| -> (f32, f32) {
        let u0_x = sx - cx;
        let u0_y = sy - cy;
        // Inverse mirroring
        let u1_x = u0_x * mirror_factor;
        let u1_y = u0_y;
        // Inverse rotation (by -rad):
        let u2_x = u1_x * cos_t + u1_y * sin_t;
        let u2_y = -u1_x * sin_t + u1_y * cos_t;
        (u2_x + scaled_w * 0.5, u2_y + scaled_h * 0.5)
    };

    let p1 = screen_to_local(0.0, 0.0);
    let p2 = screen_to_local(win_w, 0.0);
    let p3 = screen_to_local(win_w, win_h);
    let p4 = screen_to_local(0.0, win_h);

    let min_x = p1.0.min(p2.0).min(p3.0).min(p4.0);
    let max_x = p1.0.max(p2.0).max(p3.0).max(p4.0);
    let min_y = p1.1.min(p2.1).min(p3.1).min(p4.1);
    let max_y = p1.1.max(p2.1).max(p3.1).max(p4.1);

    (min_x, min_y, max_x, max_y)
}

pub struct App {
    pub config: Config,
    pub navigator: Navigator,
    pub viewport: Viewport,
    pub binds: Binds,
    pub aliases: HashMap<String, String>,
    pub console: Console,

    pub current_image: Option<LoadedImage>,
    pub gpu_image: Option<Image>,
    pub gpu_frame_idx: usize,
    pub gpu_needs_upload: bool,
    pub needs_redraw: bool,

    pub linear_sampler: Option<Sampler>,
    pub nearest_sampler: Option<Sampler>,

    pub ipc_rx: Receiver<String>,
    pub _ipc_server: Option<IpcServer>,

    pub fullscreen: bool,
    pub overlay_enabled: bool,
    pub slideshow_elapsed_s: f64,
    pub last_frame_time: Instant,
    pub last_file_poll: Instant,

    pub last_cursor_pos: Option<(f64, f64)>,
    pub mouse_dragging: bool,
    pub should_quit: bool,
    pub stdin_data: Option<Vec<u8>>,
    pub initial_command_run: bool,
    pub startup_commands: Vec<String>,
    pub on_image_change_commands: Vec<String>,
}

impl App {
    pub fn new(
        config: Config,
        paths: Vec<String>,
        stdin_data: Option<Vec<u8>>,
        startup_commands: Vec<String>,
        on_image_change_commands: Vec<String>,
    ) -> Self {
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        let ipc_server = IpcServer::start(tx);

        let mut navigator = Navigator::new();
        navigator.loop_input = config.loop_input;
        for p in &paths {
            navigator.add(p, config.recursive_load);
        }

        let mut viewport = Viewport::new(config.width as f64, config.height as f64);
        viewport.scaling_mode = config.scaling_mode;
        viewport.upscaling_method = config.upscaling_method;
        viewport.pan_factor_x = config.initial_pan_x;
        viewport.pan_factor_y = config.initial_pan_y;

        let mut binds = Binds::new();
        if !config.suppress_default_binds {
            binds.load_defaults();
        }
        for (k, cmd) in &config.binds {
            binds.add(k, cmd);
        }

        let mut aliases = default_aliases();
        for (alias, cmd) in &config.aliases {
            aliases.insert(alias.clone(), cmd.clone());
        }

        let fullscreen = config.fullscreen;
        let overlay_enabled = config.overlay_enabled;

        let mut app = Self {
            config,
            navigator,
            viewport,
            binds,
            aliases,
            console: Console::new(),

            current_image: None,
            gpu_image: None,
            gpu_frame_idx: 0,
            gpu_needs_upload: false,
            needs_redraw: true,

            linear_sampler: None,
            nearest_sampler: None,

            ipc_rx: rx,
            _ipc_server: ipc_server,

            fullscreen,
            overlay_enabled,
            slideshow_elapsed_s: 0.0,
            last_frame_time: Instant::now(),
            last_file_poll: Instant::now(),

            last_cursor_pos: None,
            mouse_dragging: false,
            should_quit: false,
            stdin_data,
            initial_command_run: false,
            startup_commands,
            on_image_change_commands,
        };

        app.reload_current_image();
        app
    }

    pub fn reload_current_image(&mut self) {
        let path_opt = self.navigator.selection().map(|s| s.to_string());
        if let Some(path) = path_opt {
            if path == "-" {
                if let Some(data) = &self.stdin_data {
                    match load_image_from_bytes(data, Some("stdin")) {
                        Ok(img) => {
                            self.viewport.reset(img.width, img.height);
                            self.current_image = Some(img);
                            self.gpu_needs_upload = true;
                            self.needs_redraw = true;
                            self.run_on_image_change_commands();
                        }
                        Err(e) => eprintln!("Error loading stdin image: {e}"),
                    }
                }
            } else {
                match load_image_from_path(&path) {
                    Ok(img) => {
                        self.viewport.reset(img.width, img.height);
                        self.current_image = Some(img);
                        self.gpu_needs_upload = true;
                        self.needs_redraw = true;
                        self.run_on_image_change_commands();
                    }
                    Err(e) => {
                        eprintln!("Error loading {path}: {e}");
                        self.navigator.remove_current();
                        if !self.navigator.is_empty() {
                            self.reload_current_image();
                        } else {
                            self.current_image = None;
                            self.gpu_image = None;
                            self.needs_redraw = true;
                        }
                    }
                }
            }
        } else {
            self.current_image = None;
            self.gpu_image = None;
            self.needs_redraw = true;
        }
    }

    fn run_on_image_change_commands(&mut self) {
        let cmds = self.on_image_change_commands.clone();
        for cmd in cmds {
            self.execute_command(&cmd);
        }
    }

    pub fn execute_command(&mut self, command_str: &str) {
        let trimmed = command_str.trim();
        if trimmed.is_empty() {
            return;
        }

        let words = split_words(trimmed);
        if words.is_empty() {
            return;
        }

        let first = &words[0];
        if let Some(target) = self.aliases.get(first).cloned() {
            let mut new_cmd = target;
            for w in &words[1..] {
                new_cmd.push(' ');
                new_cmd.push_str(w);
            }
            self.execute_command(&new_cmd);
            return;
        }

        self.needs_redraw = true;

        let args = &words[1..];
        match first.as_str() {
            "quit" => self.quit(),
            "close" => {
                if args.is_empty() {
                    self.close_image(None);
                } else if args[0].eq_ignore_ascii_case("all") {
                    self.close_all_images();
                } else if let Ok(idx) = args[0].parse::<isize>() {
                    self.close_image(Some(idx));
                }
            }
            "next" => {
                let offset = args.first().and_then(|s| s.parse::<isize>().ok()).unwrap_or(1);
                self.next_image(offset);
            }
            "prev" => {
                let offset = args.first().and_then(|s| s.parse::<isize>().ok()).unwrap_or(1);
                self.prev_image(offset);
            }
            "goto" => {
                if let Some(idx) = args.first().and_then(|s| s.parse::<isize>().ok()) {
                    self.goto_image(idx);
                }
            }
            "zoom" => {
                if let Some(arg) = args.first() {
                    if arg.eq_ignore_ascii_case("actual") {
                        self.zoom_actual();
                    } else if let Ok(amount) = arg.parse::<f64>() {
                        self.zoom(amount);
                    }
                }
            }
            "pan" => {
                if args.len() >= 2 {
                    if let (Ok(dx), Ok(dy)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) {
                        self.pan(dx, dy);
                    }
                }
            }
            "rotate" => {
                if args.len() >= 2 {
                    let to = args[0].eq_ignore_ascii_case("to");
                    if let Ok(angle) = args[1].parse::<f64>() {
                        self.rotate(to, angle);
                    }
                }
            }
            "flip" => {
                if let Some(dir) = args.first() {
                    let vert = dir.eq_ignore_ascii_case("vertical");
                    self.flip(vert);
                }
            }
            "open" => {
                let mut recursive = false;
                let mut paths = Vec::new();
                for a in args {
                    if a == "-r" {
                        recursive = true;
                    } else {
                        paths.push(a.clone());
                    }
                }
                if !paths.is_empty() {
                    self.open_paths(&paths, recursive);
                }
            }
            "fullscreen" => self.toggle_fullscreen(),
            "overlay" => self.toggle_overlay(),
            "center" => self.center(),
            "reset" => self.reset_view(),
            "next_frame" => self.next_frame(),
            "toggle_playing" => self.toggle_playing(),
            "scaling" => {
                if let Some(arg) = args.first() {
                    if arg.eq_ignore_ascii_case("next") {
                        self.set_scaling_mode(None);
                    } else if let Some(mode) = ScalingMode::parse(arg) {
                        self.set_scaling_mode(Some(mode));
                    }
                } else {
                    self.set_scaling_mode(None);
                }
            }
            "upscaling" => {
                if let Some(arg) = args.first() {
                    if arg.eq_ignore_ascii_case("next") {
                        self.set_upscaling_method(None);
                    } else if let Some(m) = UpscalingMethod::parse(arg) {
                        self.set_upscaling_method(Some(m));
                    }
                } else {
                    self.set_upscaling_method(None);
                }
            }
            "slideshow" => {
                if let Some(arg) = args.first() {
                    if let Some(val_str) = arg.strip_prefix('+') {
                        if let Ok(val) = val_str.parse::<f64>() {
                            self.set_slideshow(Some(val), None);
                        }
                    } else if let Some(val_str) = arg.strip_prefix('-') {
                        if let Ok(val) = val_str.parse::<f64>() {
                            self.set_slideshow(Some(-val), None);
                        }
                    } else if let Ok(val) = arg.parse::<f64>() {
                        self.set_slideshow(None, Some(val));
                    }
                }
            }
            "background" => {
                if let Some(arg) = args.first() {
                    if arg.eq_ignore_ascii_case("checks") {
                        self.config.background = BackgroundMode::Checks;
                    } else if let Some(c) = crate::config::parse_hex_color(arg, 255) {
                        self.config.background = BackgroundMode::Solid(c);
                    }
                }
            }
            "transparency" => {
                if let Some(arg) = args.first() {
                    if arg.eq_ignore_ascii_case("next") {
                        self.config.transparency_grid = self.config.transparency_grid.next();
                    } else if let Some(mode) = TransparencyGrid::parse(arg) {
                        self.config.transparency_grid = mode;
                    }
                } else {
                    self.config.transparency_grid = self.config.transparency_grid.next();
                }
            }
            "bind" => {
                if args.len() >= 2 {
                    let keys = &args[0];
                    let cmd = args[1..].join(" ");
                    self.binds.add(keys, &cmd);
                }
            }
            "alias" => {
                if args.len() >= 2 {
                    let alias = &args[0];
                    let cmd = args[1..].join(" ");
                    self.aliases.insert(alias.clone(), cmd);
                }
            }
            "exec" => {
                let cmd_str = args.join(" ");
                let envs = self.get_env_vars();
                std::thread::spawn(move || {
                    let mut cmd = Command::new("sh");
                    cmd.arg("-c").arg(&cmd_str);
                    for (k, v) in envs {
                        cmd.env(k, v);
                    }
                    let _ = cmd.status();
                });
            }
            _ => {}
        }
    }

    pub fn on_build(&mut self, frame: &mut Frame<'_>, input: &Input) {
        if self.should_quit {
            iris::window_close();
            return;
        }

        if !self.initial_command_run {
            self.initial_command_run = true;
            if self.fullscreen {
                iris::window_fullscreen();
            }
            let cmds = self.startup_commands.clone();
            for c in cmds {
                self.execute_command(&c);
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f64();
        self.last_frame_time = now;

        while let Ok(cmd) = self.ipc_rx.try_recv() {
            self.execute_command(&cmd);
        }

        let raw_in = input.as_raw();
        let cur_x = raw_in.cursor.x as f64;
        let cur_y = raw_in.cursor.y as f64;

        // Parse committed UTF-8 text from platform/IME
        let text_len = raw_in
            .text_utf8
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(raw_in.text_utf8.len());
        let text_bytes: Vec<u8> = raw_in.text_utf8[..text_len]
            .iter()
            .map(|&c| c as u8)
            .collect();
        let committed_text = std::str::from_utf8(&text_bytes).unwrap_or("");

        let win_w = raw_in.display_size.x as f64;
        let win_h = raw_in.display_size.y as f64;
        if win_w > 0.0 && win_h > 0.0 {
            let (img_w, img_h) = self
                .current_image
                .as_ref()
                .map(|img| (img.width, img.height))
                .unwrap_or((0, 0));
            self.viewport.set_window_size(win_w, win_h, img_w, img_h);
        }

        // Mouse drag panning
        let mouse_left_down = raw_in.mouse_down[0];
        if mouse_left_down {
            if let Some((last_x, last_y)) = self.last_cursor_pos {
                if self.mouse_dragging {
                    let dx = cur_x - last_x;
                    let dy = cur_y - last_y;
                    if dx != 0.0 || dy != 0.0 {
                        if let Some(img) = &self.current_image {
                            self.viewport.move_by(dx, dy, img.width, img.height);
                            self.needs_redraw = true;
                        }
                    }
                }
            }
            self.mouse_dragging = true;
        } else {
            self.mouse_dragging = false;
        }
        self.last_cursor_pos = Some((cur_x, cur_y));

        // Mouse scroll zooming
        let scroll_y = raw_in.scroll_y;
        if scroll_y != 0.0 {
            let (img_w, img_h) = self
                .current_image
                .as_ref()
                .map(|img| (img.width, img.height))
                .unwrap_or((0, 0));
            if img_w > 0 && img_h > 0 {
                let amount = if scroll_y > 0.0 { 1.5 } else { -1.5 };
                self.viewport
                    .zoom(amount, ZoomSource::Mouse, cur_x, cur_y, img_w, img_h);
                self.needs_redraw = true;
            }
        }

        // Process keyboard inputs
        let key_count = raw_in.key_count as usize;
        for i in 0..key_count {
            let k = raw_in.keys[i];
            if !k.pressed {
                continue;
            }

            if self.console.active {
                match k.key {
                    key::ESCAPE => {
                        self.console.deactivate();
                        self.needs_redraw = true;
                    }
                    key::RETURN => {
                        if let Some(cmd) = self.console.submit() {
                            self.execute_command(&cmd);
                        }
                    }
                    key::BACKSPACE => {
                        self.console.backspace();
                        self.needs_redraw = true;
                    }
                    key::DELETE => {
                        self.console.delete();
                        self.needs_redraw = true;
                    }
                    key::LEFT => {
                        self.console.move_left();
                        self.needs_redraw = true;
                    }
                    key::RIGHT => {
                        self.console.move_right();
                        self.needs_redraw = true;
                    }
                    key::HOME => {
                        self.console.move_home();
                        self.needs_redraw = true;
                    }
                    key::END => {
                        self.console.move_end();
                        self.needs_redraw = true;
                    }
                    key::UP => {
                        self.console.history_prev();
                        self.needs_redraw = true;
                    }
                    key::DOWN => {
                        self.console.history_next();
                        self.needs_redraw = true;
                    }
                    key::TAB => {
                        self.console.complete();
                        self.needs_redraw = true;
                    }
                    c if (32..=126).contains(&c) && committed_text.is_empty() => {
                        let ch = (c as u8) as char;
                        let shifted = (raw_in.mods & mods::SHIFT) != 0;
                        let ch = if shifted { ch.to_ascii_uppercase() } else { ch };
                        self.console.insert_char(ch);
                        self.needs_redraw = true;
                    }
                    _ => {}
                }
            } else {
                // Check if triggering console with ':'
                let is_colon = committed_text.contains(':')
                    || k.key == ':' as i32
                    || (k.key == ';' as i32 && (raw_in.mods & mods::SHIFT) != 0);

                if is_colon {
                    self.console.activate();
                    self.needs_redraw = true;
                    continue;
                }

                // Normal bind handling
                if let Some(key_str) = lens_key_to_string(k.key, raw_in.mods) {
                    match self.binds.handle_key(&key_str) {
                        BindAction::Execute(cmds) => {
                            for cmd in cmds {
                                self.execute_command(&cmd);
                            }
                        }
                        BindAction::Pending => {}
                        BindAction::None => {}
                    }
                }
            }
        }

        // Route committed text into active console
        if self.console.active && !committed_text.is_empty() {
            for c in committed_text.chars() {
                if !c.is_control() {
                    self.console.insert_char(c);
                    self.needs_redraw = true;
                }
            }
        }

        // Check external file modification
        if now.duration_since(self.last_file_poll).as_secs_f64() > 0.5 {
            self.last_file_poll = now;
            if let Some(img) = &self.current_image {
                if let Some(path) = &img.path {
                    if path != "-" {
                        if let Ok(meta) = std::fs::metadata(path) {
                            if let Ok(mod_time) = meta.modified() {
                                if Some(mod_time) != img.modified {
                                    self.reload_current_image();
                                }
                            }
                        }
                    }
                }
            }
        }

        // Animation playback tick
        let mut is_animating = false;
        if self.viewport.playing {
            if let Some(img) = &mut self.current_image {
                if img.is_animated() {
                    let changed = img.tick_animation(dt * 1000.0);
                    if changed {
                        self.gpu_needs_upload = true;
                        self.needs_redraw = true;
                    }
                    is_animating = true;
                }
            }
        }

        // Slideshow tick
        if self.config.slideshow_duration > 0.0 && self.navigator.len() > 1 {
            self.slideshow_elapsed_s += dt;
            is_animating = true;
            if self.slideshow_elapsed_s >= self.config.slideshow_duration {
                self.slideshow_elapsed_s = 0.0;
                self.next_image(1);
            }
        }

        if is_animating || self.needs_redraw {
            iris::request_animation_frame();
            self.needs_redraw = false;
        } else {
            iris::paint_mark_static();
        }

        // Handle Drag & Drop from Wayland file managers
        if let Some(drop_info) = frame.dnd_drop_target("main_window", iris::dnd::DndAction::COPY.0) {
            if drop_info.is_dropped {
                if let Some(payload) = drop_info.payload {
                    for line in payload.lines() {
                        let trimmed = line.trim();
                        let path = if let Some(p) = iris::file_uri_to_path(trimmed) {
                            p
                        } else if let Some(p) = trimmed.strip_prefix("file://") {
                            p.to_string()
                        } else {
                            trimmed.to_string()
                        };
                        if !path.is_empty() {
                            self.open_paths(&[path], self.config.recursive_load);
                        }
                    }
                }
            }
        }

        // Lens immediate-mode chrome
        let show_overlay = self.overlay_enabled;
        let show_console = self.console.active;

        if show_overlay || show_console {
            let overlay_text = if show_overlay {
                Some(self.format_overlay_text(&self.config.overlay_text))
            } else {
                None
            };

            let pos_bottom = self.config.overlay_position_bottom;

            frame.column().show(|f| {
                if show_overlay && !pos_bottom {
                    if let Some(txt) = &overlay_text {
                        f.row()
                            .bg(Color::rgba(0, 0, 0, 195))
                            .pad(8.0)
                            .rounded(4.0)
                            .show(|f| {
                                f.label(txt);
                            });
                    }
                }

                f.flex(1.0);
                f.spacer(0.0);

                if show_overlay && pos_bottom {
                    if let Some(txt) = &overlay_text {
                        f.row()
                            .bg(Color::rgba(0, 0, 0, 195))
                            .pad(8.0)
                            .rounded(4.0)
                            .show(|f| {
                                f.label(txt);
                            });
                    }
                }

                if show_console {
                    let prompt_text = format!(":{}", self.console.buffer);
                    f.row()
                        .bg(Color::rgba(15, 15, 15, 230))
                        .pad(6.0)
                        .rounded(2.0)
                        .show(|f| {
                            f.label(&prompt_text);
                        });
                }
            });
        }
    }

    pub fn on_paint(&mut self, paint_host: &PaintHost) {
        let canvas = paint_host.flux_canvas();
        let device = paint_host.flux_device();
        let scale_factor = paint_host.scale();

        if self.linear_sampler.is_none() {
            if let Ok(s) = Sampler::new(
                &device,
                SamplerDesc {
                    min_filter: Filter::Linear,
                    mag_filter: Filter::Linear,
                    mipmap_mode: Filter::Linear,
                    address_u: AddressMode::ClampToEdge,
                    address_v: AddressMode::ClampToEdge,
                    address_w: AddressMode::ClampToEdge,
                    max_anisotropy: 1.0,
                },
            ) {
                self.linear_sampler = Some(s);
            }
        }

        if self.nearest_sampler.is_none() {
            if let Ok(s) = Sampler::new(
                &device,
                SamplerDesc {
                    min_filter: Filter::Nearest,
                    mag_filter: Filter::Nearest,
                    mipmap_mode: Filter::Nearest,
                    address_u: AddressMode::ClampToEdge,
                    address_v: AddressMode::ClampToEdge,
                    address_w: AddressMode::ClampToEdge,
                    max_anisotropy: 1.0,
                },
            ) {
                self.nearest_sampler = Some(s);
            }
        }

        if self.gpu_needs_upload {
            if let Some(img) = &self.current_image {
                let frame_data = img.current_frame_data();
                match Image::from_bytes(
                    &device,
                    img.width,
                    img.height,
                    Format::Rgba8Unorm,
                    &frame_data.rgba,
                ) {
                    Ok(flux_img) => {
                        self.gpu_image = Some(flux_img);
                        self.gpu_frame_idx = img.current_frame;
                        self.gpu_needs_upload = false;
                    }
                    Err(e) => eprintln!("Failed to create GPU image: {e:?}"),
                }
            }
        }

        let win_w = self.viewport.buffer_width as f32;
        let win_h = self.viewport.buffer_height as f32;

        canvas.save();
        canvas.scale(scale_factor, scale_factor);

        match self.config.background {
            BackgroundMode::Solid(col) => {
                canvas.fill_rect(0.0, 0.0, win_w, win_h, col);
            }
            BackgroundMode::Checks => {
                draw_checkerboard(&canvas, win_w, win_h, 24.0, 0xFF202020, 0xFF363636, None);
            }
        }

        if let (Some(gpu_img), Some(img)) = (&self.gpu_image, &self.current_image) {
            let scaled_w = (img.width as f64 * self.viewport.scale) as f32;
            let scaled_h = (img.height as f64 * self.viewport.scale) as f32;

            canvas.save();
            let cx = (self.viewport.x + (scaled_w as f64 * 0.5)) as f32;
            let cy = (self.viewport.y + (scaled_h as f64 * 0.5)) as f32;
            canvas.translate(cx, cy);

            if self.viewport.mirrored {
                canvas.scale(-1.0, 1.0);
            }
            if self.viewport.rotation != 0.0 {
                let rad = (self.viewport.rotation * std::f64::consts::PI / 180.0) as f32;
                canvas.rotate(rad);
            }

            canvas.translate(-scaled_w * 0.5, -scaled_h * 0.5);

            // Compute local visible viewport AABB to cull hidden checker tiles
            let local_aabb = compute_local_viewport_aabb(
                win_w,
                win_h,
                cx,
                cy,
                scaled_w,
                scaled_h,
                self.viewport.rotation,
                self.viewport.mirrored,
            );

            // Draw image-bound transparency grid if enabled
            match self.config.transparency_grid {
                TransparencyGrid::ChecksLight => {
                    // Classic Photoshop light checkerboard (#FFFFFF and #CCCCCC)
                    draw_checkerboard(
                        &canvas,
                        scaled_w,
                        scaled_h,
                        16.0,
                        0xFFFFFFFF,
                        0xFFCCCCCC,
                        Some(local_aabb),
                    );
                }
                TransparencyGrid::ChecksDark => {
                    // Modern dark mode checkerboard (#202020 and #383838)
                    draw_checkerboard(
                        &canvas,
                        scaled_w,
                        scaled_h,
                        16.0,
                        0xFF202020,
                        0xFF383838,
                        Some(local_aabb),
                    );
                }
                TransparencyGrid::SolidWhite => {
                    let (x0, y0, x1, y1) = (
                        local_aabb.0.max(0.0),
                        local_aabb.1.max(0.0),
                        local_aabb.2.min(scaled_w),
                        local_aabb.3.min(scaled_h),
                    );
                    if x0 < x1 && y0 < y1 {
                        canvas.fill_rect(x0, y0, x1 - x0, y1 - y0, 0xFFFFFFFF);
                    }
                }
                TransparencyGrid::SolidBlack => {
                    let (x0, y0, x1, y1) = (
                        local_aabb.0.max(0.0),
                        local_aabb.1.max(0.0),
                        local_aabb.2.min(scaled_w),
                        local_aabb.3.min(scaled_h),
                    );
                    if x0 < x1 && y0 < y1 {
                        canvas.fill_rect(x0, y0, x1 - x0, y1 - y0, 0xFF000000);
                    }
                }
                TransparencyGrid::Off => {}
            }

            let sampler = if self.viewport.upscaling_method == UpscalingMethod::NearestNeighbour {
                self.nearest_sampler.as_ref()
            } else {
                self.linear_sampler.as_ref()
            };

            if let Some(s) = sampler {
                canvas.draw_image_sampled(gpu_img, s, 0.0, 0.0, scaled_w, scaled_h);
            } else {
                canvas.draw_image(gpu_img, 0.0, 0.0, scaled_w, scaled_h);
            }

            canvas.restore();
        }

        canvas.restore();
    }

    pub fn format_overlay_text(&self, template: &str) -> String {
        let envs = self.get_env_vars();
        let mut result = template.to_string();
        for (k, v) in envs {
            result = result.replace(&format!("${{{k}}}"), &v);
            result = result.replace(&format!("${k}"), &v);
        }
        result
    }

    pub fn quit(&mut self) {
        if self.config.list_files_at_exit {
            for p in self.navigator.all_paths() {
                println!("{p}");
            }
        }
        self.should_quit = true;
        iris::window_close();
    }

    pub fn toggle_fullscreen(&mut self) {
        if self.fullscreen {
            iris::window_windowed();
            self.fullscreen = false;
        } else {
            iris::window_fullscreen();
            self.fullscreen = true;
        }
    }

    pub fn toggle_overlay(&mut self) {
        self.overlay_enabled = !self.overlay_enabled;
    }

    pub fn next_image(&mut self, offset: isize) {
        self.navigator.select_rel(offset);
        self.reload_current_image();
    }

    pub fn prev_image(&mut self, offset: isize) {
        self.navigator.select_rel(-offset);
        self.reload_current_image();
    }

    pub fn goto_image(&mut self, index: isize) {
        self.navigator.select_abs(index);
        self.reload_current_image();
    }

    pub fn close_image(&mut self, index: Option<isize>) {
        if let Some(idx) = index {
            if idx > 0 {
                self.navigator.remove_at((idx - 1) as usize);
            } else if idx < 0 {
                let target = self.navigator.len() as isize + idx;
                if target >= 0 {
                    self.navigator.remove_at(target as usize);
                }
            }
        } else {
            self.navigator.remove_current();
        }
        self.reload_current_image();
    }

    pub fn close_all_images(&mut self) {
        self.navigator.remove_all();
        self.reload_current_image();
    }

    pub fn open_paths(&mut self, paths: &[String], recursive: bool) {
        for p in paths {
            self.navigator.add(p, recursive);
        }
        if self.current_image.is_none() {
            self.reload_current_image();
        }
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        let (w, h) = self
            .current_image
            .as_ref()
            .map(|i| (i.width, i.height))
            .unwrap_or((0, 0));
        self.viewport.move_by(dx, dy, w, h);
    }

    pub fn zoom(&mut self, amount: f64) {
        let (w, h) = self
            .current_image
            .as_ref()
            .map(|i| (i.width, i.height))
            .unwrap_or((0, 0));
        let mx = self.viewport.buffer_width * 0.5;
        let my = self.viewport.buffer_height * 0.5;
        self.viewport.zoom(amount, ZoomSource::Keyboard, mx, my, w, h);
    }

    pub fn zoom_actual(&mut self) {
        let (w, h) = self
            .current_image
            .as_ref()
            .map(|i| (i.width, i.height))
            .unwrap_or((0, 0));
        self.viewport.scale_to_actual(w, h);
    }

    pub fn rotate(&mut self, to: bool, angle: f64) {
        if to {
            self.viewport.rotate_to(angle);
        } else {
            self.viewport.rotate_by(angle);
        }
    }

    pub fn flip(&mut self, vertical: bool) {
        if vertical {
            self.viewport.flip_v();
        } else {
            self.viewport.flip_h();
        }
    }

    pub fn center(&mut self) {
        let (w, h) = self
            .current_image
            .as_ref()
            .map(|i| (i.width, i.height))
            .unwrap_or((0, 0));
        self.viewport.center(w, h);
    }

    pub fn reset_view(&mut self) {
        let (w, h) = self
            .current_image
            .as_ref()
            .map(|i| (i.width, i.height))
            .unwrap_or((0, 0));
        self.viewport.reset(w, h);
    }

    pub fn next_frame(&mut self) {
        if let Some(img) = &mut self.current_image {
            img.next_frame();
            self.gpu_needs_upload = true;
        }
    }

    pub fn toggle_playing(&mut self) {
        self.viewport.playing = !self.viewport.playing;
    }

    pub fn set_scaling_mode(&mut self, mode: Option<ScalingMode>) {
        let next = mode.unwrap_or_else(|| self.viewport.scaling_mode.next());
        self.viewport.scaling_mode = next;
        let (w, h) = self
            .current_image
            .as_ref()
            .map(|i| (i.width, i.height))
            .unwrap_or((0, 0));
        self.viewport.rescale(w, h);
    }

    pub fn set_upscaling_method(&mut self, method: Option<UpscalingMethod>) {
        let next = method.unwrap_or_else(|| self.viewport.upscaling_method.next());
        self.viewport.upscaling_method = next;
    }

    pub fn set_slideshow(&mut self, delta: Option<f64>, set_val: Option<f64>) {
        if let Some(d) = delta {
            self.config.slideshow_duration = (self.config.slideshow_duration + d).max(0.0);
        } else if let Some(val) = set_val {
            self.config.slideshow_duration = val.max(0.0);
        }
    }

    pub fn cycle_transparency(&mut self) {
        self.config.transparency_grid = self.config.transparency_grid.next();
        self.needs_redraw = true;
    }

    pub fn set_transparency(&mut self, mode: TransparencyGrid) {
        self.config.transparency_grid = mode;
        self.needs_redraw = true;
    }

    pub fn get_env_vars(&self) -> Vec<(&'static str, String)> {
        let pid = std::process::id().to_string();
        let cur_file = self.navigator.selection().unwrap_or("").to_string();
        let scaling_mode = self.viewport.scaling_mode.as_str().to_string();
        let upscaling_method = self.viewport.upscaling_method.as_str().to_string();
        let transparency = self.config.transparency_grid.name().to_string();
        let loading = if self.current_image.is_some() { "0" } else { "1" }.to_string();
        let cur_idx = if self.navigator.is_empty() {
            "0".to_string()
        } else {
            (self.navigator.index() + 1).to_string()
        };
        let file_count = self.navigator.len().to_string();
        let (w, h) = self
            .current_image
            .as_ref()
            .map(|i| (i.width.to_string(), i.height.to_string()))
            .unwrap_or(("0".into(), "0".into()));
        let scale = format!("{:.0}", self.viewport.scale * 100.0);
        let duration = self.config.slideshow_duration.to_string();
        let elapsed = format!("{:.1}", self.slideshow_elapsed_s);
        let format_name = self
            .current_image
            .as_ref()
            .map(|i| i.format_name.clone())
            .unwrap_or_default();

        vec![
            ("ufm_pid", pid.clone()),
            ("unframed_pid", pid),
            ("ufm_current_file", cur_file),
            ("ufm_format", format_name),
            ("ufm_scaling_mode", scaling_mode),
            ("ufm_upscaling_method", upscaling_method),
            ("ufm_transparency", transparency),
            ("ufm_loading", loading),
            ("ufm_current_index", cur_idx),
            ("ufm_file_count", file_count),
            ("ufm_width", w),
            ("ufm_height", h),
            ("ufm_scale", scale),
            ("ufm_slideshow_duration", duration),
            ("ufm_slideshow_elapsed", elapsed),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_command_execution() {
        let config = Config::default();
        let mut app = App::new(config, vec![], None, vec![], vec![]);

        app.execute_command("overlay");
        assert!(app.overlay_enabled);
        app.execute_command("overlay");
        assert!(!app.overlay_enabled);

        app.execute_command("scaling next");
        assert_eq!(app.viewport.scaling_mode, ScalingMode::Crop);

        app.execute_command("upscaling nearest");
        assert_eq!(app.viewport.upscaling_method, UpscalingMethod::NearestNeighbour);

        app.execute_command("slideshow +5");
        assert_eq!(app.config.slideshow_duration, 5.0);

        app.execute_command("rotate by 90");
        assert_eq!(app.viewport.rotation, 90.0);

        app.execute_command("flip horizontal");
        assert!(app.viewport.mirrored);

        app.execute_command("background checks");
        assert_eq!(app.config.background, BackgroundMode::Checks);

        app.execute_command("transparency next");
        assert_eq!(app.config.transparency_grid, TransparencyGrid::ChecksDark);
        app.execute_command("transparency white");
        assert_eq!(app.config.transparency_grid, TransparencyGrid::SolidWhite);
        app.execute_command("transparency off");
        assert_eq!(app.config.transparency_grid, TransparencyGrid::Off);
        app.execute_command("transparency checks-light");
        assert_eq!(app.config.transparency_grid, TransparencyGrid::ChecksLight);
    }

    #[test]
    fn test_app_overlay_formatting() {
        let config = Config::default();
        let app = App::new(config, vec![], None, vec![], vec![]);
        let text = app.format_overlay_text("[$ufm_current_index/$ufm_file_count] $ufm_scaling_mode");
        assert_eq!(text, "[0/0] full");
    }

    #[test]
    fn test_compute_local_viewport_aabb() {
        // Simple case: centered image 200x200 in 1000x1000 window, no rotation
        // cx = 500, cy = 500
        let aabb = compute_local_viewport_aabb(1000.0, 1000.0, 500.0, 500.0, 200.0, 200.0, 0.0, false);
        // screen 0,0 maps to: u0 = -500, + 100 = -400
        // screen 1000,1000 maps to: u0 = 500, + 100 = 600
        assert_eq!(aabb, (-400.0, -400.0, 600.0, 600.0));

        // When zoomed in 10x: image 2000x2000 in 1000x1000 window, center at 500, 500
        let aabb_zoomed = compute_local_viewport_aabb(1000.0, 1000.0, 500.0, 500.0, 2000.0, 2000.0, 0.0, false);
        // screen 0,0 maps to: -500 + 1000 = 500
        // screen 1000,1000 maps to: 500 + 1000 = 1500
        assert_eq!(aabb_zoomed, (500.0, 500.0, 1500.0, 1500.0));
    }
}
