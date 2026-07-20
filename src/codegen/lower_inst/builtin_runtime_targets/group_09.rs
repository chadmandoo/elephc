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

/// Lowers a target owned by bounded dispatch group 09, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {
    match target {
        BuiltinRuntimeTarget::SplAutoloadUnregister => Some({
            crate::codegen::lower_inst::builtins::spl::lower_spl_autoload_bool(
                    ctx,
                    inst,
                    "spl_autoload_unregister",
                )
        }),
        BuiltinRuntimeTarget::SplClasses => Some({
            crate::codegen::lower_inst::builtins::spl::lower_spl_classes(ctx, inst)
        }),
        BuiltinRuntimeTarget::SplObjectHash => Some({
            crate::codegen::lower_inst::builtins::spl::lower_spl_object_hash(ctx, inst)
        }),
        BuiltinRuntimeTarget::SplObjectId => Some({
            crate::codegen::lower_inst::builtins::spl::lower_spl_object_id(ctx, inst)
        }),
        BuiltinRuntimeTarget::Chop => Some({
            crate::codegen::lower_inst::builtins::strings::lower_trim_like(
                    ctx,
                    inst,
                    "chop",
                    "__rt_rtrim",
                    "__rt_rtrim_mask",
                )
        }),
        BuiltinRuntimeTarget::Chr => Some({
            crate::codegen::lower_inst::builtins::strings::lower_chr(ctx, inst)
        }),
        BuiltinRuntimeTarget::Crc32 => Some({
            crate::codegen::lower_inst::builtins::strings::lower_crc32(ctx, inst)
        }),
        BuiltinRuntimeTarget::CtypeAlnum => Some({
            crate::codegen::lower_inst::builtins::ctype::lower_ctype_alnum(ctx, inst)
        }),
        BuiltinRuntimeTarget::CtypeAlpha => Some({
            crate::codegen::lower_inst::builtins::ctype::lower_ctype_alpha(ctx, inst)
        }),
        BuiltinRuntimeTarget::CtypeDigit => Some({
            crate::codegen::lower_inst::builtins::ctype::lower_ctype_digit(ctx, inst)
        }),
        BuiltinRuntimeTarget::CtypeSpace => Some({
            crate::codegen::lower_inst::builtins::ctype::lower_ctype_space(ctx, inst)
        }),
        BuiltinRuntimeTarget::Explode => Some({
            crate::codegen::lower_inst::builtins::strings::lower_explode(ctx, inst)
        }),
        BuiltinRuntimeTarget::GraphemeStrrev => Some({
            crate::codegen::lower_inst::builtins::strings::lower_grapheme_strrev(ctx, inst)
        }),
        BuiltinRuntimeTarget::Gzcompress => Some({
            crate::codegen::lower_inst::builtins::strings::lower_gzcompress(ctx, inst)
        }),
        BuiltinRuntimeTarget::Gzdeflate => Some({
            crate::codegen::lower_inst::builtins::strings::lower_gzdeflate(ctx, inst)
        }),
        BuiltinRuntimeTarget::Gzinflate => Some({
            crate::codegen::lower_inst::builtins::strings::lower_gzinflate(ctx, inst)
        }),
        BuiltinRuntimeTarget::Gzuncompress => Some({
            crate::codegen::lower_inst::builtins::strings::lower_gzuncompress(ctx, inst)
        }),
        BuiltinRuntimeTarget::Hash => Some({
            crate::codegen::lower_inst::builtins::strings::lower_hash(ctx, inst)
        }),
        BuiltinRuntimeTarget::HashAlgos => Some({
            crate::codegen::lower_inst::builtins::strings::lower_hash_algos(ctx, inst)
        }),
        BuiltinRuntimeTarget::HashCopy => Some({
            crate::codegen::lower_inst::builtins::strings::lower_hash_copy(ctx, inst)
        }),
        BuiltinRuntimeTarget::HashEquals => Some({
            crate::codegen::lower_inst::builtins::strings::lower_hash_equals(ctx, inst)
        }),
        BuiltinRuntimeTarget::HashFinal => Some({
            crate::codegen::lower_inst::builtins::strings::lower_hash_final(ctx, inst)
        }),
        BuiltinRuntimeTarget::HashHmac => Some({
            crate::codegen::lower_inst::builtins::strings::lower_hash_hmac(ctx, inst)
        }),
        BuiltinRuntimeTarget::HashInit => Some({
            crate::codegen::lower_inst::builtins::strings::lower_hash_init(ctx, inst)
        }),
        BuiltinRuntimeTarget::HashUpdate => Some({
            crate::codegen::lower_inst::builtins::strings::lower_hash_update(ctx, inst)
        }),
        BuiltinRuntimeTarget::Htmlentities => Some({
            crate::codegen::lower_inst::builtins::strings::lower_html_escape(ctx, inst, "htmlentities")
        }),
        BuiltinRuntimeTarget::Htmlspecialchars => Some({
            crate::codegen::lower_inst::builtins::strings::lower_html_escape(ctx, inst, "htmlspecialchars")
        }),
        BuiltinRuntimeTarget::Implode => Some({
            crate::codegen::lower_inst::builtins::strings::lower_implode(ctx, inst)
        }),
        BuiltinRuntimeTarget::InetNtop => Some({
            crate::codegen::lower_inst::builtins::strings::lower_inet(
                    ctx,
                    inst,
                    "inet_ntop",
                    "__rt_inet_ntop",
                )
        }),
        BuiltinRuntimeTarget::InetPton => Some({
            crate::codegen::lower_inst::builtins::strings::lower_inet(
                    ctx,
                    inst,
                    "inet_pton",
                    "__rt_inet_pton",
                )
        }),
        BuiltinRuntimeTarget::Ip2long => Some({
            crate::codegen::lower_inst::builtins::strings::lower_ip2long(ctx, inst)
        }),
        BuiltinRuntimeTarget::Lcfirst => Some({
            crate::codegen::lower_inst::builtins::strings::lower_lcfirst(ctx, inst)
        }),
        BuiltinRuntimeTarget::Long2ip => Some({
            crate::codegen::lower_inst::builtins::strings::lower_long2ip(ctx, inst)
        }),
        BuiltinRuntimeTarget::Ltrim => Some({
            crate::codegen::lower_inst::builtins::strings::lower_trim_like(
                    ctx,
                    inst,
                    "ltrim",
                    "__rt_ltrim",
                    "__rt_ltrim_mask",
                )
        }),
        BuiltinRuntimeTarget::MbEregMatch => Some({
            crate::codegen::lower_inst::builtins::regex::lower_mb_ereg_match(ctx, inst)
        }),
        _ => None,
    }
}
