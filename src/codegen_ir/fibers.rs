//! Purpose:
//! Resolves EIR-specific Fiber wrapper requirements for closure callbacks.
//! Keeps wrapper label selection shared between wrapper emission and `new Fiber` lowering.
//!
//! Called from:
//! - `crate::codegen_ir::block_emit` when emitting deferred wrapper functions.
//! - `crate::codegen_ir::lower_inst::objects` when lowering Fiber construction.
//!
//! Key details:
//! - Wrappers are per closure signature because they adapt boxed Fiber start
//!   arguments into the concrete callback ABI.
//! - Captures remain descriptor-owned; this phase only passes visible callback parameters.

use crate::ir::{Function, Immediate, Instruction, Module, Op, ValueDef};
use crate::names::php_symbol_key;
use crate::types::{FunctionSig, PhpType};

/// Static wrapper function required for an EIR Fiber callback ABI shape.
#[derive(Clone)]
pub(crate) struct FiberWrapper {
    pub(crate) label: String,
    pub(crate) sig: FunctionSig,
    pub(crate) visible_param_count: usize,
    pub(crate) hidden_arg_types: Vec<PhpType>,
}

/// Returns the wrapper needed when a `new Fiber(...)` instruction receives a closure literal.
pub(crate) fn wrapper_for_fiber_new(
    module: &Module,
    function: &Function,
    inst: &Instruction,
) -> Option<FiberWrapper> {
    if !is_fiber_object_new(module, inst) {
        return None;
    }
    let callable = inst.operands.first().copied()?;
    let closure = closure_literal_operand(module, function, callable)?;
    Some(wrapper_for_closure(closure))
}

/// Returns true when an EIR object construction instruction targets PHP's built-in `Fiber`.
fn is_fiber_object_new(module: &Module, inst: &Instruction) -> bool {
    if !matches!(inst.op, Op::ObjectNew) {
        return false;
    }
    let Some(Immediate::Data(data)) = inst.immediate else {
        return false;
    };
    module
        .data
        .class_names
        .get(data.as_raw() as usize)
        .is_some_and(|class_name| php_symbol_key(class_name.trim_start_matches('\\')) == "fiber")
}

/// Resolves a callable operand produced by `closure_new` to its EIR closure body.
fn closure_literal_operand<'a>(
    module: &'a Module,
    function: &Function,
    callable: crate::ir::ValueId,
) -> Option<&'a Function> {
    let value = function.value(callable)?;
    let ValueDef::Instruction {
        inst: callable_inst,
        ..
    } = value.def
    else {
        return None;
    };
    let callable_inst = function.instruction(callable_inst)?;
    if !matches!(callable_inst.op, Op::ClosureNew) {
        return None;
    }
    let Some(Immediate::Data(data)) = callable_inst.immediate else {
        return None;
    };
    let closure_name = module.data.strings.get(data.as_raw() as usize)?;
    module
        .closures
        .iter()
        .find(|closure| closure.name == *closure_name)
}

/// Builds a deferred Fiber wrapper description from the concrete EIR closure signature.
fn wrapper_for_closure(closure: &Function) -> FiberWrapper {
    FiberWrapper {
        label: fiber_wrapper_label(&closure.name),
        sig: signature_from_closure(closure),
        visible_param_count: closure.params.len(),
        hidden_arg_types: Vec::new(),
    }
}

/// Returns an assembly-safe wrapper label derived from the EIR closure symbol.
fn fiber_wrapper_label(closure_name: &str) -> String {
    format!("{}_fiber_wrapper", crate::names::function_symbol(closure_name))
}

/// Reconstructs callable signature metadata from a lowered EIR closure function.
fn signature_from_closure(closure: &Function) -> FunctionSig {
    FunctionSig {
        params: closure
            .params
            .iter()
            .map(|param| (param.name.clone(), param.php_type.clone()))
            .collect(),
        defaults: closure.params.iter().map(|_| None).collect(),
        return_type: closure.return_php_type.clone(),
        declared_return: !matches!(closure.return_php_type, PhpType::Mixed),
        ref_params: closure.params.iter().map(|param| param.by_ref).collect(),
        declared_params: closure
            .params
            .iter()
            .map(|param| !matches!(param.php_type, PhpType::Mixed))
            .collect(),
        variadic: closure
            .params
            .iter()
            .find(|param| param.variadic)
            .map(|param| param.name.clone()),
        deprecation: None,
    }
}
