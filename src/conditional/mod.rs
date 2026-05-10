mod exprs;
mod stmts;

use std::collections::HashSet;

use crate::parser::ast::Program;

pub fn apply(program: Program, defines: &HashSet<String>) -> Program {
    stmts::apply_stmts(program, defines)
}
