//! Purpose:
//! Injects a minimal `Random\Randomizer` class (the PHP 8.2 random extension). elephc has no
//! native `Random\Randomizer`, so a program that type-hints or instantiates it fails with
//! "Unknown type: Random\Randomizer". This provides the class with the one method the corpus
//! uses — `getBytes()` — delegating to the native `random_bytes` builtin.
//!
//! Called from:
//! - `crate::pipeline::compile` (and the codegen test harness) via `inject_if_used`, before name
//!   resolution so a `new Randomizer()` / `Randomizer $r` binds to the injected class.
//!
//! Key details:
//! - Scoped to `getBytes(int $length): string`, the only Randomizer method the corpus calls
//!   (ward-dbal-file transaction-id generation). Other engine methods (getInt / shuffleArray /
//!   shuffleBytes / nextInt / getFloat / ...) are intentionally not emitted; add them when a call
//!   site appears rather than shipping an unexercised surface.
//! - The class is emitted inside a braced `namespace Random { ... }` block so prepending it does
//!   not re-namespace the rest of the program, and `getBytes` calls the fully-qualified
//!   `\random_bytes` so it resolves to the global builtin from inside the `Random` namespace.
//! - Pay-for-use: injected only when the program mentions `Randomizer` and does not declare its
//!   own `Randomizer` class (top level or inside any namespace block). Link-GC strips the class
//!   when it is only mentioned in a docblock/comment.

use crate::parser::ast::{Program, Stmt, StmtKind};

/// The elephc-PHP source for the injected `Random\Randomizer` class.
const RANDOM_RANDOMIZER_PRELUDE_SRC: &str = r#"<?php
namespace Random {
    final class Randomizer {
        public function getBytes(int $length): string {
            return \random_bytes($length);
        }
    }
}
"#;

/// Returns true when `stmts` already declare a `Randomizer` class (top level or inside a namespace
/// block), so the prelude does not redeclare a user-provided one.
fn declares_randomizer(stmts: &[Stmt]) -> bool {
    stmts.iter().any(|stmt| match &stmt.kind {
        StmtKind::ClassDecl { name, .. } => name.eq_ignore_ascii_case("Randomizer"),
        StmtKind::NamespaceBlock { body, .. } => declares_randomizer(body),
        _ => false,
    })
}

/// Prepends the `Random\Randomizer` class when the program mentions `Randomizer` and does not
/// declare its own; otherwise returns the program unchanged. Tokenize/parse failure is a compiler
/// bug and panics rather than degrading.
pub fn inject_if_used(program: Program) -> Program {
    if !format!("{program:?}").contains("Randomizer") {
        return program;
    }
    if declares_randomizer(&program) {
        return program;
    }
    let tokens = crate::lexer::tokenize(RANDOM_RANDOMIZER_PRELUDE_SRC)
        .expect("Random\\Randomizer prelude must tokenize");
    let mut combined =
        crate::parser::parse(&tokens).expect("Random\\Randomizer prelude must parse");
    combined.extend(program);
    combined
}
