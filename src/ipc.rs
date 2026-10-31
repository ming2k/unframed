//! Inter-Process Communication via UNIX domain sockets, matching imv-msg.

use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;

pub fn get_socket_paths(pid: u32) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(runtime) = env::var("XDG_RUNTIME_DIR") {
        dirs.push(PathBuf::from(runtime));
    }
    dirs.push(PathBuf::from("/tmp"));

    let mut paths = Vec::new();
    for d in dirs {
        paths.push(d.join(format!("imv-{pid}.sock")));
        paths.push(d.join(format!("unframed-{pid}.sock")));
    }
    paths
}

pub struct IpcServer {
    pub bound_paths: Vec<PathBuf>,
}

impl IpcServer {
    pub fn start(command_tx: Sender<String>) -> Option<Self> {
        let pid = std::process::id();
        let candidate_paths = get_socket_paths(pid);
        Self::start_with_paths(command_tx, candidate_paths)
    }

    pub fn start_with_paths(command_tx: Sender<String>, candidate_paths: Vec<PathBuf>) -> Option<Self> {
        let mut bound_paths = Vec::new();
        let mut listeners = Vec::new();

        for path in candidate_paths {
            // Remove any stale socket
            let _ = fs::remove_file(&path);
            if let Ok(listener) = UnixListener::bind(&path) {
                bound_paths.push(path);
                listeners.push(listener);
                if bound_paths.len() >= 2 {
                    break;
                }
            }
        }

        if listeners.is_empty() {
            return None;
        }

        for listener in listeners {
            let tx = command_tx.clone();
            thread::spawn(move || {
                for stream in listener.incoming().flatten() {
                    let tx_conn = tx.clone();
                    thread::spawn(move || {
                        let reader = BufReader::new(stream);
                        for line in reader.lines().map_while(Result::ok) {
                            let trimmed = line.trim().to_string();
                            if !trimmed.is_empty() {
                                let _ = tx_conn.send(trimmed);
                                let _ = iris::post_to_main_thread(|| {
                                    iris::request_animation_frame();
                                });
                            }
                        }
                    });
                }
            });
        }

        Some(Self { bound_paths })
    }
}

impl Drop for IpcServer {
    fn drop(&mut self) {
        for path in &self.bound_paths {
            let _ = fs::remove_file(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::os::unix::net::UnixStream;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(1);

    #[test]
    fn test_ipc_roundtrip() {
        let (tx, rx) = channel();
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let sock_path = std::env::temp_dir().join(format!("test_ipc_{}_{}.sock", std::process::id(), id));

        let server = IpcServer::start_with_paths(tx, vec![sock_path.clone()]).expect("start ipc server");
        assert!(!server.bound_paths.is_empty());

        let target_path = &server.bound_paths[0];
        let mut client = UnixStream::connect(target_path).expect("connect to ipc");
        writeln!(client, "next 2").expect("send line");
        client.flush().expect("flush line");

        let received = rx
            .recv_timeout(Duration::from_millis(1000))
            .expect("receive command");
        assert_eq!(received, "next 2");
    }
}
