//! Purpose:
//! Type-checks the system PHP builtin family.
//! Validates arity, argument types, warning-producing cases, and inferred return types for direct calls.
//!
//! Called from:
//! - `crate::types::checker::builtins::check_builtin()`
//!
//! Key details:
//! - Signatures, callable aliases, optimizer effects, and codegen builtin dispatch must remain in lockstep.

use crate::errors::CompileError;
use crate::parser::ast::{BinOp, Expr, ExprKind};
use crate::types::json_constants::JSON_INT_CONSTANTS;
use crate::types::{PhpType, TypeEnv};

use super::super::Checker;

type BuiltinResult = Result<Option<PhpType>, CompileError>;

/// Type-checks a system builtin call by name, validating arity, argument types,
/// and return type. Returns `Ok(Some(PhpType))` for handled builtins, `Ok(None)`
/// for unknown system builtins, or an error for misuse.
pub(super) fn check_builtin(
    checker: &mut Checker,
    name: &str,
    args: &[Expr],
    span: crate::span::Span,
    env: &TypeEnv,
) -> BuiltinResult {
    match name {
        "json_encode" => {
            if args.is_empty() || args.len() > 3 {
                return Err(CompileError::new(
                    span,
                    "json_encode() takes 1 to 3 arguments",
                ));
            }
            checker.infer_type(&args[0], env)?;
            for extra in &args[1..] {
                let ty = checker.infer_type(extra, env)?;
                if ty != PhpType::Int {
                    return Err(CompileError::new(
                        extra.span,
                        "json_encode() flags and depth must be integers",
                    ));
                }
            }
            Ok(Some(PhpType::Str))
        }
        "json_decode" => {
            if args.is_empty() || args.len() > 4 {
                return Err(CompileError::new(
                    span,
                    "json_decode() takes 1 to 4 arguments",
                ));
            }
            let json_ty = checker.infer_type(&args[0], env)?;
            if !is_json_string_arg_type(&json_ty) {
                return Err(CompileError::new(
                    args[0].span,
                    "json_decode() json argument must be string-compatible",
                ));
            }
            if let Some(assoc) = args.get(1) {
                let assoc_ty = checker.infer_type(assoc, env)?;
                if !is_json_associative_arg_type(&assoc_ty) {
                    return Err(CompileError::new(
                        assoc.span,
                        "json_decode() associative argument must be bool-compatible or null",
                    ));
                }
            }
            for extra in args.iter().skip(2) {
                let ty = checker.infer_type(extra, env)?;
                if ty != PhpType::Int {
                    return Err(CompileError::new(
                        extra.span,
                        "json_decode() depth and flags must be integers",
                    ));
                }
            }
            // Returns a structural Mixed: scalars (null/bool/int/float/string)
            // box natively; arrays and objects currently fall back to a
            // Mixed(string) wrapping the trimmed JSON slice (full structural
            // decode of containers is on the roadmap).
            Ok(Some(PhpType::Mixed))
        }
        "json_validate" => {
            if args.is_empty() || args.len() > 3 {
                return Err(CompileError::new(
                    span,
                    "json_validate() takes 1 to 3 arguments",
                ));
            }
            let json_ty = checker.infer_type(&args[0], env)?;
            if !is_json_string_arg_type(&json_ty) {
                return Err(CompileError::new(
                    args[0].span,
                    "json_validate() json argument must be string-compatible",
                ));
            }
            for extra in &args[1..] {
                let ty = checker.infer_type(extra, env)?;
                if ty != PhpType::Int {
                    return Err(CompileError::new(
                        extra.span,
                        "json_validate() depth and flags must be integers",
                    ));
                }
            }
            if let Some(flags) = args.get(2) {
                if let Some(value) = json_static_int_value(flags) {
                    const JSON_INVALID_UTF8_IGNORE: i64 = 1_048_576;
                    if value & !JSON_INVALID_UTF8_IGNORE != 0 {
                        return Err(CompileError::new(
                            flags.span,
                            "json_validate() flags must be 0 or JSON_INVALID_UTF8_IGNORE",
                        ));
                    }
                }
            }
            Ok(Some(PhpType::Bool))
        }
        "serialize" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "serialize() takes exactly 1 argument",
                ));
            }
            checker.infer_type(&args[0], env)?;
            Ok(Some(PhpType::Str))
        }
        "unserialize" => {
            if args.is_empty() || args.len() > 2 {
                return Err(CompileError::new(
                    span,
                    "unserialize() takes 1 or 2 arguments",
                ));
            }
            let data_ty = checker.infer_type(&args[0], env)?;
            if !is_json_string_arg_type(&data_ty) {
                return Err(CompileError::new(
                    args[0].span,
                    "unserialize() data argument must be string-compatible",
                ));
            }
            if let Some(options) = args.get(1) {
                checker.infer_type(options, env)?;
            }
            Ok(Some(PhpType::Mixed))
        }
        "json_last_error" => {
            if !args.is_empty() {
                return Err(CompileError::new(
                    span,
                    "json_last_error() takes no arguments",
                ));
            }
            Ok(Some(PhpType::Int))
        }
        "json_last_error_msg" => {
            if !args.is_empty() {
                return Err(CompileError::new(
                    span,
                    "json_last_error_msg() takes no arguments",
                ));
            }
            Ok(Some(PhpType::Str))
        }
        "preg_match" => {
            if !(2..=3).contains(&args.len()) {
                return Err(CompileError::new(
                    span,
                    "preg_match() takes 2 or 3 arguments",
                ));
            }
            checker.infer_type(&args[0], env)?;
            checker.infer_type(&args[1], env)?;
            if args.len() == 3 && !matches!(args[2].kind, ExprKind::Variable(_)) {
                return Err(CompileError::new(
                    args[2].span,
                    "preg_match() parameter $matches must be passed a variable",
                ));
            }
            Ok(Some(PhpType::Int))
        }
        "preg_match_all" => {
            if args.len() != 2 {
                return Err(CompileError::new(
                    span,
                    "preg_match_all() takes exactly 2 arguments",
                ));
            }
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Int))
        }
        "preg_replace" => {
            if args.len() != 3 {
                return Err(CompileError::new(
                    span,
                    "preg_replace() takes exactly 3 arguments",
                ));
            }
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Str))
        }
        "preg_split" => {
            if !(2..=4).contains(&args.len()) {
                return Err(CompileError::new(
                    span,
                    "preg_split() takes between 2 and 4 arguments",
                ));
            }
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            let elem_ty = if args.len() >= 4 {
                PhpType::Mixed
            } else {
                PhpType::Str
            };
            Ok(Some(PhpType::Array(Box::new(elem_ty))))
        }
        _ => Ok(None),
    }
}

