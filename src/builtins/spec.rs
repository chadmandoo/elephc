//! Purpose:
//! Defines the `BuiltinSpec` type that describes a single PHP builtin function:
//! its name, arity, type signature, purity, and codegen lowering hook.
//!
//! Called from:
//! - `crate::builtins::registry` (collected via `inventory`).
//! - `crate::types::checker::builtins` and `crate::codegen_ir::lower_inst::builtins`
//!   (consumed during type-check and codegen dispatch).
//!
//! Key details:
//! - Every builtin must submit exactly one `BuiltinSpec` via the `builtin!` macro;
//!   duplicate names are detected at registry init time.
