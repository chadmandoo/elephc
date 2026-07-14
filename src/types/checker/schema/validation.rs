//! Purpose:
//! Validates schema validation declarations for the checker.
//! Turns parsed declarations into canonical metadata and rejects invalid contracts before code generation.
//!
//! Called from:
//! - `crate::types::checker::schema`
//!
//! Key details:
//! - Declaration metadata must align with name resolution, inheritance flattening, and runtime/codegen expectations.

use crate::errors::CompileError;
use crate::names::php_symbol_key;
use crate::parser::ast::{Attribute, ClassMethod, Expr, ExprKind, StmtKind, Visibility};
use crate::types::{FunctionSig, PhpType};

use super::super::Checker;

/// Builds a `FunctionSig` from a parsed class method, resolving parameter and return type
/// annotations through the checker. Parameters without type hints default to `PhpType::Mixed`
/// (PHP semantics: an unhinted parameter accepts any value).
/// Validates that each declared parameter's default value is compatible with its resolved type.
/// Infers return type from method body when no return annotation is present.
pub(crate) fn build_method_sig(
    checker: &Checker,
    method: &ClassMethod,
) -> Result<FunctionSig, CompileError> {
    let method_key = php_symbol_key(&method.name);
    let params: Vec<(String, PhpType)> = method
        .params
        .iter()
        .enumerate()
        .map(|(i, (n, type_ann, _, is_ref))| {
            // PHP's __unserialize($data) always receives the associative array
            // produced by __serialize(). A bare `array` hint resolves to an indexed
            // Array(Mixed) (rejecting $data['key']); type the first parameter as a
            // string/int-keyed assoc array so both the ABI and the body agree.
            // Scoped to user-defined methods (real span): synthetic SPL __unserialize
            // bodies are written for an indexed `array` and are called directly with
            // one (e.g. SplDoublyLinkedList), so they must keep Array(Mixed).
            if method_key == "__unserialize" && i == 0 && method.span.line != 0 {
                return Ok((
                    n.clone(),
                    PhpType::AssocArray {
                        key: Box::new(PhpType::Mixed),
                        value: Box::new(PhpType::Mixed),
                    },
                ));
            }
            // Unhinted parameters of runtime-dispatched protocol methods
            // (stream wrappers, 4-arg stream filters) take the types the
            // hand-emitted runtime dispatcher actually passes — the same
            // ABI-agreement rule as the `__unserialize` case above. All other
            // unhinted parameters are `mixed`, PHP's actual semantics.
            if type_ann.is_none() && method.span.line != 0 {
                if let Some(ty) =
                    wrapper_protocol_param_type(&method_key, method.params.len(), i)
                {
                    return Ok((n.clone(), ty));
                }
            }
            let ty = match type_ann {
                Some(type_ann) => checker.resolve_declared_param_type_hint(
                    type_ann,
                    method.span,
                    &format!("Method parameter ${}", n),
                )?,
                // Unhinted BY-REF parameters keep the legacy Int default:
                // the by-ref ABI passes the caller's raw local slot address
                // (tag-11 invoker ref cells, direct-slot markers), and a
                // Mixed-typed callee would reinterpret that raw slot as a
                // Mixed cell pointer. Promoting caller storage is the
                // Mixed-by-ref codegen follow-up (#623-adjacent); value
                // parameters get PHP's actual `mixed` semantics today.
                None if *is_ref => PhpType::Int,
                None => PhpType::Mixed,
            };
            Ok((n.clone(), ty))
        })
        .collect::<Result<Vec<_>, CompileError>>()?;
    let defaults: Vec<Option<Expr>> = method.params.iter().map(|(_, _, d, _)| d.clone()).collect();
    let ref_params: Vec<bool> = method.params.iter().map(|(_, _, _, r)| *r).collect();
    for ((param_name, type_ann, default, _), (_, resolved_ty)) in
        method.params.iter().zip(params.iter())
    {
        if type_ann.is_some() {
            checker.validate_schema_declared_default_type(
                resolved_ty,
                default.as_ref(),
                method.span,
                &format!("Method parameter ${}", param_name),
            )?;
        }
    }
    let return_type = match method.return_type.as_ref() {
        Some(type_ann) => checker.resolve_declared_return_type_hint(
            type_ann,
            method.span,
            &format!("Method '{}'", method.name),
        )?,
        // A method with NO BODY (interface / abstract) declares no return type AND has no
        // statements to infer one from: the return type is genuinely UNKNOWN, which in PHP
        // means `mixed`. Falling through to `infer_return_type_syntactic` instead walks an
        // EMPTY body, collects zero return statements, and lands on that helper's
        // "no return statements" default of `Int` — which then poisons every caller:
        // `is_string($x)` cannot narrow an `Int`, so handing the value to a `string`
        // parameter fails to type-check. PSR-7's untyped `getAttribute()` is the canonical
        // victim (`$req->getAttribute('type')` -> Int -> every `string` param rejects it).
        //
        // This is the RETURN-type analogue of the unhinted-value-parameter -> `Mixed` rule
        // applied to params just above. `has_body` — not "the body is empty" — is the
        // discriminator, so a CONCRETE `function f() {}`, which really does return null,
        // keeps its existing syntactic inference and is unaffected.
        None if !method.has_body => PhpType::Mixed,
        None => super::super::infer_return_type_syntactic(&method.body),
    };
    let mut sig = Checker::callable_wrapper_sig(&FunctionSig {
        params,
        defaults,
        return_type,
        declared_return: method.return_type.is_some(),
        by_ref_return: method.by_ref_return,
        ref_params,
        declared_params: method
            .params
            .iter()
            .map(|(_, type_ann, _, _)| type_ann.is_some())
            .chain(method.variadic.iter().map(|_| method.variadic_type.is_some()))
            .collect(),
        variadic: method.variadic.clone(),
        deprecation: extract_deprecation(&method.attributes),
    });
    // A declared element type on the variadic (`int ...$xs`) constrains every collected argument.
    // `callable_wrapper_sig` defaults the variadic container to `array<mixed>`; refine it to the
    // declared element type so call validation enforces it.
    if let Some(variadic_type) = &method.variadic_type {
        let elem_ty = checker.resolve_declared_param_type_hint(
            variadic_type,
            method.span,
            &format!(
                "Method variadic parameter ${}",
                method.variadic.as_deref().unwrap_or_default()
            ),
        )?;
        if let Some((_, ty)) = sig.params.last_mut() {
            *ty = PhpType::Array(Box::new(elem_ty));
        }
    }
    Ok(sig)
}

