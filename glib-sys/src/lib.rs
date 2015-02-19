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
#![feature(libc)]

extern crate libc;

use libc::{c_int, c_void, c_uint, c_char, c_ulong};

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

extern "C" {

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
    // GType constants
    //=========================================================================
    pub static g_type_invalid: GType;
    pub static g_type_none: GType;
    pub static g_type_interface: GType;
    pub static g_type_char: GType;
    pub static g_type_uchar: GType;
    pub static g_type_boolean: GType;
    pub static g_type_int: GType;
    pub static g_type_uint: GType;
    pub static g_type_long: GType;
    pub static g_type_ulong: GType;
    pub static g_type_int64: GType;
    pub static g_type_uint64: GType;
    pub static g_type_enum: GType;
    pub static g_type_flags: GType;
    pub static g_type_float: GType;
    pub static g_type_double: GType;
    pub static g_type_string: GType;
    pub static g_type_pointer: GType;
    pub static g_type_boxed: GType;
    pub static g_type_param: GType;
    pub static g_type_object: GType;
    pub static g_type_variant: GType;
    pub static g_type_reserved_glib_first: GType;
    pub static g_type_reserved_glib_last: GType;
    pub static g_type_reserved_bse_first: GType;
    pub static g_type_reserved_bse_last: GType;
    pub static g_type_reserved_user_first: GType;
}
