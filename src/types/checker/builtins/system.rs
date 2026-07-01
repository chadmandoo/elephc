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
use crate::parser::ast::{Expr, ExprKind};
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
