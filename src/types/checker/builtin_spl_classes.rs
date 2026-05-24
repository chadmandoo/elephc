//! Purpose:
//! Injects SPL container class metadata into the checker.
//! Provides nominal class/interface/signature contracts for runtime-backed and synthetic SPL containers.
//!
//! Called from:
//! - `crate::types::checker::driver`
//!
//! Key details:
//! - Direct storage and legacy serialization methods use runtime `IntrinsicCall` backing.
//! - Structured serialization/debug helpers keep small synthetic PHP bodies.

use std::collections::HashMap;

use crate::errors::CompileError;
use crate::names::{php_symbol_key, Name};
use crate::parser::ast::{
    BinOp, ClassConst, ClassMethod, ClassProperty, Expr, ExprKind, PropertyHooks, Stmt,
    StmtKind, TypeExpr, Visibility,
};
use crate::types::{traits::FlattenedClass, PhpType};

use super::{builtin_types::InterfaceDeclInfo, Checker};

pub(crate) fn inject_builtin_spl_classes(
    interface_map: &mut HashMap<String, InterfaceDeclInfo>,
    class_map: &mut HashMap<String, FlattenedClass>,
) -> Result<(), CompileError> {
    for class_name in SPL_CLASS_NAMES {
        let class_key = php_symbol_key(class_name);
        if interface_map
            .keys()
            .any(|name| php_symbol_key(name) == class_key)
            || class_map
                .keys()
                .any(|name| php_symbol_key(name) == class_key)
        {
            return Err(CompileError::new(
                crate::span::Span::dummy(),
                &format!("Cannot redeclare built-in SPL class: {}", class_name),
            ));
        }
    }

    class_map.insert(
        "SplDoublyLinkedList".to_string(),
        FlattenedClass {
            name: "SplDoublyLinkedList".to_string(),
            extends: None,
            implements: vec![
                "Iterator".to_string(),
                "Countable".to_string(),
                "ArrayAccess".to_string(),
            ],
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: Vec::new(),
            methods: spl_doubly_linked_list_methods(),
            attributes: Vec::new(),
            constants: spl_doubly_linked_list_constants(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "SplStack".to_string(),
        FlattenedClass {
            name: "SplStack".to_string(),
            extends: Some("SplDoublyLinkedList".to_string()),
            implements: Vec::new(),
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "SplQueue".to_string(),
        FlattenedClass {
            name: "SplQueue".to_string(),
            extends: Some("SplDoublyLinkedList".to_string()),
            implements: Vec::new(),
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: Vec::new(),
            methods: vec![
                method("enqueue", vec![param("value", mixed_type())], Some(TypeExpr::Void)),
                method("dequeue", Vec::new(), Some(mixed_type())),
            ],
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "SplFixedArray".to_string(),
        FlattenedClass {
            name: "SplFixedArray".to_string(),
            extends: None,
            implements: vec![
                "ArrayAccess".to_string(),
                "Countable".to_string(),
                "JsonSerializable".to_string(),
            ],
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: Vec::new(),
            methods: spl_fixed_array_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "EmptyIterator".to_string(),
        FlattenedClass {
            name: "EmptyIterator".to_string(),
            extends: None,
            implements: vec!["Iterator".to_string()],
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: Vec::new(),
            methods: spl_empty_iterator_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "ArrayIterator".to_string(),
        FlattenedClass {
            name: "ArrayIterator".to_string(),
            extends: None,
            implements: vec![
                "Iterator".to_string(),
                "ArrayAccess".to_string(),
                "SeekableIterator".to_string(),
                "Countable".to_string(),
            ],
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: array_iterator_properties(),
            methods: spl_array_iterator_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "ArrayObject".to_string(),
        FlattenedClass {
            name: "ArrayObject".to_string(),
            extends: None,
            implements: vec![
                "IteratorAggregate".to_string(),
                "ArrayAccess".to_string(),
                "Countable".to_string(),
            ],
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: array_object_properties(),
            methods: spl_array_object_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "IteratorIterator".to_string(),
        FlattenedClass {
            name: "IteratorIterator".to_string(),
            extends: None,
            implements: vec!["OuterIterator".to_string()],
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: iterator_iterator_properties(),
            methods: spl_iterator_iterator_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "LimitIterator".to_string(),
        FlattenedClass {
            name: "LimitIterator".to_string(),
            extends: Some("IteratorIterator".to_string()),
            implements: Vec::new(),
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: limit_iterator_properties(),
            methods: spl_limit_iterator_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "NoRewindIterator".to_string(),
        FlattenedClass {
            name: "NoRewindIterator".to_string(),
            extends: Some("IteratorIterator".to_string()),
            implements: Vec::new(),
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: Vec::new(),
            methods: spl_no_rewind_iterator_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "InfiniteIterator".to_string(),
        FlattenedClass {
            name: "InfiniteIterator".to_string(),
            extends: Some("IteratorIterator".to_string()),
            implements: Vec::new(),
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: Vec::new(),
            methods: spl_infinite_iterator_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "AppendIterator".to_string(),
        FlattenedClass {
            name: "AppendIterator".to_string(),
            extends: Some("IteratorIterator".to_string()),
            implements: Vec::new(),
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: append_iterator_properties(),
            methods: spl_append_iterator_methods(),
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );

    class_map.insert(
        "MultipleIterator".to_string(),
        FlattenedClass {
            name: "MultipleIterator".to_string(),
            extends: None,
            implements: vec!["Iterator".to_string()],
            is_abstract: false,
            is_final: false,
            is_readonly_class: false,
            properties: multiple_iterator_properties(),
            methods: spl_multiple_iterator_methods(),
            attributes: Vec::new(),
            constants: multiple_iterator_constants(),
            used_traits: Vec::new(),
        },
    );

    Ok(())
}

pub(crate) fn patch_builtin_spl_storage_signatures(checker: &mut Checker) {
    let return_type = PhpType::AssocArray {
        key: Box::new(PhpType::Mixed),
        value: Box::new(PhpType::Mixed),
    };
    let method_key = php_symbol_key("getArrayCopy");
    for class_name in ["ArrayIterator", "ArrayObject"] {
        if let Some(class_info) = checker.classes.get_mut(class_name) {
            if let Some(sig) = class_info.methods.get_mut(&method_key) {
                sig.return_type = return_type.clone();
            }
        }
    }
    if let Some(class_info) = checker.classes.get_mut("IteratorIterator") {
        if let Some(sig) = class_info.methods.get_mut("__construct") {
            if let Some((_, ty)) = sig.params.first_mut() {
                *ty = PhpType::Object("Traversable".to_string());
            }
            if sig.params.len() == 1 {
                sig.params.push((
                    "class".to_string(),
                    PhpType::Union(vec![PhpType::Str, PhpType::Void]),
                ));
                sig.defaults.push(Some(null_expr()));
                sig.ref_params.push(false);
                sig.declared_params.push(true);
            }
        }
    }
    let iterator_array_type = PhpType::Array(Box::new(PhpType::Object("Iterator".to_string())));
    if let Some(class_info) = checker.classes.get_mut("AppendIterator") {
        for (name, ty) in &mut class_info.properties {
            if name == "iterators" {
                *ty = iterator_array_type.clone();
            }
        }
    }
    if let Some(class_info) = checker.classes.get_mut("MultipleIterator") {
        for (name, ty) in &mut class_info.properties {
            if name == "iterators" {
                *ty = iterator_array_type.clone();
            } else if name == "infos" {
                *ty = PhpType::Array(Box::new(PhpType::Mixed));
            }
        }
        for method in ["key", "current"] {
            if let Some(sig) = class_info.methods.get_mut(&php_symbol_key(method)) {
                sig.return_type = PhpType::Mixed;
            }
        }
    }
}

const SPL_CLASS_NAMES: &[&str] = &[
    "SplDoublyLinkedList",
    "SplStack",
    "SplQueue",
    "SplFixedArray",
    "EmptyIterator",
    "ArrayIterator",
    "ArrayObject",
    "IteratorIterator",
    "LimitIterator",
    "NoRewindIterator",
    "InfiniteIterator",
    "AppendIterator",
    "MultipleIterator",
];

fn spl_empty_iterator_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body("current", Vec::new(), Some(mixed_type()), null_return_body()),
        method_with_body("key", Vec::new(), Some(mixed_type()), null_return_body()),
        method_with_body("next", Vec::new(), Some(TypeExpr::Void), Vec::new()),
        method_with_body("rewind", Vec::new(), Some(TypeExpr::Void), Vec::new()),
        method_with_body("valid", Vec::new(), Some(TypeExpr::Bool), return_body(bool_expr(false))),
    ]
}

fn array_iterator_properties() -> Vec<ClassProperty> {
    vec![
        storage_property("keys", array_type()),
        storage_property("values", array_type()),
        storage_property("position", TypeExpr::Int),
        storage_property("flags", TypeExpr::Int),
    ]
}

fn array_object_properties() -> Vec<ClassProperty> {
    vec![
        storage_property("keys", array_type()),
        storage_property("values", array_type()),
        storage_property("flags", TypeExpr::Int),
    ]
}

fn iterator_iterator_properties() -> Vec<ClassProperty> {
    vec![storage_property("inner", named_type("Iterator"))]
}

fn limit_iterator_properties() -> Vec<ClassProperty> {
    vec![
        storage_property("position", TypeExpr::Int),
        storage_property("offset", TypeExpr::Int),
        storage_property("limit", TypeExpr::Int),
    ]
}

fn append_iterator_properties() -> Vec<ClassProperty> {
    vec![
        storage_property("iterators", array_type()),
        storage_property("index", TypeExpr::Int),
    ]
}

fn multiple_iterator_properties() -> Vec<ClassProperty> {
    vec![
        storage_property("iterators", array_type()),
        storage_property("infos", array_type()),
        storage_property("flags", TypeExpr::Int),
    ]
}

fn spl_array_iterator_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body(
            "__construct",
            vec![
                param_default("array", array_type(), empty_array_expr()),
                param_default("flags", TypeExpr::Int, int_expr(0)),
            ],
            Some(TypeExpr::Void),
            array_iterator_construct_body(),
        ),
        method_with_body("current", Vec::new(), Some(mixed_type()), array_current_body()),
        method_with_body("key", Vec::new(), Some(mixed_type()), array_key_body()),
        method_with_body("next", Vec::new(), Some(TypeExpr::Void), array_next_body()),
        method_with_body("rewind", Vec::new(), Some(TypeExpr::Void), array_rewind_body()),
        method_with_body("valid", Vec::new(), Some(TypeExpr::Bool), array_valid_body()),
        method_with_body(
            "seek",
            vec![param("offset", TypeExpr::Int)],
            Some(TypeExpr::Void),
            vec![property_assign_stmt(this_expr(), "position", var_expr("offset"))],
        ),
        method_with_body("count", Vec::new(), Some(TypeExpr::Int), array_count_body()),
        method_with_body(
            "offsetExists",
            vec![param("offset", mixed_type())],
            Some(TypeExpr::Bool),
            array_offset_exists_body(),
        ),
        method_with_body(
            "offsetGet",
            vec![param("offset", mixed_type())],
            Some(mixed_type()),
            array_offset_get_body(),
        ),
        method_with_body(
            "offsetSet",
            vec![param("offset", mixed_type()), param("value", mixed_type())],
            Some(TypeExpr::Void),
            array_offset_set_body(),
        ),
        method_with_body(
            "offsetUnset",
            vec![param("offset", mixed_type())],
            Some(TypeExpr::Void),
            array_offset_unset_body(),
        ),
        method_with_body(
            "append",
            vec![param("value", mixed_type())],
            Some(TypeExpr::Void),
            array_append_body(),
        ),
        method_with_body("getArrayCopy", Vec::new(), Some(array_type()), array_copy_body()),
    ]
}

fn spl_array_object_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body(
            "__construct",
            vec![
                param_default("array", array_type(), empty_array_expr()),
                param_default("flags", TypeExpr::Int, int_expr(0)),
            ],
            Some(TypeExpr::Void),
            array_object_construct_body(),
        ),
        method_with_body("getIterator", Vec::new(), Some(named_type("ArrayIterator")), array_object_get_iterator_body()),
        method_with_body("count", Vec::new(), Some(TypeExpr::Int), array_count_body()),
        method_with_body(
            "offsetExists",
            vec![param("offset", mixed_type())],
            Some(TypeExpr::Bool),
            array_offset_exists_body(),
        ),
        method_with_body(
            "offsetGet",
            vec![param("offset", mixed_type())],
            Some(mixed_type()),
            array_offset_get_body(),
        ),
        method_with_body(
            "offsetSet",
            vec![param("offset", mixed_type()), param("value", mixed_type())],
            Some(TypeExpr::Void),
            array_offset_set_body(),
        ),
        method_with_body(
            "offsetUnset",
            vec![param("offset", mixed_type())],
            Some(TypeExpr::Void),
            array_offset_unset_body(),
        ),
        method_with_body(
            "append",
            vec![param("value", mixed_type())],
            Some(TypeExpr::Void),
            array_append_body(),
        ),
        method_with_body("getArrayCopy", Vec::new(), Some(array_type()), array_copy_body()),
    ]
}

fn spl_iterator_iterator_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body(
            "__construct",
            vec![
                param("iterator", named_type("Traversable")),
                param_default(
                    "class",
                    TypeExpr::Nullable(Box::new(TypeExpr::Str)),
                    null_expr(),
                ),
            ],
            Some(TypeExpr::Void),
            iterator_iterator_construct_body(),
        ),
        method_with_body("current", Vec::new(), Some(mixed_type()), inner_return_body("current")),
        method_with_body("key", Vec::new(), Some(mixed_type()), inner_return_body("key")),
        method_with_body("next", Vec::new(), Some(TypeExpr::Void), inner_void_body("next")),
        method_with_body("rewind", Vec::new(), Some(TypeExpr::Void), inner_void_body("rewind")),
        method_with_body("valid", Vec::new(), Some(TypeExpr::Bool), inner_return_body("valid")),
        method_with_body(
            "getInnerIterator",
            Vec::new(),
            Some(TypeExpr::Nullable(Box::new(named_type("Iterator")))),
            return_body(inner_expr()),
        ),
    ]
}

fn spl_limit_iterator_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body(
            "__construct",
            vec![
                param("iterator", named_type("Iterator")),
                param_default("offset", TypeExpr::Int, int_expr(0)),
                param_default("limit", TypeExpr::Int, int_expr(-1)),
            ],
            Some(TypeExpr::Void),
            limit_iterator_construct_body(),
        ),
        method_with_body("rewind", Vec::new(), Some(TypeExpr::Void), limit_rewind_body()),
        method_with_body("next", Vec::new(), Some(TypeExpr::Void), limit_next_body()),
        method_with_body("valid", Vec::new(), Some(TypeExpr::Bool), limit_valid_body()),
        method_with_body(
            "seek",
            vec![param("offset", TypeExpr::Int)],
            Some(TypeExpr::Void),
            limit_seek_body(),
        ),
        method_with_body("getPosition", Vec::new(), Some(TypeExpr::Int), return_body(limit_position_expr())),
    ]
}

fn spl_no_rewind_iterator_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body(
            "__construct",
            vec![param("iterator", named_type("Iterator"))],
            Some(TypeExpr::Void),
            iterator_iterator_construct_body(),
        ),
        method_with_body("rewind", Vec::new(), Some(TypeExpr::Void), Vec::new()),
    ]
}

fn spl_infinite_iterator_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body(
            "__construct",
            vec![param("iterator", named_type("Iterator"))],
            Some(TypeExpr::Void),
            iterator_iterator_construct_body(),
        ),
        method_with_body("next", Vec::new(), Some(TypeExpr::Void), infinite_next_body()),
    ]
}

