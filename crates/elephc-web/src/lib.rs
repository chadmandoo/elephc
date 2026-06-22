//! Purpose:
//! C-ABI surface for the elephc `--web` prefork HTTP server bridge. Exposes the
//! server entry point and (in later phases) request/response marshaling under
//! `#[no_mangle] extern "C"` symbols the compiled PHP program calls/links.
//!
//! Called from:
//! - Compiled `--web` binaries: the emitted process entry tail-calls
//!   `elephc_web_run`; the staticlib is linked via the `BRIDGES` table in
//!   `crate::linker`.
//! - Tests: directly through the `rlib` crate type.
//!
//! Key details:
//! - One process per prefork worker means no shared-thread state: per-worker
//!   request/response data lives in plain process statics, not behind a mutex.

mod server;

/// Returns the elephc-web C ABI version. Bumped when the exported symbol set or
/// any symbol's signature changes shape.
#[no_mangle]
pub extern "C" fn elephc_web_version() -> i32 {
    1
}

/// Appends response-body bytes for the current request. Phase 1 stub: a no-op
/// until Task 6 wires the per-worker response buffer. The compiled `--web`
/// runtime references this symbol from `__rt_stdout_write`'s capture branch, so
/// it must resolve at link time even before the buffer exists. Because
/// `_elephc_web_capture` defaults to 0, the capture branch is never taken at
/// runtime in Phase 1, so echo still reaches stdout via the syscall path.
///
/// # Safety
/// `ptr` must point to `len` valid bytes for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn elephc_web_write(_ptr: *const u8, _len: usize) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the crate links and the ABI version constant is the v1 value.
    #[test]
    fn version_is_one() {
        assert_eq!(elephc_web_version(), 1);
    }
}
