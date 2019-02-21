// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::object::{Cast, IsA};
use glib::translate::*;

use {TreeModel, TreeModelFilter, TreePath};

impl TreeModelFilter {
    pub fn new<'a, I: Into<Option<&'a TreePath>>, T: IsA<TreeModel>>(
        child_model: &T,
        root: I,
    ) -> TreeModelFilter {
        skip_assert_initialized!();
        let root = root.into();
        unsafe {
            TreeModel::from_glib_none(ffi::gtk_tree_model_filter_new(child_model.as_ref().to_glib_none().0,
                                                                     mut_override(root.to_glib_none().0)))
                                                                    .unsafe_cast()
        }
    }
}
