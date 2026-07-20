//! Purpose:
//! Home of the PHP `count` builtin: its single-source registry declaration and semantic target.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates the argument type (Array, AssocArray, Mixed, Union-of-countable, or
//!   Countable Object) and returns `Int`. The Countable interface check delegates to
//!   `cx.checker.class_implements_interface`.
//! - `max_args: 1` reproduces the legacy checker's exactly-1 enforcement: `mode` has a
//!   default so `min` derives to 1; capping `max` at 1 yields the standard
//!   "count() takes exactly 1 argument" diagnostic. The 2-param golden is preserved for
//!   FCC and parity.
//! - Concrete indexed and associative arrays lower to `ArrayLen`/`HashLen`; dynamic
//!   `Mixed`, union, and `Countable` object values use the typed `runtime.count` function.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::builtins::semantics::{
    BuiltinCallablePolicy, BuiltinEffects, BuiltinLowering, BuiltinLoweringContext,
    BuiltinLoweringError, BuiltinRequirements, BuiltinResultOwnership, BuiltinResultType,
    BuiltinRuntimeFunctions, BuiltinSemanticInput, BuiltinSemantics, BuiltinTargetStrategy,
    BuiltinTargetSupport, BuiltinValidation, LoweredBuiltinValue, NormalizedBuiltinCall,
};
use crate::errors::CompileError;
use crate::ir::{Effects, Op, RuntimeCallTarget, RuntimeFnId};
use crate::types::checker::builtins::arrays::union_member_is_countable_array;
use crate::types::PhpType;

builtin! {
    name: "count",
    area: Array,
    params: [value: Mixed, mode: Int = DefaultSpec::Int(0)],
    max_args: 1,
    returns: Int,
    check: check,
    semantics: BuiltinSemantics {
        validation: BuiltinValidation::SignatureOnly,
        result_type: BuiltinResultType::Declared,
        effects: BuiltinEffects::Shared(effects),
        result_ownership: BuiltinResultOwnership::NonHeap,
        requirements: BuiltinRequirements::Static(&[]),
        target_strategy: BuiltinTargetStrategy::Conditional,
        target_support: BuiltinTargetSupport::All,
        runtime_functions: BuiltinRuntimeFunctions::One(RuntimeFnId::Count),
        callable: BuiltinCallablePolicy::StaticOnly(
            "runtime-selected count requires a statically typed Countable source",
        ),
        lowering: BuiltinLowering::Eir(lower),
    },
    summary: "Counts all elements in an array or Countable object.",
    php_manual: "https://www.php.net/manual/en/function.count.php",
}

/// Validates the argument type and returns `Int`.
///
/// Accepts Array, AssocArray, Mixed (heterogeneous arrays), a Union where every member
/// is countable, or an Object that implements the `Countable` interface. Arity
/// enforcement (exactly 1 argument) is handled by the registry's `check_arity` via
/// `max_args: 1`. Returns a `CompileError` for non-countable types or non-Countable objects.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let ty = cx.checker.infer_type(&cx.args[0], cx.env)?;
    match &ty {
        PhpType::Array(_) | PhpType::AssocArray { .. } | PhpType::Mixed => Ok(PhpType::Int),
        PhpType::Union(members) if members.iter().all(union_member_is_countable_array) => {
            Ok(PhpType::Int)
        }
        PhpType::Object(class_name) => {
            if cx.checker.class_implements_interface(class_name, "Countable") {
                Ok(PhpType::Int)
            } else {
                Err(CompileError::new(
                    cx.span,
                    "count() object argument must implement Countable",
                ))
            }
        }
        _ => Err(CompileError::new(
            cx.span,
            "count() argument must be array or Countable object",
        )),
    }
}

/// Resolves precise count effects from the checked operand type.
fn effects(input: &BuiltinSemanticInput<'_>) -> Effects {
    input
        .arg_types
        .first()
        .map(count_effects_for_type)
        .unwrap_or_else(Effects::all)
}

/// Returns the EIR effect contract for one countable operand representation.
fn count_effects_for_type(ty: &PhpType) -> Effects {
    match ty.codegen_repr() {
        PhpType::Array(_) => Op::ArrayLen.default_effects(),
        PhpType::AssocArray { .. } => Op::HashLen.default_effects(),
        PhpType::Mixed | PhpType::Union(_) => Effects::READS_HEAP | Effects::MAY_FATAL,
        PhpType::Object(_) => Effects::all(),
        _ => Effects::READS_HEAP | Effects::MAY_FATAL,
    }
}

/// Lowers concrete arrays to length primitives and keeps dynamic Countable values typed.
fn lower(
    ctx: &mut dyn BuiltinLoweringContext,
    call: &NormalizedBuiltinCall<'_>,
) -> Result<LoweredBuiltinValue, BuiltinLoweringError> {
    let value = call.operand(0)?;
    let ty = ctx.value_php_type(value);
    let effects = count_effects_for_type(&ty);
    match ty.codegen_repr() {
        PhpType::Array(_) => Ok(ctx.emit_value(
            Op::ArrayLen,
            vec![value],
            None,
            call.result_type.clone(),
            effects,
            Some(call.span),
        )),
        PhpType::AssocArray { .. } => Ok(ctx.emit_value(
            Op::HashLen,
            vec![value],
            None,
            call.result_type.clone(),
            effects,
            Some(call.span),
        )),
        PhpType::Mixed | PhpType::Union(_) | PhpType::Object(_) => Ok(ctx.emit_runtime_call(
            RuntimeCallTarget::Function(RuntimeFnId::Count),
            vec![value],
            call.result_type.clone(),
            effects,
            Some(call.span),
        )),
        other => Err(BuiltinLoweringError::new(format!(
            "count cannot lower checked operand type {:?}",
            other,
        ))),
    }
}
