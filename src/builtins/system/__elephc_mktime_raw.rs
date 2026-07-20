//! Purpose:
//! Home of the internal `__elephc_mktime_raw` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - This is an internal builtin (`internal: true`) not exposed as a PHP-visible function.
//!   It is used by the synthetic DateTime body as a raw mktime alias.
//! - The lower hook delegates to the same emitter as `mktime`.


builtin! {
    name: "__elephc_mktime_raw",
    area: System,
    params: [hour: Int, minute: Int, second: Int, month: Int, day: Int, year: Int],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ElephcMktimeRaw,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Internal raw mktime alias used by the synthetic DateTime body.",
    internal: true,
}
