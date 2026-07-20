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

/// Lowers a target owned by bounded dispatch group 05, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {
    match target {
        BuiltinRuntimeTarget::ObImplicitFlush => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_implicit_flush(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObListHandlers => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_list_handlers(ctx, inst)
        }),
        BuiltinRuntimeTarget::ObStart => Some({
            crate::codegen::lower_inst::builtins::output_buffering::lower_ob_start(ctx, inst)
        }),
        BuiltinRuntimeTarget::Opendir => Some({
            crate::codegen::lower_inst::builtins::io::lower_opendir(ctx, inst)
        }),
        BuiltinRuntimeTarget::Pathinfo => Some({
            crate::codegen::lower_inst::builtins::io::lower_pathinfo(ctx, inst)
        }),
        BuiltinRuntimeTarget::Pclose => Some({
            crate::codegen::lower_inst::builtins::io::lower_pclose(ctx, inst)
        }),
        BuiltinRuntimeTarget::Pfsockopen => Some({
            crate::codegen::lower_inst::builtins::io::lower_fsockopen(ctx, inst)
        }),
        BuiltinRuntimeTarget::Popen => Some({
            crate::codegen::lower_inst::builtins::io::lower_popen(ctx, inst)
        }),
        BuiltinRuntimeTarget::PrintR => Some({
            crate::codegen::lower_inst::builtins::debug::lower_print_r(ctx, inst)
        }),
        BuiltinRuntimeTarget::Readdir => Some({
            crate::codegen::lower_inst::builtins::io::lower_readdir(ctx, inst)
        }),
        BuiltinRuntimeTarget::Readfile => Some({
            crate::codegen::lower_inst::builtins::io::lower_readfile(ctx, inst)
        }),
        BuiltinRuntimeTarget::Readline => Some({
            crate::codegen::lower_inst::builtins::io::lower_readline(ctx, inst)
        }),
        BuiltinRuntimeTarget::Readlink => Some({
            crate::codegen::lower_inst::builtins::io::lower_readlink(ctx, inst)
        }),
        BuiltinRuntimeTarget::Realpath => Some({
            crate::codegen::lower_inst::builtins::io::lower_realpath(ctx, inst)
        }),
        BuiltinRuntimeTarget::RealpathCacheGet => Some({
            crate::codegen::lower_inst::builtins::io::lower_realpath_cache_get(ctx, inst)
        }),
        BuiltinRuntimeTarget::RealpathCacheSize => Some({
            crate::codegen::lower_inst::builtins::io::lower_realpath_cache_size(ctx, inst)
        }),
        BuiltinRuntimeTarget::Rename => Some({
            crate::codegen::lower_inst::builtins::io::lower_rename(ctx, inst)
        }),
        BuiltinRuntimeTarget::Rewind => Some({
            crate::codegen::lower_inst::builtins::io::lower_rewind(ctx, inst)
        }),
        BuiltinRuntimeTarget::Rewinddir => Some({
            crate::codegen::lower_inst::builtins::io::lower_rewinddir(ctx, inst)
        }),
        BuiltinRuntimeTarget::Rmdir => Some({
            crate::codegen::lower_inst::builtins::io::lower_rmdir(ctx, inst)
        }),
        BuiltinRuntimeTarget::Scandir => Some({
            crate::codegen::lower_inst::builtins::io::lower_scandir(ctx, inst)
        }),
        BuiltinRuntimeTarget::Stat => Some({
            crate::codegen::lower_inst::builtins::io::lower_stat(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamBucketAppend => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_bucket_append_or_prepend(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamBucketMakeWriteable => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_bucket_make_writeable(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamBucketNew => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_bucket_new(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamBucketPrepend => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_bucket_append_or_prepend(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamContextCreate => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_context_create(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamContextGetDefault => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_context_get_default(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamContextGetOptions => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_context_get_options(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamContextGetParams => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_context_get_params(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamContextSetDefault => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_context_set_default(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamContextSetOption => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_context_set_option(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamContextSetParams => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_context_set_params(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamCopyToStream => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_copy_to_stream(ctx, inst)
        }),
        BuiltinRuntimeTarget::StreamFilterAppend => Some({
            crate::codegen::lower_inst::builtins::io::lower_stream_filter_attach(ctx, inst, "stream_filter_append")
        }),
        _ => None,
    }
}
