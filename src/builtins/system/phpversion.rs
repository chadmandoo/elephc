//! Purpose:
//! Home of the PHP `phpversion` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with zero parameters: return type (`Str`) is fully determined
//!   by the declaration. elephc returns the compiler package version string.
//! - `lower` delegates to the module-level `lower_phpversion` in
//!   `src/codegen/lower_inst/builtins.rs`.


builtin! {
    name: "phpversion",
    area: System,
    params: [],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Phpversion,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Returns the current PHP / elephc compiler version string.",
}
