//! Purpose:
//! Home of the PHP `stream_socket_get_name` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates arg[0] is a stream resource, then returns `Union(Str, Bool)`.
//! - Arguments are pre-inferred by the registry before the hook runs.
//! - `lower` dispatches to `io::lower_stream_socket_get_name` in the EIR backend.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "stream_socket_get_name",
    area: Io,
    params: [socket: Mixed, remote: Bool],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamSocketGetName,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Retrieve the name of the local or remote sockets.",
    php_manual: "function.stream-socket-get-name",
}

/// Validates arg[0] is a stream resource, then returns `Union(Str, Bool)`.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    crate::types::checker::builtins::io::common::ensure_stream_resource(cx.checker, cx.name, &cx.args[0], cx.env)?;
    Ok(cx.checker.normalize_union_type(vec![PhpType::Str, PhpType::False]))
}
