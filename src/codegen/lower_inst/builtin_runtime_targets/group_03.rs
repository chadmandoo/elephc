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

/// Lowers a target owned by bounded dispatch group 03, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {
    match target {
        BuiltinRuntimeTarget::DiskFreeSpace => Some({
            crate::codegen::lower_inst::builtins::io::lower_disk_free_space(ctx, inst)
        }),
        BuiltinRuntimeTarget::DiskTotalSpace => Some({
            crate::codegen::lower_inst::builtins::io::lower_disk_total_space(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fclose => Some({
            crate::codegen::lower_inst::builtins::io::lower_fclose(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fdatasync => Some({
            crate::codegen::lower_inst::builtins::io::lower_fdatasync(ctx, inst)
        }),
        BuiltinRuntimeTarget::Feof => Some({
            crate::codegen::lower_inst::builtins::io::lower_feof(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fflush => Some({
            crate::codegen::lower_inst::builtins::io::lower_fflush(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fgetc => Some({
            crate::codegen::lower_inst::builtins::io::lower_fgetc(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fgetcsv => Some({
            crate::codegen::lower_inst::builtins::io::lower_fgetcsv(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fgets => Some({
            crate::codegen::lower_inst::builtins::io::lower_fgets(ctx, inst)
        }),
        BuiltinRuntimeTarget::File => Some({
            crate::codegen::lower_inst::builtins::io::lower_file(ctx, inst)
        }),
        BuiltinRuntimeTarget::FileExists => Some({
            crate::codegen::lower_inst::builtins::io::lower_file_exists(ctx, inst)
        }),
        BuiltinRuntimeTarget::FileGetContents => Some({
            crate::codegen::lower_inst::builtins::io::lower_file_get_contents(ctx, inst)
        }),
        BuiltinRuntimeTarget::FilePutContents => Some({
            crate::codegen::lower_inst::builtins::io::lower_file_put_contents(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fileatime => Some({
            crate::codegen::lower_inst::builtins::io::lower_fileatime(ctx, inst)
        }),
        BuiltinRuntimeTarget::Filectime => Some({
            crate::codegen::lower_inst::builtins::io::lower_filectime(ctx, inst)
        }),
        BuiltinRuntimeTarget::Filegroup => Some({
            crate::codegen::lower_inst::builtins::io::lower_filegroup(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fileinode => Some({
            crate::codegen::lower_inst::builtins::io::lower_fileinode(ctx, inst)
        }),
        BuiltinRuntimeTarget::Filemtime => Some({
            crate::codegen::lower_inst::builtins::io::lower_filemtime(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fileowner => Some({
            crate::codegen::lower_inst::builtins::io::lower_fileowner(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fileperms => Some({
            crate::codegen::lower_inst::builtins::io::lower_fileperms(ctx, inst)
        }),
        BuiltinRuntimeTarget::Filesize => Some({
            crate::codegen::lower_inst::builtins::io::lower_filesize(ctx, inst)
        }),
        BuiltinRuntimeTarget::Filetype => Some({
            crate::codegen::lower_inst::builtins::io::lower_filetype(ctx, inst)
        }),
        BuiltinRuntimeTarget::Flock => Some({
            crate::codegen::lower_inst::builtins::io::lower_flock(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fnmatch => Some({
            crate::codegen::lower_inst::builtins::io::lower_fnmatch(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fopen => Some({
            crate::codegen::lower_inst::builtins::io::lower_fopen(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fpassthru => Some({
            crate::codegen::lower_inst::builtins::io::lower_fpassthru(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fprintf => Some({
            crate::codegen::lower_inst::builtins::io::lower_fprintf(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fputcsv => Some({
            crate::codegen::lower_inst::builtins::io::lower_fputcsv(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fread => Some({
            crate::codegen::lower_inst::builtins::io::lower_fread(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fscanf => Some({
            crate::codegen::lower_inst::builtins::io::lower_fscanf(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fseek => Some({
            crate::codegen::lower_inst::builtins::io::lower_fseek(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fsockopen => Some({
            crate::codegen::lower_inst::builtins::io::lower_fsockopen(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fstat => Some({
            crate::codegen::lower_inst::builtins::io::lower_fstat(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fsync => Some({
            crate::codegen::lower_inst::builtins::io::lower_fsync(ctx, inst)
        }),
        BuiltinRuntimeTarget::Ftell => Some({
            crate::codegen::lower_inst::builtins::io::lower_ftell(ctx, inst)
        }),
        _ => None,
    }
}
