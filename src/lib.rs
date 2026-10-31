//! unframed - Modern Wayland image viewer built with Rust and Optics.

#![allow(clippy::chunks_exact_to_as_chunks)]

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