fn spl_append_iterator_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body("__construct", Vec::new(), Some(TypeExpr::Void), append_construct_body()),
        method_with_body(
            "append",
            vec![param("iterator", named_type("Iterator"))],
            Some(TypeExpr::Void),
            append_append_body(),
        ),
        method_with_body("rewind", Vec::new(), Some(TypeExpr::Void), append_rewind_body()),
        method_with_body("valid", Vec::new(), Some(TypeExpr::Bool), append_valid_body()),
        method_with_body("current", Vec::new(), Some(mixed_type()), append_current_body()),
        method_with_body("key", Vec::new(), Some(mixed_type()), append_key_body()),
        method_with_body("next", Vec::new(), Some(TypeExpr::Void), append_next_body()),
        method_with_body(
            "getInnerIterator",
            Vec::new(),
            Some(TypeExpr::Nullable(Box::new(named_type("Iterator")))),
            append_get_inner_iterator_body(),
        ),
        method_with_body(
            "getIteratorIndex",
            Vec::new(),
            Some(TypeExpr::Nullable(Box::new(TypeExpr::Int))),
            append_get_iterator_index_body(),
        ),
        method_with_body(
            "getArrayIterator",
            Vec::new(),
            Some(named_type("ArrayIterator")),
            append_get_array_iterator_body(),
        ),
    ]
}

