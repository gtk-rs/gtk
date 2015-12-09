// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(not(gtk_3_12), allow(unused_imports))]

use std::ptr;
use glib::object::Upcast;
use glib::translate::*;
use glib::{Type, Value};
use ffi;

use {
    TreeIter,
    TreePath,
};

glib_wrapper! {
    pub struct TreeModel(Object<ffi::GtkTreeModel>);

    match fn {
        get_type => || ffi::gtk_tree_model_get_type(),
    }
}

pub trait TreeModelExt {
    fn get_flags(&self) -> ::TreeModelFlags;
    fn get_n_columns(&self) -> i32;
    fn get_column_type(&self, index_: i32) -> Type;
    fn get_iter(&self, path: &TreePath) -> Option<TreeIter>;
    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter>;
    fn get_iter_first(&self) -> Option<TreeIter>;
    fn get_path(&self, iter: &TreeIter) -> Option<TreePath>;
    fn get_value(&self, iter: &TreeIter, column: i32) -> Value;
    fn iter_next(&self, iter: &mut TreeIter) -> bool;
    fn iter_previous(&self, iter: &mut TreeIter) -> bool;
    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter>;
    fn iter_has_child(&self, iter: &TreeIter) -> bool;
    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32;
    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter>;
    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter>;
    fn get_string_from_iter(&self, iter: &TreeIter) -> Option<String>;
    fn row_changed(&self, path: &TreePath, iter: &TreeIter);
    fn row_inserted(&self, path: &TreePath, iter: &TreeIter);
    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter);
    fn row_deleted(&self, path: &TreePath);
    fn rows_reordered(&self, path: &TreePath, iter: Option<&TreeIter>, new_order: &mut [i32]);
    fn ref_node(&self, iter: &TreeIter);
    fn unref_node(&self, iter: &TreeIter);
    // FIXME
    //fn foreach<T>(&self,
}

impl<O: Upcast<TreeModel>> TreeModelExt for O {
    fn get_flags(&self) -> ::TreeModelFlags {
        unsafe { ffi::gtk_tree_model_get_flags(self.to_glib_none().0) }
    }

    fn get_n_columns(&self) -> i32 {
        unsafe { ffi::gtk_tree_model_get_n_columns(self.to_glib_none().0) }
    }

    fn get_column_type(&self, index_: i32) -> Type {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_column_type(self.to_glib_none().0, index_))
        }
    }

    fn get_iter(&self, path: &TreePath) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_get_iter(self.to_glib_none().0, iter.to_glib_none_mut().0,
                mut_override(path.to_glib_none().0));
            some_if(ok, || iter)
        }
    }

    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_get_iter_from_string(self.to_glib_none().0,
                iter.to_glib_none_mut().0, path_string.to_glib_none().0);
            some_if(ok, || iter)
        }
    }

    fn get_iter_first(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_get_iter_first(self.to_glib_none().0,
                iter.to_glib_none_mut().0);
            some_if(ok, || iter)
        }
    }

    fn get_path(&self, iter: &TreeIter) -> Option<TreePath> {
        unsafe {
            let ptr = ffi::gtk_tree_model_get_path(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0));
            some_if(!ptr.is_null(), || from_glib_full(ptr))
        }
    }

    fn get_value(&self, iter: &TreeIter, column: i32) -> Value {
        unsafe {
            let mut value = Value::new();
            ffi::gtk_tree_model_get_value(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0), column, value.as_mut_ptr());
            value
        }
    }

    fn iter_next(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_iter_next(self.to_glib_none().0,
                    iter.to_glib_none_mut().0))
        }
    }

    fn iter_previous(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_iter_previous(self.to_glib_none().0,
                    iter.to_glib_none_mut().0))
        }
    }

    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_iter_children(self.to_glib_none().0,
                iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0));
            some_if(ok, || iter)
        }
    }

    fn iter_has_child(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_iter_has_child(self.to_glib_none().0,
                    mut_override(iter.to_glib_none().0)))
        }
    }

    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32 {
        unsafe {
            ffi::gtk_tree_model_iter_n_children(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0))
        }
    }

    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_iter_nth_child(self.to_glib_none().0,
                iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), n);
            some_if(ok, || iter)
        }
    }

    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_iter_parent(self.to_glib_none().0,
                iter.to_glib_none_mut().0, mut_override(child.to_glib_none().0));
            some_if(ok, || iter)
        }
     }

    fn get_string_from_iter(&self, iter: &TreeIter) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_tree_model_get_string_from_iter(self.to_glib_none().0,
                    mut_override(iter.to_glib_none().0)))
        }
    }

    fn row_changed(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_changed(self.to_glib_none().0,
                mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    fn row_inserted(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_inserted(self.to_glib_none().0,
                mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_has_child_toggled(self.to_glib_none().0,
                mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    fn row_deleted(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_model_row_deleted(self.to_glib_none().0,
                mut_override(path.to_glib_none().0));
        }
    }

    fn rows_reordered(&self, path: &TreePath, iter: Option<&TreeIter>, new_order: &mut [i32]) {
        unsafe {
            ffi::gtk_tree_model_rows_reordered(
                self.to_glib_none().0,
                mut_override(path.to_glib_none().0),
                iter.map_or(ptr::null_mut(), |p| mut_override(p.to_glib_none().0)),
                new_order.as_mut_ptr());
        }
    }

    fn ref_node(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_ref_node(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0));
        }
    }

    fn unref_node(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_unref_node(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0));
        }
    }
}
