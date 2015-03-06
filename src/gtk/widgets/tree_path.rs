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

extern crate libc;

use gtk::ffi;
use libc::{c_char};
use glib::translate::{FromGlibPtr, ToGlibPtr, ToTmp};

#[derive(Copy)]
pub struct TreePath {
    pointer:   *mut ffi::C_GtkTreePath
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
        let mut tmp_path = path.to_tmp_for_borrow();
        let tmp = unsafe {
            ffi::gtk_tree_path_new_from_string(tmp_path.to_glib_ptr())
        };

        if tmp.is_null() {
            None
        } else {
            Some(TreePath {
                pointer: tmp
            })
        }
    }

    pub fn new_from_indicesv(indices: &mut [i32]) -> Option<TreePath> {
        let tmp = unsafe { ffi::gtk_tree_path_new_from_indicesv(indices.as_mut_ptr(), indices.len() as ::libc::c_ulong) };

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
            FromGlibPtr::take(
                ffi::gtk_tree_path_to_string(self.pointer)
                    as *const c_char)
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
            Vec::from_raw_buf(tmp as *const i32, depth as usize)
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
        unsafe { ffi::gtk_tree_path_compare(self.pointer as *const ffi::C_GtkTreePath, other.pointer as *const ffi::C_GtkTreePath) }
    }

    pub fn next(&self) {
        unsafe { ffi::gtk_tree_path_next(self.pointer) }
    }

    pub fn prev(&self) {
        unsafe { ffi::gtk_tree_path_prev(self.pointer) }
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
        unsafe { ffi::gtk_tree_path_free(self.pointer as *mut ffi::C_GtkTreePath) }
        self.pointer = ::std::ptr::null_mut();
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkTreePath {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treepath: *mut ffi::C_GtkTreePath) -> TreePath {
        TreePath {
            pointer: c_treepath
        }
    }
}
