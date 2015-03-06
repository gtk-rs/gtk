// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_void, c_int, c_uint, c_float, c_double, c_char, c_uchar, c_long, c_ulong};

pub type GQuark = u32;

pub type GType = c_ulong;

pub type Gboolean = c_int;
pub const GFALSE:  c_int = 0;
pub const GTRUE:   c_int = 1;

pub type gpointer = *const c_void;

#[repr(C)]
#[derive(Copy)]
pub struct C_GList {
  pub data: *mut c_void,
  pub next: *mut C_GList,
  pub prev: *mut C_GList
}

#[repr(C)]
#[derive(Copy)]
pub struct C_GSList {
  pub data: *mut c_void,
  pub next: *mut C_GSList
}

#[repr(C)]
#[derive(Copy)]
pub struct C_GError {
    pub domain : GQuark,
    pub code   : i32,
    pub message: *mut c_char
}

#[repr(C)]
#[derive(Copy)]
pub struct C_GPermission;

#[repr(C)]
#[derive(Copy)]
pub struct C_GObject;

#[repr(C)]
#[derive(Copy)]
pub struct C_GValue;

//=========================================================================
// GType constants
//=========================================================================

pub const G_TYPE_FUNDAMENTAL_SHIFT: u8 = 2;
pub const G_TYPE_INVALID: GType = 0 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_NONE: GType = 1 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_INTERFACE: GType = 2 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_CHAR: GType = 3 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_UCHAR: GType = 4 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_BOOLEAN: GType = 5 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_INT: GType = 6 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_UINT: GType = 7 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_LONG: GType = 8 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_ULONG: GType = 9 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_INT64: GType = 10 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_UINT64: GType = 11 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_ENUM: GType = 12 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_FLAGS: GType = 13 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_FLOAT: GType = 14 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_DOUBLE: GType = 15 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_STRING: GType = 16 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_POINTER: GType = 17 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_BOXED: GType = 18 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_PARAM: GType = 19 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_OBJECT: GType = 20 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_VARIANT: GType = 21 << G_TYPE_FUNDAMENTAL_SHIFT;