fn spl_multiple_iterator_methods() -> Vec<ClassMethod> {
    vec![
        method_with_body(
            "__construct",
            vec![param_default("flags", TypeExpr::Int, int_expr(1))],
            Some(TypeExpr::Void),
            multiple_construct_body(),
        ),
        method_with_body("getFlags", Vec::new(), Some(TypeExpr::Int), return_body(multiple_flags_expr())),
        method_with_body(
            "setFlags",
            vec![param("flags", TypeExpr::Int)],
            Some(TypeExpr::Void),
            vec![property_assign_stmt(this_expr(), "flags", var_expr("flags"))],
        ),
        method_with_body(
            "attachIterator",
            vec![
                param("iterator", named_type("Iterator")),
                param_default(
                    "info",
                    TypeExpr::Nullable(Box::new(TypeExpr::Union(vec![TypeExpr::Str, TypeExpr::Int]))),
                    null_expr(),
                ),
            ],
            Some(TypeExpr::Void),
            multiple_attach_iterator_body(),
        ),
        method_with_body(
            "detachIterator",
            vec![param("iterator", named_type("Iterator"))],
            Some(TypeExpr::Void),
            multiple_detach_iterator_body(),
        ),
        method_with_body(
            "containsIterator",
            vec![param("iterator", named_type("Iterator"))],
            Some(TypeExpr::Bool),
            multiple_contains_iterator_body(),
        ),
        method_with_body(
            "countIterators",
            Vec::new(),
            Some(TypeExpr::Int),
            return_body(count_expr(multiple_iterators_expr())),
        ),
        method_with_body("rewind", Vec::new(), Some(TypeExpr::Void), multiple_rewind_body()),
        method_with_body("valid", Vec::new(), Some(TypeExpr::Bool), multiple_valid_body()),
        method_with_body("key", Vec::new(), Some(mixed_type()), multiple_output_body("key")),
        method_with_body("current", Vec::new(), Some(mixed_type()), multiple_output_body("current")),
        method_with_body("next", Vec::new(), Some(TypeExpr::Void), multiple_next_body()),
        method_with_body("__debugInfo", Vec::new(), Some(array_type()), multiple_debug_info_body()),
    ]
}

fn spl_doubly_linked_list_methods() -> Vec<ClassMethod> {
    vec![
        method(
            "add",
            vec![param("index", TypeExpr::Int), param("value", mixed_type())],
            Some(TypeExpr::Void),
        ),
        method("pop", Vec::new(), Some(mixed_type())),
        method("shift", Vec::new(), Some(mixed_type())),
        method("push", vec![param("value", mixed_type())], Some(TypeExpr::Void)),
        method(
            "unshift",
            vec![param("value", mixed_type())],
            Some(TypeExpr::Void),
        ),
        method("top", Vec::new(), Some(mixed_type())),
        method("bottom", Vec::new(), Some(mixed_type())),
        method("count", Vec::new(), Some(TypeExpr::Int)),
        method("isEmpty", Vec::new(), Some(TypeExpr::Bool)),
        method(
            "setIteratorMode",
            vec![param("mode", TypeExpr::Int)],
            Some(TypeExpr::Void),
        ),
        method("getIteratorMode", Vec::new(), Some(TypeExpr::Int)),
        method(
            "offsetExists",
            vec![param("index", mixed_type())],
            Some(TypeExpr::Bool),
        ),
        method(
            "offsetGet",
            vec![param("index", mixed_type())],
            Some(mixed_type()),
        ),
        method(
            "offsetSet",
            vec![param("index", mixed_type()), param("value", mixed_type())],
            Some(TypeExpr::Void),
        ),
        method(
            "offsetUnset",
            vec![param("index", mixed_type())],
            Some(TypeExpr::Void),
        ),
        method("rewind", Vec::new(), Some(TypeExpr::Void)),
        method("current", Vec::new(), Some(mixed_type())),
        method("key", Vec::new(), Some(mixed_type())),
        method("prev", Vec::new(), Some(TypeExpr::Void)),
        method("next", Vec::new(), Some(TypeExpr::Void)),
        method("valid", Vec::new(), Some(TypeExpr::Bool)),
        method("serialize", Vec::new(), Some(TypeExpr::Str)),
        method(
            "unserialize",
            vec![param("data", TypeExpr::Str)],
            Some(TypeExpr::Void),
        ),
        method_with_body(
            "__serialize",
            Vec::new(),
            Some(array_type()),
            dll_serialize_array_body(),
        ),
        method_with_body(
            "__unserialize",
            vec![param("data", array_type())],
            Some(TypeExpr::Void),
            dll_unserialize_body(),
        ),
        method_with_body(
            "__debugInfo",
            Vec::new(),
            Some(array_type()),
            dll_debug_info_body(),
        ),
    ]
}

