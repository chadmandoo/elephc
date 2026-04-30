use super::*;

#[test]
fn test_method_call_on_nullable_object_parameter() {
    // Calling a method through a `?Foo` parameter must dispatch to the
    // declared class — the runtime representation is a boxed mixed cell,
    // and the codegen now unboxes it to recover the concrete object
    // pointer before reading the class id.
    let out = compile_and_run(
        r#"<?php
class Holder {
    public string $msg;
    public function __construct(string $m) { $this->msg = $m; }
    public function getMsg(): string { return $this->msg; }
}
function deliver(?Holder $h): void {
    if ($h instanceof Holder) {
        echo $h->getMsg();
    }
}
deliver(new Holder("ok"));
"#,
    );
    assert_eq!(out, "ok");
}

#[test]
fn test_property_access_on_nullable_object_parameter() {
    let out = compile_and_run(
        r#"<?php
class Holder {
    public string $msg = "default";
    public function __construct(string $m) { $this->msg = $m; }
}
function read(?Holder $h): void {
    if ($h instanceof Holder) {
        echo $h->msg;
    }
}
read(new Holder("hi"));
"#,
    );
    assert_eq!(out, "hi");
}

#[test]
fn test_nullable_object_property_round_trip_through_typed_field() {
    // Storing a nullable object into a typed field and reading it back
    // exercises both write and load paths through the boxed
    // representation — the unbox must run on read so subsequent method
    // calls land on the real object.
    let out = compile_and_run(
        r#"<?php
class Holder {
    public string $msg;
    public function __construct(string $m) { $this->msg = $m; }
    public function getMsg(): string { return $this->msg; }
}
class Box {
    public ?Holder $h = null;
    public function setIt(?Holder $h): void { $this->h = $h; }
}
$b = new Box();
$b->setIt(new Holder("via-box"));
if ($b->h instanceof Holder) {
    echo $b->h->getMsg();
}
"#,
    );
    assert_eq!(out, "via-box");
}

#[test]
fn test_nullable_object_method_call_returns_correct_string_length() {
    // Regression for the bug where a method call on a `?Foo` receiver
    // returned the boxed-mixed tag word instead of the method's return
    // value. Asserting strlen of the returned string verifies that the
    // payload bytes match exactly.
    let out = compile_and_run(
        r#"<?php
class Tag {
    public string $name;
    public function __construct(string $n) { $this->name = $n; }
    public function getName(): string { return $this->name; }
}
function show(?Tag $t): void {
    if ($t instanceof Tag) {
        $name = $t->getName();
        echo strlen($name);
        echo ":";
        echo $name;
    }
}
show(new Tag("Europe/Paris"));
"#,
    );
    assert_eq!(out, "12:Europe/Paris");
}

#[test]
fn test_nullable_object_chain_method_calls() {
    // Two chained method calls on a `?Foo` value — both must unbox.
    let out = compile_and_run(
        r#"<?php
class Inner {
    public string $value;
    public function __construct(string $v) { $this->value = $v; }
    public function get(): string { return $this->value; }
}
class Outer {
    public ?Inner $inner = null;
    public function __construct(?Inner $i) { $this->inner = $i; }
    public function getInner(): ?Inner { return $this->inner; }
}
$o = new Outer(new Inner("nested"));
$inner = $o->getInner();
if ($inner instanceof Inner) {
    echo $inner->get();
}
"#,
    );
    assert_eq!(out, "nested");
}
