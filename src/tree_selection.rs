// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use std::ptr;
use TreeIter;
use TreeModel;
use TreeSelection;

impl TreeSelection {
    pub fn get_selected(&self) -> Option<(TreeModel, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_selection_get_selected(self.to_glib_none().0,
                &mut model, iter.to_glib_none_mut().0);
            some_if(ok, || (from_glib_none(model), iter))
        }
    }
}