fn spl_fixed_array_methods() -> Vec<ClassMethod> {
    vec![
        method(
            "__construct",
            vec![param_default("size", TypeExpr::Int, int_expr(0))],
            Some(TypeExpr::Void),
        ),
        method_with_body("__wakeup", Vec::new(), Some(TypeExpr::Void), Vec::new()),
        class_method(
            "fromArray",
            true,
            vec![
                param("array", array_type()),
                param_default("preserveKeys", TypeExpr::Bool, bool_expr(true)),
            ],
            Some(named_type("SplFixedArray")),
        ),
        method_with_body(
            "__serialize",
            Vec::new(),
            Some(array_type()),
            vec![return_stmt(method_call(this_expr(), "toArray", Vec::new()))],
        ),
        method("__unserialize", vec![param("data", array_type())], Some(TypeExpr::Void)),
        method("count", Vec::new(), Some(TypeExpr::Int)),
        method("toArray", Vec::new(), Some(array_type())),
        method("getSize", Vec::new(), Some(TypeExpr::Int)),
        method(
            "setSize",
            vec![param("size", TypeExpr::Int)],
            Some(TypeExpr::Void),
        ),
        method(
            "offsetExists",
            vec![param("index", mixed_type())],
            Some(TypeExpr::Bool),
        ),
        method(
            "offsetGet",
            vec![param("index", mixed_type())],
            Some(mixed_type()),
        ),
        method(
            "offsetSet",
            vec![param("index", mixed_type()), param("value", mixed_type())],
            Some(TypeExpr::Void),
        ),
        method(
            "offsetUnset",
            vec![param("index", mixed_type())],
            Some(TypeExpr::Void),
        ),
        method("jsonSerialize", Vec::new(), Some(array_type())),
    ]
}

fn spl_doubly_linked_list_constants() -> Vec<ClassConst> {
    vec![
        class_const("IT_MODE_LIFO", 2),
        class_const("IT_MODE_FIFO", 0),
        class_const("IT_MODE_DELETE", 1),
        class_const("IT_MODE_KEEP", 0),
    ]
}

fn multiple_iterator_constants() -> Vec<ClassConst> {
    vec![
        class_const("MIT_NEED_ANY", 0),
        class_const("MIT_NEED_ALL", 1),
        class_const("MIT_KEYS_NUMERIC", 0),
        class_const("MIT_KEYS_ASSOC", 2),
    ]
}

fn method(
    name: &str,
    params: Vec<(String, Option<TypeExpr>, Option<Expr>, bool)>,
    return_type: Option<TypeExpr>,
) -> ClassMethod {
    class_method(name, false, params, return_type)
}

fn method_with_body(
    name: &str,
    params: Vec<(String, Option<TypeExpr>, Option<Expr>, bool)>,
    return_type: Option<TypeExpr>,
    body: Vec<Stmt>,
) -> ClassMethod {
    class_method_with_body(name, false, params, return_type, body)
}

fn class_method(
    name: &str,
    is_static: bool,
    params: Vec<(String, Option<TypeExpr>, Option<Expr>, bool)>,
    return_type: Option<TypeExpr>,
) -> ClassMethod {
    class_method_with_body(
        name,
        is_static,
        params,
        return_type.clone(),
        dummy_body_for(return_type.as_ref()),
    )
}

fn class_method_with_body(
    name: &str,
    is_static: bool,
    params: Vec<(String, Option<TypeExpr>, Option<Expr>, bool)>,
    return_type: Option<TypeExpr>,
    body: Vec<Stmt>,
) -> ClassMethod {
    ClassMethod {
        name: name.to_string(),
        visibility: Visibility::Public,
        is_static,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params,
        variadic: None,
        return_type,
        body,
        span: crate::span::Span::dummy(),
        attributes: Vec::new(),
    }
}

fn storage_property(name: &str, type_expr: TypeExpr) -> ClassProperty {
    ClassProperty {
        name: name.to_string(),
        visibility: Visibility::Private,
        type_expr: Some(type_expr),
        hooks: PropertyHooks::none(),
        readonly: false,
        is_final: false,
        is_static: false,
        is_abstract: false,
        by_ref: false,
        default: None,
        span: crate::span::Span::dummy(),
        attributes: Vec::new(),
    }
}

fn dummy_body_for(return_type: Option<&TypeExpr>) -> Vec<Stmt> {
    match return_type {
        Some(TypeExpr::Void) | None => Vec::new(),
        Some(TypeExpr::Int) => return_body(int_expr(0)),
        Some(TypeExpr::Bool) => return_body(bool_expr(false)),
        Some(TypeExpr::Str) => return_body(Expr::new(
            ExprKind::StringLiteral(String::new()),
            crate::span::Span::dummy(),
        )),
        Some(TypeExpr::Named(name)) if name.as_canonical() == "array" => {
            return_body(Expr::new(ExprKind::ArrayLiteral(Vec::new()), crate::span::Span::dummy()))
        }
        _ => return_body(Expr::new(ExprKind::Null, crate::span::Span::dummy())),
    }
}

fn return_body(value: Expr) -> Vec<Stmt> {
    vec![return_stmt(value)]
}

fn null_return_body() -> Vec<Stmt> {
    return_body(expr(ExprKind::Null))
}

fn return_stmt(value: Expr) -> Stmt {
    Stmt::new(StmtKind::Return(Some(value)), crate::span::Span::dummy())
}

fn return_void_stmt() -> Stmt {
    Stmt::new(StmtKind::Return(None), crate::span::Span::dummy())
}

fn throw_stmt(value: Expr) -> Stmt {
    Stmt::new(StmtKind::Throw(value), crate::span::Span::dummy())
}

fn param(name: &str, ty: TypeExpr) -> (String, Option<TypeExpr>, Option<Expr>, bool) {
    (name.to_string(), Some(ty), None, false)
}

fn param_default(
    name: &str,
    ty: TypeExpr,
    default: Expr,
) -> (String, Option<TypeExpr>, Option<Expr>, bool) {
    (name.to_string(), Some(ty), Some(default), false)
}

