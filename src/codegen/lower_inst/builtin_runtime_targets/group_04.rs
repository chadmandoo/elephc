//! Purpose:
//! Dispatches one bounded group of typed builtin runtime targets.
//!
//! Called from:
//! - `super::lower()` while lowering typed EIR runtime calls.
//!
//! Key details:
//! - Dispatch is by enum identity, never by PHP function-name strings.
//! - Extracted bodies remain thin calls into target-aware backend emitters.

use crate::codegen::context::FunctionContext;
use crate::codegen::Result;
use crate::ir::{BuiltinRuntimeTarget, Instruction};

/// Lowers a target owned by bounded dispatch group 04, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {
    match target {
        BuiltinRuntimeTarget::Ftruncate => Some({
            crate::codegen::lower_inst::builtins::io::lower_ftruncate(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fwrite => Some({
            crate::codegen::lower_inst::builtins::io::lower_fwrite(ctx, inst)
        }),
        BuiltinRuntimeTarget::Getcwd => Some({
            crate::codegen::lower_inst::builtins::io::lower_getcwd(ctx, inst)
        }),
        BuiltinRuntimeTarget::Gethostbyaddr => Some({
            crate::codegen::lower_inst::builtins::io::lower_gethostbyaddr(ctx, inst)
        }),
        BuiltinRuntimeTarget::Gethostbyname => Some({
            crate::codegen::lower_inst::builtins::io::lower_gethostbyname(ctx, inst)
        }),
        BuiltinRuntimeTarget::Gethostname => Some({
            crate::codegen::lower_inst::builtins::io::lower_gethostname(ctx, inst)
        }),
        BuiltinRuntimeTarget::Getprotobyname => Some({
            crate::codegen::lower_inst::builtins::io::lower_getprotobyname(ctx, inst)
        }),
        BuiltinRuntimeTarget::Getprotobynumber => Some({
            crate::codegen::lower_inst::builtins::io::lower_getprotobynumber(ctx, inst)
        }),
        BuiltinRuntimeTarget::Getservbyname => Some({
            crate::codegen::lower_inst::builtins::io::lower_getservbyname(ctx, inst)
        }),
        BuiltinRuntimeTarget::Getservbyport => Some({
            crate::codegen::lower_inst::builtins::io::lower_getservbyport(ctx, inst)
        }),
        BuiltinRuntimeTarget::Glob => Some({
            crate::codegen::lower_inst::builtins::io::lower_glob(ctx, inst)
        }),
        BuiltinRuntimeTarget::HashFile => Some({
            crate::codegen::lower_inst::builtins::io::lower_hash_file(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsDir => Some({
            crate::codegen::lower_inst::builtins::io::lower_is_dir(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsExecutable => Some({
            crate::codegen::lower_inst::builtins::io::lower_is_executable(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsFile => Some({
            crate::codegen::lower_inst::builtins::io::lower_is_file(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsLink => Some({
            crate::codegen::lower_inst::builtins::io::lower_is_link(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsReadable => Some({
            crate::codegen::lower_inst::builtins::io::lower_is_readable(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsWritable => Some({
            crate::codegen::lower_inst::builtins::io::lower_is_writable(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsWriteable => Some({
            crate::codegen::lower_inst::builtins::io::lower_is_writeable(ctx, inst)
        }),
        BuiltinRuntimeTarget::Lchgrp => Some({
            crate::codegen::lower_inst::builtins::io::lower_lchgrp(ctx, inst)
        }),
        BuiltinRuntimeTarget::Lchown => Some({
            crate::codegen::lower_inst::builtins::io::lower_lchown(ctx, inst)
        }),
        BuiltinRuntimeTarget::Link => Some({
            crate::codegen::lower_inst::builtins::io::lower_link(ctx, inst)
        }),
        BuiltinRuntimeTarget::Linkinfo => Some({
            crate::codegen::lower_inst::builtins::io::lower_linkinfo(ctx, inst)
        }),
        BuiltinRuntimeTarget::Lstat => Some({
            crate::codegen::lower_inst::builtins::io::lower_lstat(ctx, inst)
        }),
        BuiltinRuntimeTarget::Mkdir => Some({
            crate::codegen::lower_inst::builtins::io::lower_mkdir(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObClean => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_clean(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObEndClean => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_end_clean(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObEndFlush => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_end_flush(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObFlush => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_flush(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObGetClean => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_get_clean(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObGetContents => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_get_contents(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObGetFlush => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_get_flush(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObGetLength => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_get_length(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObGetLevel => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_get_level(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObGetStatus => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_get_status(ctx, inst)
        }),
        _ => None,
    }
}
