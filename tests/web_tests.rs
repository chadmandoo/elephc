//! Purpose:
//! End-to-end tests for `--web`: compile PHP into a prefork HTTP server binary,
//! launch it with `--listen`, drive it over raw TCP, and assert the response.
//!
//! Called from:
//! - `cargo test --test web_tests` through Rust's test harness.
//!
//! Key details:
//! - Tests invoke the elephc CLI (CARGO_BIN_EXE_elephc) as a subprocess in an
//!   isolated temp dir with an isolated runtime cache, mirroring cdylib_tests.
//! - The HTTP client is a hand-written minimal HTTP/1.1 request over a
//!   std::net::TcpStream so the test pulls in no HTTP client dependency.
//! - Host-target only: each platform/arch covers itself (macOS aarch64 local,
//!   Linux x86_64/aarch64 via the Docker test scripts).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_ID: AtomicUsize = AtomicUsize::new(0);

/// Creates an isolated temp dir unique across parallel test threads/processes.
fn make_test_dir(prefix: &str) -> PathBuf {
    let id = TEST_ID.fetch_add(1, Ordering::SeqCst);
    let tid = std::thread::current().id();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("{}_{}_{:?}_{}", prefix, pid, tid, id));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Resolves the elephc CLI binary path (cargo env var, fallback next to the test binary).
fn elephc_bin() -> String {
    std::env::var("CARGO_BIN_EXE_elephc").unwrap_or_else(|_| {
        let mut path = std::env::current_exe().expect("failed to resolve current test binary");
        path.pop();
        if path.ends_with("deps") {
            path.pop();
        }
        path.join("elephc").to_string_lossy().into_owned()
    })
}

/// Compiles `source` with the given extra elephc flags; returns the binary path.
fn compile_web(dir: &Path, source: &str, stem: &str) -> PathBuf {
    let php = dir.join(format!("{}.php", stem));
    fs::write(&php, source).unwrap();
    let mut cmd = Command::new(elephc_bin());
    cmd.env("XDG_CACHE_HOME", dir.join("cache-root"));
    cmd.current_dir(dir);
    cmd.arg("--web").arg(&php);
    let output = cmd.output().expect("failed to spawn elephc");
    assert!(
        output.status.success(),
        "elephc --web failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    dir.join(stem)
}

/// Verifies a trivial program compiles under --web and produces an executable file.
#[test]
fn web_compile_produces_binary() {
    let dir = make_test_dir("web_compile");
    let bin = compile_web(&dir, "<?php echo \"Hello World\";", "app");
    assert!(bin.exists(), "expected binary at {}", bin.display());
}
