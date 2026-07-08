//! Purpose:
//! Synthesises the built-in reflection class checker metadata so user code can
//! receive `ReflectionAttribute` instances and query class/member attributes
//! through a small PHP-compatible Reflection surface.
//!
//! Called from:
//! - `crate::types::checker::driver::init` (alongside `inject_builtin_throwables`).
//!
//! Key details:
//! - Property and method bodies are dummies or simple private-slot accessors;
//!   runtime population is handled by codegen-only reflection constructors.

use std::collections::{HashMap, HashSet};

use crate::errors::CompileError;
use crate::names::php_symbol_key;
use crate::parser::ast::{
    BinOp, ClassMethod, ClassProperty, Expr, ExprKind, Stmt, StmtKind, TypeExpr, Visibility,
};
use crate::types::traits::FlattenedClass;
use crate::types::PhpType;

use super::super::Checker;

/// Injects the four built-in reflection types into `class_map` after verifying
/// none are already declared. Each type is a dummy shell; runtime population
/// happens in codegen. Returns an error if any reflection name is already in use.
pub(crate) fn inject_builtin_reflection(
    interface_map: &HashMap<String, super::InterfaceDeclInfo>,
    class_map: &mut HashMap<String, FlattenedClass>,
    trait_names: &HashSet<String>,
) -> Result<(), CompileError> {
    for builtin_name in [
        "ReflectionAttribute",
        "ReflectionClass",
        "ReflectionMethod",
        "ReflectionProperty",
        "ReflectionFunction",
        "ReflectionParameter",
        "ReflectionNamedType",
    ] {
        let builtin_key = php_symbol_key(builtin_name);
        if interface_map
            .keys()
            .chain(class_map.keys())
            .chain(trait_names.iter())
            .any(|name| php_symbol_key(name) == builtin_key)
        {
            return Err(CompileError::new(
                crate::span::Span::dummy(),
                &format!("Cannot redeclare built-in reflection type: {}", builtin_name),
            ));
        }
    }

    class_map.insert(
        "ReflectionAttribute".to_string(),
        FlattenedClass {
            name: "ReflectionAttribute".to_string(),
            extends: None,
            implements: Vec::new(),
            is_abstract: false,
            is_final: true,
            is_readonly_class: false,
            properties: vec![
                builtin_property("__name", Visibility::Private, Some(TypeExpr::Str), empty_string()),
                builtin_property("__args", Visibility::Private, Some(array_type()), empty_array()),
                builtin_property("__factory", Visibility::Private, Some(TypeExpr::Int), int_lit(0)),
            ],
            methods: vec![
                builtin_reflection_attribute_constructor_method(),
                builtin_reflection_attribute_get_name_method(),
                builtin_reflection_attribute_get_arguments_method(),
                builtin_reflection_attribute_new_instance_method(),
            ],
            attributes: Vec::new(),
            constants: Vec::new(),
            used_traits: Vec::new(),
        },
    );
    class_map.insert(
        "ReflectionClass".to_string(),
        builtin_reflection_class(),
    );
    class_map.insert(
        "ReflectionMethod".to_string(),
        builtin_reflection_method(),
    );
    class_map.insert(
        "ReflectionProperty".to_string(),
        builtin_reflection_owner_class(
            "ReflectionProperty",
            vec![
                ("class_name", Some(TypeExpr::Str), None, false),
                ("property_name", Some(TypeExpr::Str), None, false),
            ],
        ),
    );
    class_map.insert("ReflectionFunction".to_string(), builtin_reflection_function());
    class_map.insert(
        "ReflectionParameter".to_string(),
        builtin_reflection_parameter(),
    );
    class_map.insert(
        "ReflectionNamedType".to_string(),
        builtin_reflection_named_type(),
    );

    Ok(())
}

/// Builds a `ClassProperty` for a built-in reflection type with the given name,
/// visibility, optional type expression, and optional default value.
fn builtin_property(
    name: &str,
    visibility: Visibility,
    type_expr: Option<TypeExpr>,
    default: Option<Expr>,
) -> ClassProperty {
    ClassProperty {
        name: name.to_string(),
        visibility,
        set_visibility: None,
        type_expr,
        hooks: crate::parser::ast::PropertyHooks::none(),
        readonly: false,
        is_final: false,
        is_static: false,
        is_abstract: false,
        by_ref: false,
        default,
        span: crate::span::Span::dummy(),
        attributes: Vec::new(),
    }
}

