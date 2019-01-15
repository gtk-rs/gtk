// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::object::{Cast, IsA};
use glib::translate::*;
use std::mem::transmute;

use glib_ffi::{gboolean, gpointer};
use {TreeIter, TreeModel, TreeModelFilter, TreePath};
use ffi::{GtkTreeIter, GtkTreeModel};

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

pub trait TreeModelFilterExtManual: 'static {
    fn set_visible_func<F>(&self, func: F)
        where F: Fn(&TreeModel, &TreeIter) -> bool + 'static;
}

impl<O: IsA<TreeModelFilter>> TreeModelFilterExtManual for O {
    fn set_visible_func<F>(&self, func: F)
    where F: Fn(&TreeModel, &TreeIter) -> bool + 'static {
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_func(self.as_ref().to_glib_none().0,
                                                        Some(trampoline),
                                                        into_raw(func),
                                                        Some(destroy_closure))
        }
    }
}

unsafe extern "C" fn trampoline(this: *mut GtkTreeModel, iter: *mut GtkTreeIter,
                                f: gpointer) -> gboolean {
    let f: &&(Fn(&TreeModel, &TreeIter) -> bool) = transmute(f);
    f(&TreeModel::from_glib_none(this).unsafe_cast(), &from_glib_borrow(iter))
    .to_glib()
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    Box::<Box<Fn(&TreeModel, &TreeIter) -> bool + 'static>>::from_raw(ptr as *mut _);
}

fn into_raw<F>(func: F) -> gpointer
    where F: Fn(&TreeModel, &TreeIter) -> bool + 'static {
    skip_assert_initialized!();
    let func: Box<Box<Fn(&TreeModel, &TreeIter) -> bool + 'static>> =
        Box::new(Box::new(func));
    Box::into_raw(func) as gpointer
}
