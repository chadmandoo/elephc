//! Purpose:
//! Home of the PHP `array_slice` builtin: its single-source registry declaration and semantic target.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - `check` reproduces the legacy rule: a slice preserves the array shape, so the
//!   return type is the (array-or-assoc) input type unchanged; a boxed `Mixed`/`Union`
//!   input yields `Mixed`. A check hook is required because the return type depends on
//!   the inferred first-argument type.
//! - The declared signature carries the golden param list (`array`, `offset`,
//!   `length`), with `length` optional (default `null`), so the registry's
//!   `check_arity` accepts 2 or 3 arguments — matching the legacy CHECK arm.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "array_slice",
    area: Array,
    params: [array: Mixed, offset: Mixed, length: Mixed = DefaultSpec::Null],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::ArraySlice,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Extracts a slice of an array.",
    php_manual: "https://www.php.net/manual/en/function.array-slice.php",
}

/// Returns the slice's array type for an `array_slice` call.
///
/// A slice preserves the input array shape, so the (array-or-assoc) first-argument
/// type is returned unchanged; a boxed `Mixed`/`Union` first argument yields `Mixed`.
/// Non-array first arguments are rejected. The first argument is re-inferred here;
/// the registry already inferred every argument once for side effects, and arity
/// (2 or 3) is pre-validated by the registry.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let ty = cx.checker.infer_type(&cx.args[0], cx.env)?;
    if matches!(ty, PhpType::Mixed | PhpType::Union(_)) {
        return Ok(PhpType::Mixed);
    }
    if !matches!(ty, PhpType::Array(_) | PhpType::AssocArray { .. }) {
        return Err(CompileError::new(
            cx.span,
            "array_slice() first argument must be array",
        ));
    }
    Ok(ty)
}
