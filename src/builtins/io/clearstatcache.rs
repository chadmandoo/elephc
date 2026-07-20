//! Purpose:
//! Home of the PHP `clearstatcache` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `clearstatcache` is a pure-data builtin whose return
//!   type (`Void`) is fully determined by its declaration. The registry common path
//!   infers arguments and enforces arity before falling back to `returns`.
//! - PHP accepts up to 2 optional arguments; elephc has no stat cache but accepts
//!   and ignores them (matching legacy behavior).
//! - `lower` is a thin wrapper over `io::lower_clearstatcache` in the EIR backend.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "clearstatcache",
    area: Io,
    params: [
        clear_realpath_cache: Bool = DefaultSpec::Bool(false),
        filename: Str = DefaultSpec::Str("")
    ],
    returns: Void,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Clearstatcache,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Clears file status cache.",
    php_manual: "function.clearstatcache",
}
