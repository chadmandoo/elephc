//! Purpose:
//! Home of the PHP `realpath_cache_size` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `realpath_cache_size` is a pure-data builtin whose
//!   return type (`Int`) is fully determined by its declaration.
//! - `arity_error` is overridden to preserve the legacy message
//!   "realpath_cache_size() takes exactly 0 arguments" (the registry default for
//!   0-arg builtins produces "takes no arguments").
//! - `lower` is a thin wrapper over `io::lower_realpath_cache_size` in the EIR backend.


builtin! {
    name: "realpath_cache_size",
    area: Io,
    params: [],
    arity_error: "realpath_cache_size() takes exactly 0 arguments",
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::RealpathCacheSize,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the amount of memory used by the realpath cache.",
    php_manual: "function.realpath-cache-size",
}