/// The parameter types the hand-emitted runtime dispatchers pass to
/// user-defined stream-wrapper / stream-filter protocol methods, keyed by
/// `(method key, declared arity)`. Consulted only for UNHINTED parameters of
/// real (user-span) methods: the dispatch ABI is fixed assembly
/// (`codegen_support::runtime::io`), so an unhinted parameter must be typed
/// as what that assembly passes, not as `mixed`. Hinted parameters are left
/// to the user's declaration. Path-mutation names shared with ordinary
/// classes (`unlink`, `rename`, `mkdir`, `rmdir`) are deliberately absent —
/// their dispatch tests declare hints, and the collision surface with
/// non-wrapper classes is too broad.
fn wrapper_protocol_param_type(
    method_key: &str,
    arity: usize,
    index: usize,
) -> Option<PhpType> {
    let table: &[PhpType] = match (method_key, arity) {
        // fopen dispatch: path ptr/len, mode ptr/len, options int,
        // &$opened_path as a zeroed 16-byte Mixed scratch cell.
        ("stream_open", 4) => &[PhpType::Str, PhpType::Str, PhpType::Int, PhpType::Mixed],
        ("stream_read", 1) => &[PhpType::Int],
        ("stream_write", 1) => &[PhpType::Str],
        ("stream_seek", 2) => &[PhpType::Int, PhpType::Int],
        ("stream_cast", 1) | ("stream_lock", 1) | ("stream_truncate", 1) => &[PhpType::Int],
        ("stream_set_option", 3) => &[PhpType::Int, PhpType::Int, PhpType::Int],
        ("stream_metadata", 3) => &[PhpType::Str, PhpType::Int, PhpType::Int],
        ("url_stat", 2) | ("dir_opendir", 2) => &[PhpType::Str, PhpType::Int],
        // Brigade dispatch: raw stdClass brigade pointers for $in/$out,
        // Mixed(int) cells for $consumed/$closing.
        ("filter", 4) => {
            return Some(match index {
                0 | 1 => PhpType::Object("stdClass".to_string()),
                _ => PhpType::Mixed,
            });
        }
        _ => return None,
    };
    table.get(index).cloned()
}

