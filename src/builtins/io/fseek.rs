//! Purpose:
//! Home of the PHP `fseek` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` calls `ensure_stream_resource` on the stream argument for validation and
//!   returns `Int`. PHP declares `fseek(): int` — 0 on success, -1 on failure, never `false`
//!   (`ReflectionFunction('fseek')->getReturnType()` is `int`), and `lower_fseek` already
//!   emits exactly that (`mov x0, #-1` / `mov rax, -1` on the failure path, 0 on success).
//!   It previously declared `Union(Int, False)`, which forced every caller passing a seek
//!   result into an `int` parameter through a spurious union. Arguments are pre-inferred by
//!   the registry before the hook runs.
//! - `lower` is a thin wrapper over `io::lower_fseek` in the EIR backend.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "fseek",
    area: Io,
    params: [stream: Mixed, offset: Int, whence: Int = DefaultSpec::Int(0)],
    returns: Int,
    check: check,
    lower: lower,
    summary: "Seeks on a file pointer.",
    php_manual: "function.fseek",
}

/// Validates the stream argument and returns `Int` for the seek result (0 success, -1 failure).
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    crate::types::checker::builtins::io::common::ensure_stream_resource(
        cx.checker,
        cx.name,
        &cx.args[0],
        cx.env,
    )?;
    Ok(PhpType::Int)
}

/// Lowers an `fseek` call by dispatching to the shared io emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::io::lower_fseek(ctx, inst)
}
