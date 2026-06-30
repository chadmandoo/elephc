//! Purpose:
//! Parity gate between registry-derived builtin signatures and the legacy
//! `builtin_call_sig()` table. For every builtin registered in the inventory
//! that also has a legacy table entry, this file asserts that the
//! behavior-bearing fields of the two `FunctionSig`s agree.
//!
//! Called from:
//! - `cargo test` through Rust's test harness (unit test module).
//!
//! Key details:
//! - Placed here (not in `tests/`) because `builtin_call_sig` is `pub(crate)`
//!   and cannot be reached from an integration test without widening visibility.
//! - Type fields (`params[*].1`, `return_type`, `declared_return`,
//!   `declared_params`) are intentionally excluded from comparison: the registry
//!   derives precise types while the legacy table uses `PhpType::Mixed`
//!   placeholders. That precision is an intended improvement and is
//!   behavior-neutral (call-arg planning reads only param NAMES, never types).
//! - The test loop body runs zero times until area-migration tasks (Task 8+)
//!   register real builtins. That is correct — this file builds the harness;
//!   it does real work once production builtins are added to the inventory.

use crate::builtins::registry;
use crate::types::{builtin_call_sig, FunctionSig};

/// Asserts that the behavior-bearing fields of `derived` and `legacy` agree.
///
/// Fields compared:
/// - param names (`.0` of each `(String, PhpType)` pair, in order)
/// - defaults (rendered via `{:?}` for a stable comparison of `Option<Expr>`)
/// - ref_params (per-parameter by-reference flags)
/// - variadic (the variadic parameter name, if any)
/// - by_ref_return
/// - total param count and required param count (arity)
///
/// Panics with a message naming the builtin and the diverging field.
fn assert_behavior_fields_match(name: &str, derived: &FunctionSig, legacy: &FunctionSig) {
    // Arity: total param count.
    assert_eq!(
        derived.params.len(),
        legacy.params.len(),
        "signature drift for {name}: param count differs (derived={}, legacy={})",
        derived.params.len(),
        legacy.params.len(),
    );

    // Required param count: params with no default.
    let derived_required = derived.defaults.iter().filter(|d| d.is_none()).count();
    let legacy_required = legacy.defaults.iter().filter(|d| d.is_none()).count();
    assert_eq!(
        derived_required,
        legacy_required,
        "signature drift for {name}: required param count differs (derived={derived_required}, legacy={legacy_required})",
    );

    // Param names (in order).
    let derived_names: Vec<&str> = derived.params.iter().map(|(n, _)| n.as_str()).collect();
    let legacy_names: Vec<&str> = legacy.params.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        derived_names,
        legacy_names,
        "signature drift for {name}: param names differ (derived={derived_names:?}, legacy={legacy_names:?})",
    );

    // Defaults (stable debug representation for Option<Expr>).
    let derived_defaults = format!("{:?}", derived.defaults);
    let legacy_defaults = format!("{:?}", legacy.defaults);
    assert_eq!(
        derived_defaults,
        legacy_defaults,
        "signature drift for {name}: defaults differ\n  derived={derived_defaults}\n  legacy={legacy_defaults}",
    );

    // Per-parameter by-reference flags.
    assert_eq!(
        derived.ref_params,
        legacy.ref_params,
        "signature drift for {name}: ref_params differ (derived={:?}, legacy={:?})",
        derived.ref_params,
        legacy.ref_params,
    );

    // Variadic parameter name.
    assert_eq!(
        derived.variadic,
        legacy.variadic,
        "signature drift for {name}: variadic differs (derived={:?}, legacy={:?})",
        derived.variadic,
        legacy.variadic,
    );

    // By-reference return flag.
    assert_eq!(
        derived.by_ref_return,
        legacy.by_ref_return,
        "signature drift for {name}: by_ref_return differs (derived={}, legacy={})",
        derived.by_ref_return,
        legacy.by_ref_return,
    );
}

/// Verifies that every registry-derived builtin signature agrees with the legacy
/// `builtin_call_sig()` table on all behavior-bearing fields.
///
/// Iterates all names registered in the inventory. For each name that also has
/// a legacy table entry, runs `assert_behavior_fields_match`. Names with no
/// legacy entry (not yet migrated) are skipped — the gate activates
/// incrementally as migration tasks (Task 8+) register real builtins.
///
/// At this point only internal test probes are registered, so the assertion
/// body runs zero times and the test is trivially green. That is expected.
#[test]
fn derived_signatures_match_legacy() {
    for name in registry::names() {
        // Skip builtins not yet present in the legacy table.
        // The parity check activates as each area is migrated (Task 8+).
        let Some(legacy) = builtin_call_sig(name) else {
            continue;
        };

        let derived = registry::function_sig(name)
            .unwrap_or_else(|| panic!("registry::names() yielded {name} but function_sig returned None"));

        assert_behavior_fields_match(name, &derived, &legacy);
    }
}
