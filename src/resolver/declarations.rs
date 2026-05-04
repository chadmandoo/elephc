use crate::parser::ast::{Stmt, StmtKind};

pub(super) fn extract_discoverable_declarations(stmts: &[Stmt]) -> Vec<Stmt> {
    let mut declarations = Vec::new();
    let mut context = Vec::new();
    let mut context_flushed = false;

    for stmt in stmts {
        match &stmt.kind {
            StmtKind::NamespaceDecl { .. } => {
                context.clear();
                context.push(stmt.clone());
                context_flushed = false;
            }
            StmtKind::UseDecl { .. } => {
                context.push(stmt.clone());
                context_flushed = false;
            }
            StmtKind::NamespaceBlock { name, body } => {
                let body_declarations = extract_discoverable_declarations(body);
                if !body_declarations.is_empty() {
                    declarations.push(Stmt::new(
                        StmtKind::NamespaceBlock {
                            name: name.clone(),
                            body: body_declarations,
                        },
                        stmt.span,
                    ));
                }
            }
            StmtKind::Synthetic(body) => {
                let body_declarations = extract_discoverable_declarations(body);
                if !body_declarations.is_empty() {
                    if !context_flushed {
                        declarations.extend(context.clone());
                        context_flushed = true;
                    }
                    declarations.push(Stmt::new(StmtKind::Synthetic(body_declarations), stmt.span));
                }
            }
            kind if is_discoverable_declaration(kind) => {
                if !context_flushed {
                    declarations.extend(context.clone());
                    context_flushed = true;
                }
                declarations.push(stmt.clone());
            }
            _ => {}
        }
    }

    declarations
}

pub(super) fn strip_discoverable_declarations(stmts: Vec<Stmt>) -> Vec<Stmt> {
    stmts.into_iter().filter_map(strip_stmt).collect()
}

fn strip_stmt(stmt: Stmt) -> Option<Stmt> {
    let span = stmt.span;
    match stmt.kind {
        kind if is_discoverable_declaration(&kind) => None,
        StmtKind::NamespaceBlock { name, body } => Some(Stmt::new(
            StmtKind::NamespaceBlock {
                name,
                body: strip_discoverable_declarations(body),
            },
            span,
        )),
        StmtKind::Synthetic(body) => {
            let body = strip_discoverable_declarations(body);
            if body.is_empty() {
                None
            } else {
                Some(Stmt::new(StmtKind::Synthetic(body), span))
            }
        }
        other => Some(Stmt::new(other, span)),
    }
}

fn is_discoverable_declaration(kind: &StmtKind) -> bool {
    matches!(
        kind,
        StmtKind::FunctionDecl { .. }
            | StmtKind::ClassDecl { .. }
            | StmtKind::EnumDecl { .. }
            | StmtKind::InterfaceDecl { .. }
            | StmtKind::TraitDecl { .. }
            | StmtKind::PackedClassDecl { .. }
            | StmtKind::ExternFunctionDecl { .. }
            | StmtKind::ExternClassDecl { .. }
            | StmtKind::ExternGlobalDecl { .. }
    )
}
