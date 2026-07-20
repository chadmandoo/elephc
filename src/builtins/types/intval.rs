//! Purpose:
//! Home of the PHP `intval` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.
//! - Declared with exactly one parameter `value` (no `base` param) matching the legacy golden signature.
//! - `lower` is a thin wrapper over the shared intval emitter.


builtin! {
    name: "intval",
    area: Types,
    params: [value: Mixed],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Intval,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the integer value of a variable.",
    php_manual: "function.intval",
}
