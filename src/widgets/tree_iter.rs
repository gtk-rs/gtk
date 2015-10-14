// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

glib_wrapper! {
    /// Text buffer iterator
    pub struct TreeIter(Boxed<ffi::GtkTreeIter>);

    impl TextIter {
        copy: gtk_tree_iter_copy,
        free: ffi::gtk_tree_iter_free,
    }
}

#[inline]
unsafe fn gtk_tree_iter_copy(ptr: *const ffi::GtkTreeIter) -> *mut ffi::GtkTreeIter {
    ffi::gtk_tree_iter_copy(ptr as *mut _) // pretend it's const
}
