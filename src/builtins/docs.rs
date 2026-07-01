//! Purpose:
//! Serialises the single-source builtin registry to a JSON array for documentation tooling.
//! Every PHP-visible registered builtin is emitted as one object; internal builtins are skipped.
//!
//! Called from:
//! - `src/bin/gen_builtins.rs` via `elephc::builtins::docs::export_builtins_json()`.
//!
//! Key details:
//! - Uses `crate::builtins::registry::{names, lookup}` so `inventory::iter` runs in the same
//!   crate that submitted the `builtin!` entries — required for the iterator to see all specs.
//! - `TypeSpec` rendering is recursive (handles `ArrayOf`/`AssocOf`/`Union` nesting).
//! - Builtins with `internal: true` are excluded from the export.
//! - `#![allow(dead_code)]` suppresses warnings when the module is compiled in the context of
//!   the `elephc` binary (which never calls `export_builtins_json`); all items here are live
//!   from the `gen_builtins` binary's perspective.
#![allow(dead_code)]

use crate::builtins::registry::{lookup, names};
use crate::builtins::spec::{Area, DefaultSpec, TypeSpec};
use serde_json::{json, Value};

/// Renders a `TypeSpec` as a PHP-style type string for documentation JSON.
fn type_spec_str(ty: &TypeSpec) -> String {
    match ty {
        TypeSpec::Int => "int".to_string(),
        TypeSpec::Float => "float".to_string(),
        TypeSpec::Str => "string".to_string(),
        TypeSpec::Bool => "bool".to_string(),
        TypeSpec::Mixed => "mixed".to_string(),
        TypeSpec::Null => "null".to_string(),
        TypeSpec::Void => "void".to_string(),
        TypeSpec::ArrayOf(inner) => format!("array<{}>", type_spec_str(inner)),
        TypeSpec::AssocOf(inner) => format!("array<string, {}>", type_spec_str(inner)),
        TypeSpec::Union(members) => members
            .iter()
            .map(type_spec_str)
            .collect::<Vec<_>>()
            .join("|"),
    }
}

/// Maps a builtin `Area` to its lowercase documentation category name.
fn area_str(area: Area) -> &'static str {
    match area {
        Area::String => "string",
        Area::Array => "array",
        Area::Math => "math",
        Area::Io => "io",
        Area::System => "system",
        Area::Types => "types",
        Area::Callables => "callables",
        Area::Spl => "spl",
        Area::Pointers => "pointers",
        Area::Internal => "internal",
    }
}

/// Renders a parameter `DefaultSpec` as its documentation JSON value.
fn default_spec_json(default: &DefaultSpec) -> Value {
    match default {
        DefaultSpec::Null => Value::Null,
        DefaultSpec::Int(v) => json!(v),
        DefaultSpec::Bool(v) => json!(v),
        DefaultSpec::Float(v) => json!(v),
        DefaultSpec::Str(v) => json!(v),
        DefaultSpec::IntMax => json!("PHP_INT_MAX"),
        DefaultSpec::IntMin => json!("PHP_INT_MIN"),
        DefaultSpec::EmptyArray => json!([]),
    }
}

/// Builds the documentation JSON array for every PHP-visible registered builtin.
///
/// Iterates the registry in sorted name order, skips `internal` builtins, and emits one object per
/// builtin with its area, parameters (name/type/by_ref/optional/default), variadic name, arity overrides,
/// return type, summary, examples, PHP-manual fragment, and deprecation. Consumed by the
/// `gen_builtins` binary for documentation generation.
pub fn export_builtins_json() -> Value {
    let mut out: Vec<Value> = Vec::new();
    for name in names() {
        let Some(def) = lookup(name) else { continue };
        let spec = def.spec;
        if spec.internal {
            continue;
        }
        let params: Vec<Value> = spec
            .params
            .iter()
            .map(|p| {
                json!({
                    "name": p.name,
                    "type": type_spec_str(&p.ty),
                    "by_ref": p.by_ref,
                    // `optional` disambiguates a required param (no default) from an
                    // optional param whose default value is literally `null`; both would
                    // otherwise render as JSON `null` under the `default` key.
                    "optional": p.default.is_some(),
                    "default": p.default.as_ref().map(default_spec_json).unwrap_or(Value::Null),
                })
            })
            .collect();
        out.push(json!({
            "name": spec.name,
            "area": area_str(spec.area),
            "params": params,
            "variadic": spec.variadic,
            "returns": type_spec_str(&spec.returns),
            "by_ref_return": spec.by_ref_return,
            "min_args": spec.min_args,
            "max_args": spec.max_args,
            "arity_error": spec.arity_error,
            "summary": spec.summary,
            "examples": spec.examples,
            "php_manual": spec.php_manual,
            "deprecated": spec.deprecation,
        }));
    }
    Value::Array(out)
}

#[cfg(test)]
mod tests {
    /// Verifies the exporter emits a non-empty array and a known builtin (`strlen`) with its
    /// documented shape (required string param, int return, non-internal).
    #[test]
    fn export_contains_strlen_with_expected_shape() {
        let v = super::export_builtins_json();
        let arr = v.as_array().expect("top-level array");
        assert!(!arr.is_empty());
        let strlen = arr
            .iter()
            .find(|e| e["name"] == "strlen")
            .expect("strlen present");
        assert_eq!(strlen["area"], "string");
        assert_eq!(strlen["returns"], "int");
        assert_eq!(strlen["params"][0]["name"], "string");
        // `strlen`'s sole param is required, so `optional` must be false (it has no default).
        assert_eq!(strlen["params"][0]["optional"], false);
        // No internal builtins leak into the docs export.
        assert!(arr.iter().all(|e| e["name"].as_str().map_or(false, |n| !n.starts_with("__elephc_"))));
    }
}
