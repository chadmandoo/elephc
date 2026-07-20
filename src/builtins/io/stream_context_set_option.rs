//! Purpose:
//! Home of the PHP `stream_context_set_option` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers all arguments and returns `Bool`.
//!   PHP accepts two call shapes — (ctx, options_array) or (ctx, wrapper, option, value) —
//!   both accepted inertly.
//! - `lower` is a thin wrapper over `io::lower_stream_context_set_option` in the EIR backend.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "stream_context_set_option",
    area: Io,
    params: [
        context: Mixed,
        wrapper_or_options: Mixed,
        option_name: Str = DefaultSpec::Null,
        value: Mixed = DefaultSpec::Null
    ],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamContextSetOption,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Sets an option on the specified context.",
    php_manual: "function.stream-context-set-option",
}
