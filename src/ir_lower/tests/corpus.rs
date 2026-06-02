//! Purpose:
//! Corpus validation tests for AST-to-EIR lowering over real example programs.
//!
//! Called from:
//! - `crate::ir_lower::tests`.
//!
//! Key details:
//! - Exercises the full frontend ordering, including resolver and autoload, on
//!   each `examples/*/main.php` fixture before EIR validation.

use std::path::{Path, PathBuf};

/// Verifies every checked example program lowers to validated printable EIR.
#[test]
fn lowers_examples_corpus() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut fixtures = example_main_files(root);
    fixtures.sort();
    assert!(!fixtures.is_empty(), "expected example PHP fixtures");

    for fixture in fixtures {
        let module = super::lower_file(&fixture);
        assert!(
            !module.functions.is_empty(),
            "expected at least main function for {}",
            fixture.display()
        );
    }
}

/// Returns all example `main.php` fixtures in deterministic order.
fn example_main_files(root: &Path) -> Vec<PathBuf> {
    let examples = root.join("examples");
    std::fs::read_dir(&examples)
        .expect("examples directory should exist")
        .map(|entry| entry.expect("example entry").path().join("main.php"))
        .filter(|path| path.exists())
        .collect()
}
