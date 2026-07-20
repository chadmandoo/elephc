//! Purpose:
//! Home of the PHP `strtotime` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` always returns `Union(Int, Bool)` to reflect PHP's behaviour where
//!   `strtotime` returns a Unix timestamp on success or `false` on failure.
//! - `lower` is a thin wrapper over the shared `system::lower_strtotime` emitter.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "strtotime",
    area: System,
    params: [datetime: Str, baseTimestamp: Int = DefaultSpec::Null],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Strtotime,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Parses an English textual datetime description into a Unix timestamp.",
}

/// Returns `Union(Int, Bool)` to reflect that `strtotime` can return a timestamp or `false`.
///
/// The registry pre-infers arguments before calling this hook.
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::Union(vec![PhpType::Int, PhpType::False]))
}