/// Returns `Some(reason)` when the attribute list contains a `#[\Deprecated]`
/// marker, with `reason` set to the attribute's first string argument (or an
/// empty string if absent). Match is case-insensitive on the last segment of
/// the attribute name.
pub(crate) fn extract_deprecation(
    groups: &[crate::parser::ast::AttributeGroup],
) -> Option<String> {
    for group in groups {
        for attr in &group.attributes {
            if !matches_global_builtin_attribute(attr, "Deprecated") {
                continue;
            }
            let reason = attr.args.iter().find_map(|expr| match &expr.kind {
                ExprKind::StringLiteral(s) => Some(s.clone()),
                _ => None,
            });
            return Some(reason.unwrap_or_default());
        }
    }
    None
}

/// Returns `true` if `attr` is a global builtin attribute matching `builtin` by name.
/// Fully-qualified names must match exactly (case-insensitive); unqualified names
/// match the last segment case-insensitively. Used to detect `#[\Deprecated]` and similar.
pub(crate) fn matches_global_builtin_attribute(attr: &Attribute, builtin: &str) -> bool {
    let name = attr.name.as_canonical();
    if attr.name.is_fully_qualified() {
        return name.eq_ignore_ascii_case(builtin);
    }
    attr.name.is_unqualified() && name.eq_ignore_ascii_case(builtin)
}

/// Builds a mapping from constructor parameter index to property name for each parameter.
/// For each parameter, searches constructor body for `PropertyAssign` statements where
/// the right-hand side is a Variable with the same name as the parameter; if found,
/// returns `Some(property_name)`, otherwise `None`. Returns empty vec if no constructor.
pub(crate) fn build_constructor_param_map(methods: &[ClassMethod]) -> Vec<Option<String>> {
    let mut param_to_prop = Vec::new();
    if let Some(constructor) = methods
        .iter()
        .find(|m| php_symbol_key(&m.name) == "__construct")
    {
        param_to_prop = constructor
            .params
            .iter()
            .map(|(pname, _, _, _)| {
                for stmt in &constructor.body {
                    if let StmtKind::PropertyAssign {
                        property, value, ..
                    } = &stmt.kind
                    {
                        if let ExprKind::Variable(vn) = &value.kind {
                            if vn == pname {
                                return Some(property.clone());
                            }
                        }
                    }
                }
                None
            })
            .collect();
    }
    param_to_prop
}

/// Returns a numeric rank for visibility levels: `private=0`, `protected=1`, `public=2`.
/// Used to enforce that overriding methods are not less visible than the parent method.
pub(crate) fn visibility_rank(visibility: &Visibility) -> u8 {
    match visibility {
        Visibility::Private => 0,
        Visibility::Protected => 1,
        Visibility::Public => 2,
    }
}

/// Counts how many parameters in `sig` are required (have no default).
/// The variadic parameter, if present, is never considered required even if it has no default.
pub(crate) fn required_param_count(sig: &FunctionSig) -> usize {
    sig.defaults
        .iter()
        .enumerate()
        .filter(|(idx, default)| {
            if sig.variadic.is_some() && *idx + 1 == sig.defaults.len() {
                return false;
            }
            default.is_none()
        })
        .count()
}

