//! Purpose:
//! Groups declarative registry dispatch hooks for eval builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins::spec` re-exports used by `eval_builtin!`.
//!
//! Key details:
//! - Direct expression dispatch and already-evaluated argument dispatch are kept
//!   in separate files so each hook table can grow independently.

mod direct;
mod values;

pub(in crate::interpreter) use direct::EvalDirectHook;
pub(in crate::interpreter) use values::EvalValuesHook;
