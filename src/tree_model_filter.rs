// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::object::{Downcast, IsA};
use glib::translate::*;
use std::mem::transmute;

use glib_ffi::{gboolean, gpointer};
use {TreeIter, TreeModel, TreeModelFilter, TreePath};
use ffi::{GtkTreeIter, GtkTreeModel};

impl TreeModelFilter {
    pub fn new<T: IsA<TreeModel>>(child_model: &T, root: Option<&TreePath>) -> TreeModelFilter {
        skip_assert_initialized!();
        unsafe {
            TreeModel::from_glib_none(ffi::gtk_tree_model_filter_new(child_model.to_glib_none().0,
                                                                     mut_override(root.to_glib_none().0)))
                                                                    .downcast_unchecked()
        }
    }

    pub fn set_visible_func<F>(&self, func: F)
    where F: Fn(&TreeModel, &TreeIter) -> bool + 'static {
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_func(self.to_glib_none().0,
                                                        Some(trampoline),
                                                        into_raw(func),
                                                        Some(destroy_closure))
        }
    }
}

unsafe extern "C" fn trampoline(this: *mut GtkTreeModel, iter: *mut GtkTreeIter,
                                f: gpointer) -> gboolean {
    callback_guard!();
    let f: &Box<Fn(&TreeModel, &TreeIter) -> bool> = transmute(f);
    f(&TreeModel::from_glib_none(this).downcast_unchecked(), &from_glib_borrow(iter))
    .to_glib()
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    callback_guard!();
    Box::<Box<Fn(&TreeModel, &TreeIter) -> bool + 'static>>::from_raw(ptr as *mut _);
}

fn into_raw<F>(func: F) -> gpointer
    where F: Fn(&TreeModel, &TreeIter) -> bool + 'static {
    skip_assert_initialized!();
    let func: Box<Box<Fn(&TreeModel, &TreeIter) -> bool + 'static>> =
        Box::new(Box::new(func));
    Box::into_raw(func) as gpointer
}
