//! Purpose:
//! Home of the PHP `cosh` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `cosh` is a pure-data builtin whose return type
//!   (`Float`) is fully determined by its declaration. The registry common path
//!   infers the argument and enforces arity before falling back to `returns`.


builtin! {
    name: "cosh",
    area: Math,
    params: [num: Float],
    returns: Float,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Cosh,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Returns the hyperbolic cosine of a number.",
    php_manual: "https://www.php.net/manual/en/function.cosh.php",
}