/// Returns a `StringLiteral` expression with an empty string value.
fn empty_string() -> Option<Expr> {
    Some(Expr::new(
        ExprKind::StringLiteral(String::new()),
        crate::span::Span::dummy(),
    ))
}

/// Returns an `ArrayLiteral` expression with no elements.
fn empty_array() -> Option<Expr> {
    Some(Expr::new(
        ExprKind::ArrayLiteral(Vec::new()),
        crate::span::Span::dummy(),
    ))
}

/// Returns an `IntLiteral` expression with the given value.
fn int_lit(value: i64) -> Option<Expr> {
    Some(Expr::new(
        ExprKind::IntLiteral(value),
        crate::span::Span::dummy(),
    ))
}

/// Returns a `Null` literal expression.
fn null_lit() -> Option<Expr> {
    Some(Expr::new(ExprKind::Null, crate::span::Span::dummy()))
}

/// Returns a `BoolLiteral` expression with the given value.
fn bool_lit(value: bool) -> Option<Expr> {
    Some(Expr::new(
        ExprKind::BoolLiteral(value),
        crate::span::Span::dummy(),
    ))
}

/// Returns a `TypeExpr` for the unqualified name `array`.
fn array_type() -> TypeExpr {
    TypeExpr::Named(crate::names::Name::unqualified("array"))
}

/// Returns a `TypeExpr` for the unqualified name `mixed`.
fn mixed_type() -> TypeExpr {
    TypeExpr::Named(crate::names::Name::unqualified("mixed"))
}