fn class_const(name: &str, value: i64) -> ClassConst {
    ClassConst {
        name: name.to_string(),
        visibility: Visibility::Public,
        is_final: false,
        value: int_expr(value),
        span: crate::span::Span::dummy(),
        attributes: Vec::new(),
    }
}

fn int_expr(value: i64) -> Expr {
    Expr::new(ExprKind::IntLiteral(value), crate::span::Span::dummy())
}

fn bool_expr(value: bool) -> Expr {
    Expr::new(ExprKind::BoolLiteral(value), crate::span::Span::dummy())
}

fn empty_array_expr() -> Expr {
    expr(ExprKind::ArrayLiteral(Vec::new()))
}

fn empty_assoc_array_expr() -> Expr {
    expr(ExprKind::ArrayLiteralAssoc(Vec::new()))
}

fn mixed_type() -> TypeExpr {
    named_type("mixed")
}

fn array_type() -> TypeExpr {
    named_type("array")
}

fn named_type(name: &str) -> TypeExpr {
    TypeExpr::Named(Name::unqualified(name))
}

fn expr(kind: ExprKind) -> Expr {
    Expr::new(kind, crate::span::Span::dummy())
}

fn string_expr(value: &str) -> Expr {
    expr(ExprKind::StringLiteral(value.to_string()))
}

fn var_expr(name: &str) -> Expr {
    expr(ExprKind::Variable(name.to_string()))
}

fn this_expr() -> Expr {
    expr(ExprKind::This)
}

fn null_expr() -> Expr {
    expr(ExprKind::Null)
}

fn binary_expr(left: Expr, op: BinOp, right: Expr) -> Expr {
    expr(ExprKind::BinaryOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
    })
}

fn not_expr(value: Expr) -> Expr {
    expr(ExprKind::Not(Box::new(value)))
}

fn method_call(object: Expr, method: &str, args: Vec<Expr>) -> Expr {
    expr(ExprKind::MethodCall {
        object: Box::new(object),
        method: method.to_string(),
        args,
    })
}

fn function_call(name: &str, args: Vec<Expr>) -> Expr {
    expr(ExprKind::FunctionCall {
        name: Name::unqualified(name),
        args,
    })
}

fn new_object_expr(class_name: &str, args: Vec<Expr>) -> Expr {
    expr(ExprKind::NewObject {
        class_name: Name::unqualified(class_name),
        args,
    })
}

fn property_access(object: Expr, property: &str) -> Expr {
    expr(ExprKind::PropertyAccess {
        object: Box::new(object),
        property: property.to_string(),
    })
}

fn array_access(array: Expr, index: Expr) -> Expr {
    expr(ExprKind::ArrayAccess {
        array: Box::new(array),
        index: Box::new(index),
    })
}

fn assign_stmt(name: &str, value: Expr) -> Stmt {
    Stmt::new(
        StmtKind::Assign {
            name: name.to_string(),
            value,
        },
        crate::span::Span::dummy(),
    )
}

fn typed_assign_stmt(name: &str, type_expr: TypeExpr, value: Expr) -> Stmt {
    Stmt::new(
        StmtKind::TypedAssign {
            type_expr,
            name: name.to_string(),
            value,
        },
        crate::span::Span::dummy(),
    )
}

fn expr_stmt(value: Expr) -> Stmt {
    Stmt::new(StmtKind::ExprStmt(value), crate::span::Span::dummy())
}

fn property_assign_stmt(object: Expr, property: &str, value: Expr) -> Stmt {
    Stmt::new(
        StmtKind::PropertyAssign {
            object: Box::new(object),
            property: property.to_string(),
            value,
        },
        crate::span::Span::dummy(),
    )
}

fn property_array_push_stmt(object: Expr, property: &str, value: Expr) -> Stmt {
    Stmt::new(
        StmtKind::PropertyArrayPush {
            object: Box::new(object),
            property: property.to_string(),
            value,
        },
        crate::span::Span::dummy(),
    )
}

fn property_array_assign_stmt(object: Expr, property: &str, index: Expr, value: Expr) -> Stmt {
    Stmt::new(
        StmtKind::PropertyArrayAssign {
            object: Box::new(object),
            property: property.to_string(),
            index,
            value,
        },
        crate::span::Span::dummy(),
    )
}

fn array_assign_stmt(array: &str, index: Expr, value: Expr) -> Stmt {
    Stmt::new(
        StmtKind::ArrayAssign {
            array: array.to_string(),
            index,
            value,
        },
        crate::span::Span::dummy(),
    )
}

fn array_push_stmt(array: &str, value: Expr) -> Stmt {
    Stmt::new(
        StmtKind::ArrayPush {
            array: array.to_string(),
            value,
        },
        crate::span::Span::dummy(),
    )
}

fn while_stmt(condition: Expr, body: Vec<Stmt>) -> Stmt {
    Stmt::new(
        StmtKind::While { condition, body },
        crate::span::Span::dummy(),
    )
}

fn if_stmt(condition: Expr, then_body: Vec<Stmt>, else_body: Option<Vec<Stmt>>) -> Stmt {
    Stmt::new(
        StmtKind::If {
            condition,
            then_body,
            elseif_clauses: Vec::new(),
            else_body,
        },
        crate::span::Span::dummy(),
    )
}

fn foreach_stmt(array: Expr, key_var: Option<&str>, value_var: &str, body: Vec<Stmt>) -> Stmt {
    Stmt::new(
        StmtKind::Foreach {
            array,
            key_var: key_var.map(str::to_string),
            value_var: value_var.to_string(),
            value_by_ref: false,
            body,
        },
        crate::span::Span::dummy(),
    )
}

fn increment_stmt(name: &str) -> Stmt {
    assign_stmt(name, binary_expr(var_expr(name), BinOp::Add, int_expr(1)))
}

fn keys_expr() -> Expr {
    property_access(this_expr(), "keys")
}

fn values_expr() -> Expr {
    property_access(this_expr(), "values")
}

fn position_expr() -> Expr {
    property_access(this_expr(), "position")
}

fn count_expr(value: Expr) -> Expr {
    function_call("count", vec![value])
}

fn key_at(index: Expr) -> Expr {
    array_access(keys_expr(), index)
}

fn value_at(index: Expr) -> Expr {
    array_access(values_expr(), index)
}

fn array_iterator_construct_body() -> Vec<Stmt> {
    let mut body = array_object_construct_body();
    body.insert(2, property_assign_stmt(this_expr(), "position", int_expr(0)));
    body
}

fn array_object_construct_body() -> Vec<Stmt> {
    vec![
        property_assign_stmt(this_expr(), "keys", function_call("array_keys", vec![var_expr("array")])),
        property_assign_stmt(
            this_expr(),
            "values",
            function_call("array_values", vec![var_expr("array")]),
        ),
        property_assign_stmt(this_expr(), "flags", var_expr("flags")),
    ]
}

fn array_current_body() -> Vec<Stmt> {
    return_body(value_at(position_expr()))
}

fn array_key_body() -> Vec<Stmt> {
    return_body(key_at(position_expr()))
}

