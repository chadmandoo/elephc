//! Purpose:
//! Groups declarative eval metadata for core callable, constant, and
//! process-control builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins` re-exports used by registry hooks.
//!
//! Key details:
//! - Runtime behavior stays delegated to existing focused interpreter helpers.

mod declarations;

pub(in crate::interpreter) use declarations::*;
