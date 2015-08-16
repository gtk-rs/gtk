// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(not(gtk_3_12), allow(unused_imports))]

use std::mem;
use std::ptr;
use std::slice;
use libc::{c_void, c_int};

use glib::translate::*;
use glib::types::{StaticType, Type};
use glib::Value;
use ffi;

use object::{Object, Upcast};

///////////////////////////////////////////////////////////////////////////////

pub struct TreeIter {
    data: ffi::GtkTreeIter,
    pointer: *mut ffi::GtkTreeIter,
    is_owned: bool,
    is_true_pointer: bool
}

impl TreeIter {
    pub fn new() -> TreeIter {
        let mut t = TreeIter {
            data: ffi::GtkTreeIter {
                stamp: 0,
                user_data: ptr::null_mut(),
                user_data2: ptr::null_mut(),
                user_data3: ptr::null_mut()
            },
            pointer: ptr::null_mut(),
            is_owned: false,
            is_true_pointer: false
        };

        unsafe { t.pointer = mem::transmute(&mut t.data); }
        t
    }

    pub fn copy(&self) -> Option<TreeIter> {
        let tmp_pointer = unsafe { ffi::gtk_tree_iter_copy(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe {
                Some(TreeIter {
                    data: mem::uninitialized(),
                    pointer: tmp_pointer,
                    is_owned: true,
                    is_true_pointer: true
                })
            }
        }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkTreeIter {
        if self.is_true_pointer {
            self.pointer
        } else {
            unsafe { mem::transmute(&self.data) }
        }
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treeiter: *mut ffi::GtkTreeIter) -> TreeIter {
        unsafe {
            TreeIter {
                data: mem::uninitialized(),
                pointer: c_treeiter,
                is_owned: false,
                is_true_pointer: true
            }
        }
    }
}

impl Drop for TreeIter {
    fn drop(&mut self) {
        if !self.pointer.is_null() && self.is_owned {
            unsafe { ffi::gtk_tree_iter_free(self.pointer) };
            self.pointer = ptr::null_mut();
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct TreePath {
    pointer:   *mut ffi::GtkTreePath
}

impl TreePath {
    pub fn new() -> TreePath {
        unsafe { TreePath { pointer: ffi::gtk_tree_path_new() } }
    }

    pub fn new_from_string(path: &str) -> Result<TreePath, ()> {
        let tmp = unsafe {
            ffi::gtk_tree_path_new_from_string(path.to_glib_none().0)
        };

        if tmp.is_null() {
            Err(())
        } else {
            Ok(TreePath { pointer: tmp })
        }
    }

    #[cfg(gtk_3_12)]
    pub fn new_from_indices(indices: &[i32]) -> TreePath {
        assert!(!indices.is_empty());
        unsafe {
            TreePath { pointer: ffi::gtk_tree_path_new_from_indicesv(indices.as_ptr() as *mut c_int,
                                                                     indices.len() as ffi::gsize)
            }
        }
    }

    pub fn new_first() -> TreePath {
        unsafe { TreePath { pointer: ffi::gtk_tree_path_new_first() } }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_tree_path_to_string(self.pointer))
        }
    }

    pub fn append_index(&self, index_: i32) {
        unsafe { ffi::gtk_tree_path_append_index(self.pointer, index_) }
    }

    pub fn prepend_index(&self, index_: i32) {
        unsafe { ffi::gtk_tree_path_prepend_index(self.pointer, index_) }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe { ffi::gtk_tree_path_get_depth(self.pointer) }
    }

    pub fn get_indices(&self) -> Vec<i32> {
        let tmp = unsafe { ffi::gtk_tree_path_get_indices(self.pointer) };
        let depth = self.get_depth();

        unsafe { Vec::from( slice::from_raw_parts(tmp, depth as usize)) }
    }

    pub fn copy(&self) -> TreePath {
        unsafe { TreePath { pointer: ffi::gtk_tree_path_copy(self.pointer) } }
    }

    pub fn compare(&self, other: &TreePath) -> i32 {
        unsafe { ffi::gtk_tree_path_compare(self.pointer, other.pointer) }
    }

    pub fn next(&self) {
        unsafe { ffi::gtk_tree_path_next(self.pointer) }
    }

    pub fn prev(&self) {
        unsafe { ffi::gtk_tree_path_prev(self.pointer) }
    }

    pub fn path_up(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_path_up(self.pointer)) }
    }

    pub fn path_down(&self) {
        unsafe { ffi::gtk_tree_path_down(self.pointer) }
    }

    pub fn is_ancestor(&self, descendant: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_is_ancestor(self.pointer, descendant.pointer))
        }
    }

    pub fn is_descendant(&self, ancestor: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_is_descendant(self.pointer, ancestor.pointer))
        }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkTreePath {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treepath: *mut ffi::GtkTreePath) -> TreePath {
        TreePath {
            pointer: c_treepath
        }
    }
}

