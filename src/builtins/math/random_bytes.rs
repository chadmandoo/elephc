//! Purpose:
//! Home of the PHP `random_bytes` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: one `Int` length in, one `Str` (owned byte string) out.
//! - `lower` dispatches to `lower_random_bytes`, which loads the length into the integer
//!   argument register and calls the `__rt_random_bytes` CSPRNG helper. EIR-only, so it
//!   joins the callable-dispatch wrapper exclusion.

use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "random_bytes",
    area: Math,
    params: [length: Int],
    returns: Str,
    lower: lower,
    summary: "Returns a string of cryptographically secure random bytes of the given length.",
    php_manual: "https://www.php.net/manual/en/function.random-bytes.php",
}

/// Lowers a `random_bytes` call by dispatching to the shared CSPRNG string emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::strings::lower_random_bytes(ctx, inst)
}
