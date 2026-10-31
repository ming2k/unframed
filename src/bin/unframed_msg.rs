//! unframed-msg companion binary for IPC scripting.

use std::env;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <pid> <command ...>", args[0]);
        std::process::exit(1);
    }

    let pid: u32 = match args[1].parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Invalid PID: {}", args[1]);
            std::process::exit(1);
        }
    };

    let command = args[2..].join(" ");

    let mut candidate_dirs = Vec::new();
    if let Ok(dir) = env::var("XDG_RUNTIME_DIR") {
        candidate_dirs.push(PathBuf::from(dir));
    }
    candidate_dirs.push(PathBuf::from("/tmp"));

    let mut stream = None;
    for dir in candidate_dirs {
        let p1 = dir.join(format!("unframed-{pid}.sock"));
        let p2 = dir.join(format!("imv-{pid}.sock"));
        if let Ok(s) = UnixStream::connect(&p1) {
            stream = Some(s);
            break;
        }
        if let Ok(s) = UnixStream::connect(&p2) {
            stream = Some(s);
            break;
        }
    }

    let mut stream = match stream {
        Some(s) => s,
        None => {
            eprintln!("Failed to connect to unframed instance with PID {pid}");
            std::process::exit(1);
        }
    };

    if let Err(e) = writeln!(stream, "{command}") {
        eprintln!("Failed to send command: {e}");
        std::process::exit(1);
    }
}
