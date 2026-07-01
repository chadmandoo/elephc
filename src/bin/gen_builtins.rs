//! Purpose:
//! Standalone tool that prints the single-source PHP builtin registry as documentation JSON.
//!
//! Called from:
//! - `cargo run --bin gen_builtins` (documentation generation / CI docs export).
//!
//! Key details:
//! - Delegates all logic to `elephc::builtins::docs::export_builtins_json()`; this binary only
//!   serializes that value to pretty JSON on stdout.

/// Prints the builtin documentation JSON (pretty-printed) to stdout.
fn main() {
    let value = elephc::builtins::docs::export_builtins_json();
    let json = serde_json::to_string_pretty(&value).expect("serialize builtins JSON");
    println!("{}", json);
}
