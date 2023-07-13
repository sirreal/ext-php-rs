// This is a cheeky hack - since we need the list of allowed bindings in both
// the build script and in the CLI crate (in different formats), we define the
// `allowed_bindings.rs` file, which calls a macro called `bind` that doesn't
// exist in the bindings file. Which ever script include!s the bindings must
// define the `bind` macro. This allows us to have the list in string format
// inside the build script and in macro format inside the CLI crate.
//
// NOTE TO EDITORS:
//   When updating this file, you must re-generate the `docsrs_bindings.rs`
//   file used by docs.rs to build documentation. To perform this:
//
//     $ cargo clean
//     $ cargo build
//     $ cp target/debug/build/ext-php-rs-e2cb315d27898d01/out/bindings.rs
//       docsrs_bindings.rs
//     $ git add . && git commit -m "update docs.rs bindings"
//
//   The hash after `ext-php-rs-` in the bindings path may change. There should
//   be two folders beginning with `ext-php-rs-` in `target/debug/build`, so
//   check both for the presence of the bindings file.

bind! {
    HashTable,
    _Bucket,
    _call_user_function_impl,
    _efree,
    _emalloc,
    _zend_executor_globals,
    _zend_expected_type,
    _zend_expected_type_Z_EXPECTED_ARRAY,
    _zend_expected_type_Z_EXPECTED_BOOL,
    _zend_expected_type_Z_EXPECTED_DOUBLE,
    _zend_expected_type_Z_EXPECTED_LONG,
    _zend_expected_type_Z_EXPECTED_OBJECT,
    _zend_expected_type_Z_EXPECTED_RESOURCE,
    _zend_expected_type_Z_EXPECTED_STRING,
    _zend_new_array,
    _zval_struct__bindgen_ty_1,
    _zval_struct__bindgen_ty_2,
    // ext_php_rs_executor_globals,
    // ext_php_rs_php_build_id,
    // ext_php_rs_zend_object_alloc,
    // ext_php_rs_zend_object_release,
    // ext_php_rs_zend_string_init,
    // ext_php_rs_zend_string_release,
    // ext_php_rs_is_kown_valid_utf8,
    // ext_php_rs_set_kown_valid_utf8,
    object_properties_init,
    php_info_print_table_end,
    php_info_print_table_header,
    php_info_print_table_row,
    php_info_print_table_start,
    std_object_handlers,
    zend_array_destroy,
    zend_array_dup,
    zend_call_known_function,
    zend_ce_argument_count_error,
    zend_ce_arithmetic_error,
    zend_ce_compile_error,
    zend_ce_division_by_zero_error,
    zend_ce_error_exception,
    zend_ce_exception,
    zend_ce_parse_error,
    zend_ce_throwable,
    zend_ce_type_error,
    zend_ce_unhandled_match_error,
    zend_ce_value_error,
    zend_ce_traversable,
    zend_ce_aggregate,
    zend_ce_iterator,
    zend_ce_arrayaccess,
    zend_ce_serializable,
    zend_ce_countable,
    zend_ce_stringable,
    zend_class_entry,
    zend_declare_class_constant,
    zend_declare_property,
    zend_do_implement_interface,
    zend_execute_data,
    zend_function_entry,
    zend_hash_clean,
    zend_hash_index_del,
    zend_hash_index_find,
    zend_hash_index_update,
    zend_hash_next_index_insert,
    zend_hash_str_del,
    zend_hash_str_find,
    zend_hash_str_update,
    zend_internal_arg_info,
    zend_is_callable,
    zend_is_identical,
    zend_long,
    zend_lookup_class_ex,
    zend_module_entry,
    zend_object,
    zend_object_handlers,
    zend_object_std_init,
    zend_objects_clone_members,
    zend_register_bool_constant,
    zend_register_double_constant,
    zend_register_internal_class_ex,
    zend_register_long_constant,
    zend_register_string_constant,
    zend_resource,
    zend_string,
    zend_string_init_interned,
    zend_throw_exception_ex,
    zend_type,
    zend_value,
    zend_wrong_parameters_count_error,
    zval,
    CONST_CS,
    CONST_DEPRECATED,
    CONST_NO_FILE_CACHE,
    CONST_PERSISTENT,
    HT_MIN_SIZE,
    IS_ARRAY,
    IS_ARRAY_EX,
    IS_CALLABLE,
    IS_CONSTANT_AST,
    IS_CONSTANT_AST_EX,
    IS_DOUBLE,
    IS_FALSE,
    IS_INTERNED_STRING_EX,
    IS_LONG,
    IS_MIXED,
    IS_NULL,
    IS_OBJECT,
    IS_OBJECT_EX,
    IS_REFERENCE,
    IS_REFERENCE_EX,
    IS_RESOURCE,
    IS_RESOURCE_EX,
    IS_STRING,
    IS_STRING_EX,
    IS_TRUE,
    IS_TYPE_COLLECTABLE,
    IS_TYPE_REFCOUNTED,
    IS_UNDEF,
    IS_VOID,
    IS_PTR,
    MAY_BE_ANY,
    MAY_BE_BOOL,
    USING_ZTS,
    ZEND_ACC_ABSTRACT,
    ZEND_ACC_ANON_CLASS,
    ZEND_ACC_CALL_VIA_TRAMPOLINE,
    ZEND_ACC_CHANGED,
    ZEND_ACC_CLOSURE,
    ZEND_ACC_CONSTANTS_UPDATED,
    ZEND_ACC_CTOR,
    ZEND_ACC_DEPRECATED,
    ZEND_ACC_DONE_PASS_TWO,
    ZEND_ACC_EARLY_BINDING,
    ZEND_ACC_FAKE_CLOSURE,
    ZEND_ACC_FINAL,
    ZEND_ACC_GENERATOR,
    ZEND_ACC_HAS_FINALLY_BLOCK,
    ZEND_ACC_HAS_RETURN_TYPE,
    ZEND_ACC_HAS_TYPE_HINTS,
    ZEND_ACC_HAS_UNLINKED_USES,
    ZEND_ACC_HEAP_RT_CACHE,
    ZEND_ACC_IMMUTABLE,
    ZEND_ACC_IMPLICIT_ABSTRACT_CLASS,
    ZEND_ACC_INTERFACE,
    ZEND_ACC_LINKED,
    ZEND_ACC_NEARLY_LINKED,
    ZEND_ACC_NEVER_CACHE,
    ZEND_ACC_NO_DYNAMIC_PROPERTIES,
    ZEND_ACC_PRELOADED,
    ZEND_ACC_PRIVATE,
    ZEND_ACC_PROMOTED,
    ZEND_ACC_PROPERTY_TYPES_RESOLVED,
    ZEND_ACC_PROTECTED,
    ZEND_ACC_PUBLIC,
    ZEND_ACC_RESOLVED_INTERFACES,
    ZEND_ACC_RESOLVED_PARENT,
    ZEND_ACC_RETURN_REFERENCE,
    ZEND_ACC_REUSE_GET_ITERATOR,
    ZEND_ACC_STATIC,
    ZEND_ACC_STRICT_TYPES,
    ZEND_ACC_TOP_LEVEL,
    ZEND_ACC_TRAIT,
    ZEND_ACC_TRAIT_CLONE,
    ZEND_ACC_UNRESOLVED_VARIANCE,
    ZEND_ACC_USES_THIS,
    ZEND_ACC_USE_GUARDS,
    ZEND_ACC_VARIADIC,
    ZEND_DEBUG,
    ZEND_HAS_STATIC_IN_METHODS,
    ZEND_ISEMPTY,
    // ZEND_MM_ALIGNMENT,
    // ZEND_MM_ALIGNMENT_MASK,
    ZEND_MODULE_API_NO,
    ZEND_PROPERTY_EXISTS,
    ZEND_PROPERTY_ISSET,
    Z_TYPE_FLAGS_SHIFT,
    _IS_BOOL,
    _ZEND_IS_VARIADIC_BIT,
    _ZEND_SEND_MODE_SHIFT,
    _ZEND_TYPE_NULLABLE_BIT,
    ts_rsrc_id,
    _ZEND_TYPE_NAME_BIT,
    zval_ptr_dtor,
    zend_refcounted_h,
    zend_is_true,
    zend_object_std_dtor,
    zend_std_read_property,
    zend_std_write_property,
    zend_std_get_properties,
    zend_std_has_property,
    zend_objects_new,
    zend_standard_class_def,
    zend_class_serialize_deny,
    zend_class_unserialize_deny,
    zend_executor_globals,
    zend_objects_store_del,
    zend_hash_move_forward_ex,
    zend_hash_get_current_key_type_ex,
    zend_hash_get_current_key_zval_ex,
    zend_hash_get_current_data_ex,
    zend_hash_move_backwards_ex,
    zend_array_count,
    gc_possible_root,
    ZEND_ACC_NOT_SERIALIZABLE,
    executor_globals,
    php_core_globals,
    core_globals,
    sapi_globals_struct,
    sapi_globals,
    php_printf,
    __zend_malloc,
    tsrm_get_ls_cache,
    TRACK_VARS_POST,
    TRACK_VARS_GET,
    TRACK_VARS_COOKIE,
    TRACK_VARS_SERVER,
    TRACK_VARS_ENV,
    TRACK_VARS_FILES,
    TRACK_VARS_REQUEST,
    sapi_request_info,
    sapi_header_struct,
    zend_is_auto_global,
    zend_llist_get_next_ex,
    zend_llist_get_prev_ex
}
