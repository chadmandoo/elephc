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

/// Runs the compiled binary with the given args, returns (stdout, exit code).
fn run_bin(bin: &Path, args: &[&str]) -> (String, i32) {
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to spawn compiled web binary");
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        output.status.code().unwrap_or(-1),
    )
}

/// Verifies a trivial program compiles under --web and produces an executable file.
#[test]
fn web_compile_produces_binary() {
    let dir = make_test_dir("web_compile");
    let bin = compile_web(&dir, "<?php echo \"Hello World\";", "app");
    assert!(bin.exists(), "expected binary at {}", bin.display());
}

/// Verifies the restructured entry runs the top-level body once via the bridge
/// scaffold (server loop arrives in Task 7). Now that per-request output capture
/// is active and the scaffold runs the handler TWICE, both runs' bytes flow
/// top-level body → `_elephc_web_handler` → `elephc_web_write` capture buffer,
/// which the scaffold flushes to stdout once, so `"Hello World"` appears twice.
/// The entry stub exits with the bridge's return code (0).
#[test]
fn web_scaffold_runs_handler_once() {
    let dir = make_test_dir("web_scaffold");
    let bin = compile_web(&dir, "<?php echo \"Hello World\";", "app");
    let (stdout, code) = run_bin(&bin, &[]);
    assert_eq!(stdout, "Hello WorldHello World");
    assert_eq!(code, 0);
}

/// Verifies per-request reset of top-level PHP variables: a global mutated each
/// run does not accumulate across the scaffold's two handler invocations. Note:
/// top-level `$g` is a handler LOCAL, so this passes via the handler prologue's
/// per-request zero-init, NOT via `__rt_web_reset`; it does not isolate the
/// static-state reset (the two tests below do).
#[test]
fn web_reset_clears_globals_between_runs() {
    let dir = make_test_dir("web_reset");
    // $g is a top-level handler local: reset to "" then appended each run, so it
    // yields "x" both runs, not "x" then "xx". (elephc rejects reading an
    // undefined variable, so the var is written before it is read.)
    let src = "<?php $g = \"\"; $g = $g . \"x\"; echo $g;";
    let bin = compile_web(&dir, src, "app");
    let (stdout, code) = run_bin(&bin, &[]);
    assert_eq!(stdout, "xx"); // scaffold runs handler twice; each run prints "x"
    assert_eq!(code, 0);
}

/// Verifies per-request reset of a FUNCTION STATIC local: `static $n` must be
/// re-initialized to 0 each request, so each of the scaffold's two handler runs
/// prints `"1"`. Without `__rt_web_reset` the init marker would persist and `$n`
/// would accumulate (`"1"` then `"2"` → `"12"`); the reset makes it `"11"`.
#[test]
fn web_reset_clears_function_static() {
    let dir = make_test_dir("web_reset_static");
    let src = "<?php function c() { static $n = 0; $n++; return $n; } echo c();";
    let bin = compile_web(&dir, src, "app");
    let (stdout, code) = run_bin(&bin, &[]);
    assert_eq!(stdout, "11");
    assert_eq!(code, 0);
}

/// Verifies per-request reset of a STATIC CLASS PROPERTY: `C::$n` must read fresh
/// each request, so each of the scaffold's two handler runs prints `"1"`. The
/// handler re-runs the property initializer every request, and `__rt_web_reset`
/// releases the previous request's value first (no leak); without per-request
/// re-init the property would accumulate (`"1"` then `"2"` → `"12"`).
#[test]
fn web_reset_clears_static_property() {
    let dir = make_test_dir("web_reset_prop");
    // elephc supports `C::$n = C::$n + 1` but not `C::$n++` on a static property,
    // so the increment is written out longhand.
    let src = "<?php class C { public static int $n = 0; } C::$n = C::$n + 1; echo C::$n;";
    let bin = compile_web(&dir, src, "app");
    let (stdout, code) = run_bin(&bin, &[]);
    assert_eq!(stdout, "11");
    assert_eq!(code, 0);
}
