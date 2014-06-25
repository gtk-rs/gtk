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

use libc::{c_int, c_void, c_uint};

pub struct C_GList;

pub struct C_GSList {
  pub data: *c_void,
  next: *C_GSList
}



extern "C" {

    //=========================================================================
    // GList
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

}