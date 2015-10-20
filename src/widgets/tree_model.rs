// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use glib::{Value, Type};
use glib::translate::{
    from_glib_full, from_glib_borrow, from_glib, ToGlibPtr, ToGlibPtrMut, Uninitialized, some_if,
};
use ffi;
use {TreeIter, TreePath};
use libc::c_void;
use glib;

pub struct TreeModel {
    pointer: *mut ffi::GtkTreeModel
}

impl TreeModel {
    pub fn get_flags(&self) -> ::TreeModelFlags {
        unsafe { ffi::gtk_tree_model_get_flags(self.pointer) }
    }

    pub fn get_n_columns(&self) -> i32 {
        unsafe { ffi::gtk_tree_model_get_n_columns(self.pointer) }
    }

    pub fn get_column_type(&self, index_: i32) -> Type {
        unsafe { from_glib(ffi::gtk_tree_model_get_column_type(self.pointer, index_)) }
    }

    pub fn get_iter(&self, path: &TreePath) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_get_iter(self.pointer, iter.to_glib_none_mut().0,
                path.unwrap_pointer());
            some_if(ok, iter)
        }
    }

    pub fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_get_iter_from_string(self.pointer,
                iter.to_glib_none_mut().0, path_string.to_glib_none().0);
            some_if(ok, iter)
        }
    }

    pub fn get_iter_first(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_get_iter_first(self.pointer, iter.to_glib_none_mut().0);
            some_if(ok, iter)
        }
     }

    pub fn get_path(&self, iter: &TreeIter) -> Option<TreePath> {
        let tmp_pointer = unsafe {
            ffi::gtk_tree_model_get_path(self.pointer, iter.to_glib_none().0 as *mut _)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreePath::wrap_pointer(tmp_pointer))
        }
    }

    pub fn get_value(&self, iter: &TreeIter, column: i32) -> Value {
        unsafe {
            let mut value = Value::new();
            ffi::gtk_tree_model_get_value(self.pointer, iter.to_glib_none().0 as *mut _, column,
                value.as_mut_ptr());
            value
        }
    }

    pub fn iter_next(&self, iter: &mut TreeIter) -> bool {
        unsafe { from_glib(ffi::gtk_tree_model_iter_next(self.pointer, iter.to_glib_none_mut().0)) }
    }

    pub fn iter_previous(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_previous(self.pointer, iter.to_glib_none_mut().0))
        }
    }

    pub fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_iter_children(self.pointer, iter.to_glib_none_mut().0,
                parent.to_glib_none().0 as *mut _);
            some_if(ok, iter)
        }
    }

    pub fn iter_has_child(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_has_child(self.pointer, iter.to_glib_none().0 as *mut _))
        }
    }

    pub fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32 {
        unsafe {
            ffi::gtk_tree_model_iter_n_children(self.pointer, iter.to_glib_none().0 as *mut _)
        }
    }

    pub fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_iter_nth_child(self.pointer, iter.to_glib_none_mut().0,
                parent.to_glib_none().0 as *mut _, n);
            some_if(ok, iter)
        }
    }

    pub fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_model_iter_parent(self.pointer, iter.to_glib_none_mut().0,
                child.to_glib_none().0 as *mut _);
            some_if(ok, iter)
        }
     }

    pub fn get_string_from_iter(&self, iter: &TreeIter) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_tree_model_get_string_from_iter(self.pointer,
                                                         iter.to_glib_none().0 as *mut _))
        }
    }

    pub fn row_changed(&self, path: &TreePath, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_model_row_changed(self.pointer, path.unwrap_pointer(), iter.to_glib_none().0 as *mut _) }
    }

    pub fn row_inserted(&self, path: &TreePath, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_model_row_inserted(self.pointer, path.unwrap_pointer(), iter.to_glib_none().0 as *mut _) }
    }

    pub fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_model_row_has_child_toggled(self.pointer, path.unwrap_pointer(), iter.to_glib_none().0 as *mut _) }
    }

    pub fn row_deleted(&self, path: &TreePath) {
        unsafe { ffi::gtk_tree_model_row_deleted(self.pointer, path.unwrap_pointer()) }
    }

    pub fn foreach<T>(&self, func: fn(&mut TreeModel, &mut TreePath, &mut TreeIter, data: &mut T) -> bool, user_data: &mut T) {
        let t = &(func, user_data);
        let ca = t as *const (fn(&mut TreeModel, &mut TreePath, &mut TreeIter, data: &mut T) -> bool, &mut T);

        unsafe { ffi::gtk_tree_model_foreach(self.pointer, Some(my_fn), ca as ::glib_ffi::gpointer) }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkTreeModel {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treemodel: *mut ffi::GtkTreeModel) -> TreeModel {
        TreeModel {
            pointer: c_treemodel
        }
    }
}

unsafe extern "C" fn my_fn(model: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, data: *mut c_void) -> ::glib_ffi::gboolean {
    let data: &mut (fn(&mut TreeModel, &mut TreePath, &mut TreeIter, data: *mut c_void) -> bool, &mut c_void) = mem::transmute(data);
    glib::to_gboolean(data.0(&mut TreeModel::wrap_pointer(model), &mut TreePath::wrap_pointer(path), &mut from_glib_borrow(iter), data.1))
}

impl_drop!(TreeModel, GTK_TREE_MODEL);
