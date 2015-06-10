// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use std;

pub struct TreeIter {
    data: ffi::GtkTreeIter,
    pointer: *mut ffi::GtkTreeIter,
    is_owned: bool,
    is_true_pointer: bool
}

impl TreeIter {
    pub fn new() -> TreeIter {
        let mut t = TreeIter {
            data: ffi::GtkTreeIter {
                stamp: 0,
                user_data: std::ptr::null_mut(),
                user_data2: std::ptr::null_mut(),
                user_data3: std::ptr::null_mut()
            },
            pointer: ::std::ptr::null_mut(),
            is_owned: false,
            is_true_pointer: false
        };

        unsafe { t.pointer = std::mem::transmute(&mut t.data); }
        t
    }

    pub fn copy(&self) -> Option<TreeIter> {
        let tmp_pointer = unsafe { ffi::gtk_tree_iter_copy(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe {
                Some(TreeIter {
                    data: std::mem::uninitialized(),
                    pointer: tmp_pointer,
                    is_owned: true,
                    is_true_pointer: true
                })
            }
        }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkTreeIter {
        if self.is_true_pointer {
            self.pointer
        } else {
            unsafe { std::mem::transmute(&self.data) }
        }
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treeiter: *mut ffi::GtkTreeIter) -> TreeIter {
        unsafe {
            TreeIter {
                data: std::mem::uninitialized(),
                pointer: c_treeiter,
                is_owned: false,
                is_true_pointer: true
            }
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
