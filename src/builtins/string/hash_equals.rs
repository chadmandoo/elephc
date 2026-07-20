//! Purpose:
//! Home of the PHP `hash_equals` builtin: declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook is needed: `returns: Bool` expresses the return type inline and no
//!   bridge library is required (this is a pure timing-safe byte comparison).
//! - Arity (exactly 2 args) is validated by the registry.


builtin! {
    name: "hash_equals",
    area: String,
    params: [known_string: Str, user_string: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::HashEquals,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Compares two strings using a constant-time algorithm.",
    php_manual: "https://www.php.net/manual/en/function.hash-equals.php",
}
