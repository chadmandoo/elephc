//! Purpose:
//! Home of the PHP `tan` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `tan` is a pure-data builtin whose return type
//!   (`Float`) is fully determined by its declaration. The registry common path
//!   infers the argument and enforces arity before falling back to `returns`.


builtin! {
    name: "tan",
    area: Math,
    params: [num: Float],
    returns: Float,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Tan,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Returns the tangent of a number (radians).",
    php_manual: "https://www.php.net/manual/en/function.tan.php",
}
