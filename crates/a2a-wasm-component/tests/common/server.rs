use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

pub struct TestServer {
    process: Child,
    pub url: String,
}

impl TestServer {
    pub fn start() -> Self {
        let process = Command::new("uv")
            .args(["run", "."])
            .current_dir("tests/fixtures/helloworld")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("uv must be installed");

        wait_for_port(9999, Duration::from_secs(10));

        Self {
            process,
            url: "http://localhost:9999".into(),
        }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

fn wait_for_port(port: u16, timeout: Duration) {
    // Poll until port is accepting connections
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!("Server did not start within {:?}", timeout);
}
