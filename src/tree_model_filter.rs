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
        where F: Fn(&Self, &TreeIter) -> bool + 'static;
}

impl<O: IsA<TreeModelFilter>> TreeModelFilterExtManual for O {
    fn set_visible_func<F>(&self, func: F)
    where F: Fn(&Self, &TreeIter) -> bool + 'static {
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_func(self.as_ref().to_glib_none().0,
                                                        Some(trampoline::<Self, F>),
                                                        into_raw(func),
                                                        Some(destroy_closure::<Self, F>))
        }
    }
}

unsafe extern "C" fn trampoline<T, F: Fn(&T, &TreeIter) -> bool + 'static>(this: *mut GtkTreeModel, iter: *mut GtkTreeIter,
                                                                           f: gpointer) -> gboolean
where T: IsA<TreeModelFilter> {
    let f: &F = transmute(f);
    f(&TreeModel::from_glib_none(this).unsafe_cast(), &from_glib_borrow(iter))
    .to_glib()
}

unsafe extern "C" fn destroy_closure<T, F: Fn(&T, &TreeIter) -> bool + 'static>(ptr: gpointer)
where T: IsA<TreeModelFilter> {
    Box::<F>::from_raw(ptr as *mut _);
}

fn into_raw<T, F>(func: F) -> gpointer
    where T: IsA<TreeModelFilter>,
          F: Fn(&T, &TreeIter) -> bool + 'static {
    skip_assert_initialized!();
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as gpointer
}