/// Validates that `child_sig` is compatible with `parent_sig` for override purposes.
/// Checks parameter count, ref params, defaults layout, variadic flag, and required param count.
/// Reports errors with `context` and `kind` (e.g., "overriding method") in the message.
pub(crate) fn validate_signature_compatibility(
    span: crate::span::Span,
    owner_name: &str,
    method_name: &str,
    child_sig: &FunctionSig,
    parent_sig: &FunctionSig,
    kind: &str,
    context: &str,
) -> Result<(), CompileError> {
    if child_sig.params.len() != parent_sig.params.len() {
        return Err(CompileError::new(
            span,
            &format!(
                "Cannot change parameter count when {} {}: {}::{}",
                context, kind, owner_name, method_name
            ),
        ));
    }

    if child_sig.ref_params != parent_sig.ref_params {
        return Err(CompileError::new(
            span,
            &format!(
                "Cannot change pass-by-reference parameters when {} {}: {}::{}",
                context, kind, owner_name, method_name
            ),
        ));
    }

    let child_defaults: Vec<bool> = child_sig
        .defaults
        .iter()
        .map(|default| default.is_some())
        .collect();
    let parent_defaults: Vec<bool> = parent_sig
        .defaults
        .iter()
        .map(|default| default.is_some())
        .collect();
    if child_defaults != parent_defaults {
        return Err(CompileError::new(
            span,
            &format!(
                "Cannot change optional parameter layout when {} {}: {}::{}",
                context, kind, owner_name, method_name
            ),
        ));
    }

    if child_sig.variadic != parent_sig.variadic {
        return Err(CompileError::new(
            span,
            &format!(
                "Cannot change variadic parameter shape when {} {}: {}::{}",
                context, kind, owner_name, method_name
            ),
        ));
    }

    if required_param_count(child_sig) != required_param_count(parent_sig) {
        return Err(CompileError::new(
            span,
            &format!(
                "Cannot change required parameter count when {} {}: {}::{}",
                context, kind, owner_name, method_name
            ),
        ));
    }

    Ok(())
}

/// Returns `true` if `actual` is a compatible declared return type for `expected`.
/// Allows `PhpType::Never` (unreachable) as a wildcard match. Otherwise delegates to
/// `checker.type_accepts(expected, actual)` for standard subtype checking.
pub(crate) fn declared_return_type_compatible(
    checker: &Checker,
    expected: &PhpType,
    actual: &PhpType,
) -> bool {
    matches!(actual, PhpType::Never) || checker.type_accepts(expected, actual)
}

/// Validates that `method` can override `parent_sig` in class `class_name`.
/// Builds the child signature via `build_method_sig`, skips validation for `__construct`,
/// checks signature compatibility, and ensures the child does not remove a declared
/// return type when the parent has one or make it incompatible.
pub(crate) fn validate_override_signature(
    checker: &Checker,
    class_name: &str,
    method: &ClassMethod,
    parent_sig: &FunctionSig,
    is_static: bool,
) -> Result<(), CompileError> {
    let kind = if is_static { "static method" } else { "method" };
    let child_sig = build_method_sig(checker, method)?;
    if php_symbol_key(&method.name) == "__construct" {
        return Ok(());
    }
    validate_signature_compatibility(
        method.span,
        class_name,
        &method.name,
        &child_sig,
        parent_sig,
        kind,
        "overriding",
    )?;
    if parent_sig.declared_return && !child_sig.declared_return {
        return Err(CompileError::new(
            method.span,
            &format!(
                "Cannot override {} {}::{} without declaring a compatible return type (parent returns {})",
                kind, class_name, method.name, parent_sig.return_type
            ),
        ));
    }
    if parent_sig.declared_return
        && !declared_return_type_compatible(checker, &parent_sig.return_type, &child_sig.return_type)
    {
        return Err(CompileError::new(
            method.span,
            &format!(
                "Cannot override {} {}::{} with incompatible return type {} (parent returns {})",
                kind, class_name, method.name, child_sig.return_type, parent_sig.return_type
            ),
        ));
    }
    Ok(())
}
