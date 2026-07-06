//! Purpose:
//! Defines preg/PCRE flag constants exposed as PHP integer constants.
//! Keeps regex-related global constants in one shared table for checker and codegen seeding.
//!
//! Called from:
//! - `crate::types::checker::driver::init`
//! - `crate::codegen::prescan`
//!
//! Key details:
//! - Values match PHP's ext/pcre constants for RegexIterator and preg_split flag parity.

/// PHP preg integer constants used by regex builtins and SPL regex iterators.
pub(crate) const PREG_INT_CONSTANTS: &[(&str, i64)] = &[
    ("PREG_PATTERN_ORDER", 1),
    ("PREG_SET_ORDER", 2),
    ("PREG_OFFSET_CAPTURE", 256),
    ("PREG_UNMATCHED_AS_NULL", 512),
    ("PREG_SPLIT_NO_EMPTY", 1),
    ("PREG_SPLIT_DELIM_CAPTURE", 2),
    ("PREG_SPLIT_OFFSET_CAPTURE", 4),
];

/// Statically evaluates a preg flags argument expression.
///
/// Returns the value for int literals and `PREG_*` constant references (with or
/// without a leading backslash), or `None` for dynamic expressions. The checker's
/// `preg_match_all` output typing and the EIR desugar both key the matches-array
/// shape off this, so they must agree on what counts as statically known.
pub(crate) fn static_preg_flags_value(expr: &crate::parser::ast::Expr) -> Option<i64> {
    match &expr.kind {
        crate::parser::ast::ExprKind::IntLiteral(value) => Some(*value),
        crate::parser::ast::ExprKind::ConstRef(name) => {
            let key = name.as_str();
            let key = key.strip_prefix('\\').unwrap_or(key);
            PREG_INT_CONSTANTS
                .iter()
                .find(|(constant, _)| *constant == key)
                .map(|(_, value)| *value)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::PREG_INT_CONSTANTS;

    /// Verifies that PHP's offset-capture bit keeps its ext/pcre value.
    #[test]
    fn test_preg_offset_capture_value() {
        let entry = PREG_INT_CONSTANTS
            .iter()
            .find(|(name, _)| *name == "PREG_OFFSET_CAPTURE")
            .expect("PREG_OFFSET_CAPTURE defined");
        assert_eq!(entry.1, 256);
    }

    /// Verifies that no preg constant name is declared twice.
    #[test]
    fn test_preg_constants_have_unique_names() {
        let mut names: Vec<&str> = PREG_INT_CONSTANTS.iter().map(|(name, _)| *name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), PREG_INT_CONSTANTS.len());
    }
}
