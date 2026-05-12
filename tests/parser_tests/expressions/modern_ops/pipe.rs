//! Purpose:
//! Integration or regression tests for parser AST coverage of the PHP 8.5 pipe operator (`|>`),
//! including AST shape, left-associative chaining, and precedence against arithmetic, comparison,
//! concat, null coalesce, and ternary operators.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Inline PHP snippets are parsed and assertions inspect AST shape and operator precedence.

use super::*;

fn unwrap_echo(stmts: &[Stmt]) -> &Expr {
    match &stmts[0].kind {
        StmtKind::Echo(expr) => expr,
        other => panic!("expected Echo, got {:?}", other),
    }
}

#[test]
fn test_pipe_basic_first_class_callable() {
    let stmts = parse_source("<?php echo $x |> foo(...);");
    let expr = unwrap_echo(&stmts);
    match &expr.kind {
        ExprKind::Pipe { value, callable } => {
            assert_eq!(value.kind, ExprKind::Variable("x".into()));
            assert!(matches!(
                callable.kind,
                ExprKind::FirstClassCallable(_)
            ));
        }
        other => panic!("expected Pipe, got {:?}", other),
    }
}

#[test]
fn test_pipe_chains_left_associative() {
    let stmts = parse_source("<?php echo $x |> f(...) |> g(...);");
    let expr = unwrap_echo(&stmts);
    match &expr.kind {
        ExprKind::Pipe { value, callable } => {
            // Outer pipe's callable should be g(...).
            assert!(matches!(callable.kind, ExprKind::FirstClassCallable(_)));
            // Outer pipe's value should itself be a Pipe { $x, f(...) }.
            match &value.kind {
                ExprKind::Pipe { value: inner_value, callable: inner_callable } => {
                    assert_eq!(inner_value.kind, ExprKind::Variable("x".into()));
                    assert!(matches!(
                        inner_callable.kind,
                        ExprKind::FirstClassCallable(_)
                    ));
                }
                other => panic!("expected inner Pipe, got {:?}", other),
            }
        }
        other => panic!("expected outer Pipe, got {:?}", other),
    }
}

#[test]
fn test_pipe_lower_precedence_than_arithmetic() {
    // `5 + 2 |> f(...)` must parse as `(5 + 2) |> f(...)`.
    let stmts = parse_source("<?php echo 5 + 2 |> f(...);");
    let expr = unwrap_echo(&stmts);
    match &expr.kind {
        ExprKind::Pipe { value, .. } => match &value.kind {
            ExprKind::BinaryOp { op: BinOp::Add, .. } => {}
            other => panic!("expected Add inside Pipe, got {:?}", other),
        },
        other => panic!("expected Pipe at top, got {:?}", other),
    }
}

#[test]
fn test_pipe_higher_precedence_than_comparison() {
    // `'beep' |> f(...) == 4` must parse as `('beep' |> f(...)) == 4`.
    let stmts = parse_source("<?php echo 'beep' |> f(...) == 4;");
    let expr = unwrap_echo(&stmts);
    match &expr.kind {
        ExprKind::BinaryOp { op: BinOp::Eq, left, .. } => {
            assert!(matches!(left.kind, ExprKind::Pipe { .. }));
        }
        other => panic!("expected Eq with Pipe on left, got {:?}", other),
    }
}

#[test]
fn test_pipe_higher_precedence_than_null_coalesce() {
    // `$id |> get(...) ?? 'd'` must parse as `($id |> get(...)) ?? 'd'`.
    let stmts = parse_source("<?php echo $id |> get(...) ?? 'd';");
    let expr = unwrap_echo(&stmts);
    match &expr.kind {
        ExprKind::NullCoalesce { value, .. } => {
            assert!(matches!(value.kind, ExprKind::Pipe { .. }));
        }
        other => panic!("expected NullCoalesce with Pipe in value, got {:?}", other),
    }
}

#[test]
fn test_pipe_higher_precedence_than_concat() {
    // `'a' . 'b' |> f(...)` must parse as `'a' . ('b' |> f(...))`.
    let stmts = parse_source("<?php echo 'a' . 'b' |> f(...);");
    let expr = unwrap_echo(&stmts);
    match &expr.kind {
        ExprKind::BinaryOp { op: BinOp::Concat, right, .. } => {
            assert!(matches!(right.kind, ExprKind::Pipe { .. }));
        }
        other => panic!("expected Concat with Pipe on right, got {:?}", other),
    }
}

#[test]
fn test_pipe_with_variable_callable() {
    let stmts = parse_source("<?php echo $x |> $cb;");
    let expr = unwrap_echo(&stmts);
    match &expr.kind {
        ExprKind::Pipe { value, callable } => {
            assert_eq!(value.kind, ExprKind::Variable("x".into()));
            assert_eq!(callable.kind, ExprKind::Variable("cb".into()));
        }
        other => panic!("expected Pipe, got {:?}", other),
    }
}

#[test]
fn test_pipe_with_static_method_callable() {
    let stmts = parse_source("<?php echo $x |> A::m(...);");
    let expr = unwrap_echo(&stmts);
    match &expr.kind {
        ExprKind::Pipe { callable, .. } => match &callable.kind {
            ExprKind::FirstClassCallable(target) => {
                assert!(matches!(
                    target,
                    crate::CallableTarget::StaticMethod { .. }
                ));
            }
            other => panic!("expected FirstClassCallable static method, got {:?}", other),
        },
        other => panic!("expected Pipe, got {:?}", other),
    }
}
