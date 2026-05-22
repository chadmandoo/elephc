//! Purpose:
//! Type-checks SPL helper builtins implemented by the current SPL foundation.
//! Enforces conservative argument contracts that the AOT codegen can lower safely.
//!
//! Called from:
//! - `crate::types::checker::builtins::check_builtin()`
//!
//! Key details:
//! - Autoload helpers are static/AOT approximations rather than runtime code loaders.
//! - `spl_autoload_extensions()` only accepts literal setters until the runtime owns copied strings.

use crate::errors::CompileError;
use crate::parser::ast::{Expr, ExprKind};
use crate::types::{PhpType, TypeEnv};

use super::super::Checker;

type BuiltinResult = Result<Option<PhpType>, CompileError>;

pub(super) fn check_builtin(
    checker: &mut Checker,
    name: &str,
    args: &[Expr],
    span: crate::span::Span,
    env: &TypeEnv,
) -> BuiltinResult {
    match name {
        "spl_autoload_register" => {
            if args.len() > 3 {
                return Err(CompileError::new(
                    span,
                    "spl_autoload_register() takes at most 3 arguments",
                ));
            }
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Bool))
        }
        "spl_autoload_unregister" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "spl_autoload_unregister() takes exactly 1 argument",
                ));
            }
            checker.infer_type(&args[0], env)?;
            Ok(Some(PhpType::Bool))
        }
        "spl_autoload_functions" => {
            if !args.is_empty() {
                return Err(CompileError::new(
                    span,
                    "spl_autoload_functions() takes no arguments",
                ));
            }
            Ok(Some(PhpType::Array(Box::new(PhpType::Mixed))))
        }
        "spl_autoload_extensions" => {
            if args.len() > 1 {
                return Err(CompileError::new(
                    span,
                    "spl_autoload_extensions() takes at most 1 argument",
                ));
            }
            if let Some(arg) = args.first() {
                checker.infer_type(arg, env)?;
                if !matches!(
                    arg.kind,
                    ExprKind::StringLiteral(_) | ExprKind::Null
                ) {
                    return Err(CompileError::new(
                        span,
                        "spl_autoload_extensions() argument must be a string literal or null",
                    ));
                }
            }
            Ok(Some(PhpType::Str))
        }
        "spl_autoload_call" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "spl_autoload_call() takes exactly 1 argument",
                ));
            }
            checker.infer_type(&args[0], env)?;
            Ok(Some(PhpType::Void))
        }
        "spl_autoload" => {
            if args.is_empty() || args.len() > 2 {
                return Err(CompileError::new(
                    span,
                    "spl_autoload() takes 1 or 2 arguments",
                ));
            }
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Void))
        }
        "spl_object_id" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "spl_object_id() takes exactly 1 argument",
                ));
            }
            let ty = checker.infer_type(&args[0], env)?;
            if !matches!(ty, PhpType::Object(_)) {
                return Err(CompileError::new(
                    span,
                    "spl_object_id() argument must be an object",
                ));
            }
            Ok(Some(PhpType::Int))
        }
        "spl_object_hash" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "spl_object_hash() takes exactly 1 argument",
                ));
            }
            let ty = checker.infer_type(&args[0], env)?;
            if !matches!(ty, PhpType::Object(_)) {
                return Err(CompileError::new(
                    span,
                    "spl_object_hash() argument must be an object",
                ));
            }
            Ok(Some(PhpType::Str))
        }
        "spl_classes" => {
            if !args.is_empty() {
                return Err(CompileError::new(
                    span,
                    "spl_classes() takes no arguments",
                ));
            }
            Ok(Some(PhpType::Array(Box::new(PhpType::Str))))
        }
        "iterator_count" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "iterator_count() takes exactly 1 argument",
                ));
            }
            check_iterator_source(checker, &args[0], span, env, "iterator_count()")?;
            Ok(Some(PhpType::Int))
        }
        "iterator_to_array" => {
            if args.is_empty() || args.len() > 2 {
                return Err(CompileError::new(
                    span,
                    "iterator_to_array() takes 1 or 2 arguments",
                ));
            }
            let source_ty =
                check_iterator_source(checker, &args[0], span, env, "iterator_to_array()")?;
            let preserve_keys = if let Some(arg) = args.get(1) {
                static_preserve_keys(arg).ok_or_else(|| {
                    CompileError::new(
                        arg.span,
                        "iterator_to_array() preserve_keys must be a boolean literal",
                    )
                })?
            } else {
                true
            };
            if !preserve_keys && matches!(source_ty, PhpType::AssocArray { .. }) {
                return Err(CompileError::new(
                    args[1].span,
                    "iterator_to_array() with preserve_keys=false for associative arrays is not supported yet",
                ));
            }
            Ok(Some(iterator_to_array_return_type(&source_ty, preserve_keys)))
        }
        "iterator_apply" => {
            if args.len() < 2 || args.len() > 3 {
                return Err(CompileError::new(
                    span,
                    "iterator_apply() takes 2 or 3 arguments",
                ));
            }
            check_iterator_apply_source(checker, &args[0], span, env)?;
            let callback_args = if let Some(args_expr) = args.get(2) {
                iterator_apply_callback_args(args_expr, span)?
            } else {
                &[]
            };
            super::callables::check_callback_builtin_call(
                checker,
                &args[1],
                callback_args,
                span,
                env,
                "iterator_apply() callback",
            )?;
            Ok(Some(PhpType::Int))
        }
        _ => Ok(None),
    }
}