impl Drop for TreePath {
    fn drop(&mut self) {
        unsafe { ffi::gtk_tree_path_free(self.pointer) }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub type TreeModel = Object<ffi::GtkTreeModel>;

impl StaticType for TreeModel {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_tree_model_get_type()) }
    }
}

pub trait TreeModelExt {
    fn get_flags(&self) -> ::TreeModelFlags;
    fn get_n_columns(&self) -> i32;
    fn get_column_type(&self, index_: i32) -> Type;
    fn get_iter(&self, iter: &mut TreeIter, path: &TreePath) -> bool;
    fn get_iter_from_string(&self, iter: &mut TreeIter, path_string: &str) -> bool;
    fn get_iter_first(&self, iter: &mut TreeIter) -> bool;
    fn get_path(&self, iter: &TreeIter) -> Option<TreePath>;
    fn get_value(&self, iter: &TreeIter, column: i32) -> Value;
    fn iter_next(&self, iter: &mut TreeIter) -> bool;
    fn iter_previous(&self, iter: &mut TreeIter) -> bool;
    fn iter_children(&self, iter: &mut TreeIter, parent: Option<&TreeIter>) -> bool;
    fn iter_has_child(&self, iter: &TreeIter) -> bool;
    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32;
    fn iter_nth_child(&self, iter: &mut TreeIter, parent: Option<&TreeIter>, n: i32) -> bool;
    fn iter_parent(&self, iter: &mut TreeIter, child: &TreeIter) -> bool;
    fn get_string_from_iter(&self, iter: &TreeIter) -> Option<String>;
    fn row_changed(&self, path: &TreePath, iter: &TreeIter);
    fn row_inserted(&self, path: &TreePath, iter: &TreeIter);
    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter);
    fn row_deleted(&self, path: &TreePath);
    fn rows_reordered(&self, path: &TreePath, iter: Option<&TreeIter>, new_order: &mut [i32]);
    fn ref_node(&self, iter: &TreeIter);
    fn unref_node(&self, iter: &TreeIter);
    fn foreach<T>(&self,
                  func: fn(&mut TreeModel, &mut TreePath, &mut TreeIter, data: &mut T)-> bool,
                  user_data: &mut T);
}

impl<O: Upcast<TreeModel>> TreeModelExt for O {
    fn get_flags(&self) -> ::TreeModelFlags {
        unsafe { ffi::gtk_tree_model_get_flags(self.upcast().to_glib_none().0) }
    }

    fn get_n_columns(&self) -> i32 {
        unsafe { ffi::gtk_tree_model_get_n_columns(self.upcast().to_glib_none().0) }
    }

