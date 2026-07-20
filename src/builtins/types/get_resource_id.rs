//! Purpose:
//! Home of the PHP `get_resource_id` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.
//! - The parameter is named `resource` (matching the PHP golden signature).
//! - `lower` is a thin wrapper over the EIR types-module resource-id emitter.


builtin! {
    name: "get_resource_id",
    area: Types,
    params: [resource: Mixed],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::GetResourceId,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Returns an integer identifier for the given resource.",
    php_manual: "function.get-resource-id",
}