/// Returns a private parameterless `__construct` method for `ReflectionAttribute`.
fn builtin_reflection_attribute_constructor_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "__construct".to_string(),
        visibility: Visibility::Private,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: None,
        by_ref_return: false,
        body: Vec::new(),
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `getName()` method that returns the private `__name` property
/// as a `Str`.
fn builtin_reflection_attribute_get_name_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "getName".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Str),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::PropertyAccess {
                    object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                    property: "__name".to_string(),
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `getArguments()` method that returns the private `__args`
/// property as an `array`.
fn builtin_reflection_attribute_get_arguments_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "getArguments".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Named(crate::names::Name::unqualified("array"))),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::PropertyAccess {
                    object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                    property: "__args".to_string(),
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `newInstance()` method that returns `null` (placeholder until
/// codegen supplies the real implementation).
fn builtin_reflection_attribute_new_instance_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "newInstance".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(mixed_type()),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(ExprKind::Null, dummy_span))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public no-op method that returns the private `property` slot typed
/// `return_type`. Reflection getters are populated at codegen; their bodies just
/// surface the corresponding private slot.
fn builtin_reflection_slot_getter(
    method_name: &str,
    property: &str,
    return_type: TypeExpr,
) -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: method_name.to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(return_type),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::PropertyAccess {
                    object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                    property: property.to_string(),
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns the public `__construct(string $name)` for `ReflectionFunction`. The
/// body is empty; codegen populates the metadata slots from the reflected
/// function's signature.
fn builtin_reflection_function_constructor_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "__construct".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: vec![("name".to_string(), Some(TypeExpr::Str), None, false)],
        variadic: None,
        variadic_type: None,
        return_type: None,
        by_ref_return: false,
        body: Vec::new(),
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Builds the `ReflectionFunction` shell with private name/short-name and
/// parameter-count slots plus public accessors. The slots are populated at
/// codegen from the reflected function's signature.
fn builtin_reflection_function() -> FlattenedClass {
    FlattenedClass {
        name: "ReflectionFunction".to_string(),
        extends: None,
        implements: Vec::new(),
        is_abstract: false,
        is_final: true,
        is_readonly_class: false,
        properties: vec![
            builtin_property("__name", Visibility::Private, Some(TypeExpr::Str), empty_string()),
            builtin_property("__short", Visibility::Private, Some(TypeExpr::Str), empty_string()),
            builtin_property("__num_params", Visibility::Private, Some(TypeExpr::Int), int_lit(0)),
            builtin_property(
                "__num_required",
                Visibility::Private,
                Some(TypeExpr::Int),
                int_lit(0),
            ),
            builtin_property("__params", Visibility::Private, Some(array_type()), empty_array()),
        ],
        methods: vec![
            builtin_reflection_function_constructor_method(),
            builtin_reflection_slot_getter("getName", "__name", TypeExpr::Str),
            builtin_reflection_slot_getter("getShortName", "__short", TypeExpr::Str),
            builtin_reflection_slot_getter("getNumberOfParameters", "__num_params", TypeExpr::Int),
            builtin_reflection_slot_getter(
                "getNumberOfRequiredParameters",
                "__num_required",
                TypeExpr::Int,
            ),
            builtin_reflection_slot_getter("getParameters", "__params", array_type()),
        ],
        attributes: Vec::new(),
        constants: Vec::new(),
        used_traits: Vec::new(),
    }
}

/// Builds the `ReflectionParameter` shell with private name/position/optional/
/// variadic slots and public accessors, populated at codegen from the reflected
/// function's signature.
fn builtin_reflection_parameter() -> FlattenedClass {
    FlattenedClass {
        name: "ReflectionParameter".to_string(),
        extends: None,
        implements: Vec::new(),
        is_abstract: false,
        is_final: true,
        is_readonly_class: false,
        properties: vec![
            builtin_property("__name", Visibility::Private, Some(TypeExpr::Str), empty_string()),
            builtin_property("__position", Visibility::Private, Some(TypeExpr::Int), int_lit(0)),
            builtin_property(
                "__optional",
                Visibility::Private,
                Some(TypeExpr::Bool),
                bool_lit(false),
            ),
            builtin_property(
                "__variadic",
                Visibility::Private,
                Some(TypeExpr::Bool),
                bool_lit(false),
            ),
            builtin_property("__has_type", Visibility::Private, Some(TypeExpr::Bool), bool_lit(false)),
            builtin_property("__type", Visibility::Private, Some(mixed_type()), null_lit()),
        ],
        methods: vec![
            builtin_reflection_slot_getter("getName", "__name", TypeExpr::Str),
            builtin_reflection_slot_getter("getPosition", "__position", TypeExpr::Int),
            builtin_reflection_slot_getter("isOptional", "__optional", TypeExpr::Bool),
            builtin_reflection_slot_getter("isVariadic", "__variadic", TypeExpr::Bool),
            builtin_reflection_slot_getter("hasType", "__has_type", TypeExpr::Bool),
            builtin_reflection_slot_getter("getType", "__type", mixed_type()),
        ],
        attributes: Vec::new(),
        constants: Vec::new(),
        used_traits: Vec::new(),
    }
}

/// Builds the `ReflectionNamedType` shell: a parameter/return type rendered as a
/// runtime object with a name, nullability flag, and builtin flag. Populated at
/// codegen from the declared type.
fn builtin_reflection_named_type() -> FlattenedClass {
    FlattenedClass {
        name: "ReflectionNamedType".to_string(),
        extends: None,
        implements: Vec::new(),
        is_abstract: false,
        is_final: true,
        is_readonly_class: false,
        properties: vec![
            builtin_property("__name", Visibility::Private, Some(TypeExpr::Str), empty_string()),
            builtin_property(
                "__allows_null",
                Visibility::Private,
                Some(TypeExpr::Bool),
                bool_lit(false),
            ),
            builtin_property(
                "__builtin",
                Visibility::Private,
                Some(TypeExpr::Bool),
                bool_lit(false),
            ),
        ],
        methods: vec![
            builtin_reflection_slot_getter("getName", "__name", TypeExpr::Str),
            builtin_reflection_slot_getter("allowsNull", "__allows_null", TypeExpr::Bool),
            builtin_reflection_slot_getter("isBuiltin", "__builtin", TypeExpr::Bool),
        ],
        attributes: Vec::new(),
        constants: Vec::new(),
        used_traits: Vec::new(),
    }
}

/// Builds the `ReflectionClass` shell with a private resolved-name slot,
/// private attribute array slot, public constructor, `getName()`, and
/// `getAttributes()`.
fn builtin_reflection_class() -> FlattenedClass {
    FlattenedClass {
        name: "ReflectionClass".to_string(),
        extends: None,
        implements: Vec::new(),
        is_abstract: false,
        is_final: true,
        is_readonly_class: false,
        properties: vec![
            builtin_property("__name", Visibility::Private, Some(TypeExpr::Str), empty_string()),
            builtin_property(
                "__attrs",
                Visibility::Private,
                Some(array_type()),
                empty_array(),
            ),
        ],
        methods: vec![
            builtin_reflection_owner_constructor_method(vec![(
                "class_name",
                Some(TypeExpr::Str),
                None,
                false,
            )]),
            builtin_reflection_class_get_name_method(),
            builtin_reflection_class_get_file_name_method(),
            builtin_reflection_class_is_abstract_method(),
            builtin_reflection_class_get_parent_class_method(),
            builtin_reflection_class_get_constructor_method(),
            builtin_reflection_class_get_method_method(),
            builtin_reflection_class_new_instance_without_constructor_method(),
            builtin_reflection_owner_get_attributes_method(),
        ],
        attributes: Vec::new(),
        constants: Vec::new(),
        used_traits: Vec::new(),
    }
}

/// Returns a public `ReflectionClass::getFileName()` method whose body looks up
/// the reflected class's declaring file through the `__elephc_class_file`
/// intrinsic (a `_classes_by_name` scan over the stamped `__ELEPHC_FILE__`
/// paths). Declared `Str` rather than PHP's `string|false`: every compiled
/// user class has a recorded file, and an unknown name yields the empty
/// string instead of `false`.
fn builtin_reflection_class_get_file_name_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "getFileName".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Str),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::FunctionCall {
                    name: crate::names::Name::from("__elephc_class_file"),
                    args: vec![Expr::new(
                        ExprKind::PropertyAccess {
                            object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                            property: "__name".to_string(),
                        },
                        dummy_span,
                    )],
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `ReflectionClass::isAbstract()` method whose body checks
/// the reflected class's abstract flag through the `__elephc_class_is_abstract`
/// intrinsic (a `_classes_by_name` scan indexing the parallel
/// `_class_is_abstract` flag table). An unknown class name yields `false`,
/// matching the intrinsic's 0-on-miss contract.
fn builtin_reflection_class_is_abstract_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "isAbstract".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Bool),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::BinaryOp {
                    left: Box::new(Expr::new(
                        ExprKind::FunctionCall {
                            name: crate::names::Name::from("__elephc_class_is_abstract"),
                            args: vec![Expr::new(
                                ExprKind::PropertyAccess {
                                    object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                                    property: "__name".to_string(),
                                },
                                dummy_span,
                            )],
                        },
                        dummy_span,
                    )),
                    op: BinOp::StrictEq,
                    right: Box::new(Expr::new(ExprKind::IntLiteral(1), dummy_span)),
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `ReflectionClass::getParentClass()` method whose body
/// resolves the parent-class name through the `__elephc_class_parent_name`
/// intrinsic (a `_classes_by_name` scan mapped through `_class_parent_ids`
/// and `_class_name_entries`) and constructs a `ReflectionClass` for it, or
/// returns `false` when the class is unknown or has no parent — matching
/// PHP's `ReflectionClass|false` contract.
fn builtin_reflection_class_get_parent_class_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    let parent_name_call = Expr::new(
        ExprKind::FunctionCall {
            name: crate::names::Name::from("__elephc_class_parent_name"),
            args: vec![Expr::new(
                ExprKind::PropertyAccess {
                    object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                    property: "__name".to_string(),
                },
                dummy_span,
            )],
        },
        dummy_span,
    );
    ClassMethod {
        name: "getParentClass".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Union(vec![
            TypeExpr::Named(crate::names::Name::from("ReflectionClass")),
            TypeExpr::Bool,
        ])),
        by_ref_return: false,
        body: vec![
            Stmt::new(
                StmtKind::Assign {
                    name: "parentName".to_string(),
                    value: parent_name_call,
                },
                dummy_span,
            ),
            Stmt::new(
                StmtKind::If {
                    condition: Expr::new(
                        ExprKind::BinaryOp {
                            left: Box::new(Expr::new(
                                ExprKind::Variable("parentName".to_string()),
                                dummy_span,
                            )),
                            op: BinOp::StrictEq,
                            right: Box::new(Expr::new(
                                ExprKind::StringLiteral(String::new()),
                                dummy_span,
                            )),
                        },
                        dummy_span,
                    ),
                    then_body: vec![Stmt::new(
                        StmtKind::Return(Some(Expr::new(
                            ExprKind::BoolLiteral(false),
                            dummy_span,
                        ))),
                        dummy_span,
                    )],
                    elseif_clauses: Vec::new(),
                    else_body: None,
                },
                dummy_span,
            ),
            Stmt::new(
                StmtKind::Return(Some(Expr::new(
                    ExprKind::NewObject {
                        class_name: crate::names::Name::from("ReflectionClass"),
                        args: vec![Expr::new(
                            ExprKind::Variable("parentName".to_string()),
                            dummy_span,
                        )],
                    },
                    dummy_span,
                ))),
                dummy_span,
            ),
        ],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `ReflectionClass::getConstructor()` method whose body
/// checks constructor presence through the `__elephc_class_has_constructor`
/// intrinsic (the `_classes_by_name` scan indexing the parallel
/// `_class_has_ctor` flag table) and constructs a `ReflectionMethod` bound to
/// `__construct`, or returns null when the class declares none — matching
/// PHP's `?ReflectionMethod` contract.
fn builtin_reflection_class_get_constructor_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    let has_ctor_call = Expr::new(
        ExprKind::FunctionCall {
            name: crate::names::Name::from("__elephc_class_has_constructor"),
            args: vec![Expr::new(
                ExprKind::PropertyAccess {
                    object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                    property: "__name".to_string(),
                },
                dummy_span,
            )],
        },
        dummy_span,
    );
    ClassMethod {
        name: "getConstructor".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Nullable(Box::new(TypeExpr::Named(
            crate::names::Name::from("ReflectionMethod"),
        )))),
        by_ref_return: false,
        body: vec![
            Stmt::new(
                StmtKind::If {
                    condition: Expr::new(
                        ExprKind::BinaryOp {
                            left: Box::new(has_ctor_call),
                            op: BinOp::StrictEq,
                            right: Box::new(Expr::new(ExprKind::IntLiteral(1), dummy_span)),
                        },
                        dummy_span,
                    ),
                    then_body: vec![Stmt::new(
                        StmtKind::Return(Some(Expr::new(
                            ExprKind::NewObject {
                                class_name: crate::names::Name::from("ReflectionMethod"),
                                args: vec![
                                    Expr::new(
                                        ExprKind::PropertyAccess {
                                            object: Box::new(Expr::new(
                                                ExprKind::This,
                                                dummy_span,
                                            )),
                                            property: "__name".to_string(),
                                        },
                                        dummy_span,
                                    ),
                                    Expr::new(
                                        ExprKind::StringLiteral("__construct".to_string()),
                                        dummy_span,
                                    ),
                                ],
                            },
                            dummy_span,
                        ))),
                        dummy_span,
                    )],
                    elseif_clauses: Vec::new(),
                    else_body: None,
                },
                dummy_span,
            ),
            Stmt::new(
                StmtKind::Return(Some(Expr::new(ExprKind::Null, dummy_span))),
                dummy_span,
            ),
        ],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `ReflectionClass::newInstanceWithoutConstructor()` method
/// whose body allocates through the `__elephc_new_without_ctor` intrinsic:
/// `__rt_new_by_name` zero-fills, stamps the class id, and runs the
/// property-default thunk WITHOUT invoking a constructor — PHP's exact
/// semantics for this method. Declared `mixed` (an unknown class yields null
/// instead of PHP's ReflectionException).
fn builtin_reflection_class_new_instance_without_constructor_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "newInstanceWithoutConstructor".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Named(crate::names::Name::from("mixed"))),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::FunctionCall {
                    name: crate::names::Name::from("__elephc_new_without_ctor"),
                    args: vec![Expr::new(
                        ExprKind::PropertyAccess {
                            object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                            property: "__name".to_string(),
                        },
                        dummy_span,
                    )],
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `ReflectionClass::getName()` method that returns the
/// resolved reflected class name from the private `__name` slot.
fn builtin_reflection_class_get_name_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "getName".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: Vec::new(),
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Str),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::PropertyAccess {
                    object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                    property: "__name".to_string(),
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Builds a `FlattenedClass` for `ReflectionMethod` or `ReflectionProperty`
/// with a private `__attrs` array property and two methods: `__construct`
/// (public, accepting the supplied params) and `getAttributes` (public,
/// returning the `__attrs` array).
fn builtin_reflection_owner_class(
    name: &str,
    constructor_params: Vec<(&str, Option<TypeExpr>, Option<Expr>, bool)>,
) -> FlattenedClass {
    FlattenedClass {
        name: name.to_string(),
        extends: None,
        implements: Vec::new(),
        is_abstract: false,
        is_final: true,
        is_readonly_class: false,
        properties: vec![builtin_property(
            "__attrs",
            Visibility::Private,
            Some(array_type()),
            empty_array(),
        )],
        methods: vec![
            builtin_reflection_owner_constructor_method(constructor_params),
            builtin_reflection_owner_get_attributes_method(),
        ],
        attributes: Vec::new(),
        constants: Vec::new(),
        used_traits: Vec::new(),
    }
}

/// Builds a public `__construct` method for a reflection owner class using the
/// provided parameter list: each tuple is (name, type_expr, default, by_ref).
fn builtin_reflection_owner_constructor_method(
    params: Vec<(&str, Option<TypeExpr>, Option<Expr>, bool)>,
) -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "__construct".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: params
            .into_iter()
            .map(|(name, ty, default, by_ref)| (name.to_string(), ty, default, by_ref))
            .collect(),
        variadic: None,
        variadic_type: None,
        return_type: None,
        by_ref_return: false,
        body: Vec::new(),
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `getAttributes(?string $name = null, int $flags = 0)`
/// method over the private `__attrs` array of `ReflectionAttribute` objects.
/// A non-null `$name` filters by exact attribute name (PHP's default flag
/// behavior); `$flags` is accepted for signature parity and ignored (the
/// synthesized attributes carry no inheritance hierarchy to match against).
fn builtin_reflection_owner_get_attributes_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    let this_attrs = Expr::new(
        ExprKind::PropertyAccess {
            object: Box::new(Expr::new(ExprKind::This, dummy_span)),
            property: "__attrs".to_string(),
        },
        dummy_span,
    );
    ClassMethod {
        name: "getAttributes".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: vec![
            (
                "name".to_string(),
                Some(TypeExpr::Nullable(Box::new(TypeExpr::Str))),
                Some(Expr::new(ExprKind::Null, dummy_span)),
                false,
            ),
            (
                "flags".to_string(),
                Some(TypeExpr::Int),
                Some(Expr::new(ExprKind::IntLiteral(0), dummy_span)),
                false,
            ),
        ],
        variadic: None,
        variadic_type: None,
        return_type: Some(array_type()),
        by_ref_return: false,
        body: vec![
            // if ($name === null) { return $this->__attrs; }
            Stmt::new(
                StmtKind::If {
                    condition: Expr::new(
                        ExprKind::BinaryOp {
                            left: Box::new(Expr::new(
                                ExprKind::Variable("name".to_string()),
                                dummy_span,
                            )),
                            op: crate::parser::ast::BinOp::StrictEq,
                            right: Box::new(Expr::new(ExprKind::Null, dummy_span)),
                        },
                        dummy_span,
                    ),
                    then_body: vec![Stmt::new(
                        StmtKind::Return(Some(this_attrs.clone())),
                        dummy_span,
                    )],
                    elseif_clauses: Vec::new(),
                    else_body: None,
                },
                dummy_span,
            ),
            // $out = [];
            Stmt::new(
                StmtKind::Assign {
                    name: "out".to_string(),
                    value: Expr::new(ExprKind::ArrayLiteral(Vec::new()), dummy_span),
                },
                dummy_span,
            ),
            // foreach ($this->__attrs as $a) { if ($a->getName() === $name) { $out[] = $a; } }
            Stmt::new(
                StmtKind::Foreach {
                    array: this_attrs,
                    key_var: None,
                    value_var: "a".to_string(),
                    value_by_ref: false,
                    body: vec![Stmt::new(
                        StmtKind::If {
                            condition: Expr::new(
                                ExprKind::BinaryOp {
                                    left: Box::new(Expr::new(
                                        ExprKind::MethodCall {
                                            object: Box::new(Expr::new(
                                                ExprKind::Variable("a".to_string()),
                                                dummy_span,
                                            )),
                                            method: "getName".to_string(),
                                            args: Vec::new(),
                                        },
                                        dummy_span,
                                    )),
                                    op: crate::parser::ast::BinOp::StrictEq,
                                    right: Box::new(Expr::new(
                                        ExprKind::Variable("name".to_string()),
                                        dummy_span,
                                    )),
                                },
                                dummy_span,
                            ),
                            then_body: vec![Stmt::new(
                                StmtKind::ArrayPush {
                                    array: "out".to_string(),
                                    value: Expr::new(
                                        ExprKind::Variable("a".to_string()),
                                        dummy_span,
                                    ),
                                },
                                dummy_span,
                            )],
                            elseif_clauses: Vec::new(),
                            else_body: None,
                        },
                        dummy_span,
                    )],
                },
                dummy_span,
            ),
            Stmt::new(
                StmtKind::Return(Some(Expr::new(
                    ExprKind::Variable("out".to_string()),
                    dummy_span,
                ))),
                dummy_span,
            ),
        ],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Builds the `ReflectionMethod` shell: private class/method-name strings, a
/// `__params` slot (ReflectionParameter objects, populated at codegen when the
/// reflected pair is compile-time resolvable), and the shared `__attrs` array.
fn builtin_reflection_method() -> FlattenedClass {
    FlattenedClass {
        name: "ReflectionMethod".to_string(),
        extends: None,
        implements: Vec::new(),
        is_abstract: false,
        is_final: true,
        is_readonly_class: false,
        properties: vec![
            builtin_property("__class", Visibility::Private, Some(TypeExpr::Str), empty_string()),
            builtin_property("__name", Visibility::Private, Some(TypeExpr::Str), empty_string()),
            builtin_property("__params", Visibility::Private, Some(array_type()), empty_array()),
            builtin_property(
                "__attrs",
                Visibility::Private,
                Some(array_type()),
                empty_array(),
            ),
        ],
        methods: vec![
            builtin_reflection_owner_constructor_method(vec![
                ("class_name", Some(TypeExpr::Str), None, false),
                ("method_name", Some(TypeExpr::Str), None, false),
            ]),
            builtin_reflection_slot_getter("getName", "__name", TypeExpr::Str),
            builtin_reflection_slot_getter("getDeclaringClassName", "__class", TypeExpr::Str),
            builtin_reflection_slot_getter("getParameters", "__params", array_type()),
            builtin_reflection_method_invoke_method(),
            builtin_reflection_owner_get_attributes_method(),
        ],
        attributes: Vec::new(),
        constants: Vec::new(),
        used_traits: Vec::new(),
    }
}

/// Returns `ReflectionMethod::invoke(?object $object, mixed ...$args)` whose
/// body dispatches through the runtime callable-array machinery:
/// `call_user_func_array([$object, $this->__name], $args)`.
fn builtin_reflection_method_invoke_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "invoke".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        // Declared `object`, not PHP's `?object`: the runtime callable-array
        // dispatch needs a concrete receiver, and the static-method null form
        // is out of scope for the synthesized surface.
        params: vec![(
            "object".to_string(),
            Some(TypeExpr::Named(crate::names::Name::unqualified("object"))),
            None,
            false,
        )],
        variadic: Some("args".to_string()),
        // Untyped: `callable_wrapper_sig` defaults the collected container to
        // `array<mixed>`, which is exactly PHP's `mixed ...$args` contract.
        variadic_type: None,
        return_type: Some(TypeExpr::Named(crate::names::Name::unqualified("mixed"))),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::FunctionCall {
                    name: crate::names::Name::from("call_user_func_array"),
                    args: vec![
                        Expr::new(
                            ExprKind::ArrayLiteral(vec![
                                Expr::new(ExprKind::Variable("object".to_string()), dummy_span),
                                Expr::new(
                                    ExprKind::PropertyAccess {
                                        object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                                        property: "__name".to_string(),
                                    },
                                    dummy_span,
                                ),
                            ]),
                            dummy_span,
                        ),
                        Expr::new(ExprKind::Variable("args".to_string()), dummy_span),
                    ],
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Returns a public `ReflectionClass::getMethod(string $name)` whose body
/// constructs a `ReflectionMethod` for the runtime class/method pair (the
/// dynamic-tolerant construction path stores the runtime strings; attribute
/// and parameter metadata stay empty for late-bound names).
fn builtin_reflection_class_get_method_method() -> ClassMethod {
    let dummy_span = crate::span::Span::dummy();
    ClassMethod {
        name: "getMethod".to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_final: false,
        has_body: true,
        params: vec![("name".to_string(), Some(TypeExpr::Str), None, false)],
        variadic: None,
        variadic_type: None,
        return_type: Some(TypeExpr::Named(crate::names::Name::from("ReflectionMethod"))),
        by_ref_return: false,
        body: vec![Stmt::new(
            StmtKind::Return(Some(Expr::new(
                ExprKind::NewObject {
                    class_name: crate::names::Name::from("ReflectionMethod"),
                    args: vec![
                        Expr::new(
                            ExprKind::PropertyAccess {
                                object: Box::new(Expr::new(ExprKind::This, dummy_span)),
                                property: "__name".to_string(),
                            },
                            dummy_span,
                        ),
                        Expr::new(ExprKind::Variable("name".to_string()), dummy_span),
                    ],
                },
                dummy_span,
            ))),
            dummy_span,
        )],
        span: dummy_span,
        attributes: Vec::new(),
    }
}

/// Overrides the return types on the synthesized reflection class methods inside
/// `checker` to match PHP's actual signatures:
/// - `__construct` → `void`
/// - `getName` / `getArguments` → `string` / `array`
/// - `newInstance` → `mixed`
/// - `getAttributes` → `array<ReflectionAttribute>`
pub(crate) fn patch_builtin_reflection_signatures(checker: &mut Checker) {
    if let Some(class_info) = checker.classes.get_mut("ReflectionAttribute") {
        if let Some(sig) = class_info.methods.get_mut("__construct") {
            sig.return_type = PhpType::Void;
        }
        if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("getName")) {
            sig.return_type = PhpType::Str;
        }
        if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("getArguments")) {
            // Attribute arguments can be keyed (named arguments / associative
            // arrays), so the result is an associative array of mixed values.
            sig.return_type = PhpType::AssocArray {
                key: Box::new(PhpType::Mixed),
                value: Box::new(PhpType::Mixed),
            };
        }
        if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("newInstance")) {
            sig.return_type = PhpType::Mixed;
        }
    }
    for class_name in ["ReflectionClass", "ReflectionMethod", "ReflectionProperty"] {
        if let Some(class_info) = checker.classes.get_mut(class_name) {
            if let Some(sig) = class_info.methods.get_mut("__construct") {
                sig.return_type = PhpType::Void;
            }
            if class_name == "ReflectionClass" {
                if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("getName")) {
                    sig.return_type = PhpType::Str;
                }
            }
            if class_name == "ReflectionMethod" {
                if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("getName")) {
                    sig.return_type = PhpType::Str;
                }
                if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("getParameters")) {
                    sig.return_type = PhpType::Array(Box::new(PhpType::Object(
                        "ReflectionParameter".to_string(),
                    )));
                }
            }
            if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("getAttributes")) {
                sig.return_type = PhpType::Array(Box::new(PhpType::Object(
                    "ReflectionAttribute".to_string(),
                )));
            }
        }
    }
    if let Some(class_info) = checker.classes.get_mut("ReflectionFunction") {
        if let Some(sig) = class_info.methods.get_mut("__construct") {
            sig.return_type = PhpType::Void;
        }
        if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("getParameters")) {
            sig.return_type = PhpType::Array(Box::new(PhpType::Object(
                "ReflectionParameter".to_string(),
            )));
        }
    }
    if let Some(class_info) = checker.classes.get_mut("ReflectionParameter") {
        if let Some(sig) = class_info.methods.get_mut(&php_symbol_key("getType")) {
            // ?ReflectionNamedType — null for untyped parameters.
            sig.return_type = PhpType::Union(vec![
                PhpType::Object("ReflectionNamedType".to_string()),
                PhpType::Void,
            ]);
        }
    }
}
