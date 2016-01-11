// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use std::ptr;
use TreePath;
use TreeView;
use TreeViewColumn;

impl TreeView {
    pub fn get_cursor(&self) -> (Option<TreePath>, Option<TreeViewColumn>) {
        unsafe {
            let mut path = ptr::null_mut();
            let mut focus_column = ptr::null_mut();
            ffi::gtk_tree_view_get_cursor(self.to_glib_none().0, &mut path, &mut focus_column);
            (from_glib_full(path), from_glib_none(focus_column))
        }
    }
}
