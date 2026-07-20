//! Purpose:
//! Home of the PHP `crc32` builtin: declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), both via
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook needed: `returns: Int` expresses the return type inline and no
//!   bridge library is required (crc32 is a pure table-free computation in __rt_crc32).
//! - Arity (exactly 1 arg) is validated by the registry.


builtin! {
    name: "crc32",
    area: String,
    params: [string: Str],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Crc32,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Calculates the CRC32 polynomial of a string.",
    php_manual: "https://www.php.net/manual/en/function.crc32.php",
}