fn array_next_body() -> Vec<Stmt> {
    vec![property_assign_stmt(
        this_expr(),
        "position",
        binary_expr(position_expr(), BinOp::Add, int_expr(1)),
    )]
}

fn array_rewind_body() -> Vec<Stmt> {
    vec![property_assign_stmt(this_expr(), "position", int_expr(0))]
}

fn array_valid_body() -> Vec<Stmt> {
    return_body(binary_expr(position_expr(), BinOp::Lt, count_expr(values_expr())))
}

fn array_count_body() -> Vec<Stmt> {
    return_body(count_expr(values_expr()))
}

fn array_append_body() -> Vec<Stmt> {
    vec![
        property_array_push_stmt(this_expr(), "keys", count_expr(keys_expr())),
        property_array_push_stmt(this_expr(), "values", var_expr("value")),
    ]
}

fn array_offset_exists_body() -> Vec<Stmt> {
    let mut body = array_search_prelude();
    body.push(while_stmt(
        binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
        vec![
            if_stmt(
                binary_expr(key_at(var_expr("i")), BinOp::StrictEq, var_expr("offset")),
                return_body(bool_expr(true)),
                None,
            ),
            increment_stmt("i"),
        ],
    ));
    body.push(return_stmt(bool_expr(false)));
    body
}

fn array_offset_get_body() -> Vec<Stmt> {
    let mut body = array_search_prelude();
    body.push(while_stmt(
        binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
        vec![
            if_stmt(
                binary_expr(key_at(var_expr("i")), BinOp::StrictEq, var_expr("offset")),
                return_body(value_at(var_expr("i"))),
                None,
            ),
            increment_stmt("i"),
        ],
    ));
    body.push(return_stmt(null_expr()));
    body
}

fn array_offset_set_body() -> Vec<Stmt> {
    let mut body = vec![if_stmt(
        binary_expr(var_expr("offset"), BinOp::StrictEq, null_expr()),
        vec![
            expr_stmt(method_call(this_expr(), "append", vec![var_expr("value")])),
            return_void_stmt(),
        ],
        None,
    )];
    body.extend(array_search_prelude());
    body.push(while_stmt(
        binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
        vec![
            if_stmt(
                binary_expr(key_at(var_expr("i")), BinOp::StrictEq, var_expr("offset")),
                vec![
                    property_array_assign_stmt(this_expr(), "values", var_expr("i"), var_expr("value")),
                    return_void_stmt(),
                ],
                None,
            ),
            increment_stmt("i"),
        ],
    ));
    body.push(property_array_push_stmt(this_expr(), "keys", var_expr("offset")));
    body.push(property_array_push_stmt(this_expr(), "values", var_expr("value")));
    body
}

fn array_offset_unset_body() -> Vec<Stmt> {
    vec![
        assign_stmt("newKeys", empty_array_expr()),
        assign_stmt("newValues", empty_array_expr()),
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(keys_expr())),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                if_stmt(
                    not_expr(binary_expr(key_at(var_expr("i")), BinOp::StrictEq, var_expr("offset"))),
                    vec![
                        array_push_stmt("newKeys", key_at(var_expr("i"))),
                        array_push_stmt("newValues", value_at(var_expr("i"))),
                    ],
                    None,
                ),
                increment_stmt("i"),
            ],
        ),
        property_assign_stmt(this_expr(), "keys", var_expr("newKeys")),
        property_assign_stmt(this_expr(), "values", var_expr("newValues")),
    ]
}

fn array_copy_body() -> Vec<Stmt> {
    vec![
        assign_stmt("out", empty_assoc_array_expr()),
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(keys_expr())),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                array_assign_stmt("out", key_at(var_expr("i")), value_at(var_expr("i"))),
                increment_stmt("i"),
            ],
        ),
        return_stmt(var_expr("out")),
    ]
}

fn array_object_get_iterator_body() -> Vec<Stmt> {
    vec![
        assign_stmt("it", new_object_expr("ArrayIterator", vec![empty_array_expr()])),
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(keys_expr())),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                expr_stmt(method_call(
                    var_expr("it"),
                    "offsetSet",
                    vec![key_at(var_expr("i")), value_at(var_expr("i"))],
                )),
                increment_stmt("i"),
            ],
        ),
        return_stmt(var_expr("it")),
    ]
}

fn iterator_iterator_construct_body() -> Vec<Stmt> {
    vec![property_assign_stmt(this_expr(), "inner", var_expr("iterator"))]
}

fn inner_expr() -> Expr {
    property_access(this_expr(), "inner")
}

fn inner_call(method: &str) -> Expr {
    method_call(inner_expr(), method, Vec::new())
}

fn inner_return_body(method: &str) -> Vec<Stmt> {
    return_body(inner_call(method))
}

fn inner_void_body(method: &str) -> Vec<Stmt> {
    vec![expr_stmt(inner_call(method))]
}

fn limit_position_expr() -> Expr {
    property_access(this_expr(), "position")
}

fn limit_offset_expr() -> Expr {
    property_access(this_expr(), "offset")
}

fn limit_bound_expr() -> Expr {
    property_access(this_expr(), "limit")
}

fn limit_iterator_construct_body() -> Vec<Stmt> {
    vec![
        property_assign_stmt(this_expr(), "inner", var_expr("iterator")),
        property_assign_stmt(this_expr(), "offset", var_expr("offset")),
        property_assign_stmt(this_expr(), "limit", var_expr("limit")),
        property_assign_stmt(this_expr(), "position", int_expr(0)),
    ]
}

fn limit_rewind_body() -> Vec<Stmt> {
    vec![
        expr_stmt(inner_call("rewind")),
        property_assign_stmt(this_expr(), "position", int_expr(0)),
        while_stmt(
            binary_expr(limit_position_expr(), BinOp::Lt, limit_offset_expr()),
            vec![
                if_stmt(not_expr(inner_call("valid")), vec![return_void_stmt()], None),
                expr_stmt(inner_call("next")),
                property_assign_stmt(
                    this_expr(),
                    "position",
                    binary_expr(limit_position_expr(), BinOp::Add, int_expr(1)),
                ),
            ],
        ),
    ]
}

fn limit_next_body() -> Vec<Stmt> {
    vec![
        expr_stmt(inner_call("next")),
        property_assign_stmt(
            this_expr(),
            "position",
            binary_expr(limit_position_expr(), BinOp::Add, int_expr(1)),
        ),
    ]
}

fn limit_valid_body() -> Vec<Stmt> {
    vec![
        if_stmt(not_expr(inner_call("valid")), return_body(bool_expr(false)), None),
        if_stmt(
            binary_expr(limit_bound_expr(), BinOp::Lt, int_expr(0)),
            return_body(bool_expr(true)),
            None,
        ),
        return_stmt(binary_expr(
            binary_expr(limit_position_expr(), BinOp::Sub, limit_offset_expr()),
            BinOp::Lt,
            limit_bound_expr(),
        )),
    ]
}

