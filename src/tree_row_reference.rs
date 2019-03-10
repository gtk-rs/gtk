// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib;
use glib::object::IsA;
use glib::translate::*;
use libc::c_int;
use gtk_sys;
use TreeIter;
use TreePath;
use TreeRowReference;

impl TreeRowReference {
    // This is unsafe because new_order bounds can't be checked.
    pub unsafe fn reordered<T: IsA<glib::Object>>(
        proxy: &T,
        path: &TreePath,
        iter: Option<&TreeIter>,
        new_order: &[u32],
    ) {
        assert_initialized_main_thread!();
        assert!(iter.is_some() || path.get_depth() == 0,
                "If 'iter' is None, 'path' must point to the root.");
        gtk_sys::gtk_tree_row_reference_reordered(proxy.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0),
            mut_override(iter.to_glib_none().0), mut_override(new_order.as_ptr() as *const c_int));
    }
}
