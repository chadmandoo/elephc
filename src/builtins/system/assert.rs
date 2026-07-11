//! Purpose:
//! Home of the PHP `assert` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `assert` accepts any assertion value plus an optional
//!   description and returns `Bool`, all expressible inline.
//! - Under PHP's production configuration (`zend.assertions=-1`) `assert()` is a no-op
//!   that always returns `true` and never throws; the lowering materializes a constant
//!   `true`, matching the reference runtime. See `lower_assert`.

use crate::builtins::spec::DefaultSpec;
use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "assert",
    area: System,
    params: [assertion: Mixed, description: Mixed = DefaultSpec::Null],
    returns: Bool,
    lower: lower,
    summary: "Checks an assertion; a no-op returning true under production assertion settings.",
    php_manual: "https://www.php.net/manual/en/function.assert.php",
}

/// Lowers an `assert` call by dispatching to the shared emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::lower_assert(ctx, inst)
}