/// Returns `true` if `ty` is a valid type for the JSON string argument in
/// `json_decode` / `json_validate` / `json_encode` (scalar types and `Mixed`).
fn is_json_string_arg_type(ty: &PhpType) -> bool {
    match ty {
        PhpType::Str
        | PhpType::Int
        | PhpType::Float
        | PhpType::Bool
        | PhpType::Void
        | PhpType::Mixed => true,
        PhpType::Union(types) => types.iter().all(is_json_string_arg_type),
        _ => false,
    }
}

/// Returns `true` if `ty` is a valid type for the associative argument in
/// `json_decode` (bool-compatible types plus `Mixed`).
fn is_json_associative_arg_type(ty: &PhpType) -> bool {
    match ty {
        PhpType::Bool
        | PhpType::Int
        | PhpType::Float
        | PhpType::Str
        | PhpType::Void
        | PhpType::Mixed => true,
        PhpType::Union(types) => types.iter().all(is_json_associative_arg_type),
        _ => false,
    }
}

/// Attempts to evaluate an expression as a static integer at compile time.
/// Supports literals, known constants, negation, and bitwise ops.
/// Returns `Some(value)` if the expression is statically computable, `None` otherwise.
fn json_static_int_value(expr: &Expr) -> Option<i64> {
    match &expr.kind {
        ExprKind::IntLiteral(value) => Some(*value),
        ExprKind::ConstRef(name) => JSON_INT_CONSTANTS
            .iter()
            .find_map(|(constant, value)| (*constant == name.as_str()).then_some(*value)),
        ExprKind::Negate(inner) => json_static_int_value(inner).map(|value| -value),
        ExprKind::BinaryOp { left, op, right } => {
            let left = json_static_int_value(left)?;
            let right = json_static_int_value(right)?;
            match op {
                BinOp::BitAnd => Some(left & right),
                BinOp::BitOr => Some(left | right),
                BinOp::BitXor => Some(left ^ right),
                _ => None,
            }
        }
        _ => None,
    }
}

