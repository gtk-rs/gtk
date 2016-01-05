// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::slice;
use ffi;
use glib::translate::*;
use TreePath;

impl TreePath {
    pub fn get_indices(&self) -> Vec<i32> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::gtk_tree_path_get_indices_with_depth(mut_override(self.to_glib_none().0),
                &mut count);
            if ptr.is_null() {
                vec![]
            }
            else {
                slice::from_raw_parts(ptr, count as usize).to_owned()
            }
        }
    }
}
