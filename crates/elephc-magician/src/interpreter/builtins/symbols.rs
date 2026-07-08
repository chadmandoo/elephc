//! Purpose:
//! Orchestrates symbol, constant, class, and language-construct eval builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins` re-exports used by core call dispatch.
//!
//! Key details:
//! - Concrete builtin behavior lives in focused `symbols/` modules so each
//!   source file stays cohesive and below the ordinary 500 LoC guideline.

use super::super::*;

mod callable_probe;
mod class_names;
mod class_relations;
mod constants;
mod declarations;
mod function_probe;
mod language_constructs;

pub(in crate::interpreter) use callable_probe::*;
pub(in crate::interpreter) use class_names::*;
pub(in crate::interpreter) use class_relations::*;
pub(in crate::interpreter) use constants::*;
pub(in crate::interpreter) use declarations::{
    eval_builtin_symbols_call, eval_symbols_values_result,
};
pub(in crate::interpreter) use function_probe::*;
pub(in crate::interpreter) use language_constructs::*;
use super::*;
