//! Purpose:
//! Home of the PHP `strpos` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The declared signature carries the full golden param list (`haystack`, `needle`,
//!   `offset`); `max_args: 3` admits the optional 3-arg offset form (EC-64 #558). A 3-arg
//!   call is desugared by ir_lower to the `__elephc_strpos_offset` prelude (native
//!   `substr` + native 2-arg `strpos`); the 2-arg form stays on the native runtime helper.
//! - `check` returns `PhpType::Union([Int, Bool])` (position, or `false` on no match).
//!   A check hook is required because the `builtin!` macro `returns:` field only accepts
//!   a simple type identifier and cannot express a union inline. Argument types are
//!   inferred by the common registry dispatch path before the hook fires.
//! - `lower` is a thin wrapper over the shared `lower_string_position` emitter, passing
//!   the `__rt_strpos` runtime helper.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "strpos",
    area: String,
    params: [haystack: Str, needle: Str, offset: Int = DefaultSpec::Int(0)],
    max_args: 3,
    returns: Mixed,
    check: check,
    lower: lower,
    summary: "Finds the numeric position of the first occurrence of a substring.",
    php_manual: "https://www.php.net/manual/en/function.strpos.php",
}

/// Returns `PhpType::Union([Int, Bool])` for a `strpos` call (position, or `false`).
///
/// A check hook is required because the `builtin!` macro cannot express a union return
/// type inline. Argument types are inferred by the common registry dispatch path before
/// this hook fires; arity (2 or 3 via `max_args`) is validated by the registry.
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::Union(vec![PhpType::Int, PhpType::Bool]))
}

/// Lowers a `strpos` call by dispatching to the shared string-position emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::strings::lower_string_position(
        ctx,
        inst,
        "strpos",
        "__rt_strpos",
    )
}
