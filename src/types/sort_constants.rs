//! Purpose:
//! Defines PHP sort-related integer constants exposed by elephc.
//! Keeps the `SORT_*` flags in one source of truth for type checking and codegen.
//!
//! Called from:
//! - `crate::types::checker` when registering predefined constants.
//! - `crate::codegen::prescan` when materializing constant literal values.
//!
//! Key details:
//! - Values must match PHP's sort-flag constants exactly for `sort()`/`ksort()`
//!   comparison-mode parity.

/// Tuple of `(name, value)` pairs for PHP sort integer constants.
///
/// The `sort()` family (`sort`, `rsort`, `ksort`, `asort`, `usort`, …) accepts these
/// flags to select the comparison mode; `array_multisort` uses `SORT_ASC`/`SORT_DESC`.
pub(crate) const SORT_INT_CONSTANTS: &[(&str, i64)] = &[
    ("SORT_REGULAR", 0),
    ("SORT_NUMERIC", 1),
    ("SORT_STRING", 2),
    ("SORT_DESC", 3),
    ("SORT_ASC", 4),
    ("SORT_LOCALE_STRING", 5),
    ("SORT_NATURAL", 6),
    ("SORT_FLAG_CASE", 8),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies PHP's string-comparison sort mode carries its canonical value.
    #[test]
    fn sort_string_is_two() {
        let entry = SORT_INT_CONSTANTS
            .iter()
            .find(|(name, _)| *name == "SORT_STRING")
            .expect("SORT_STRING defined");
        assert_eq!(entry.1, 2);
    }

    /// Asserts no duplicate names exist in `SORT_INT_CONSTANTS`.
    #[test]
    fn no_duplicate_constant_names() {
        let mut names: Vec<&str> = SORT_INT_CONSTANTS.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        let len_before = names.len();
        names.dedup();
        assert_eq!(names.len(), len_before, "duplicate sort constant name");
    }
}
