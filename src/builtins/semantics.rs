//! Purpose:
//! Defines backend-neutral semantic contracts shared by builtin checking, EIR
//! lowering, optimizer effects, ownership, callable dispatch, and requirements.
//!
//! Called from:
//! - `crate::builtins::spec::BuiltinSpec` for the single-source descriptor.
//! - `crate::types::checker::builtins` and `crate::ir_lower::expr` while consuming it.
//!
//! Key details:
//! - Semantic hooks only see PHP types, AST arguments, EIR values/opcodes, and source spans.
//! - Assembly contexts, physical registers, frame layout, ABI details, and raw symbols are absent.
//! - `LEGACY` is a migration-only marker and is rejected by the final structural audit.

#![allow(dead_code)]

use std::fmt;

use crate::errors::CompileError;
use crate::ir::{BuiltinRuntimeTarget, Effects, Immediate, Op, RuntimeCallTarget, ValueId};
use crate::parser::ast::Expr;
use crate::span::Span;
use crate::types::PhpType;

/// Inputs shared by backend-neutral validation, result-type, and effect resolvers.
pub struct BuiltinSemanticInput<'a> {
    /// Canonical lower-case PHP builtin name.
    pub name: &'a str,
    /// Source-order argument expressions after common call-argument normalization.
    pub args: &'a [Expr],
    /// Inferred PHP types in the same order as `args`.
    pub arg_types: &'a [PhpType],
    /// Source span of the complete call expression.
    pub span: Span,
}

/// Backend-neutral validator for argument semantics beyond signature and arity checks.
pub type ValidateFn = for<'a> fn(&BuiltinSemanticInput<'a>) -> Result<(), CompileError>;

/// Backend-neutral resolver for argument- or value-dependent return types.
pub type ResultTypeFn = for<'a> fn(&BuiltinSemanticInput<'a>) -> PhpType;

/// Backend-neutral resolver for argument- or value-dependent effect summaries.
pub type EffectsFn = for<'a> fn(&BuiltinSemanticInput<'a>) -> Effects;

/// Describes how checker validation is provided for a builtin.
#[derive(Clone, Copy)]
pub enum BuiltinValidation {
    /// Migration adapter: use the old checker hook stored on `BuiltinSpec`.
    LegacyCheckerHook,
    /// Signature/arity validation is sufficient.
    SignatureOnly,
    /// Run one backend-neutral semantic validator after inferring arguments once.
    Shared(ValidateFn),
}

/// Describes the single authoritative return-type resolver for a builtin.
#[derive(Clone, Copy)]
pub enum BuiltinResultType {
    /// Migration adapter: checker and EIR still use their historical paths.
    LegacySplit,
    /// Use the registry's declared `returns` type in checker and EIR.
    Declared,
    /// Resolve from normalized argument types and source constants in both consumers.
    Shared(ResultTypeFn),
}

/// Describes the single authoritative effect resolver for a builtin.
#[derive(Clone, Copy)]
pub enum BuiltinEffects {
    /// Migration adapter: use the old AST list and pessimistic EIR fallback.
    LegacySplit,
    /// The builtin always has this precise conservative effect summary.
    Static(Effects),
    /// Resolve effects from normalized argument types and source constants.
    Shared(EffectsFn),
}

/// Describes ownership and argument-aliasing of the builtin result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinResultOwnership {
    /// Migration adapter: use `returns_fresh_storage`/`returns_independent_storage`.
    LegacyFlags,
    /// Scalar or otherwise non-refcounted result.
    NonHeap,
    /// Fresh caller-owned result that aliases no argument.
    Fresh,
    /// Borrowed result owned by runtime or persistent storage.
    Borrowed,
    /// Non-fresh result guaranteed not to alias an argument.
    Independent,
    /// Result may alias the listed zero-based argument positions.
    Aliases(&'static [usize]),
}

/// Explains whether and how a builtin participates in dynamic callable dispatch.
pub type CallableSourceFn = for<'a> fn(Option<&'a PhpType>) -> bool;

