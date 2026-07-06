//! Purpose:
//! Coordinates PHP magic-constant lowering for file paths, lexical scopes, and trait binding.
//! Owns the public pass entrypoints that turn magic constants into plain literals before later compiler passes.
//!
//! Called from:
//! - `crate::pipeline::compile()` and `crate::resolver` when preparing main and included files.
//!
//! Key details:
//! - `__LINE__` is lowered by the parser, while file/scope constants must be resolved before type checking and codegen.
//!
//! Public passes:
//! - [`substitute_file_constants`] resolves `__FILE__` and `__DIR__` against
//!   the canonical path of the file the AST nodes came from. Run once per
//!   source file before inlining (resolver) and once for the main file.
//! - [`substitute_scope_constants_in_file`] resolves the scope-dependent
//!   constants (`__FUNCTION__`, `__CLASS__`, `__METHOD__`, `__NAMESPACE__`,
//!   `__TRAIT__`) based on lexical position inside a single source file.
//! - [`substitute_file_and_scope_constants`] applies both passes for a single
//!   source file before that file is inlined into another file.
//! - [`bind_trait_class_constants`] rebinds trait-origin `__CLASS__` literals
//!   when trait members are flattened into a concrete class. `__METHOD__` and
//!   `__TRAIT__` keep the trait identity, matching PHP.

mod file_pass;
mod scope_pass;
mod trait_binding;
mod walker;

use std::path::Path;

use crate::names::Name;
use crate::parser::ast::{ClassMethod, ClassProperty, Program, Stmt};

const TRAIT_CLASS_PLACEHOLDER: &str = "\x1F__ELEPHC_TRAIT_CLASS__\x1F";

/// Replaces `MagicConstant::File` and `MagicConstant::Dir` with string
/// literals derived from `file_path`. Other magic constants are left untouched
/// for the scope pass to resolve later.
pub fn substitute_file_constants(stmts: Vec<Stmt>, file_path: &Path) -> Vec<Stmt> {
    file_pass::substitute_file_constants(stmts, file_path)
}

/// Applies file-local and lexical-scope magic-constant lowering for one PHP
/// source file. Resolver calls this before inlining included files so lexical
/// scopes from one file cannot leak into another.
pub fn substitute_file_and_scope_constants(stmts: Vec<Stmt>, file_path: &Path) -> Vec<Stmt> {
    let stmts = substitute_file_constants(stmts, file_path);
    let stmts = stamp_class_declaring_files(stmts, file_path);
    substitute_scope_constants_in_file(stmts, file_path)
}

/// The synthetic class constant that records each class's declaring file.
///
/// `ReflectionClass::getFileName()` needs a runtime class-name→file lookup, but
/// file provenance only exists here — before includes are inlined into one
/// statement stream. Riding the path inside the declaration node itself (as a
/// class constant) carries it through the resolver, name resolution, and
/// flattening with no extra plumbing; `emit_classes_by_name_table` reads it
/// back out of `ClassInfo.constants` when emitting the runtime lookup table.
pub const CLASS_FILE_CONSTANT: &str = "__ELEPHC_FILE__";

/// Appends the `__ELEPHC_FILE__` constant (the file's canonical path) to every
/// class and enum declared in this file, including inside namespace blocks.
/// Runs once per source file, before inlining, alongside `__FILE__` lowering.
fn stamp_class_declaring_files(stmts: Vec<Stmt>, file_path: &Path) -> Vec<Stmt> {
    let canonical = file_path
        .canonicalize()
        .unwrap_or_else(|_| file_path.to_path_buf())
        .display()
        .to_string();
    stmts
        .into_iter()
        .map(|stmt| stamp_stmt_declaring_file(stmt, &canonical))
        .collect()
}

/// Stamps one statement (recursing into namespace/synthetic blocks).
fn stamp_stmt_declaring_file(mut stmt: Stmt, canonical: &str) -> Stmt {
    use crate::parser::ast::{ClassConst, ExprKind, StmtKind, Visibility};
    match &mut stmt.kind {
        StmtKind::ClassDecl { constants, .. } | StmtKind::EnumDecl { constants, .. } => {
            if !constants
                .iter()
                .any(|constant| constant.name == CLASS_FILE_CONSTANT)
            {
                constants.push(ClassConst {
                    name: CLASS_FILE_CONSTANT.to_string(),
                    visibility: Visibility::Public,
                    is_final: false,
                    value: crate::parser::ast::Expr::new(
                        ExprKind::StringLiteral(canonical.to_string()),
                        crate::span::Span::dummy(),
                    ),
                    span: crate::span::Span::dummy(),
                    attributes: Vec::new(),
                });
            }
        }
        StmtKind::NamespaceBlock { body, .. } | StmtKind::Synthetic(body) => {
            let stamped = std::mem::take(body)
                .into_iter()
                .map(|inner| stamp_stmt_declaring_file(inner, canonical))
                .collect();
            *body = stamped;
        }
        _ => {}
    }
    stmt
}

/// Resolves scope-dependent magic constants (`__FUNCTION__`, `__CLASS__`, `__METHOD__`,
/// `__NAMESPACE__`, `__TRAIT__`) based on lexical position within `program`.
/// `__FILE__` and `__DIR__` must already be lowered by `substitute_file_constants`.
pub fn substitute_scope_constants_in_file(program: Program, file_path: &Path) -> Program {
    scope_pass::substitute_scope_constants_in_file(program, file_path)
}

/// Rebinds `__CLASS__` placeholders to `class_name` in `properties` and `methods`
/// after a trait is flattened into a concrete class. `__METHOD__` and `__TRAIT__`
/// retain the trait's identity, matching PHP semantics.
pub fn bind_trait_class_constants(
    properties: Vec<ClassProperty>,
    methods: Vec<ClassMethod>,
    class_name: &str,
) -> (Vec<ClassProperty>, Vec<ClassMethod>) {
    trait_binding::bind_trait_class_constants(properties, methods, class_name)
}

/// Converts `name` to its canonical string representation, or returns an empty
/// string if `name` is `None`. Used to produce the `__NAMESPACE__` literal.
fn namespace_string(name: &Option<Name>) -> String {
    name.as_ref().map(Name::as_canonical).unwrap_or_default()
}

/// Constructs a fully-qualified name by prepending `namespace` if present and non-empty,
/// otherwise returns `name` unchanged. Used to build `__CLASS__` and `__TRAIT__` literals.
fn qualify(namespace: Option<&str>, name: &str) -> String {
    match namespace {
        Some(ns) if !ns.is_empty() => format!("{}\\{}", ns, name),
        _ => name.to_string(),
    }
}
