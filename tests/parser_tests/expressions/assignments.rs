use super::*;

#[test]
fn test_compound_assignment_missing_ops_parse() {
    let cases = [
        ("<?php $x **= 3;", BinOp::Pow),
        ("<?php $x &= 3;", BinOp::BitAnd),
        ("<?php $x |= 3;", BinOp::BitOr),
        ("<?php $x ^= 3;", BinOp::BitXor),
        ("<?php $x <<= 3;", BinOp::ShiftLeft),
        ("<?php $x >>= 3;", BinOp::ShiftRight),
    ];

    for (src, expected_op) in cases {
        let stmts = parse_source(src);
        match &stmts[0].kind {
            StmtKind::Assign { name, value } => {
                assert_eq!(name, "x");
                match &value.kind {
                    ExprKind::BinaryOp { left, op, right } => {
                        assert_eq!(left.kind, ExprKind::Variable("x".into()));
                        assert_eq!(op, &expected_op);
                        assert_eq!(right.kind, ExprKind::IntLiteral(3));
                    }
                    other => panic!("expected BinaryOp, got {:?}", other),
                }
            }
            other => panic!("expected Assign, got {:?}", other),
        }
    }
}

#[test]
fn test_parse_nullable_typed_assign() {
    let stmts = parse_source("<?php ?int $value = null;");
    match &stmts[0].kind {
        StmtKind::TypedAssign {
            type_expr,
            name,
            value,
        } => {
            assert_eq!(name, "value");
            assert_eq!(type_expr, &TypeExpr::Nullable(Box::new(TypeExpr::Int)));
            assert_eq!(value.kind, ExprKind::Null);
        }
        other => panic!("Expected typed assign, got {:?}", other),
    }
}

#[test]
fn test_parse_union_typed_assign() {
    let stmts = parse_source("<?php int|string $value = 1;");
    match &stmts[0].kind {
        StmtKind::TypedAssign {
            type_expr,
            name,
            value,
        } => {
            assert_eq!(name, "value");
            assert_eq!(type_expr, &TypeExpr::Union(vec![TypeExpr::Int, TypeExpr::Str]));
            assert_eq!(value.kind, ExprKind::IntLiteral(1));
        }
        other => panic!("Expected typed assign, got {:?}", other),
    }
}
