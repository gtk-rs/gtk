// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

pub struct TreeIter {
    pointer: *mut ffi::C_GtkTreeIter,
    is_owned: bool,
}

impl TreeIter {
    pub fn new() -> Option<TreeIter> {
        let tmp_pointer = unsafe { ffi::gtk_tree_iter_copy(::std::mem::uninitialized()) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreeIter {
                pointer: tmp_pointer,
                is_owned: true,
            })
        }
    }

    pub fn copy(&self) -> Option<TreeIter> {
        let tmp_pointer = unsafe { ffi::gtk_tree_iter_copy(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreeIter {
                pointer: tmp_pointer,
                is_owned: true,
            })
        }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkTreeIter {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treeiter: *mut ffi::C_GtkTreeIter) -> TreeIter {
        TreeIter {
            pointer: c_treeiter,
            is_owned: false,
        }
    }
}

impl Drop for TreeIter {
    fn drop(&mut self) {
        if !self.pointer.is_null() && self.is_owned {
            unsafe { ffi::gtk_tree_iter_free(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }
}
