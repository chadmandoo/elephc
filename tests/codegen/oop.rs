//! Purpose:
//! Groups the object-oriented PHP integration test submodules into the parent suite.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Submodules group focused fixtures for instanceof, traits, inheritance, interfaces, class modifiers and properties, and related suites.

use crate::support::*;

#[path = "oop/instanceof.rs"]
mod instanceof;
#[path = "oop/traits.rs"]
mod traits;
#[path = "oop/inheritance.rs"]
mod inheritance;
#[path = "oop/interfaces.rs"]
mod interfaces;
#[path = "oop/modifiers_and_properties.rs"]
mod modifiers_and_properties;
#[path = "oop/callables/mod.rs"]
mod callables;
#[path = "oop/union_types.rs"]
mod union_types;
#[path = "oop/relative_types.rs"]
mod relative_types;
#[path = "oop/anonymous_classes.rs"]
mod anonymous_classes;
#[path = "oop/intersection_types.rs"]
mod intersection_types;
#[path = "oop/dynamic_dispatch.rs"]
mod dynamic_dispatch;
#[path = "oop/misc.rs"]
mod misc;
#[path = "oop/clone.rs"]
mod clone;
#[path = "oop/attributes.rs"]
mod attributes;
#[path = "oop/constants.rs"]
mod constants;
#[path = "oop/abstract_properties.rs"]
mod abstract_properties;
#[path = "oop/property_hooks.rs"]
mod property_hooks;
#[path = "oop/datetime.rs"]
mod datetime;

/// EC-38 (#531): a `self::` typed-class-constant default on a promoted constructor
/// parameter materializes at OMITTING call sites — defaults are lowered in the CALLER's
/// scope, where `self` has no meaning, so the flattening rewrites relative receivers to
/// the declaring class (PgsqlDriver DEFAULT_PREPARED_CACHE_SIZE pattern).
#[test]
fn test_promoted_param_self_const_default() {
    let out = compile_and_run(
        r#"<?php
final class Driver {
    private const int DEFAULT_SIZE = 256;
    public function __construct(
        private int $size = self::DEFAULT_SIZE,
    ) {}
    public function size(): int {
        return $this->size;
    }
}
$d = new Driver();
echo $d->size(), "|";
$e = new Driver(32);
echo $e->size();
"#,
    );
    assert_eq!(out, "256|32");
}

/// EC-38 (#531): an UNTYPED parameter on a body-less interface method accepts any
/// argument (PSR-7 `withHeader(string $name, $value)` pattern), and the implementing
/// class's untyped parameter inherits the interface's param type so caller coercion
/// and the body's reads agree on one representation.
#[test]
fn test_interface_untyped_param_accepts_any_argument() {
    let out = compile_and_run(
        r#"<?php
interface ResponseLike {
    public function withHeader(string $name, $value): static;
}
final class Resp implements ResponseLike {
    /** @var array<string, string> */
    public array $headers = [];
    public function withHeader(string $name, $value): static {
        $c = clone $this;
        $c->headers[$name] = (string) $value;
        return $c;
    }
}
function handle(ResponseLike $r): ResponseLike {
    return $r->withHeader("Content-Type", "text/html");
}
$r = handle(new Resp());
if ($r instanceof Resp) {
    echo $r->headers["Content-Type"];
}
"#,
    );
    assert_eq!(out, "text/html");
}
