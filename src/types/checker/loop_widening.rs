//! Purpose:
//! Pre-scans a loop body for `$array[] = value` pushes whose element types widen a
//! local indexed array to `mixed` across the loop back-edge (issue #452), so both the
//! type checker and EIR lowering can fix the array's element type to `mixed` *before*
//! processing the body once.
//!
//! Called from:
//! - `crate::types::checker::stmt_check::control_flow` (loop arms widen the `TypeEnv`).
//! - `crate::ir_lower::stmt` (loop lowering widens `local_types` and materializes the
//!   promotion before emitting the body).
//!
//! Key details:
//! - Both passes are single-pass over loop bodies; without this scan an early push site
//!   is typed/lowered against the pre-promotion element type and writes an unboxed
//!   scalar into mixed-element storage on iterations >= 2, corrupting the heap.
//! - Only the widening-to-`mixed` transition is reported: it is the one that changes the
//!   element representation (raw scalar slots vs boxed cells). Same-type pushes and
//!   `never -> T` growth keep their current lowering.
//! - Value types resolve syntactically (`infer_expr_type_syntactic`); variables resolve
//!   through the caller-supplied lookup so loop-bound names (e.g. a `foreach` value
//!   variable) can be given their real type. Unknown values conservatively join as
//!   `mixed`, which matches what a single push site of that value would do itself.

use crate::parser::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::types::checker::infer_expr_type_syntactic;
use crate::types::PhpType;

/// Returns the names of locals that currently hold a non-`mixed` indexed array and whose
/// element type joins to `mixed` across every `$name[] = value` push found in the loop
/// body (and the optional `for` update statement). `lookup` supplies the current type of
/// a local at loop entry; names it does not know are skipped as push targets and resolve
/// to `mixed` as pushed values.
pub fn loop_grown_mixed_array_pushes(
    body: &[Stmt],
    update: Option<&Stmt>,
    lookup: &dyn Fn(&str) -> Option<PhpType>,
) -> Vec<String> {
    let mut pushes: Vec<(&str, &Expr)> = Vec::new();
    collect_array_pushes(body, &mut pushes);
    if let Some(stmt) = update {
        collect_array_push_stmt(stmt, &mut pushes);
    }
    let mut names: Vec<String> = Vec::new();
    for (name, _) in &pushes {
        if names.iter().any(|n| n == name) {
            continue;
        }
        let Some(PhpType::Array(elem)) = lookup(name) else {
            continue;
        };
        if *elem == PhpType::Mixed {
            continue;
        }
        let joined = pushes
            .iter()
            .filter(|(n, _)| n == name)
            .fold(*elem, |acc, (_, value)| {
                join_pushed_element_type(acc, resolve_pushed_value_type(value, lookup))
            });
        if joined == PhpType::Mixed {
            names.push(name.to_string());
        }
    }
    names
}

/// Resolves the static type a pushed value contributes to the element join: variables go
/// through the caller's lookup (defaulting to `mixed` when unknown), everything else uses
/// the shared syntactic inference.
fn resolve_pushed_value_type(value: &Expr, lookup: &dyn Fn(&str) -> Option<PhpType>) -> PhpType {
    match &value.kind {
        ExprKind::Variable(name) => lookup(name).unwrap_or(PhpType::Mixed),
        _ => infer_expr_type_syntactic(value),
    }
}

/// Joins two indexed-array element types on the widening lattice used by push sites:
/// equal types stay, `never` adopts the other side, and any other combination widens to
/// `mixed` (the representation-changing transition this scan exists to detect).
fn join_pushed_element_type(a: PhpType, b: PhpType) -> PhpType {
    if a == b {
        a
    } else if a == PhpType::Never {
        b
    } else if b == PhpType::Never {
        a
    } else {
        PhpType::Mixed
    }
}

/// Collects `$name[] = value` pushes from every statement in `stmts`, recursively.
fn collect_array_pushes<'a>(stmts: &'a [Stmt], out: &mut Vec<(&'a str, &'a Expr)>) {
    for stmt in stmts {
        collect_array_push_stmt(stmt, out);
    }
}

/// Collects pushes from one statement, recursing into every nested statement body that
/// executes as part of the enclosing loop iteration. Declaration bodies (functions,
/// classes) do not execute in the loop and closures capture by value by default, so
/// neither is descended into.
fn collect_array_push_stmt<'a>(stmt: &'a Stmt, out: &mut Vec<(&'a str, &'a Expr)>) {
    match &stmt.kind {
        StmtKind::ArrayPush { array, value } => out.push((array.as_str(), value)),
        StmtKind::If {
            then_body,
            elseif_clauses,
            else_body,
            ..
        } => {
            collect_array_pushes(then_body, out);
            for (_, clause_body) in elseif_clauses {
                collect_array_pushes(clause_body, out);
            }
            if let Some(else_body) = else_body {
                collect_array_pushes(else_body, out);
            }
        }
        StmtKind::IfDef {
            then_body,
            else_body,
            ..
        } => {
            collect_array_pushes(then_body, out);
            if let Some(else_body) = else_body {
                collect_array_pushes(else_body, out);
            }
        }
        StmtKind::While { body, .. }
        | StmtKind::DoWhile { body, .. }
        | StmtKind::Foreach { body, .. }
        | StmtKind::IncludeOnceGuard { body, .. } => collect_array_pushes(body, out),
        StmtKind::For {
            init,
            update,
            body,
            ..
        } => {
            if let Some(init) = init {
                collect_array_push_stmt(init, out);
            }
            if let Some(update) = update {
                collect_array_push_stmt(update, out);
            }
            collect_array_pushes(body, out);
        }
        StmtKind::Switch { cases, default, .. } => {
            for (_, case_body) in cases {
                collect_array_pushes(case_body, out);
            }
            if let Some(default) = default {
                collect_array_pushes(default, out);
            }
        }
        StmtKind::Try {
            try_body,
            catches,
            finally_body,
        } => {
            collect_array_pushes(try_body, out);
            for catch in catches {
                collect_array_pushes(&catch.body, out);
            }
            if let Some(finally_body) = finally_body {
                collect_array_pushes(finally_body, out);
            }
        }
        StmtKind::Synthetic(stmts) => collect_array_pushes(stmts, out),
        _ => {}
    }
}
