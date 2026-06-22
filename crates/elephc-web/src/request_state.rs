//! Purpose:
//! Owns the per-worker request/response state the `--web` bridge shares with the
//! compiled PHP runtime: the output-capture flag the runtime reads and the
//! response-body buffer the runtime appends to. Provides the real
//! `elephc_web_write` (replacing the Phase-1 no-op stub) plus the buffer
//! lifecycle helpers the server scaffold drives each request.
//!
//! Called from:
//! - The compiled `--web` runtime's `__rt_stdout_write` capture branch, which
//!   calls `elephc_web_write(ptr, len)` when `elephc_web_capture` is non-zero.
//! - `crate::server::elephc_web_run`, which sets capture, clears the buffer,
//!   runs the handler, and flushes the captured body.
//!
//! Key details:
//! - One process per prefork worker, single-threaded: each request runs to
//!   completion on the worker's one thread, so the process-static buffer and the
//!   extern capture flag are never touched concurrently and need no lock.
//! - All access to the `static mut` buffer and the extern `static mut` flag goes
//!   through raw pointers (`core::ptr::addr_of_mut!`), never `&mut`/`&`
//!   references, to stay clear of the `static_mut_refs` lint (a hard error under
//!   the workspace's zero-warnings gate).

extern "C" {
    /// Per-request output-capture flag defined in the compiled program's runtime
    /// `.comm` storage (`elephc_web_capture`). Non-zero routes the runtime's
    /// `__rt_stdout_write` through `elephc_web_write` instead of the plain
    /// `write(1, …)` syscall. The compiler mangles this name per target, so the
    /// clean C name here resolves to `_elephc_web_capture` on macOS and
    /// `elephc_web_capture` on Linux — matching the runtime's `.comm` and load.
    static mut elephc_web_capture: u8;
}

/// Process-static per-worker response body. Bytes echoed by the PHP handler land
/// here while capture is enabled; the server scaffold flushes it to the client
/// (currently stdout) once the handler returns.
static mut RESPONSE_BODY: Vec<u8> = Vec::new();

/// Enables or disables per-request output capture by writing the runtime's
/// extern capture flag. When `on` is true, `__rt_stdout_write` routes echo
/// output to `elephc_web_write` (the buffer below) instead of stdout.
///
/// # Safety
/// Single-threaded per worker (see module docs): the extern flag is reached only
/// through a raw pointer, never a reference to the `static mut`.
pub fn set_capture(on: bool) {
    unsafe {
        core::ptr::write(core::ptr::addr_of_mut!(elephc_web_capture), u8::from(on));
    }
}

/// Clears the response-body buffer before a request begins, so each request
/// starts with an empty body regardless of the previous request's output.
pub fn clear_body() {
    // SAFETY: single-threaded per worker; the buffer is mutated through a raw
    // pointer to avoid forming a reference to the `static mut`.
    unsafe {
        (*core::ptr::addr_of_mut!(RESPONSE_BODY)).clear();
    }
}

/// Appends `len` bytes starting at `ptr` to the per-worker response body. This
/// is the real destination for captured PHP output: the compiled runtime's
/// `__rt_stdout_write` capture branch calls this with the same C ABI as the
/// Phase-1 stub (byte pointer + length, no return value).
///
/// # Safety
/// `ptr` must point to `len` valid bytes for the duration of the call. Single-
/// threaded per worker (see module docs), so the buffer append cannot race.
#[no_mangle]
pub unsafe extern "C" fn elephc_web_write(ptr: *const u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    let bytes = core::slice::from_raw_parts(ptr, len);
    (*core::ptr::addr_of_mut!(RESPONSE_BODY)).extend_from_slice(bytes);
}

/// Takes ownership of the accumulated response body, leaving the buffer empty for
/// the next request. The server scaffold writes the returned bytes to the client.
pub fn take_body() -> Vec<u8> {
    // SAFETY: single-threaded per worker; the buffer is replaced through a raw
    // pointer to avoid forming a reference to the `static mut`.
    unsafe { core::mem::take(&mut *core::ptr::addr_of_mut!(RESPONSE_BODY)) }
}