extern "C" {

    pub fn g_free                          (ptr: gpointer);
    //=========================================================================
    // GSList
    //=========================================================================
    pub fn g_slist_free                    (list: *mut C_GSList);
    pub fn g_slist_append                  (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    pub fn g_slist_prepend                 (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    pub fn g_slist_insert                  (list: *mut C_GSList, data: *mut c_void, position: c_int) -> *mut C_GSList;
    pub fn g_slist_concat                  (list: *mut C_GSList, list2: *mut C_GSList) -> *mut C_GSList;
    pub fn g_slist_nth_data                (list: *mut C_GSList, n: c_uint) -> *mut c_void;
    pub fn g_slist_length                  (list: *mut C_GSList) -> c_uint;
    pub fn g_slist_last                    (list: *mut C_GSList) -> *mut C_GSList;
    pub fn g_slist_copy                    (list: *mut C_GSList) -> *mut C_GSList;
    pub fn g_slist_reverse                 (list: *mut C_GSList) -> *mut C_GSList;
    // pub fn g_slist_free_full               (list: *C_GSList, GDestroyNotify    free_func);
    // pub fn g_slist_free_1                  (list: *C_GSList);
    // pub fn g_slist_insert_sorted           (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    // pub fn g_slist_insert_sorted_with_data (list: *C_GSList, data: *c_void, GCompareDataFunc  func, gpointer          user_data) -> *C_GSList;
    // pub fn g_slist_insert_before           (list: *C_GSList, GSList           *sibling, gpointer          data) -> *C_GSList;
    pub fn g_slist_remove                  (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    pub fn g_slist_remove_all              (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    pub fn g_slist_remove_link             (list: *mut C_GSList, link_: C_GSList) -> *mut C_GSList;
    pub fn g_slist_delete_link             (list: *mut C_GSList, link_: C_GSList) -> *mut C_GSList;
    pub fn g_slist_find                    (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    // pub fn g_slist_find_custom             (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    pub fn g_slist_position                (list: *mut C_GSList, link_: C_GSList) -> c_int;
    // pub fn g_slist_index                   (list: *C_GSList, data: *c_void) -> c_int;

    //=========================================================================
    // GList
    //=========================================================================
    pub fn g_list_free                    (list: *mut C_GList);
    pub fn g_list_append                  (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    pub fn g_list_prepend                 (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    pub fn g_list_insert                  (list: *mut C_GList, data: *mut c_void, position: c_int) -> *mut C_GList;
    pub fn g_list_concat                  (list: *mut C_GList, list2: *mut C_GList) -> *mut C_GList;
    pub fn g_list_nth_data                (list: *mut C_GList, n: c_uint) -> *mut c_void;
    pub fn g_list_length                  (list: *mut C_GList) -> c_uint;
    pub fn g_list_last                    (list: *mut C_GList) -> *mut C_GList;
    pub fn g_list_first                    (list: *mut C_GList) -> *mut C_GList;
    pub fn g_list_copy                    (list: *mut C_GList) -> *mut C_GList;
    pub fn g_list_reverse                 (list: *mut C_GList) -> *mut C_GList;
    // pub fn g_slist_free_full               (list: *C_GSList, GDestroyNotify    free_func);
    // pub fn g_slist_free_1                  (list: *C_GSList);
    // pub fn g_slist_insert_sorted           (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    // pub fn g_slist_insert_sorted_with_data (list: *C_GSList, data: *c_void, GCompareDataFunc  func, gpointer          user_data) -> *C_GSList;
    // pub fn g_slist_insert_before           (list: *C_GSList, GSList           *sibling, gpointer          data) -> *C_GSList;
    pub fn g_list_remove                  (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    pub fn g_list_remove_all              (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    pub fn g_list_remove_link             (list: *mut C_GList, link_: C_GList) -> *mut C_GList;
    pub fn g_list_delete_link             (list: *mut C_GList, link_: C_GList) -> *mut C_GList;
    pub fn g_list_find                    (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    // pub fn g_slist_find_custom             (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    pub fn g_list_position                (list: *mut C_GList, link_: C_GList) -> c_int;
    // pub fn g_slist_index                   (list: *C_GSList, data: *c_void) -> c_int;



    //=========================================================================
    // GError
    //=========================================================================
    //pub fn g_error_new                    (domain: GQuark, code: c_int, format: *c_char, ...) -> *C_GError;
    pub fn g_error_new_literal            (domain: GQuark, code: c_int, message: *const c_char) -> *mut C_GError;
    //pub fn g_error_new_valist             (domain: GQuark, code: c_int, fomat: *c_char, args: va_list) -> *C_GError;
    pub fn g_error_free                   (error: *mut C_GError) -> ();
    pub fn g_error_copy                   (error: *mut C_GError) -> *mut C_GError;
    pub fn g_error_matches                (error: *mut C_GError, domain: GQuark, code: c_int) -> Gboolean;
    //pub fn g_set_error                    (error: **C_GError, domain: GQuark, code: c_int, format: *c_char, ...) -> ();
    pub fn g_set_error_literal            (error: *mut *mut C_GError, domain: GQuark, code: c_int, message: *const c_char) -> ();
    pub fn g_propagate_error              (dest: *mut *mut C_GError, src: *mut C_GError) -> ();
    pub fn g_clear_error                  (err: *mut *mut C_GError) -> ();
    //pub fn g_prefix_error                 (err: **C_GError, format: *c_char, ...) -> ();
    //pub fn g_propagate_prefixed_error     (dest: **C_GError, src: *C_GError, format: *c_char, ...) -> ();

    //=========================================================================
    // GPermission                                                       NOT OK
    //=========================================================================
    pub fn g_permission_get_allowed     (permission: *mut C_GPermission) -> Gboolean;
    pub fn g_permission_get_can_acquire (permission: *mut C_GPermission) -> Gboolean;
    pub fn g_permission_get_can_release (permission: *mut C_GPermission) -> Gboolean;
    //pub fn g_permission_acquire         (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    error: *mut *mut C_GError) -> Gboolean;
    //pub fn g_permission_acquire_async   (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    callback: GAsyncReadyCallback, user_data: gpointer);
    //pub fn g_permission_acquire_finish  (permission: *mut C_GPermission, result: *mut C_GAsyncResult,
    //    error: *mut *mut C_GError) -> Gboolean;
    //pub fn g_permission_release         (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    error: *mut *mut C_GError) -> Gboolean;
    //pub fn g_permission_release_async   (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    callback: GAsyncReadyCallback, user_data: gpointer);
    //pub fn g_permission_release_finish  (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    error: *mut *mut C_GError) -> Gboolean;
    pub fn g_permission_impl_update     (permission: *mut C_GPermission, allowed: Gboolean, can_acquire: Gboolean, can_release: Gboolean);

    //pub type GAsyncReadyCallback = Option<extern "C" fn(source_object: *mut C_GObject, res: *mut C_GAsyncResult, user_data: gpointer)>;

    //=========================================================================
    // GObject
    //=========================================================================
    pub fn g_object_ref(object: *mut C_GObject) -> *mut C_GObject;
    pub fn g_object_unref(object: *mut C_GObject);

    pub fn glue_signal_connect(g_object: *mut C_GObject,
                               signal: *const c_char,
                               func: Option<extern "C" fn()>,
                               user_data: *const c_void);

    //=========================================================================
    // GValue
    //=========================================================================
    pub fn create_gvalue                       () -> *mut C_GValue;
    pub fn get_gtype                           (_type: GType) -> GType;
    pub fn g_value_init                        (value: *mut C_GValue, _type: GType);
    pub fn g_value_reset                       (value: *mut C_GValue);
    pub fn g_value_unset                       (value: *mut C_GValue);
    pub fn g_strdup_value_contents             (value: *mut C_GValue) -> *mut c_char;
    pub fn g_value_set_boolean                 (value: *mut C_GValue, b: Gboolean);
    pub fn g_value_get_boolean                 (value: *mut C_GValue) -> Gboolean;
    pub fn g_value_set_schar                   (value: *mut C_GValue, b: c_char);
    pub fn g_value_get_schar                   (value: *mut C_GValue) -> c_char;
    pub fn g_value_set_uchar                   (value: *mut C_GValue, b: c_uchar);
    pub fn g_value_get_uchar                   (value: *mut C_GValue) -> c_uchar;
    pub fn g_value_set_int                     (value: *mut C_GValue, b: c_int);
    pub fn g_value_get_int                     (value: *mut C_GValue) -> c_int;
    pub fn g_value_set_uint                    (value: *mut C_GValue, b: c_uint);
    pub fn g_value_get_uint                    (value: *mut C_GValue) -> c_uint;
    pub fn g_value_set_long                    (value: *mut C_GValue, b: c_long);
    pub fn g_value_get_long                    (value: *mut C_GValue) -> c_long;
    pub fn g_value_set_ulong                   (value: *mut C_GValue, b: c_ulong);
    pub fn g_value_get_ulong                   (value: *mut C_GValue) -> c_ulong;
    pub fn g_value_set_int64                   (value: *mut C_GValue, b: i64);
    pub fn g_value_get_int64                   (value: *mut C_GValue) -> i64;
    pub fn g_value_set_uint64                  (value: *mut C_GValue, b: u64);
    pub fn g_value_get_uint64                  (value: *mut C_GValue) -> u64;
    pub fn g_value_set_float                   (value: *mut C_GValue, b: c_float);
    pub fn g_value_get_float                   (value: *mut C_GValue) -> c_float;
    pub fn g_value_set_double                  (value: *mut C_GValue, b: c_double);
    pub fn g_value_get_double                  (value: *mut C_GValue) -> c_double;
    pub fn g_value_set_enum                    (value: *mut C_GValue, b: GType);
    pub fn g_value_get_enum                    (value: *mut C_GValue) -> GType;
    pub fn g_value_set_flags                   (value: *mut C_GValue, b: GType);
    pub fn g_value_get_flags                   (value: *mut C_GValue) -> GType;
    pub fn g_value_set_string                  (value: *mut C_GValue, b: *const c_char);
    pub fn g_value_set_static_string           (value: *mut C_GValue, b: *const c_char);
    pub fn g_value_get_string                  (value: *mut C_GValue) -> *const c_char;
    pub fn g_value_dup_string                  (value: *mut C_GValue) -> *mut c_char;
    pub fn g_value_set_boxed                   (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_set_static_boxed            (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_get_boxed                   (value: *mut C_GValue) -> *const c_void;
    pub fn g_value_set_pointer                 (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_get_pointer                 (value: *mut C_GValue) -> *const c_void;
    pub fn g_value_set_object                  (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_take_object                 (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_get_object                  (value: *mut C_GValue) -> *const c_void;
    pub fn g_value_set_gtype                   (value: *mut C_GValue, b: GType);
    pub fn g_value_get_gtype                   (value: *mut C_GValue) -> GType;
    pub fn g_value_type_compatible             (src_type: GType, dest_type: GType) -> Gboolean;
    pub fn g_value_type_transformable          (src_type: GType, dest_type: GType) -> Gboolean;
}
