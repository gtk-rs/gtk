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

use libc::{c_int, c_void, c_uint, c_char};
use gtk::ffi;

pub type GQuark = u32;

pub struct C_GList {
  pub data: *c_void,
  pub next: *C_GList,
  pub prev: *C_GList
}

pub struct C_GSList {
  pub data: *c_void,
  pub next: *C_GSList
}

pub struct C_GError {
    pub domain : GQuark,
    pub code   : i32,
    pub message: *c_char
}

extern "C" {

    //=========================================================================
    // GSList
    //=========================================================================
    pub fn g_slist_free                    (list: *C_GSList);
    pub fn g_slist_append                  (list: *C_GSList, data: *c_void) -> *C_GSList;
    pub fn g_slist_prepend                 (list: *C_GSList, data: *c_void) -> *C_GSList;
    pub fn g_slist_insert                  (list: *C_GSList, data: *c_void, position: c_int) -> *C_GSList;
    pub fn g_slist_concat                  (list: *C_GSList, list2: *C_GSList) -> *C_GSList;
    pub fn g_slist_nth_data                (list: *C_GSList, n: c_uint) -> *c_void;
    pub fn g_slist_length                  (list: *C_GSList) -> c_uint;
    pub fn g_slist_last                    (list: *C_GSList) -> *C_GSList;
    pub fn g_slist_copy                    (list: *C_GSList) -> *C_GSList;
    pub fn g_slist_reverse                 (list: *C_GSList) -> *C_GSList;
    // pub fn g_slist_free_full               (list: *C_GSList, GDestroyNotify    free_func);
    // pub fn g_slist_free_1                  (list: *C_GSList);
    // pub fn g_slist_insert_sorted           (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    // pub fn g_slist_insert_sorted_with_data (list: *C_GSList, data: *c_void, GCompareDataFunc  func, gpointer          user_data) -> *C_GSList;
    // pub fn g_slist_insert_before           (list: *C_GSList, GSList           *sibling, gpointer          data) -> *C_GSList;
    pub fn g_slist_remove                  (list: *C_GSList, data: *c_void) -> *C_GSList;
    pub fn g_slist_remove_all              (list: *C_GSList, data: *c_void) -> *C_GSList;
    pub fn g_slist_remove_link             (list: *C_GSList, link_: C_GSList) -> *C_GSList;
    pub fn g_slist_delete_link             (list: *C_GSList, link_: C_GSList) -> *C_GSList;
    pub fn g_slist_find                    (list: *C_GSList, data: *c_void) -> *C_GSList;
    // pub fn g_slist_find_custom             (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    pub fn g_slist_position                (list: *C_GSList, link_: C_GSList) -> c_int;
    // pub fn g_slist_index                   (list: *C_GSList, data: *c_void) -> c_int;

    //=========================================================================
    // GList
    //=========================================================================
    pub fn g_list_free                    (list: *C_GList);
    pub fn g_list_append                  (list: *C_GList, data: *c_void) -> *C_GList;
    pub fn g_list_prepend                 (list: *C_GList, data: *c_void) -> *C_GList;
    pub fn g_list_insert                  (list: *C_GList, data: *c_void, position: c_int) -> *C_GList;
    pub fn g_list_concat                  (list: *C_GList, list2: *C_GList) -> *C_GList;
    pub fn g_list_nth_data                (list: *C_GList, n: c_uint) -> *c_void;
    pub fn g_list_length                  (list: *C_GList) -> c_uint;
    pub fn g_list_last                    (list: *C_GList) -> *C_GList;
    pub fn g_list_first                    (list: *C_GList) -> *C_GList;
    pub fn g_list_copy                    (list: *C_GList) -> *C_GList;
    pub fn g_list_reverse                 (list: *C_GList) -> *C_GList;
    // pub fn g_slist_free_full               (list: *C_GSList, GDestroyNotify    free_func);
    // pub fn g_slist_free_1                  (list: *C_GSList);
    // pub fn g_slist_insert_sorted           (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    // pub fn g_slist_insert_sorted_with_data (list: *C_GSList, data: *c_void, GCompareDataFunc  func, gpointer          user_data) -> *C_GSList;
    // pub fn g_slist_insert_before           (list: *C_GSList, GSList           *sibling, gpointer          data) -> *C_GSList;
    pub fn g_list_remove                  (list: *C_GList, data: *c_void) -> *C_GList;
    pub fn g_list_remove_all              (list: *C_GList, data: *c_void) -> *C_GList;
    pub fn g_list_remove_link             (list: *C_GList, link_: C_GList) -> *C_GList;
    pub fn g_list_delete_link             (list: *C_GList, link_: C_GList) -> *C_GList;
    pub fn g_list_find                    (list: *C_GList, data: *c_void) -> *C_GList;
    // pub fn g_slist_find_custom             (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    pub fn g_list_position                (list: *C_GList, link_: C_GList) -> c_int;
    // pub fn g_slist_index                   (list: *C_GSList, data: *c_void) -> c_int;

    //=========================================================================
    // GError
    //=========================================================================
    //pub fn g_error_new                    (domain: GQuark, code: c_int, format: *c_char, ...) -> *C_GError;
    pub fn g_error_new_literal            (domain: GQuark, code: c_int, message: *c_char) -> *C_GError;
    //pub fn g_error_new_valist             (domain: GQuark, code: c_int, fomat: *c_char, args: va_list) -> *C_GError;
    pub fn g_error_free                   (error: *C_GError) -> ();
    pub fn g_error_copy                   (error: *C_GError) -> *C_GError;
    pub fn g_error_matches                (error: *C_GError, domain: GQuark, code: c_int) -> ffi::Gboolean;
    //pub fn g_set_error                    (error: **C_GError, domain: GQuark, code: c_int, format: *c_char, ...) -> ();
    pub fn g_set_error_literal            (error: **C_GError, domain: GQuark, code: c_int, message: *c_char) -> ();
    pub fn g_propagate_error              (dest: **C_GError, src: *C_GError) -> ();
    pub fn g_clear_error                  (err: **C_GError) -> ();
    //pub fn g_prefix_error                 (err: **C_GError, format: *c_char, ...) -> ();
    //pub fn g_propagate_prefixed_error     (dest: **C_GError, src: *C_GError, format: *c_char, ...) -> ();
}