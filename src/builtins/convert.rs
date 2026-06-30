//! Purpose:
//! Conversion helpers that bridge `BuiltinSpec` fields into the legacy catalog,
//! signature, and type-checker representations during the migration period.
//!
//! Called from:
//! - `crate::builtins::registry` when populating the legacy dispatch tables.
//!
//! Key details:
//! - This module is intentionally private (`mod convert;` without `pub`) and will
//!   shrink as each legacy dispatch point is replaced by direct registry queries.