fn limit_seek_body() -> Vec<Stmt> {
    vec![
        expr_stmt(method_call(this_expr(), "rewind", Vec::new())),
        while_stmt(
            binary_expr(limit_position_expr(), BinOp::Lt, var_expr("offset")),
            vec![
                if_stmt(
                    not_expr(method_call(this_expr(), "valid", Vec::new())),
                    vec![return_void_stmt()],
                    None,
                ),
                expr_stmt(method_call(this_expr(), "next", Vec::new())),
            ],
        ),
    ]
}

fn infinite_next_body() -> Vec<Stmt> {
    vec![
        expr_stmt(inner_call("next")),
        if_stmt(not_expr(inner_call("valid")), inner_void_body("rewind"), None),
    ]
}

fn append_iterators_expr() -> Expr {
    property_access(this_expr(), "iterators")
}

fn append_index_expr() -> Expr {
    property_access(this_expr(), "index")
}

fn append_current_iterator_expr() -> Expr {
    array_access(append_iterators_expr(), append_index_expr())
}

fn append_construct_body() -> Vec<Stmt> {
    vec![
        property_assign_stmt(this_expr(), "inner", new_object_expr("EmptyIterator", Vec::new())),
        property_assign_stmt(this_expr(), "iterators", empty_array_expr()),
        property_assign_stmt(this_expr(), "index", int_expr(0)),
    ]
}

fn append_append_body() -> Vec<Stmt> {
    vec![property_array_push_stmt(this_expr(), "iterators", var_expr("iterator"))]
}

fn append_rewind_body() -> Vec<Stmt> {
    vec![
        property_assign_stmt(this_expr(), "index", int_expr(0)),
        property_assign_stmt(this_expr(), "inner", new_object_expr("EmptyIterator", Vec::new())),
        if_stmt(
            binary_expr(count_expr(append_iterators_expr()), BinOp::StrictEq, int_expr(0)),
            vec![return_void_stmt()],
            None,
        ),
        typed_assign_stmt("iterator", named_type("Iterator"), append_current_iterator_expr()),
        property_assign_stmt(this_expr(), "inner", var_expr("iterator")),
        expr_stmt(method_call(var_expr("iterator"), "rewind", Vec::new())),
        expr_stmt(method_call(this_expr(), "valid", Vec::new())),
    ]
}

fn append_valid_body() -> Vec<Stmt> {
    vec![
        while_stmt(
            binary_expr(append_index_expr(), BinOp::Lt, count_expr(append_iterators_expr())),
            vec![
                typed_assign_stmt("iterator", named_type("Iterator"), append_current_iterator_expr()),
                if_stmt(
                    method_call(var_expr("iterator"), "valid", Vec::new()),
                    vec![
                        property_assign_stmt(this_expr(), "inner", var_expr("iterator")),
                        return_stmt(bool_expr(true)),
                    ],
                    None,
                ),
                property_assign_stmt(
                    this_expr(),
                    "index",
                    binary_expr(append_index_expr(), BinOp::Add, int_expr(1)),
                ),
                if_stmt(
                    binary_expr(append_index_expr(), BinOp::Lt, count_expr(append_iterators_expr())),
                    vec![
                        typed_assign_stmt("iterator", named_type("Iterator"), append_current_iterator_expr()),
                        property_assign_stmt(this_expr(), "inner", var_expr("iterator")),
                        expr_stmt(method_call(var_expr("iterator"), "rewind", Vec::new())),
                    ],
                    None,
                ),
            ],
        ),
        property_assign_stmt(this_expr(), "inner", new_object_expr("EmptyIterator", Vec::new())),
        return_stmt(bool_expr(false)),
    ]
}

fn append_current_body() -> Vec<Stmt> {
    vec![
        if_stmt(
            not_expr(method_call(this_expr(), "valid", Vec::new())),
            null_return_body(),
            None,
        ),
        return_stmt(inner_call("current")),
    ]
}

fn append_key_body() -> Vec<Stmt> {
    vec![
        if_stmt(
            not_expr(method_call(this_expr(), "valid", Vec::new())),
            null_return_body(),
            None,
        ),
        return_stmt(inner_call("key")),
    ]
}

fn append_next_body() -> Vec<Stmt> {
    vec![
        if_stmt(
            not_expr(method_call(this_expr(), "valid", Vec::new())),
            vec![return_void_stmt()],
            None,
        ),
        typed_assign_stmt("iterator", named_type("Iterator"), inner_expr()),
        expr_stmt(method_call(var_expr("iterator"), "next", Vec::new())),
        expr_stmt(method_call(this_expr(), "valid", Vec::new())),
    ]
}

fn append_get_inner_iterator_body() -> Vec<Stmt> {
    vec![
        if_stmt(
            not_expr(method_call(this_expr(), "valid", Vec::new())),
            null_return_body(),
            None,
        ),
        return_stmt(inner_expr()),
    ]
}

fn append_get_iterator_index_body() -> Vec<Stmt> {
    vec![
        if_stmt(
            not_expr(method_call(this_expr(), "valid", Vec::new())),
            null_return_body(),
            None,
        ),
        return_stmt(append_index_expr()),
    ]
}

fn append_get_array_iterator_body() -> Vec<Stmt> {
    return_body(new_object_expr("ArrayIterator", vec![append_iterators_expr()]))
}

fn multiple_iterators_expr() -> Expr {
    property_access(this_expr(), "iterators")
}

fn multiple_infos_expr() -> Expr {
    property_access(this_expr(), "infos")
}

fn multiple_flags_expr() -> Expr {
    property_access(this_expr(), "flags")
}

fn multiple_iterator_at(index: Expr) -> Expr {
    array_access(multiple_iterators_expr(), index)
}

fn multiple_info_at(index: Expr) -> Expr {
    array_access(multiple_infos_expr(), index)
}

fn multiple_need_all_expr() -> Expr {
    binary_expr(
        binary_expr(multiple_flags_expr(), BinOp::BitAnd, int_expr(1)),
        BinOp::NotEq,
        int_expr(0),
    )
}

fn multiple_assoc_keys_expr() -> Expr {
    binary_expr(
        binary_expr(multiple_flags_expr(), BinOp::BitAnd, int_expr(2)),
        BinOp::NotEq,
        int_expr(0),
    )
}

fn multiple_construct_body() -> Vec<Stmt> {
    vec![
        property_assign_stmt(this_expr(), "iterators", empty_array_expr()),
        property_assign_stmt(this_expr(), "infos", empty_array_expr()),
        property_assign_stmt(this_expr(), "flags", var_expr("flags")),
    ]
}

fn multiple_attach_iterator_body() -> Vec<Stmt> {
    vec![
        property_array_push_stmt(this_expr(), "iterators", var_expr("iterator")),
        property_array_push_stmt(this_expr(), "infos", var_expr("info")),
    ]
}

