use super::super::context::Context;
use super::super::data_section::DataSection;
use super::super::emit::Emitter;
use crate::parser::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::types::PhpType;

pub(super) fn emit_assignment_expr(
    target: &Expr,
    value: &Expr,
    result_target: Option<&Expr>,
    prelude: &[Stmt],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> PhpType {
    emit_assignment_prelude(prelude, emitter, ctx, data);

    let ExprKind::Variable(name) = &target.kind else {
        return emit_non_local_assignment_expr(target, value, result_target, emitter, ctx, data);
    };

    super::super::stmt::emit_assign_stmt(name, value, emitter, ctx, data);
    match result_target {
        Some(target) => super::emit_expr(target, emitter, ctx, data),
        None => super::variables::emit_variable(name, emitter, ctx),
    }
}

pub(super) fn emit_non_local_assignment_expr(
    target: &Expr,
    value: &Expr,
    result_target: Option<&Expr>,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> PhpType {
    match &target.kind {
        ExprKind::ArrayAccess { array, index } => match &array.kind {
            ExprKind::Variable(array) => {
                super::super::stmt::emit_array_assign_stmt(array, index, value, emitter, ctx, data);
            }
            ExprKind::PropertyAccess { object, property } => {
                super::super::stmt::emit_property_array_assign_stmt(
                    object, property, index, value, emitter, ctx, data,
                );
            }
            ExprKind::StaticPropertyAccess { receiver, property } => {
                super::super::stmt::emit_static_property_array_assign_stmt(
                    receiver, property, index, value, emitter, ctx, data,
                );
            }
            _ => {
                emitter.comment("WARNING: assignment expression target is not supported in codegen");
                return PhpType::Int;
            }
        },
        ExprKind::PropertyAccess { object, property } => {
            super::super::stmt::emit_property_assign_stmt(
                object, property, value, emitter, ctx, data,
            );
        }
        ExprKind::StaticPropertyAccess { receiver, property } => {
            super::super::stmt::emit_static_property_assign_stmt(
                receiver, property, value, emitter, ctx, data,
            );
        }
        _ => {
            emitter.comment("WARNING: assignment expression target is not supported in codegen");
            return PhpType::Int;
        }
    }

    super::emit_expr(result_target.unwrap_or(target), emitter, ctx, data)
}

fn emit_assignment_prelude(
    prelude: &[Stmt],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) {
    for stmt in prelude {
        match &stmt.kind {
            StmtKind::Assign { name, value } => {
                super::super::stmt::emit_assign_stmt(name, value, emitter, ctx, data);
            }
            StmtKind::Synthetic(stmts) => {
                emit_assignment_prelude(stmts, emitter, ctx, data);
            }
            _ => {
                super::super::stmt::emit_stmt(stmt, emitter, ctx, data);
            }
        }
    }
}