    fn get_column_type(&self, index_: i32) -> Type {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_column_type(self.upcast().to_glib_none().0, index_))
        }
    }

    fn get_iter(&self, iter: &mut TreeIter, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_iter(self.upcast().to_glib_none().0,
                iter.unwrap_pointer(), path.unwrap_pointer()))
        }
    }

    fn get_iter_from_string(&self, iter: &mut TreeIter, path_string: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_get_iter_from_string(self.upcast().to_glib_none().0,
                    iter.unwrap_pointer(), path_string.to_glib_none().0))
        }
    }

    fn get_iter_first(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_get_iter_first(self.upcast().to_glib_none().0,
                    iter.unwrap_pointer()))
        }
    }

    fn get_path(&self, iter: &TreeIter) -> Option<TreePath> {
        let tmp_pointer = unsafe {
            ffi::gtk_tree_model_get_path(self.upcast().to_glib_none().0, iter.unwrap_pointer())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreePath::wrap_pointer(tmp_pointer))
        }
    }

    fn get_value(&self, iter: &TreeIter, column: i32) -> Value {
        unsafe {
            let mut value = Value::new();
            ffi::gtk_tree_model_get_value(self.upcast().to_glib_none().0, iter.unwrap_pointer(),
                column, value.as_mut_ptr());
            value
        }
    }

    fn iter_next(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_iter_next(self.upcast().to_glib_none().0,
                    iter.unwrap_pointer()))
        }
    }

    fn iter_previous(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_iter_previous(self.upcast().to_glib_none().0,
                    iter.unwrap_pointer()))
        }
    }

    fn iter_children(&self, iter: &mut TreeIter, parent: Option<&TreeIter>) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_iter_children(
                    self.upcast().to_glib_none().0,
                    iter.unwrap_pointer(),
                    parent.map_or(ptr::null_mut(), |p| p.unwrap_pointer())))
        }
    }

    fn iter_has_child(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_iter_has_child(self.upcast().to_glib_none().0,
                    iter.unwrap_pointer()))
        }
    }

    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32 {
        unsafe {
            ffi::gtk_tree_model_iter_n_children(
                self.upcast().to_glib_none().0,
                iter.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    fn iter_nth_child(&self, iter: &mut TreeIter, parent: Option<&TreeIter>, n: i32) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_model_iter_nth_child(
                    self.upcast().to_glib_none().0,
                    iter.unwrap_pointer(),
                    parent.map_or(ptr::null_mut(), |p| p.unwrap_pointer()),
                    n))
        }
    }

    fn iter_parent(&self, iter: &mut TreeIter, child: &TreeIter) -> bool {
        unsafe { 
            from_glib(
                ffi::gtk_tree_model_iter_parent(self.upcast().to_glib_none().0,
                    iter.unwrap_pointer(), child.unwrap_pointer()))
        }
    }

    fn get_string_from_iter(&self, iter: &TreeIter) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_tree_model_get_string_from_iter(self.upcast().to_glib_none().0,
                    iter.unwrap_pointer()))
        }
    }

    fn row_changed(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_changed(self.upcast().to_glib_none().0, path.unwrap_pointer(),
                iter.unwrap_pointer());
        }
    }

    fn row_inserted(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_inserted(self.upcast().to_glib_none().0, path.unwrap_pointer(), iter.unwrap_pointer());
        }
    }

    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_has_child_toggled(self.upcast().to_glib_none().0,
                path.unwrap_pointer(), iter.unwrap_pointer());
        }
    }

    fn row_deleted(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_model_row_deleted(self.upcast().to_glib_none().0, path.unwrap_pointer());
        }
    }

    fn rows_reordered(&self, path: &TreePath, iter: Option<&TreeIter>, new_order: &mut [i32]) {
        unsafe {
            ffi::gtk_tree_model_rows_reordered(
                self.upcast().to_glib_none().0,
                path.unwrap_pointer(),
                iter.map_or(ptr::null_mut(), |p| p.unwrap_pointer()),
                new_order.as_mut_ptr());
        }
    }

    fn ref_node(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_ref_node(self.upcast().to_glib_none().0, iter.unwrap_pointer());
        }
    }

    fn unref_node(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_unref_node(self.upcast().to_glib_none().0, iter.unwrap_pointer());
        }
    }

    fn foreach<T>(&self,
                  func: fn(&mut TreeModel, &mut TreePath, &mut TreeIter, data: &mut T) -> bool,
                  user_data: &mut T) {
        let t = &(func, user_data);
        let ca = t as *const (fn(&mut TreeModel, &mut TreePath, &mut TreeIter, data: &mut T) -> bool, &mut T);

        unsafe {
            ffi::gtk_tree_model_foreach(self.upcast().to_glib_none().0, my_fn as ffi::gpointer,
                ca as ffi::gpointer)
        }
    }
}

fn my_fn(model: *mut ffi::GtkTreeModel,
         path: *mut ffi::GtkTreePath,
         iter: *mut ffi::GtkTreeIter,
         data: &mut (fn(&mut TreeModel, &mut TreePath, &mut TreeIter, data: *mut c_void) -> bool,
         &mut c_void)) -> ffi::gboolean {
    unsafe {
        data.0(&mut from_glib_none(model), &mut TreePath::wrap_pointer(path),
            &mut TreeIter::wrap_pointer(iter), data.1).to_glib()
    }
}