/// Explains whether and how a builtin participates in dynamic callable dispatch.
#[derive(Clone, Copy, Debug)]
pub enum BuiltinCallablePolicy {
    /// Migration adapter: consult the historical codegen allowlists.
    Legacy,
    /// Direct, first-class, and runtime-known dynamic callable paths are supported.
    Dynamic(CallableSourceFn),
    /// Only statically resolved direct calls are meaningful for this builtin.
    DirectOnly(&'static str),
}

/// Explicit runtime or linker requirement declared by builtin semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinRequirement {
    /// Link a bridge/static library on demand.
    Bridge(&'static str),
    /// Link a target-neutral system library on demand.
    SystemLibrary(&'static str),
    /// Link a macOS-only system library while Linux resolves the API from libc.
    MacOsLibrary(&'static str),
    /// Enable a named runtime feature collected from the final EIR module.
    RuntimeFeature(&'static str),
}

/// Declares the backend-neutral implementation shape selected for a builtin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinTargetStrategy {
    /// Migration adapter: infer behavior from the historical assembly hook.
    LegacyAssembly,
    /// Lower directly to one existing general-purpose EIR operation.
    EirPrimitive,
    /// Lower to a graph of reusable EIR operations and control flow.
    EirGraph,
    /// Lower through a typed runtime call whose ABI is resolved by the backend.
    RuntimeCall,
    /// Select among multiple explicit EIR/runtime strategies from semantic inputs.
    Conditional,
}

/// Declares which supported compiler targets implement the builtin semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinTargetSupport {
    /// Migration adapter: support is implicit in historical backend code.
    LegacyImplicit,
    /// The semantic strategy is valid on macOS AArch64, Linux AArch64, and Linux x86_64.
    All,
}

/// One value produced by backend-neutral builtin EIR lowering.
#[derive(Debug, Clone, Copy)]
pub struct LoweredBuiltinValue {
    /// SSA value produced by the lowering.
    pub value: ValueId,
}

/// Error returned when checked builtin semantics cannot be represented in EIR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinLoweringError {
    message: String,
}

impl BuiltinLoweringError {
    /// Creates an explicit semantic-lowering error with user-facing context.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for BuiltinLoweringError {
    /// Formats the backend-neutral lowering diagnostic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// Minimal EIR construction surface exposed to builtin semantic lowering.
pub trait BuiltinLoweringContext {
    /// Returns the PHP type metadata attached to an existing SSA operand.
    fn value_php_type(&self, value: ValueId) -> PhpType;

    /// Emits one typed value-producing EIR operation with explicit effects and span.
    fn emit_value(
        &mut self,
        op: Op,
        operands: Vec<ValueId>,
        immediate: Option<Immediate>,
        php_type: PhpType,
        effects: Effects,
        span: Option<Span>,
    ) -> LoweredBuiltinValue;

    /// Emits one typed runtime operation whose symbol and ABI are backend-owned.
    fn emit_runtime_call(
        &mut self,
        target: RuntimeCallTarget,
        operands: Vec<ValueId>,
        php_type: PhpType,
        effects: Effects,
        span: Option<Span>,
    ) -> LoweredBuiltinValue;
}

/// Normalized builtin call consumed by backend-neutral EIR lowering.
pub struct NormalizedBuiltinCall<'a> {
    /// Canonical lower-case PHP builtin name.
    pub name: &'a str,
    /// EIR operands after shared named/positional/spread planning.
    pub operands: &'a [ValueId],
    /// Authoritative PHP result type resolved from the registry.
    pub result_type: &'a PhpType,
    /// Authoritative conservative effects resolved from the registry semantics.
    pub effects: Effects,
    /// Source span of the complete call expression.
    pub span: Span,
}

impl NormalizedBuiltinCall<'_> {
    /// Returns one required operand or a structured lowering error.
    pub fn operand(&self, index: usize) -> Result<ValueId, BuiltinLoweringError> {
        self.operands.get(index).copied().ok_or_else(|| {
            BuiltinLoweringError::new(format!(
                "{} lowering expected operand {} but received {} operands",
                self.name,
                index,
                self.operands.len(),
            ))
        })
    }
}

/// Backend-neutral EIR lowering hook for one normalized builtin call.
pub type BuiltinLowerFn = for<'a> fn(
    &mut dyn BuiltinLoweringContext,
    &NormalizedBuiltinCall<'a>,
) -> Result<LoweredBuiltinValue, BuiltinLoweringError>;

/// Selects the active lowering path for a registry-backed builtin.
#[derive(Clone, Copy)]
pub enum BuiltinLowering {
    /// Migration adapter: emit `Op::BuiltinCall` and use the assembly hook.
    LegacyAssembly,
    /// Emit backend-neutral EIR through the registered semantic hook.
    Eir(BuiltinLowerFn),
    /// Emit one typed runtime operation using the call's normalized operands.
    Runtime(RuntimeCallTarget),
}

/// Complete shared semantic descriptor referenced by `BuiltinSpec`.
#[derive(Clone, Copy)]
pub struct BuiltinSemantics {
    /// Argument validation contract.
    pub validation: BuiltinValidation,
    /// Authoritative result-type contract.
    pub result_type: BuiltinResultType,
    /// Precise conservative effect contract.
    pub effects: BuiltinEffects,
    /// Result ownership and aliasing contract.
    pub result_ownership: BuiltinResultOwnership,
    /// Runtime/link requirements visible without inspecting a PHP function name.
    pub requirements: &'static [BuiltinRequirement],
    /// Backend-neutral implementation shape used after call normalization.
    pub target_strategy: BuiltinTargetStrategy,
    /// Explicit supported-target coverage contract.
    pub target_support: BuiltinTargetSupport,
    /// Callable availability contract.
    pub callable: BuiltinCallablePolicy,
    /// Backend-neutral lowering strategy.
    pub lowering: BuiltinLowering,
}

