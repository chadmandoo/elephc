//! Purpose:
//! Provides the `builtin!` declarative macro used to register PHP builtin function
//! descriptors into the inventory-based registry at link time.
//!
//! Called from:
//! - Each `crate::builtins::<area>::<name>` leaf file via `#[macro_use]` on this module.
//!
//! Key details:
//! - This module is included with `#[macro_use]` so the macro is available crate-wide
//!   without explicit import at every call site.
