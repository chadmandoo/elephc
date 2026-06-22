//! Purpose:
//! The `--web` server entry point. Phase 1 scaffold: validates nothing yet and
//! invokes the PHP handler once so the entry-point restructuring can be tested
//! before the prefork/hyper server loop lands (Task 7 replaces the body).
//!
//! Called from:
//! - The compiled `--web` binary's process entry, which tail-calls
//!   `elephc_web_run` with argc/argv and the address of `_elephc_web_handler`.
//!
//! Key details:
//! - The handler is a C-ABI `extern "C" fn()`: it runs the PHP top-level body
//!   and returns (it does not exit the process).

/// Runs the compiled PHP program as a server. Phase 1 scaffold: calls the
/// handler once and returns success. Returns a process exit code.
///
/// # Safety
/// `handler` must be the compiler-emitted `_elephc_web_handler` symbol.
#[no_mangle]
pub extern "C" fn elephc_web_run(
    _argc: i32,
    _argv: *const *const u8,
    handler: extern "C" fn(),
) -> i32 {
    handler();
    0
}
