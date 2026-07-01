//! Purpose:
//! Type-checks the internal `__elephc_phar_*` PHAR intrinsics: arity, argument
//! inference, PHAR bridge library linkage, and return types.
//!
//! Called from:
//! - `crate::types::checker::builtins::io::check_builtin()`
//!
//! Key details:
//! - The regular filesystem builtins (file_get_contents, file_put_contents, copy,
//!   chmod, touch, …) now live in the `builtin!` registry under `src/builtins/io/`.
//!   Only the `__elephc_phar_*` internal intrinsics remain here (io-C2 migrates them).
//! - Return types and diagnostics must stay aligned with `crate::types::signatures`
//!   and builtin codegen emitters.

use crate::errors::CompileError;
use crate::parser::ast::Expr;
use crate::types::{PhpType, TypeEnv};

use super::common::BuiltinResult;
use super::super::super::Checker;

/// Type-checks an internal `__elephc_phar_*` intrinsic call by name, validating
/// argument count, inferring argument types, linking the PHAR bridge library, and
/// returning the intrinsic's return type.
///
/// Returns `Ok(Some(PhpType))` with the return type on recognized intrinsics,
/// `Ok(None)` when `name` is not a PHAR intrinsic (caller should try the next
/// builtin category), or `Err(CompileError)` on arity mismatch.
pub(super) fn check_builtin(
    checker: &mut Checker,
    name: &str,
    args: &[Expr],
    span: crate::span::Span,
    env: &TypeEnv,
) -> BuiltinResult {
    match name {
        "__elephc_phar_set_compression" => {
            if args.len() != 2 {
                return Err(CompileError::new(
                    span,
                    "__elephc_phar_set_compression() takes exactly 2 arguments",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Bool))
        }
        "__elephc_phar_list_entries" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "__elephc_phar_list_entries() takes exactly 1 argument",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Array(Box::new(PhpType::Str))))
        }
        "__elephc_phar_get_metadata" | "__elephc_phar_get_stub" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "__elephc_phar_get_metadata()/__elephc_phar_get_stub() take exactly 1 argument",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Str))
        }
        "__elephc_phar_set_metadata" | "__elephc_phar_set_stub" => {
            if args.len() != 2 {
                return Err(CompileError::new(
                    span,
                    "__elephc_phar_set_metadata()/__elephc_phar_set_stub() take exactly 2 arguments",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Bool))
        }
        "__elephc_phar_get_file_metadata" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "__elephc_phar_get_file_metadata() takes exactly 1 argument",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Str))
        }
        "__elephc_phar_set_file_metadata" => {
            if args.len() != 2 {
                return Err(CompileError::new(
                    span,
                    "__elephc_phar_set_file_metadata() takes exactly 2 arguments",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Bool))
        }
        "__elephc_phar_gzip_archive"
        | "__elephc_phar_bzip2_archive"
        | "__elephc_phar_decompress_archive"
        | "__elephc_phar_get_signature_hash"
        | "__elephc_phar_get_signature_type" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "phar archive (de)compression/signature-read intrinsics take exactly 1 argument",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Str))
        }
        "__elephc_phar_sign_openssl" | "__elephc_phar_sign_hash" => {
            if args.len() != 2 {
                return Err(CompileError::new(
                    span,
                    "phar signing intrinsics take exactly 2 arguments",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            for arg in args {
                checker.infer_type(arg, env)?;
            }
            Ok(Some(PhpType::Bool))
        }
        "__elephc_phar_set_zip_password" => {
            if args.len() != 1 {
                return Err(CompileError::new(
                    span,
                    "__elephc_phar_set_zip_password takes exactly 1 argument",
                ));
            }
            checker.require_builtin_library("elephc_phar");
            checker.infer_type(&args[0], env)?;
            Ok(Some(PhpType::Bool))
        }
        _ => Ok(None),
    }
}
