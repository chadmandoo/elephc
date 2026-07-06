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
//!   through the caller-supplied lookup (loop-entry types, e.g. a `foreach` value
//!   variable's real element type) joined with self-evident literal assignments found
//!   inside the loop body (`$x = 1; $a[] = $x;`).
//! - A pushed value with no usable evidence — a variable defined only inside the loop
//!   from non-literal sources — contributes nothing to the join. Treating it as `mixed`
//!   would spuriously widen same-typed rebuild loops (e.g. the synthesized
//!   `MultipleIterator::detachIterator` body, whose `array<Iterator>` rebuild must stay
//!   assignable to its typed property); genuinely-widening pushes of such values are
//!   still promoted per-site by the existing lowering.

use crate::parser::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::types::checker::infer_expr_type_syntactic;
use crate::types::PhpType;

/// Returns the names of locals that currently hold a non-`mixed` indexed array and whose
/// element type joins to `mixed` across the `$name[] = value` pushes found in the loop
/// body (and the optional `for` update statement). `lookup` supplies the current type of
/// a local at loop entry; names it does not know are skipped as push targets. A pushed
/// value with no usable type evidence — a variable defined only inside the loop from a
/// non-literal source, such as `$candidate = $this->iterators[$i]` — contributes nothing
/// to the join instead of forcing `mixed`, so same-typed rebuild loops (common in
/// compiler-synthesized SPL method bodies) are not spuriously widened.
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
    let mut literal_assignments: Vec<(&str, Option<PhpType>)> = Vec::new();
    collect_literal_assignments(body, &mut literal_assignments);
    if let Some(stmt) = update {
        collect_literal_assignment_stmt(stmt, &mut literal_assignments);
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
                match resolve_pushed_value_type(value, lookup, &literal_assignments) {
                    Some(pushed) => join_pushed_element_type(acc, pushed),
                    None => acc,
                }
            });
        if joined == PhpType::Mixed {
            names.push(name.to_string());
        }
    }
    names
}

/// Resolves the static type a pushed value contributes to the element join, or `None`
/// when there is no usable evidence. A pushed variable joins its loop-entry type with any
/// self-evident literal assignments to it inside the loop body (`$x = 1; $a[] = $x;`);
/// a variable that is only ever assigned from non-literal sources inside the loop yields
/// no evidence — the loop-entry lookup is authoritative and unknown stays unknown, never
/// `mixed`. Non-variable values use the shared syntactic inference as before.
fn resolve_pushed_value_type(
    value: &Expr,
    lookup: &dyn Fn(&str) -> Option<PhpType>,
    literal_assignments: &[(&str, Option<PhpType>)],
) -> Option<PhpType> {
    match &value.kind {
        ExprKind::Variable(name) => {
            let entry = lookup(name);
            let body = joined_literal_assignment_type(name, literal_assignments);
            match (entry, body) {
                (Some(a), Some(b)) => Some(join_pushed_element_type(a, b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            }
        }
        _ => Some(infer_expr_type_syntactic(value)),
    }
}

/// Joins the recorded in-loop literal assignments for `name`. Returns `None` when the
/// variable has no literal assignment or any of its assignments is non-literal (a poison
/// entry) — partial literal evidence must not overrule the unknown non-literal sources.
fn joined_literal_assignment_type(
    name: &str,
    literal_assignments: &[(&str, Option<PhpType>)],
) -> Option<PhpType> {
    let mut joined: Option<PhpType> = None;
    for (candidate, ty) in literal_assignments {
        if *candidate != name {
            continue;
        }
        let Some(ty) = ty else {
            return None;
        };
        joined = Some(match joined {
            Some(acc) => join_pushed_element_type(acc, ty.clone()),
            None => ty.clone(),
        });
    }
    joined
}

/// Returns the self-evident scalar type of a literal expression, or `None` for anything
/// that needs real inference (the shared syntactic helper defaults unknown constructs to
/// `Int`, which is not usable as widening evidence).
fn literal_expr_type(value: &Expr) -> Option<PhpType> {
    match &value.kind {
        ExprKind::IntLiteral(_) => Some(PhpType::Int),
        ExprKind::FloatLiteral(_) => Some(PhpType::Float),
        ExprKind::StringLiteral(_) => Some(PhpType::Str),
        ExprKind::BoolLiteral(_) => Some(PhpType::Bool),
        _ => None,
    }
}

/// Collects `$name = <literal>` assignments from every statement in `stmts`, recursively,
/// recording a poison entry (`None` type) for non-literal assignments so partial evidence
/// never overrules an unknown source.
fn collect_literal_assignments<'a>(stmts: &'a [Stmt], out: &mut Vec<(&'a str, Option<PhpType>)>) {
    for stmt in stmts {
        collect_literal_assignment_stmt(stmt, out);
    }
}

/// Collects literal assignments from one statement, mirroring the nested-statement
/// recursion of `collect_array_push_stmt` (only bodies that execute within the loop
/// iteration are descended into).
fn collect_literal_assignment_stmt<'a>(stmt: &'a Stmt, out: &mut Vec<(&'a str, Option<PhpType>)>) {
    match &stmt.kind {
        StmtKind::Assign { name, value } | StmtKind::TypedAssign { name, value, .. } => {
            out.push((name.as_str(), literal_expr_type(value)));
        }
        StmtKind::If {
            then_body,
            elseif_clauses,
            else_body,
            ..
        } => {
            collect_literal_assignments(then_body, out);
            for (_, clause_body) in elseif_clauses {
                collect_literal_assignments(clause_body, out);
            }
            if let Some(else_body) = else_body {
                collect_literal_assignments(else_body, out);
            }
        }
        StmtKind::IfDef {
            then_body,
            else_body,
            ..
        } => {
            collect_literal_assignments(then_body, out);
            if let Some(else_body) = else_body {
                collect_literal_assignments(else_body, out);
            }
        }
        StmtKind::While { body, .. }
        | StmtKind::DoWhile { body, .. }
        | StmtKind::IncludeOnceGuard { body, .. } => collect_literal_assignments(body, out),
        StmtKind::Foreach {
            key_var,
            value_var,
            body,
            ..
        } => {
            // The foreach bindings assign non-literal values each iteration: poison them.
            if let Some(key_var) = key_var {
                out.push((key_var.as_str(), None));
            }
            out.push((value_var.as_str(), None));
            collect_literal_assignments(body, out);
        }
        StmtKind::For {
            init,
            update,
            body,
            ..
        } => {
            if let Some(init) = init {
                collect_literal_assignment_stmt(init, out);
            }
            if let Some(update) = update {
                collect_literal_assignment_stmt(update, out);
            }
            collect_literal_assignments(body, out);
        }
        StmtKind::Switch { cases, default, .. } => {
            for (_, case_body) in cases {
                collect_literal_assignments(case_body, out);
            }
            if let Some(default) = default {
                collect_literal_assignments(default, out);
            }
        }
        StmtKind::Try {
            try_body,
            catches,
            finally_body,
        } => {
            collect_literal_assignments(try_body, out);
            for catch in catches {
                collect_literal_assignments(&catch.body, out);
            }
            if let Some(finally_body) = finally_body {
                collect_literal_assignments(finally_body, out);
            }
        }
        StmtKind::Synthetic(stmts) => collect_literal_assignments(stmts, out),
        _ => {}
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
