//! Purpose:
//! Collects all `BuiltinSpec` entries submitted via `builtin!` into a lazy registry,
//! and exposes lookup helpers used by the catalog, type checker, and codegen dispatcher.
//!
//! Called from:
//! - `crate::types::checker::builtins::catalog` for name-based lookup.
//! - `crate::codegen_ir::lower_inst::builtins` for lowering-hook dispatch.
//!
//! Key details:
//! - Registry is initialized once at first access via a `OnceLock`; subsequent calls
//!   are read-only and lock-free.
//! - Lookup is case-insensitive to match PHP's builtin name semantics.
