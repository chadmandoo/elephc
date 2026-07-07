//! Purpose:
//! Home of the PHP `mkdir` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `mkdir` is a pure-data builtin whose `Bool` return type is
//!   fully determined by its declaration. Unlike `unlink`, `mkdir` has no PHAR
//!   side effect, so no library-linking check hook is required. The registry
//!   common path infers the arguments and enforces the 1..3 arity before
//!   falling back to `returns`.
//! - `permissions` defaults to 0777 and `recursive` to false, matching PHP;
//!   the kernel applies the process umask to the mode like PHP does. The
//!   `$context` fourth argument is not supported (no stream contexts in AOT).
//! - `lower` is a thin wrapper over `io::lower_mkdir` in the EIR backend.

use crate::builtins::spec::DefaultSpec;
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "mkdir",
    area: Io,
    params: [
        directory: Str,
        permissions: Int = DefaultSpec::Int(511),
        recursive: Bool = DefaultSpec::Bool(false),
    ],
    returns: Bool,
    lower: lower,
    summary: "Makes a directory.",
    php_manual: "function.mkdir",
}

/// Lowers a `mkdir` call by dispatching to the shared io emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::io::lower_mkdir(ctx, inst)
}
