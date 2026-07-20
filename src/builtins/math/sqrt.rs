//! Purpose:
//! Home of the PHP `sqrt` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `sqrt` is a pure-data builtin whose return type
//!   (`Float`) is fully determined by its declaration. The registry common path
//!   infers the argument and enforces arity before falling back to `returns`.


builtin! {
    name: "sqrt",
    area: Math,
    params: [num: Float],
    returns: Float,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Sqrt,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the square root of a number.",
    php_manual: "https://www.php.net/manual/en/function.sqrt.php",
}
