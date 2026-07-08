//! Purpose:
//! Home of the PHP `array_keys` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` reproduces the legacy return-type rule: an indexed array yields
//!   `Array<Int>` (positional keys) while an associative array yields `Array<key>`.
//!   A check hook is required because the return type depends on the inferred
//!   argument type, which the `builtin!` `returns:` field cannot express.
//! - Arity (exactly 1 argument) is validated by the registry's `check_arity` before
//!   the hook fires; the inline arity check from the legacy arm is not reproduced here.
//! - `lower` is a thin wrapper over the shared `arrays::lower_array_keys` emitter.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "array_keys",
    area: Array,
    params: [array: Mixed],
    returns: Mixed,
    check: check,
    lower: lower,
    summary: "Returns all the keys of an array.",
    php_manual: "https://www.php.net/manual/en/function.array-keys.php",
}

/// Returns the key-array type for an `array_keys` call.
///
/// An indexed array produces `Array<Int>`; an associative array produces
/// `Array<key>`. Any other argument type is rejected. The argument is re-inferred
/// here to drive the return type; the registry already inferred it once for side
/// effects, and arity is pre-validated by the registry.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let ty = cx.checker.infer_type(&cx.args[0], cx.env)?;
    match ty {
        // A Mixed-element array is usually a bare `array` hint — its KEY
        // layout is erased too (an array<string,string> argument arrives
        // under it), so the positional-keys claim would be wrong statically
        // and the packed native walk wrong physically. Route adaptive when
        // the prelude impl exists (user programs mentioning array_keys);
        // synthesized-only programs keep the legacy native path, mirroring
        // the ir_lower desugar's availability guard.
        PhpType::Array(elem)
            if matches!(*elem, PhpType::Mixed)
                && cx.span.line != 0
                && cx.checker.functions.contains_key("__elephc_array_keys_any") =>
        {
            Ok(PhpType::Mixed)
        }
        PhpType::Array(_) => Ok(PhpType::Array(Box::new(PhpType::Int))),
        PhpType::AssocArray { key, .. } => Ok(PhpType::Array(key)),
        // Mixed receivers (json_decode results, adaptive locals) desugar to
        // the prelude `__elephc_array_keys_any` impl; keys may be int or
        // string, so the result shape stays Mixed.
        PhpType::Mixed => Ok(PhpType::Mixed),
        _ => Err(CompileError::new(
            cx.span,
            "array_keys() argument must be array",
        )),
    }
}

/// Lowers an `array_keys` call by dispatching to the shared array emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::arrays::lower_array_keys(ctx, inst)
}
