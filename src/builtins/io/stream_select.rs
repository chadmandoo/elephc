//! Purpose:
//! Home of the PHP `stream_select` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers all arguments and returns `Int`.
//! - `read`, `write`, and `except` are by-reference parameters (`ref` marker) for parity
//!   with PHP's mutating select semantics and EIR by-ref lowering.
//! - `lower` is a thin wrapper over `io::lower_stream_select` in the EIR backend.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "stream_select",
    area: Io,
    params: [
        ref read: Mixed,
        ref write: Mixed,
        ref except: Mixed,
        seconds: Int,
        microseconds: Int = DefaultSpec::Int(0)
    ],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamSelect,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Runs the equivalent of the select() system call on the given arrays of streams.",
    php_manual: "function.stream-select",
}
