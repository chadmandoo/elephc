//! Purpose:
//! Home of the PHP `sys_get_temp_dir` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `sys_get_temp_dir` is a pure-data builtin whose `Str` return
//!   type is fully determined by its declaration. The registry common path enforces
//!   its 0-argument arity before falling back to `returns`.
//! - `lower` is a thin wrapper over `io::lower_sys_get_temp_dir` in the EIR backend.


builtin! {
    name: "sys_get_temp_dir",
    area: Io,
    params: [],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::SysGetTempDir,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Returns the directory path used for temporary files.",
    php_manual: "function.sys-get-temp-dir",
}
