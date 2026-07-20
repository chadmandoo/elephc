//! Purpose:
//! Home of the PHP `header` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Void`) is fully determined by the declaration.
//! - `lower` is a thin wrapper over `system::lower_header` in the EIR backend.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "header",
    area: System,
    params: [header: Str, replace: Bool = DefaultSpec::Bool(true), response_code: Int = DefaultSpec::Int(0)],
    returns: Void,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Header,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Sends a raw HTTP header.",
}
