//! Purpose:
//! Resolves string-literal function names used by callable/introspection builtins.
//! Shares PHP case-insensitive lookup between `function_exists()` and `is_callable()`.
//!
//! Called from:
//! - `crate::codegen::builtins::arrays::function_exists`
//! - `crate::codegen::builtins::types::is_callable`
//!
//! Key details:
//! - Include-discovered function variants stay distinguishable so `function_exists()` can keep runtime load-order behavior.

use crate::codegen::context::Context;
use crate::names::php_symbol_key;
use crate::types::checker::builtins::is_supported_builtin_function;

pub(super) enum FunctionLookup {
    AlwaysAvailable,
    IncludeVariant(String),
}

pub(super) fn lookup_function(ctx: &Context, name: &str) -> Option<FunctionLookup> {
    lookup_folded(ctx.function_variant_groups.iter(), name)
        .map(FunctionLookup::IncludeVariant)
        .or_else(|| {
            (is_supported_builtin_function(name)
                || lookup_folded(ctx.functions.keys(), name).is_some()
                || lookup_folded(ctx.extern_functions.keys(), name).is_some())
            .then_some(FunctionLookup::AlwaysAvailable)
        })
}

fn lookup_folded<'a, I>(names: I, name: &str) -> Option<String>
where
    I: IntoIterator<Item = &'a String>,
{
    let key = php_symbol_key(name);
    names
        .into_iter()
        .find(|candidate| php_symbol_key(candidate) == key)
        .cloned()
}
