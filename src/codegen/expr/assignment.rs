use super::super::context::Context;
use super::super::data_section::DataSection;
use super::super::emit::Emitter;
use crate::parser::ast::{Expr, ExprKind};
use crate::types::PhpType;

pub(super) fn emit_assignment_expr(
    target: &Expr,
    value: &Expr,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> PhpType {
    let ExprKind::Variable(name) = &target.kind else {
        emitter.comment("WARNING: assignment expression target is not supported in codegen");
        return PhpType::Int;
    };

    super::super::stmt::emit_assign_stmt(name, value, emitter, ctx, data);
    super::variables::emit_variable(name, emitter, ctx)
}
