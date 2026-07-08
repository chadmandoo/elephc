//! Purpose:
//! Declarative eval registry entries for printf-family formatting builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins::formatting` module loading.
//!
//! Key details:
//! - Formatting algorithms remain in sibling helper modules; this module owns
//!   per-builtin registry metadata only.

mod printf;
mod sprintf;
mod sscanf;
mod vprintf;
mod vsprintf;
