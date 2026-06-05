//! Purpose:
//! Resolves EIR-specific Fiber wrapper requirements for no-argument closure callbacks.
//! Keeps wrapper label selection shared between wrapper emission and `new Fiber` lowering.
//!
//! Called from:
//! - `crate::codegen_ir::block_emit` when emitting deferred wrapper functions.
//! - `crate::codegen_ir::lower_inst::objects` when lowering Fiber construction.
//!
//! Key details:
//! - Wrappers are generic per return ABI shape because the callable descriptor
//!   supplies the concrete closure entry at runtime.
//! - Start arguments remain unsupported here; only no-argument callbacks are selected.

use crate::ir::{Function, Immediate, Instruction, Module, Op, ValueDef};
use crate::names::php_symbol_key;
use crate::types::PhpType;

const FIBER_NOARG_VOID_WRAPPER_LABEL: &str = "_eir_fiber_noarg_void_wrapper";
const FIBER_NOARG_INT_WRAPPER_LABEL: &str = "_eir_fiber_noarg_int_wrapper";
const FIBER_NOARG_FLOAT_WRAPPER_LABEL: &str = "_eir_fiber_noarg_float_wrapper";
const FIBER_NOARG_STRING_WRAPPER_LABEL: &str = "_eir_fiber_noarg_string_wrapper";
const FIBER_NOARG_BOOL_WRAPPER_LABEL: &str = "_eir_fiber_noarg_bool_wrapper";
const FIBER_NOARG_MIXED_WRAPPER_LABEL: &str = "_eir_fiber_noarg_mixed_wrapper";
const FIBER_NOARG_ARRAY_WRAPPER_LABEL: &str = "_eir_fiber_noarg_array_wrapper";
const FIBER_NOARG_ASSOC_ARRAY_WRAPPER_LABEL: &str = "_eir_fiber_noarg_assoc_array_wrapper";
const FIBER_NOARG_OBJECT_WRAPPER_LABEL: &str = "_eir_fiber_noarg_object_wrapper";
const FIBER_NOARG_ITERABLE_WRAPPER_LABEL: &str = "_eir_fiber_noarg_iterable_wrapper";
const FIBER_NOARG_CALLABLE_WRAPPER_LABEL: &str = "_eir_fiber_noarg_callable_wrapper";
const FIBER_NOARG_POINTER_WRAPPER_LABEL: &str = "_eir_fiber_noarg_pointer_wrapper";
const FIBER_NOARG_BUFFER_WRAPPER_LABEL: &str = "_eir_fiber_noarg_buffer_wrapper";
const FIBER_NOARG_PACKED_WRAPPER_LABEL: &str = "_eir_fiber_noarg_packed_wrapper";

/// Static wrapper function required for an EIR Fiber callback return ABI shape.
#[derive(Clone)]
pub(crate) struct NoArgWrapper {
    pub(crate) label: &'static str,
    pub(crate) return_type: PhpType,
}

/// Returns the wrapper needed when a `new Fiber(...)` instruction receives a no-arg closure literal.
pub(crate) fn noarg_wrapper_for_fiber_new(
    module: &Module,
    function: &Function,
    inst: &Instruction,
) -> Option<NoArgWrapper> {
    if !is_fiber_object_new(module, inst) {
        return None;
    }
    let callable = inst.operands.first().copied()?;
    let closure = closure_literal_operand(module, function, callable)?;
    if !closure.params.is_empty() {
        return None;
    }
    wrapper_for_return_type(&closure.return_php_type.codegen_repr())
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

/// Maps a callback return type to the generic no-argument wrapper for its ABI shape.
fn wrapper_for_return_type(return_type: &PhpType) -> Option<NoArgWrapper> {
    let (label, return_type) = match return_type {
        PhpType::Void | PhpType::Never => (FIBER_NOARG_VOID_WRAPPER_LABEL, PhpType::Void),
        PhpType::Int => (FIBER_NOARG_INT_WRAPPER_LABEL, PhpType::Int),
        PhpType::Float => (FIBER_NOARG_FLOAT_WRAPPER_LABEL, PhpType::Float),
        PhpType::Str => (FIBER_NOARG_STRING_WRAPPER_LABEL, PhpType::Str),
        PhpType::Bool => (FIBER_NOARG_BOOL_WRAPPER_LABEL, PhpType::Bool),
        PhpType::Mixed | PhpType::Union(_) => (FIBER_NOARG_MIXED_WRAPPER_LABEL, PhpType::Mixed),
        PhpType::Array(_) => (
            FIBER_NOARG_ARRAY_WRAPPER_LABEL,
            PhpType::Array(Box::new(PhpType::Mixed)),
        ),
        PhpType::AssocArray { .. } => (
            FIBER_NOARG_ASSOC_ARRAY_WRAPPER_LABEL,
            PhpType::AssocArray {
                key: Box::new(PhpType::Mixed),
                value: Box::new(PhpType::Mixed),
            },
        ),
        PhpType::Object(_) => (
            FIBER_NOARG_OBJECT_WRAPPER_LABEL,
            PhpType::Object(String::new()),
        ),
        PhpType::Iterable => (FIBER_NOARG_ITERABLE_WRAPPER_LABEL, PhpType::Iterable),
        PhpType::Callable => (FIBER_NOARG_CALLABLE_WRAPPER_LABEL, PhpType::Callable),
        PhpType::Pointer(_) => (FIBER_NOARG_POINTER_WRAPPER_LABEL, PhpType::Pointer(None)),
        PhpType::Buffer(_) => (
            FIBER_NOARG_BUFFER_WRAPPER_LABEL,
            PhpType::Buffer(Box::new(PhpType::Mixed)),
        ),
        PhpType::Packed(_) => (
            FIBER_NOARG_PACKED_WRAPPER_LABEL,
            PhpType::Packed(String::new()),
        ),
        PhpType::Resource(_) => return None,
    };
    Some(NoArgWrapper { label, return_type })
}