fn multiple_detach_iterator_body() -> Vec<Stmt> {
    vec![
        assign_stmt("newIterators", empty_array_expr()),
        assign_stmt("newInfos", empty_array_expr()),
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(multiple_iterators_expr())),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                assign_stmt("candidate", multiple_iterator_at(var_expr("i"))),
                if_stmt(
                    not_expr(binary_expr(var_expr("candidate"), BinOp::StrictEq, var_expr("iterator"))),
                    vec![
                        array_push_stmt("newIterators", var_expr("candidate")),
                        array_push_stmt("newInfos", multiple_info_at(var_expr("i"))),
                    ],
                    None,
                ),
                increment_stmt("i"),
            ],
        ),
        property_assign_stmt(this_expr(), "iterators", var_expr("newIterators")),
        property_assign_stmt(this_expr(), "infos", var_expr("newInfos")),
    ]
}

fn multiple_contains_iterator_body() -> Vec<Stmt> {
    vec![
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(multiple_iterators_expr())),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                if_stmt(
                    binary_expr(multiple_iterator_at(var_expr("i")), BinOp::StrictEq, var_expr("iterator")),
                    return_body(bool_expr(true)),
                    None,
                ),
                increment_stmt("i"),
            ],
        ),
        return_stmt(bool_expr(false)),
    ]
}

fn multiple_each_iterator_body(method: &str) -> Vec<Stmt> {
    vec![
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(multiple_iterators_expr())),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                typed_assign_stmt("iterator", named_type("Iterator"), multiple_iterator_at(var_expr("i"))),
                expr_stmt(method_call(var_expr("iterator"), method, Vec::new())),
                increment_stmt("i"),
            ],
        ),
    ]
}

fn multiple_rewind_body() -> Vec<Stmt> {
    multiple_each_iterator_body("rewind")
}

fn multiple_next_body() -> Vec<Stmt> {
    multiple_each_iterator_body("next")
}

fn multiple_valid_body() -> Vec<Stmt> {
    vec![
        if_stmt(
            binary_expr(count_expr(multiple_iterators_expr()), BinOp::StrictEq, int_expr(0)),
            return_body(bool_expr(false)),
            None,
        ),
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(multiple_iterators_expr())),
        if_stmt(
            multiple_need_all_expr(),
            vec![
                while_stmt(
                    binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
                    vec![
                        typed_assign_stmt("iterator", named_type("Iterator"), multiple_iterator_at(var_expr("i"))),
                        if_stmt(
                            not_expr(method_call(var_expr("iterator"), "valid", Vec::new())),
                            return_body(bool_expr(false)),
                            None,
                        ),
                        increment_stmt("i"),
                    ],
                ),
                return_stmt(bool_expr(true)),
            ],
            None,
        ),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                typed_assign_stmt("iterator", named_type("Iterator"), multiple_iterator_at(var_expr("i"))),
                if_stmt(
                    method_call(var_expr("iterator"), "valid", Vec::new()),
                    return_body(bool_expr(true)),
                    None,
                ),
                increment_stmt("i"),
            ],
        ),
        return_stmt(bool_expr(false)),
    ]
}

fn multiple_output_body(method: &str) -> Vec<Stmt> {
    vec![
        assign_stmt("out", empty_array_expr()),
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(multiple_iterators_expr())),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                typed_assign_stmt("iterator", named_type("Iterator"), multiple_iterator_at(var_expr("i"))),
                assign_stmt("info", multiple_info_at(var_expr("i"))),
                if_stmt(
                    multiple_assoc_keys_expr(),
                    vec![if_stmt(
                        function_call("is_null", vec![var_expr("info")]),
                        vec![throw_stmt(new_object_expr(
                            "InvalidArgumentException",
                            vec![string_expr("Sub-Iterator is associated with NULL")],
                        ))],
                        None,
                    )],
                    None,
                ),
                typed_assign_stmt("item", mixed_type(), null_expr()),
                if_stmt(
                    method_call(var_expr("iterator"), "valid", Vec::new()),
                    vec![assign_stmt("item", method_call(var_expr("iterator"), method, Vec::new()))],
                    None,
                ),
                if_stmt(
                    multiple_assoc_keys_expr(),
                    vec![array_assign_stmt("out", var_expr("info"), var_expr("item"))],
                    Some(vec![array_assign_stmt("out", var_expr("i"), var_expr("item"))]),
                ),
                increment_stmt("i"),
            ],
        ),
        return_stmt(var_expr("out")),
    ]
}

fn multiple_debug_info_body() -> Vec<Stmt> {
    return_body(expr(ExprKind::ArrayLiteralAssoc(vec![
        (string_expr("\0MultipleIterator\0iterators"), multiple_iterators_expr()),
        (string_expr("\0MultipleIterator\0infos"), multiple_infos_expr()),
        (string_expr("\0MultipleIterator\0flags"), multiple_flags_expr()),
    ])))
}

fn array_search_prelude() -> Vec<Stmt> {
    vec![
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", count_expr(keys_expr())),
    ]
}

fn dll_items_snapshot_prelude() -> Vec<Stmt> {
    vec![
        assign_stmt("items", expr(ExprKind::ArrayLiteral(Vec::new()))),
        assign_stmt("i", int_expr(0)),
        assign_stmt("limit", method_call(this_expr(), "count", Vec::new())),
        while_stmt(
            binary_expr(var_expr("i"), BinOp::Lt, var_expr("limit")),
            vec![
                array_push_stmt("items", method_call(this_expr(), "offsetGet", vec![var_expr("i")])),
                increment_stmt("i"),
            ],
        ),
    ]
}

fn dll_serialize_array_body() -> Vec<Stmt> {
    let mut body = dll_items_snapshot_prelude();
    body.push(return_stmt(expr(ExprKind::ArrayLiteral(vec![
        method_call(this_expr(), "getIteratorMode", Vec::new()),
        var_expr("items"),
        expr(ExprKind::ArrayLiteral(Vec::new())),
    ]))));
    body
}

fn dll_debug_info_body() -> Vec<Stmt> {
    let mut body = vec![
        assign_stmt("mode", method_call(this_expr(), "getIteratorMode", Vec::new())),
        expr_stmt(method_call(this_expr(), "setIteratorMode", vec![int_expr(0)])),
    ];
    body.extend(dll_items_snapshot_prelude());
    body.push(expr_stmt(method_call(
        this_expr(),
        "setIteratorMode",
        vec![var_expr("mode")],
    )));
    body.push(return_stmt(expr(ExprKind::ArrayLiteralAssoc(vec![
        (
            string_expr("\0SplDoublyLinkedList\0flags"),
            var_expr("mode"),
        ),
        (
            string_expr("\0SplDoublyLinkedList\0dllist"),
            var_expr("items"),
        ),
    ]))));
    body
}

fn dll_unserialize_body() -> Vec<Stmt> {
    vec![
        expr_stmt(method_call(
            this_expr(),
            "setIteratorMode",
            vec![array_access(var_expr("data"), int_expr(0))],
        )),
        while_stmt(
            not_expr(method_call(this_expr(), "isEmpty", Vec::new())),
            vec![expr_stmt(method_call(this_expr(), "pop", Vec::new()))],
        ),
        foreach_stmt(
            array_access(var_expr("data"), int_expr(1)),
            None,
            "value",
            vec![expr_stmt(method_call(this_expr(), "push", vec![var_expr("value")]))],
        ),
    ]
}