impl BuiltinSemantics {
    /// Migration-only descriptor used until a builtin family receives complete metadata.
    pub const LEGACY: Self = Self {
        validation: BuiltinValidation::LegacyCheckerHook,
        result_type: BuiltinResultType::LegacySplit,
        effects: BuiltinEffects::LegacySplit,
        result_ownership: BuiltinResultOwnership::LegacyFlags,
        requirements: &[],
        target_strategy: BuiltinTargetStrategy::LegacyAssembly,
        target_support: BuiltinTargetSupport::LegacyImplicit,
        callable: BuiltinCallablePolicy::Legacy,
        lowering: BuiltinLowering::LegacyAssembly,
    };

    /// Returns true when every semantic surface has migrated off compatibility paths.
    pub fn is_complete(self) -> bool {
        !matches!(self.validation, BuiltinValidation::LegacyCheckerHook)
            && !matches!(self.result_type, BuiltinResultType::LegacySplit)
            && !matches!(self.effects, BuiltinEffects::LegacySplit)
            && !matches!(self.result_ownership, BuiltinResultOwnership::LegacyFlags)
            && !matches!(self.target_strategy, BuiltinTargetStrategy::LegacyAssembly)
            && !matches!(self.target_support, BuiltinTargetSupport::LegacyImplicit)
            && !matches!(self.callable, BuiltinCallablePolicy::Legacy)
            && !matches!(self.lowering, BuiltinLowering::LegacyAssembly)
    }
}

/// Builds the complete semantic descriptor for a fresh `Str -> Str` runtime transform.
pub const fn unary_string_runtime(
    target: RuntimeCallTarget,
    effects: Effects,
) -> BuiltinSemantics {
    BuiltinSemantics {
        validation: BuiltinValidation::SignatureOnly,
        result_type: BuiltinResultType::Declared,
        effects: BuiltinEffects::Static(effects),
        result_ownership: BuiltinResultOwnership::Fresh,
        requirements: &[],
        target_strategy: BuiltinTargetStrategy::RuntimeCall,
        target_support: BuiltinTargetSupport::All,
        callable: BuiltinCallablePolicy::Dynamic(callable_accepts_string_source),
        lowering: BuiltinLowering::Runtime(target),
    }
}

/// Builds a transitional typed backend target while semantic metadata migrates family by family.
pub const fn backend_target_adapter(
    target: BuiltinRuntimeTarget,
    strategy: BuiltinTargetStrategy,
) -> BuiltinSemantics {
    BuiltinSemantics {
        validation: BuiltinValidation::LegacyCheckerHook,
        result_type: BuiltinResultType::LegacySplit,
        effects: BuiltinEffects::LegacySplit,
        result_ownership: BuiltinResultOwnership::LegacyFlags,
        requirements: &[],
        target_strategy: strategy,
        target_support: BuiltinTargetSupport::All,
        callable: BuiltinCallablePolicy::Legacy,
        lowering: BuiltinLowering::Runtime(RuntimeCallTarget::Builtin(target)),
    }
}

/// Accepts runtime wrapper sources that already use concrete string storage.
pub fn callable_accepts_string_source(source: Option<&PhpType>) -> bool {
    source.is_none_or(|source| source.codegen_repr() == PhpType::Str)
}

/// Accepts the dynamic string-like sources supported by shared `strlen` validation.
pub fn callable_accepts_strlen_source(source: Option<&PhpType>) -> bool {
    source.is_none_or(|source| {
        matches!(
            source.codegen_repr(),
            PhpType::Mixed | PhpType::Str | PhpType::Union(_)
        )
    })
}

/// Lowers one registry builtin through its complete backend-neutral semantic descriptor.
pub fn lower_registry_call(
    ctx: &mut dyn BuiltinLoweringContext,
    name: &str,
    operands: &[ValueId],
    result_type: &PhpType,
    span: Span,
) -> Result<Option<LoweredBuiltinValue>, BuiltinLoweringError> {
    let Some(def) = crate::builtins::registry::lookup(name) else {
        return Ok(None);
    };
    if matches!(def.spec.semantics.lowering, BuiltinLowering::LegacyAssembly) {
        return Ok(None);
    }
    let arg_types = operands
        .iter()
        .map(|operand| ctx.value_php_type(*operand))
        .collect::<Vec<_>>();
    let semantic_input = BuiltinSemanticInput {
        name: def.name,
        args: &[],
        arg_types: &arg_types,
        span,
    };
    let effects = match def.spec.semantics.effects {
        BuiltinEffects::Static(effects) => effects,
        BuiltinEffects::Shared(resolve) => resolve(&semantic_input),
        BuiltinEffects::LegacySplit => Op::RuntimeCall.default_effects(),
    };
    let normalized = NormalizedBuiltinCall {
        name: def.name,
        operands,
        result_type,
        effects,
        span,
    };
    match def.spec.semantics.lowering {
        BuiltinLowering::Eir(lower) => lower(ctx, &normalized).map(Some),
        BuiltinLowering::Runtime(target) => Ok(Some(ctx.emit_runtime_call(
            target,
            operands.to_vec(),
            result_type.clone(),
            effects,
            Some(span),
        ))),
        BuiltinLowering::LegacyAssembly => Ok(None),
    }
}
