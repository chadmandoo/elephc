//! Purpose:
//! Implements the checker driver init phase.
//! Owns one ordered step in building checker state and validating the program before optimization/codegen.
//!
//! Called from:
//! - `crate::types::checker::driver::check_types_impl()`
//!
//! Key details:
//! - Phase order controls diagnostics, available declarations, required libraries, and function-local environments.

use std::collections::{HashMap, HashSet};

use crate::codegen::platform::Platform;
use crate::types::array_constants::ARRAY_INT_CONSTANTS;
use crate::types::date_constants::DATE_INT_CONSTANTS;
use crate::types::json_constants::JSON_INT_CONSTANTS;
use crate::types::stream_constants::STREAM_INT_CONSTANTS;
use crate::types::preg_constants::PREG_INT_CONSTANTS;
use crate::types::sort_constants::SORT_INT_CONSTANTS;
use crate::types::PhpType;

use super::super::Checker;

impl Checker {
    /// Constructs a new `Checker` with pre-populated builtin constants and empty declaration tables.
    ///
    /// Initializes the global constant map with PHP built-in constants (`PHP_OS`, pathinfo
    /// constants, `FNM_*` flags, `STDIN`/`STDOUT`/`STDERR` stream resources, `LOCK_*` constants),
    /// array constants, JSON integer constants, and preg flag constants. All other tables (function declarations,
    /// classes, interfaces, enums, etc.) are initialized empty.
    ///
    /// # Arguments
    /// * `target_platform` - The compilation target platform, stored for use in platform-specific
    ///   type checks and library requirements.
    ///
    /// # Returns
    /// A `Checker` instance ready for the program to be loaded into.
    pub(super) fn new(target_platform: Platform) -> Self {
        let mut constants = HashMap::new();
        constants.insert("PHP_OS".to_string(), PhpType::Str);
        constants.insert("PATHINFO_DIRNAME".to_string(), PhpType::Int);
        constants.insert("PATHINFO_BASENAME".to_string(), PhpType::Int);
        constants.insert("PATHINFO_EXTENSION".to_string(), PhpType::Int);
        constants.insert("PATHINFO_FILENAME".to_string(), PhpType::Int);
        constants.insert("PATHINFO_ALL".to_string(), PhpType::Int);
        for ent in [
            "ENT_QUOTES",
            "ENT_COMPAT",
            "ENT_NOQUOTES",
            "ENT_HTML401",
            "ENT_HTML5",
            "ENT_XHTML",
            "ENT_XML1",
            "ENT_SUBSTITUTE",
            "ENT_IGNORE",
        ] {
            constants.insert(ent.to_string(), PhpType::Int);
        }
        for upload_or_filter in [
            "UPLOAD_ERR_OK",
            "UPLOAD_ERR_INI_SIZE",
            "UPLOAD_ERR_FORM_SIZE",
            "UPLOAD_ERR_PARTIAL",
            "UPLOAD_ERR_NO_FILE",
            "UPLOAD_ERR_NO_TMP_DIR",
            "UPLOAD_ERR_CANT_WRITE",
            "UPLOAD_ERR_EXTENSION",
            "FILTER_VALIDATE_INT",
            "FILTER_VALIDATE_BOOLEAN",
            "FILTER_VALIDATE_BOOL",
            "FILTER_VALIDATE_FLOAT",
            "FILTER_VALIDATE_REGEXP",
            "FILTER_VALIDATE_DOMAIN",
            "FILTER_VALIDATE_URL",
            "FILTER_VALIDATE_EMAIL",
            "FILTER_VALIDATE_IP",
            "FILTER_VALIDATE_MAC",
            "FILTER_DEFAULT",
            "PHP_URL_SCHEME",
            "PHP_URL_HOST",
            "PHP_URL_PORT",
            "PHP_URL_USER",
            "PHP_URL_PASS",
            "PHP_URL_PATH",
            "PHP_URL_QUERY",
            "PHP_URL_FRAGMENT",
        ] {
            constants.insert(upload_or_filter.to_string(), PhpType::Int);
        }
        constants.insert("FNM_NOESCAPE".to_string(), PhpType::Int);
        constants.insert("FNM_PATHNAME".to_string(), PhpType::Int);
        constants.insert("FNM_PERIOD".to_string(), PhpType::Int);
        constants.insert("FNM_CASEFOLD".to_string(), PhpType::Int);
        constants.insert("STDIN".to_string(), PhpType::stream_resource());
        constants.insert("STDOUT".to_string(), PhpType::stream_resource());
        constants.insert("STDERR".to_string(), PhpType::stream_resource());
        constants.insert("LOCK_SH".to_string(), PhpType::Int);
        constants.insert("LOCK_EX".to_string(), PhpType::Int);
        constants.insert("LOCK_UN".to_string(), PhpType::Int);
        constants.insert("LOCK_NB".to_string(), PhpType::Int);
        for (name, _value) in ARRAY_INT_CONSTANTS {
            constants.insert((*name).to_string(), PhpType::Int);
        }
        for (name, _value) in JSON_INT_CONSTANTS {
            constants.insert((*name).to_string(), PhpType::Int);
        }
        for (name, _value) in STREAM_INT_CONSTANTS {
            constants.insert((*name).to_string(), PhpType::Int);
        }
        for (name, _value) in PREG_INT_CONSTANTS {
            constants.insert((*name).to_string(), PhpType::Int);
        }
        for (name, _value) in DATE_INT_CONSTANTS {
            constants.insert((*name).to_string(), PhpType::Int);
        }
        for (name, _value) in SORT_INT_CONSTANTS {
            constants.insert((*name).to_string(), PhpType::Int);
        }
        // password_hash() algorithm identifiers (PHP 7.4+ string values).
        for password_algo in [
            "PASSWORD_DEFAULT",
            "PASSWORD_BCRYPT",
            "PASSWORD_ARGON2I",
            "PASSWORD_ARGON2ID",
        ] {
            constants.insert(password_algo.to_string(), PhpType::Str);
        }

        Self {
            target_platform,
            fn_decls: HashMap::new(),
            function_variant_groups: HashMap::new(),
            functions: HashMap::new(),
            resolving_functions: HashSet::new(),
            constants,
            closure_return_types: HashMap::new(),
            callable_sigs: HashMap::new(),
            callable_param_names: HashSet::new(),
            callable_param_sigs: HashMap::new(),
            param_specialization_seen: HashSet::new(),
            callable_return_sigs: HashMap::new(),
            callable_array_return_sigs: HashMap::new(),
            callable_captures: HashMap::new(),
            callable_array_targets: HashMap::new(),
            first_class_callable_targets: HashMap::new(),
            interfaces: HashMap::new(),
            classes: HashMap::new(),
            static_return_methods: HashSet::new(),
            declared_classes: HashSet::new(),
            enums: HashMap::new(),
            declared_interfaces: HashSet::new(),
            current_class: None,
            current_method: None,
            current_method_is_static: false,
            current_by_ref_return: false,
            closure_depth: 0,
            extern_functions: HashMap::new(),
            extern_classes: HashMap::new(),
            packed_classes: HashMap::new(),
            extern_globals: HashMap::new(),
            required_libraries: Vec::new(),
            top_level_env: HashMap::new(),
            active_ref_params: HashSet::new(),
            active_globals: HashSet::new(),
            active_statics: HashSet::new(),
            foreach_key_locals: HashSet::new(),
            break_continue_depth: 0,
            finally_break_continue_bases: Vec::new(),
            warnings: Vec::new(),
            reference_property_promotions: HashSet::new(),
            throw_access_sites: HashMap::new(),
        }
    }
}
