//! Purpose:
//! Builtin existence name table used by eval function probes.
//!
//! Called from:
//! - `crate::interpreter::builtins::registry` re-exports.
//!
//! Key details:
//! - The slice is the source of truth for PHP-visible eval builtin names.
//! - Lookup callers pass canonical lowercase PHP symbol names.

use std::sync::OnceLock;

use super::{eval_declared_builtin_exists, eval_declared_builtin_function_names};

/// PHP-visible builtin names implemented by the eval interpreter.
pub(in crate::interpreter) const EVAL_PHP_VISIBLE_BUILTIN_FUNCTIONS: &[&str] = &[];

/// Combined PHP-visible builtin names from legacy and declarative registries.
static EVAL_PHP_VISIBLE_BUILTIN_FUNCTION_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();

/// Returns the eval interpreter's PHP-visible builtin names.
pub(in crate::interpreter) fn eval_php_visible_builtin_function_names() -> &'static [&'static str] {
    EVAL_PHP_VISIBLE_BUILTIN_FUNCTION_NAMES
        .get_or_init(|| {
            let mut names = EVAL_PHP_VISIBLE_BUILTIN_FUNCTIONS.to_vec();
            for name in eval_declared_builtin_function_names() {
                if !names.contains(name) {
                    names.push(name);
                }
            }
            names.sort_unstable();
            names
        })
        .as_slice()
}

/// Returns true for PHP-visible builtin names implemented by the eval interpreter.
pub(in crate::interpreter) fn eval_php_visible_builtin_exists(name: &str) -> bool {
    eval_declared_builtin_exists(name) || EVAL_PHP_VISIBLE_BUILTIN_FUNCTIONS.contains(&name)
}
