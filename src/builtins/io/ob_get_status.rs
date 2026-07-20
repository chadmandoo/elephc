//! Purpose:
//! Home of the PHP `ob_get_status` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `AssocArray<Mixed, Mixed>`: simple mode yields the top
//!   buffer's status (string keys), full mode a list of per-level status arrays.
//! - Every entry reports the default output handler (user handlers unsupported).

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "ob_get_status",
    area: Io,
    params: [full_status: Bool = DefaultSpec::Bool(false)],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::runtime_fn_semantics(
        crate::ir::RuntimeFnId::ObGetStatus,
    ),
    summary: "Gets status of output buffers.",
    php_manual: "function.ob-get-status",
}

/// Returns `AssocArray<Mixed, Mixed>`: string-keyed status fields in simple mode,
/// an int-keyed list of per-level status arrays in full mode.
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::AssocArray {
        key: Box::new(PhpType::Mixed),
        value: Box::new(PhpType::Mixed),
    })
}
