//! Purpose:
//! Registers PHP's `strval` conversion with a typed backend operation.
//!
//! Called from:
//! - The builtin registry through `crate::builtins::types`.
//!
//! Key details:
//! - Conversion remains target-aware downstream while registry lowering is backend-neutral.

builtin! {
    name: "strval",
    area: Types,
    params: [value: Mixed],
    returns: Str,
    semantics: crate::builtins::semantics::runtime_target_semantics(
        crate::ir::BuiltinRuntimeTarget::Strval,
        crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the string value of a variable.",
    php_manual: "function.strval",
}
