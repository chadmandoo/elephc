//! Purpose:
//! Emits fixed JSON literal and escape lookup data for JSON runtime helpers.
//! The data symbols are consumed by encoder and decoder state machines across target emitters.
//!
//! Called from:
//! - `crate::codegen::runtime::system::emit_json_data()` during fixed data emission.
//!
//! Key details:
//! - Data symbol names are consumed directly by JSON helpers and must not drift from scanner logic.

/// Emit JSON string constants for the data section.
pub(crate) fn emit_json_data() -> String {
    let mut out = String::new();
    out.push_str(".globl _json_true\n_json_true:\n    .ascii \"true\"\n");
    out.push_str(".globl _json_false\n_json_false:\n    .ascii \"false\"\n");
    out.push_str(".globl _json_null\n_json_null:\n    .ascii \"null\"\n");
    out
}
