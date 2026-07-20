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

/// Lowers a target owned by bounded dispatch group 02, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {
    match target {
        BuiltinRuntimeTarget::GetClass => Some({
            crate::codegen::lower_inst::builtins::types::lower_class_name_lookup(ctx, inst, "get_class")
        }),
        BuiltinRuntimeTarget::GetDeclaredClasses => Some({
            crate::codegen::lower_inst::builtins::types::lower_get_declared_names(
                    ctx,
                    inst,
                    "get_declared_classes",
                )
        }),
        BuiltinRuntimeTarget::GetDeclaredInterfaces => Some({
            crate::codegen::lower_inst::builtins::types::lower_get_declared_names(
                    ctx,
                    inst,
                    "get_declared_interfaces",
                )
        }),
        BuiltinRuntimeTarget::GetDeclaredTraits => Some({
            crate::codegen::lower_inst::builtins::types::lower_get_declared_names(
                    ctx,
                    inst,
                    "get_declared_traits",
                )
        }),
        BuiltinRuntimeTarget::GetParentClass => Some({
            crate::codegen::lower_inst::builtins::types::lower_class_name_lookup(
                    ctx,
                    inst,
                    "get_parent_class",
                )
        }),
        BuiltinRuntimeTarget::InterfaceExists => Some({
            crate::codegen::lower_inst::builtins::lower_class_like_exists(
                    ctx,
                    inst,
                    "interface_exists",
                )
        }),
        BuiltinRuntimeTarget::IsA => Some({
            crate::codegen::lower_inst::builtins::types::lower_is_a_relation(ctx, inst, "is_a")
        }),
        BuiltinRuntimeTarget::IsSubclassOf => Some({
            crate::codegen::lower_inst::builtins::types::lower_is_a_relation(
                    ctx,
                    inst,
                    "is_subclass_of",
                )
        }),
        BuiltinRuntimeTarget::PregReplaceCallback => Some({
            crate::codegen::lower_inst::builtins::regex::lower_preg_replace_callback(ctx, inst)
        }),
        BuiltinRuntimeTarget::TraitExists => Some({
            crate::codegen::lower_inst::builtins::lower_class_like_exists(ctx, inst, "trait_exists")
        }),
        BuiltinRuntimeTarget::ElephcPharBzip2Archive => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_bzip2_archive(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharDecompressArchive => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_decompress_archive(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharGetFileMetadata => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_get_file_metadata(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharGetMetadata => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_get_metadata(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharGetSignatureHash => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_get_signature_hash(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharGetSignatureType => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_get_signature_type(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharGetStub => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_get_stub(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharGzipArchive => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_gzip_archive(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharListEntries => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_list_entries(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharSetCompression => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_set_compression(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharSetFileMetadata => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_set_file_metadata(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharSetMetadata => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_set_metadata(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharSetStub => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_set_stub(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharSetZipPassword => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_set_zip_password(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharSignHash => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_sign_hash(ctx, inst)
        }),
        BuiltinRuntimeTarget::ElephcPharSignOpenssl => Some({
            crate::codegen::lower_inst::builtins::io::lower_elephc_phar_sign_openssl(ctx, inst)
        }),
        BuiltinRuntimeTarget::Basename => Some({
            crate::codegen::lower_inst::builtins::io::lower_basename(ctx, inst)
        }),
        BuiltinRuntimeTarget::Chdir => Some({
            crate::codegen::lower_inst::builtins::io::lower_chdir(ctx, inst)
        }),
        BuiltinRuntimeTarget::Chgrp => Some({
            crate::codegen::lower_inst::builtins::io::lower_chgrp(ctx, inst)
        }),
        BuiltinRuntimeTarget::Chmod => Some({
            crate::codegen::lower_inst::builtins::io::lower_chmod(ctx, inst)
        }),
        BuiltinRuntimeTarget::Chown => Some({
            crate::codegen::lower_inst::builtins::io::lower_chown(ctx, inst)
        }),
        BuiltinRuntimeTarget::Clearstatcache => Some({
            crate::codegen::lower_inst::builtins::io::lower_clearstatcache(ctx, inst)
        }),
        BuiltinRuntimeTarget::Closedir => Some({
            crate::codegen::lower_inst::builtins::io::lower_closedir(ctx, inst)
        }),
        BuiltinRuntimeTarget::Copy => Some({
            crate::codegen::lower_inst::builtins::io::lower_copy(ctx, inst)
        }),
        BuiltinRuntimeTarget::Dirname => Some({
            crate::codegen::lower_inst::builtins::io::lower_dirname(ctx, inst)
        }),
        _ => None,
    }
}
