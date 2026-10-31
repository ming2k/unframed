//! unframed - Modern Wayland image viewer built with Rust and Optics.

#![allow(clippy::chunks_exact_to_as_chunks)]

use std::cell::RefCell;
use std::env;
use std::io::{self, BufRead, IsTerminal, Read};
use std::path::Path;
use std::rc::Rc;

pub mod app;
pub mod binds;
pub mod commands;
pub mod config;
pub mod console;
pub mod image_loader;
pub mod ipc;
pub mod navigator;
pub mod ppm;
pub mod svg;
pub mod viewport;

use app::App;
use config::{find_config_file, parse_hex_color, BackgroundMode, Config};
use viewport::{ScalingMode, UpscalingMethod};

pub const VERSION: &str = "0.1.0";

pub fn print_help() {
    println!(
        r#"Usage: ufm [options] [paths...]
       unframed [options] [paths...]

A fast, lightweight image viewer for Wayland powered by optics.

Options:
  -h                         Show this help message and quit
  -v                         Show version and quit
  -b <background>            Set background color (6-digit hex or 'checks')
  -c <command>               Command to execute on launch (can be repeated)
  -C <command>               Command to execute on image change (can be repeated)
  -d                         Start with overlay visible
  -f                         Start fullscreen
  -i <id>                    Set Wayland application ID
  -l                         List remaining paths to stdout on exit
  -n <path|index>            Start with given path or index selected
  -r                         Load directories recursively
  -s <none|shrink|full|crop> Set initial scaling mode (default: full)
  -t <seconds>               Start in slideshow mode with given delay
  -u <linear|nearest_neighbour> Set upscaling method (default: linear)
  -w <title>                 Set window title
  -x                         Disable looping of input paths
  -V                         Enable verbose logging
  -W <width>                 Initial window width (default: 1280)
  -H <height>                Initial window height (default: 720)

Supported Formats:
  PNG, JPEG, GIF (animated), WebP (animated), BMP, TIFF, SVG (vector),
  PPM / PGM / PBM / PAM (ASCII and binary), QOI, Farbfeld, TGA, ICO, HDR, EXR.
"#
    );
}

pub fn run() {
    let args: Vec<String> = env::args().collect();

    // Check help / version first
    for arg in &args[1..] {
        if arg == "-h" || arg == "--help" {
            print_help();
            return;
        } else if arg == "-v" || arg == "--version" {
            println!("ufm (unframed) version {VERSION}");
            return;
        }
    }

    // Check Wayland-only requirement
    if env::var("WAYLAND_DISPLAY").is_err() && env::var("WAYLAND_SOCKET").is_err() {
        eprintln!("ufm: Wayland connection required (WAYLAND_DISPLAY is not set). X11 is not supported.");
        std::process::exit(1);
    }

    let mut config = Config::default();

    // Load config file if found
    if let Some(cfg_path) = find_config_file() {
        let _ = config.load_from_file(&cfg_path);
    }

    let mut paths = Vec::new();
    let mut startup_commands = Vec::new();
    let mut on_image_change_commands = Vec::new();
    let mut app_id = "ai.opencode.unframed".to_string();
    let mut start_at_index_or_path: Option<String> = None;
    let mut _verbose = false;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-h" || arg == "--help" {
            print_help();
            return;
        } else if arg == "-v" || arg == "--version" {
            println!("ufm (unframed) version {VERSION}");
            return;
        } else if arg == "-b" && i + 1 < args.len() {
            i += 1;
            let val = &args[i];
            if val.eq_ignore_ascii_case("checks") {
                config.background = BackgroundMode::Checks;
            } else if let Some(c) = parse_hex_color(val, 255) {
                config.background = BackgroundMode::Solid(c);
            }
        } else if arg == "-c" && i + 1 < args.len() {
            i += 1;
            startup_commands.push(args[i].clone());
        } else if arg == "-C" && i + 1 < args.len() {
            i += 1;
            on_image_change_commands.push(args[i].clone());
        } else if arg == "-d" {
            config.overlay_enabled = true;
        } else if arg == "-f" {
            config.fullscreen = true;
        } else if arg == "-i" && i + 1 < args.len() {
            i += 1;
            app_id = args[i].clone();
        } else if arg == "-l" {
            config.list_files_at_exit = true;
        } else if arg == "-n" && i + 1 < args.len() {
            i += 1;
            start_at_index_or_path = Some(args[i].clone());
        } else if arg == "-r" {
            config.recursive_load = true;
        } else if arg == "-s" && i + 1 < args.len() {
            i += 1;
            if let Some(mode) = ScalingMode::parse(&args[i]) {
                config.scaling_mode = mode;
            }
        } else if arg == "-t" && i + 1 < args.len() {
            i += 1;
            if let Ok(d) = args[i].parse::<f64>() {
                config.slideshow_duration = d;
            }
        } else if arg == "-u" && i + 1 < args.len() {
            i += 1;
            if let Some(m) = UpscalingMethod::parse(&args[i]) {
                config.upscaling_method = m;
            }
        } else if arg == "-w" && i + 1 < args.len() {
            i += 1;
            config.title_text = args[i].clone();
        } else if arg == "-x" {
            config.loop_input = false;
        } else if arg == "-V" {
            _verbose = true;
        } else if arg == "-W" && i + 1 < args.len() {
            i += 1;
            if let Ok(w) = args[i].parse::<i32>() {
                config.width = w;
            }
        } else if arg == "-H" && i + 1 < args.len() {
            i += 1;
            if let Ok(h) = args[i].parse::<i32>() {
                config.height = h;
            }
        } else if !arg.starts_with('-') || arg == "-" {
            paths.push(arg.clone());
        }
        i += 1;
    }

    // Handle stdin
    let stdin_is_piped = !io::stdin().is_terminal();
    let mut stdin_data = None;

    if paths.iter().any(|p| p == "-") {
        let mut buf = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut buf) {
            eprintln!("Failed to read stdin: {e}");
        } else {
            stdin_data = Some(buf);
        }
    } else if paths.is_empty() && stdin_is_piped {
        let stdin = io::stdin();
        for line in stdin.lock().lines().map_while(Result::ok) {
            let trimmed = line.trim();
            if !trimmed.is_empty() && Path::new(trimmed).exists() {
                paths.push(trimmed.to_string());
            }
        }
    }

    // Initial window configuration
    let iris_cfg = iris::Config::new(&config.title_text)
        .expect("Valid window title")
        .app_id(&app_id)
        .expect("Valid app ID")
        .size(config.width, config.height);

    let app = Rc::new(RefCell::new(App::new(
        config,
        paths,
        stdin_data,
        startup_commands,
        on_image_change_commands,
    )));

    // Handle start_at_index_or_path if provided
    if let Some(start_target) = start_at_index_or_path {
        let mut app_mut = app.borrow_mut();
        if let Ok(idx) = start_target.parse::<isize>() {
            app_mut.navigator.select_abs(idx);
        } else if let Some(pos) = app_mut.navigator.find_path(&start_target) {
            app_mut.navigator.select_abs((pos + 1) as isize);
        }
        app_mut.reload_current_image();
    }

    let app_build = Rc::clone(&app);
    let app_paint = Rc::clone(&app);

    let build_cb = move |frame: &mut iris::Frame<'_>, input: &iris::Input| {
        app_build.borrow_mut().on_build(frame, input);
    };

    let paint_cb = move |paint_host: iris::PaintHost| {
        app_paint.borrow_mut().on_paint(&paint_host);
    };

    if let Err(e) = iris::Application::run(iris_cfg, build_cb, Some(paint_cb)) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
