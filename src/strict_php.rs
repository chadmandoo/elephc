//! Purpose:
//! Owns the `--strict-php` compilation mode: the thread-local enablement state
//! consulted by the builtin catalog and checker, and (in later sections) the
//! AST audit pass that rejects elephc-only syntax extensions.
//!
//! Called from:
//! - `crate::pipeline::compile()` (sets the mode from the CLI flag).
//! - `crate::types::checker::builtins::catalog` (filters extension builtins).
//! - Test helpers that drive compiler phases in-process.
//!
//! Key details:
//! - The state is a thread-local `Cell`, mirroring `codegen_support`'s
//!   `AUTOLOAD_RULE_COUNT` precedent, so parallel test runs cannot interfere
//!   (the compile pipeline runs on a single thread per invocation).
//! - Strict mode must never hide `internal: true` builtins: injected compiler
//!   preludes call them, and they are already invisible to user programs.

mod audit;

pub use audit::check;

use std::cell::Cell;

thread_local! {
    /// Whether `--strict-php` is active for the compilation running on this thread.
    static STRICT_PHP: Cell<bool> = const { Cell::new(false) };
}

/// Enables or disables strict-PHP mode for the current thread's compilation.
pub fn set_enabled(enabled: bool) {
    STRICT_PHP.with(|cell| cell.set(enabled));
}

/// Returns whether strict-PHP mode is active for the current thread's compilation.
pub fn is_enabled() -> bool {
    STRICT_PHP.with(|cell| cell.get())
}

/// Audits one user source file's parsed statements when strict mode is active.
///
/// No-op (always `Ok`) when strict mode is off. Violations are bundled into a
/// single `CompileError` (primary + related) attributed to `file`, so callers
/// that parse user files mid-pipeline — the include resolver and the autoloader
/// — can propagate every violation through their existing single-error paths.
/// Must run on the freshly parsed AST, before any pass synthesizes
/// compiler-internal names or nodes into it.
pub fn check_file(
    program: &crate::parser::ast::Program,
    file: &str,
) -> Result<(), crate::errors::CompileError> {
    if !is_enabled() {
        return Ok(());
    }
    let violations = check(program);
    if violations.is_empty() {
        return Ok(());
    }
    Err(crate::errors::CompileError::from_many(violations).with_file(file.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies strict mode defaults to off so ordinary compiles are unaffected.
    #[test]
    fn strict_mode_defaults_off() {
        assert!(!is_enabled());
    }

    /// Verifies enabling and disabling strict mode round-trips on the same thread.
    #[test]
    fn strict_mode_set_and_clear_roundtrip() {
        set_enabled(true);
        assert!(is_enabled());
        set_enabled(false);
        assert!(!is_enabled());
    }

    /// Verifies strict mode is thread-local: enabling it on another thread must
    /// not leak into this thread's compilation state.
    #[test]
    fn strict_mode_is_thread_local() {
        let handle = std::thread::spawn(|| {
            set_enabled(true);
            is_enabled()
        });
        assert!(handle.join().expect("thread must not panic"));
        assert!(!is_enabled());
    }
}
