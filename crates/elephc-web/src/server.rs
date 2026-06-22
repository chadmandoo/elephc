//! Purpose:
//! The `--web` server entry point. Phase 1 scaffold: enables per-request output
//! capture, runs the PHP handler twice (proving per-request state reset), and
//! flushes the captured response body once — all before the prefork/hyper server
//! loop lands (Task 7 replaces the body).
//!
//! Called from:
//! - The compiled `--web` binary's process entry, which tail-calls
//!   `elephc_web_run` with argc/argv and the address of `_elephc_web_handler`.
//!
//! Key details:
//! - The handler is a C-ABI `extern "C" fn()`: it runs the PHP top-level body
//!   and returns (it does not exit the process).
//! - With capture enabled, the handler's echo output is appended to the
//!   per-worker response buffer instead of stdout; the scaffold flushes that
//!   buffer to stdout once after both runs. Running the handler twice makes
//!   per-request state reset (`__rt_web_reset`) observable: correct reset yields
//!   the single-run output doubled, not accumulated.

use std::io::Write;

use crate::request_state;

/// Runs the compiled PHP program as a server. Phase 1 scaffold: enables capture,
/// clears the response buffer, invokes the handler twice, then writes the
/// captured body to stdout once and returns success. Returns a process exit code.
///
/// # Safety
/// `handler` must be the compiler-emitted `_elephc_web_handler` symbol.
#[no_mangle]
pub extern "C" fn elephc_web_run(
    _argc: i32,
    _argv: *const *const u8,
    handler: extern "C" fn(),
) -> i32 {
    request_state::set_capture(true);
    request_state::clear_body();
    handler();
    handler();
    let body = request_state::take_body();
    let _ = std::io::stdout().write_all(&body);
    let _ = std::io::stdout().flush();
    0
}
