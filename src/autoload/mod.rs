//! Purpose:
//! Resolves static Composer autoload mappings and supported SPL registration patterns.
//! Prefixes Composer `autoload.files` and inlines class files discovered by the AOT autoload registry.
//!
//! Called from:
//! - `crate::pipeline::compile()`
//!
//! Key details:
//! - Runtime autoload callbacks cannot run in native binaries; supported rules are interpreted at compile time.
//! - Composer files execute before the entry program while class-triggered files append at discovery points.

mod alias;
mod index;
mod interpret;
mod registry;
mod rule;
mod walk;

use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub use registry::Registry;

use crate::errors::CompileError;
use crate::parser::ast::Program;
use crate::parser::ast::Stmt;
use crate::span::Span;

use walk::{collect_declared_fqns, collect_referenced_fqns};

const BUILTIN_CLASS_LIKE_NAMES: &[&str] = &[
    "ArrayAccess",
    "BadFunctionCallException",
    "BadMethodCallException",
    "Countable",
    "DomainException",
    "Exception",
    "Fiber",
    "FiberError",
    "Generator",
    "InvalidArgumentException",
    "Iterator",
    "IteratorAggregate",
    "JsonException",
    "JsonSerializable",
    "LengthException",
    "LogicException",
    "OutOfBoundsException",
    "OutOfRangeException",
    "OuterIterator",
    "OverflowException",
    "RangeException",
    "RecursiveIterator",
    "ReflectionAttribute",
    "ReflectionClass",
    "ReflectionMethod",
    "ReflectionProperty",
    "RuntimeException",
    "SeekableIterator",
    "SplObserver",
    "SplSubject",
    "Stringable",
    "Throwable",
    "Traversable",
    "UnderflowException",
    "UnexpectedValueException",
    "stdClass",
];

/// Run the autoload pass over a fully resolver+name_resolver-processed
/// program. For every canonical class reference that isn't declared in
/// the program, look it up first in the composer.json PSR-4 index and
/// then in the user-registered closure rules; parse the referenced file,
/// run resolver+name_resolver on it, and append. Iterate until stable.
pub fn run(
    mut program: Program,
    base_dir: &Path,
    registry: &Registry,
) -> Result<Program, CompileError> {
    if registry.is_empty() {
        return Ok(program);
    }
    let mut included: HashSet<PathBuf> = HashSet::new();
    const MAX_ITERATIONS: usize = 64;

    // -- prefix always-included files first --
    // composer.json's `autoload.files` declares files that must always be
    // included. Prefix them in Composer order so their top-level statements
    // execute before the entry program.
    let mut prefix: Program = Vec::new();
    for path in registry.always_included_files() {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if included.insert(canonical.clone()) {
            prefix.extend(load_autoloaded_file(&canonical, base_dir)?);
        }
    }
    if !prefix.is_empty() {
        prefix.extend(program);
        program = prefix;
    }

    for _ in 0..MAX_ITERATIONS {
        let mut declared = collect_declared_fqns(&program);
        seed_builtin_declared_fqns(&mut declared);
        let referenced = collect_referenced_fqns(&program);
        let mut new_paths: Vec<PathBuf> = Vec::new();
        for fqn in &referenced {
            if declared.contains(fqn) {
                continue;
            }
            if let Some(path) = resolve_class(fqn, registry) {
                let canonical = path.canonicalize().unwrap_or(path);
                if included.insert(canonical.clone()) {
                    new_paths.push(canonical);
                }
            }
        }
        if new_paths.is_empty() {
            break;
        }
        for path in new_paths {
            program = splice_autoloaded_file(program, &path, base_dir)?;
        }
    }
    Ok(program)
}

fn seed_builtin_declared_fqns(declared: &mut HashSet<String>) {
    for name in BUILTIN_CLASS_LIKE_NAMES {
        declared.insert((*name).to_string());
    }
}

/// Try the resolution chain in order: composer.json PSR-4 first, then each
/// user-registered closure rule. Returns the first rule that produces a
/// path matching an existing file on disk.
fn resolve_class(fqn: &str, registry: &Registry) -> Option<PathBuf> {
    if let Some(path) = registry.psr4().lookup(fqn) {
        return Some(path.to_path_buf());
    }
    for rule in registry.rules() {
        if let Some(path) = interpret::resolve(rule, fqn) {
            if path.is_file() {
                return Some(path);
            }
        }
    }
    None
}

/// Parse, resolve includes, and name-resolve a single file, then append the
/// resulting statements to `program`. Used by class-triggered autoloads whose
/// top-level effects happen at the point the class is first referenced.
pub(super) fn splice_autoloaded_file(
    mut program: Program,
    path: &Path,
    base_dir: &Path,
) -> Result<Program, CompileError> {
    let canonicalized = load_autoloaded_file(path, base_dir)?;
    program.extend(canonicalized);
    Ok(program)
}

fn load_autoloaded_file(path: &Path, base_dir: &Path) -> Result<Program, CompileError> {
    let content = std::fs::read_to_string(path).map_err(|e| {
        CompileError::new(
            Span::dummy(),
            &format!("Autoload: cannot read '{}': {}", path.display(), e),
        )
    })?;
    let file_label = path.display().to_string();
    let tokens = crate::lexer::tokenize(&content).map_err(|e| e.with_file(file_label.clone()))?;
    let parsed = crate::parser::parse(&tokens).map_err(|e| e.with_file(file_label.clone()))?;
    let parsed = crate::magic_constants::substitute_file_and_scope_constants(parsed, path);
    let resolved = crate::resolver::resolve(parsed, path.parent().unwrap_or(base_dir))?;
    let canonicalized: Vec<Stmt> = crate::name_resolver::resolve(resolved)?;
    // name_resolver has already flattened namespace nodes and canonicalized
    // declarations, so we splice the statements directly into the top-level
    // program.
    Ok(canonicalized)
}
