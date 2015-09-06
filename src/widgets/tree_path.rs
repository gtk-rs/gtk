// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::slice;
use ffi;
use glib::translate::{from_glib_full, ToGlibPtr};

pub struct TreePath {
    pointer:   *mut ffi::GtkTreePath
}

impl TreePath {
    pub fn new() -> Option<TreePath> {
        let tmp = unsafe { ffi::gtk_tree_path_new() };

        if tmp.is_null() {
            None
        } else {
            Some(TreePath {
                pointer: tmp
            })
        }
    }

    pub fn new_from_string(path: &str) -> Option<TreePath> {
        let tmp = unsafe {
            ffi::gtk_tree_path_new_from_string(path.to_glib_none().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(TreePath {
                pointer: tmp
            })
        }
    }

    #[cfg(gtk_3_12)]
    pub fn new_from_indicesv(indices: &mut [i32]) -> Option<TreePath> {
        let tmp = unsafe { ffi::gtk_tree_path_new_from_indicesv(indices.as_mut_ptr(), indices.len() as ::libc::size_t) };

        if tmp.is_null() {
            None
        } else {
            Some(TreePath {
                pointer: tmp
            })
        }
    }

    pub fn new_first() -> Option<TreePath> {
        let tmp = unsafe { ffi::gtk_tree_path_new_first() };

        if tmp.is_null() {
            None
        } else {
            Some(TreePath {
                pointer: tmp
            })
        }
    }

    #[allow(unused_variables)]
    pub fn to_string(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_tree_path_to_string(self.pointer))
        }
    }

    pub fn append_index(&self, index_: i32) {
        unsafe { ffi::gtk_tree_path_append_index(self.pointer, index_) }
    }

    pub fn prepend_index(&self, index_: i32) {
        unsafe { ffi::gtk_tree_path_prepend_index(self.pointer, index_) }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe { ffi::gtk_tree_path_get_depth(self.pointer) }
    }

    pub fn get_indices(&self) -> Vec<i32> {
        let tmp = unsafe { ffi::gtk_tree_path_get_indices(self.pointer) };
        let depth = self.get_depth();

        unsafe {
            Vec::from(
                slice::from_raw_parts(tmp as *const i32, depth as usize))
        }
    }

    pub fn copy(&self) -> Option<TreePath> {
        let tmp = unsafe { ffi::gtk_tree_path_copy(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(TreePath {
                pointer: tmp
            })
        }
    }

    pub fn compare(&self, other: &TreePath) -> i32 {
        unsafe { ffi::gtk_tree_path_compare(self.pointer as *const ffi::GtkTreePath, other.pointer as *const ffi::GtkTreePath) }
    }

    pub fn next(&self) {
        unsafe { ffi::gtk_tree_path_next(self.pointer) }
    }

    pub fn prev(&self) {
        unsafe { ffi::gtk_tree_path_prev(self.pointer); }
    }

    pub fn path_up(&self) -> bool {
        match unsafe { ffi::gtk_tree_path_up(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn path_down(&self) {
        unsafe { ffi::gtk_tree_path_down(self.pointer) }
    }

    pub fn is_ancestor(&self, descendant: &TreePath) -> bool {
        match unsafe { ffi::gtk_tree_path_is_ancestor(self.pointer, descendant.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn is_descendant(&self, ancestor: &TreePath) -> bool {
        match unsafe { ffi::gtk_tree_path_is_descendant(self.pointer, ancestor.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn drop(&mut self) {
        unsafe { ffi::gtk_tree_path_free(self.pointer as *mut ffi::GtkTreePath) }
        self.pointer = ::std::ptr::null_mut();
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkTreePath {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treepath: *mut ffi::GtkTreePath) -> TreePath {
        TreePath {
            pointer: c_treepath
        }
    }
}