fn check_iterator_source(
    checker: &mut Checker,
    arg: &Expr,
    span: crate::span::Span,
    env: &TypeEnv,
    label: &str,
) -> Result<PhpType, CompileError> {
    let ty = checker.infer_type(arg, env)?;
    if iterator_source_supported(checker, &ty) {
        return Ok(ty);
    }
    Err(CompileError::new(
        span,
        &format!(
            "{} first argument must be a statically known array or Traversable",
            label
        ),
    ))
}

fn iterator_source_supported(checker: &Checker, ty: &PhpType) -> bool {
    match ty {
        PhpType::Array(_) | PhpType::AssocArray { .. } => true,
        PhpType::Object(name) => traversable_object_supported(checker, name),
        _ => false,
    }
}

fn check_iterator_apply_source(
    checker: &mut Checker,
    arg: &Expr,
    span: crate::span::Span,
    env: &TypeEnv,
) -> Result<PhpType, CompileError> {
    let ty = checker.infer_type(arg, env)?;
    if matches!(&ty, PhpType::Object(name) if traversable_object_supported(checker, name)) {
        return Ok(ty);
    }
    Err(CompileError::new(
        span,
        "iterator_apply() first argument must be a statically known Traversable",
    ))
}

fn traversable_object_supported(checker: &Checker, name: &str) -> bool {
    checker.object_type_implements_interface(name, "Iterator")
        || checker.object_type_implements_interface(name, "IteratorAggregate")
}

fn static_preserve_keys(expr: &Expr) -> Option<bool> {
    match &expr.kind {
        ExprKind::BoolLiteral(value) => Some(*value),
        ExprKind::IntLiteral(value) => Some(*value != 0),
        _ => None,
    }
}

fn iterator_to_array_return_type(source_ty: &PhpType, preserve_keys: bool) -> PhpType {
    match source_ty {
        PhpType::Array(elem_ty) => PhpType::Array(elem_ty.clone()),
        PhpType::AssocArray { key, value } if preserve_keys => PhpType::AssocArray {
            key: key.clone(),
            value: value.clone(),
        },
        _ if preserve_keys => PhpType::AssocArray {
            key: Box::new(PhpType::Mixed),
            value: Box::new(PhpType::Mixed),
        },
        _ => PhpType::Array(Box::new(PhpType::Mixed)),
    }
}

fn iterator_apply_callback_args(
    args_expr: &Expr,
    span: crate::span::Span,
) -> Result<&[Expr], CompileError> {
    match &args_expr.kind {
        ExprKind::Null => Ok(&[]),
        ExprKind::ArrayLiteral(elems) => {
            if elems.iter().all(is_static_callback_arg_literal) {
                Ok(elems.as_slice())
            } else {
                Err(CompileError::new(
                    span,
                    "iterator_apply() args must be null or a literal array of scalar literals",
                ))
            }
        }
        _ => Err(CompileError::new(
            span,
            "iterator_apply() args must be null or a literal array of scalar literals",
        )),
    }
}

fn is_static_callback_arg_literal(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::StringLiteral(_)
        | ExprKind::IntLiteral(_)
        | ExprKind::FloatLiteral(_)
        | ExprKind::BoolLiteral(_)
        | ExprKind::Null => true,
        ExprKind::Negate(inner) => matches!(
            inner.kind,
            ExprKind::IntLiteral(_) | ExprKind::FloatLiteral(_)
        ),
        _ => false,
    }
}
