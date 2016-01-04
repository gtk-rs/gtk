// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use glib::{Type, Value};
use libc::{c_char, c_int};
use std::ptr;
use ListStore;
use TreeIter;

impl ListStore {
    pub fn new(column_types: &[Type]) -> ListStore {
        unsafe {
            let mut column_types = column_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
            from_glib_full(
                ffi::gtk_list_store_newv(column_types.len() as c_int,
                    column_types.as_mut_ptr()))
        }
    }

    pub fn reorder(&self, new_order: &[i32]) {
        unsafe {
            let count = ffi::gtk_tree_model_iter_n_children(self.to_glib_none().0, ptr::null_mut());
            let safe_count = count as usize == new_order.len();
            debug_assert!(safe_count,
                          "Incorrect `new_order` slice length. Expected `{}`, found `{}`.",
                          count,
                          new_order.len());
            let safe_values = new_order.iter()
                .max()
                .map(|&max| {
                    let max = max as i32;
                    max >= 0 && max < count
                })
                .unwrap_or(true);
            debug_assert!(safe_values,
                          "Some `new_order` slice values are out of range. Maximum safe value: \
                           `{}`. The slice contents: `{:?}`",
                          count - 1,
                          new_order);
            if safe_count && safe_values {
                ffi::gtk_list_store_reorder(self.to_glib_none().0, mut_override(new_order.as_ptr()));
            }
        }
    }

    pub fn set_column_types(&self, column_types: &[Type]) {
        unsafe {
            let mut column_types = column_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
            ffi::gtk_list_store_set_column_types(self.to_glib_none().0, column_types.len() as c_int,
                column_types.as_mut_ptr())
        }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        unsafe {
            let text: Stash<*const c_char, _> = text.to_glib_none();
            ffi::gtk_list_store_set(self.to_glib_none().0, mut_override(iter.to_glib_none().0),
                column, text.0, -1)
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &Value) {
        unsafe {
            ffi::gtk_list_store_set_value(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0), column, mut_override(value.as_ptr()));
        }
    }
}
