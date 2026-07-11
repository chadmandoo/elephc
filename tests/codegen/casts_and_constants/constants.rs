//! Purpose:
//! Integration or regression tests for end-to-end codegen coverage of casts, constants, and introspection constants, including php integer max, php integer min, and m pi.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Inline PHP fixtures are compiled to native binaries and assertions compare stdout or expected failures.

use super::*;

/// Verifies `PHP_INT_MAX` constant is correctly substituted at compile time and the
/// resulting binary outputs the maximum 64-bit signed integer value.
/// Fixture: `<?php echo PHP_INT_MAX;` → expects `9223372036854775807`.
#[test]
fn test_php_int_max() {
    let out = compile_and_run("<?php echo PHP_INT_MAX;");
    assert_eq!(out, "9223372036854775807");
}

/// Verifies `PHP_INT_MIN` constant is correctly substituted at compile time and the
/// resulting binary outputs the minimum 64-bit signed integer value.
/// Fixture: `<?php echo PHP_INT_MIN;` → expects `-9223372036854775808`.
#[test]
fn test_php_int_min() {
    let out = compile_and_run("<?php echo PHP_INT_MIN;");
    assert_eq!(out, "-9223372036854775808");
}

/// Verifies `M_PI` math constant is correctly substituted at compile time and the
/// resulting binary outputs the correct float approximation.
/// Fixture: `<?php echo M_PI;` → expects `3.1415926535898`.
#[test]
fn test_m_pi() {
    let out = compile_and_run("<?php echo M_PI;");
    assert_eq!(out, "3.1415926535898");
}

/// Verifies `PHP_FLOAT_MAX` constant is correctly substituted and the resulting binary
/// runs without crash; also verifies `is_float()` returns true for the value.
/// Fixture: `<?php echo is_float(PHP_FLOAT_MAX);` → expects `1`.
#[test]
fn test_php_float_max() {
    let out = compile_and_run("<?php echo is_float(PHP_FLOAT_MAX);");
    assert_eq!(out, "1");
}

/// Verifies the `SORT_*` sort-flag integer constants are substituted at compile time
/// with PHP's canonical values (`SORT_REGULAR`=0 … `SORT_FLAG_CASE`=8).
/// Fixture echoes each flag joined by `,`.
#[test]
fn test_sort_flag_constants() {
    let out = compile_and_run(
        "<?php echo SORT_REGULAR, ',', SORT_NUMERIC, ',', SORT_STRING, ',', \
         SORT_DESC, ',', SORT_ASC, ',', SORT_LOCALE_STRING, ',', SORT_NATURAL, \
         ',', SORT_FLAG_CASE;",
    );
    assert_eq!(out, "0,1,2,3,4,5,6,8");
}

/// Verifies the `PASSWORD_*` algorithm-identifier constants are substituted as their
/// PHP 7.4+ string values (`PASSWORD_ARGON2ID` = "argon2id", etc.).
/// Fixture echoes each identifier joined by `|`.
#[test]
fn test_password_algo_constants() {
    let out = compile_and_run(
        "<?php echo PASSWORD_DEFAULT, '|', PASSWORD_BCRYPT, '|', \
         PASSWORD_ARGON2I, '|', PASSWORD_ARGON2ID;",
    );
    assert_eq!(out, "2y|2y|argon2i|argon2id");
}
