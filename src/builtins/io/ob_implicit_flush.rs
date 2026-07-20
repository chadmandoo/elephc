//! Purpose:
//! Home of the PHP `ob_implicit_flush` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The stored flag is semantically inert in elephc: terminal writes are
//! -   unbuffered syscalls, so implicit flushing is always effectively on.
//! - Returns `true` like PHP 8.
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_implicit_flush`.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "ob_implicit_flush",
    area: Io,
    params: [enable: Bool = DefaultSpec::Bool(true)],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObImplicitFlush,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Turns implicit flush on/off.",
    php_manual: "function.ob-implicit-flush",
}
