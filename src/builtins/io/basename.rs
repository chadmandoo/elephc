//! Purpose:
//! Home of the PHP `basename` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `basename` is a pure-data builtin whose return type
//!   (`Str`) is fully determined by its declaration. The registry common path
//!   infers arguments and enforces arity before falling back to `returns`.
//! - `lower` is a thin wrapper over `io::lower_basename` in the EIR backend.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "basename",
    area: Io,
    params: [path: Str, suffix: Str = DefaultSpec::Str("")],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Basename,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the trailing name component of a path.",
    php_manual: "function.basename",
}
