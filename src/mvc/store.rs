// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use libc::{c_char, c_int};

use glib::translate::*;
use glib::types::{StaticType, Type};
use glib::Value;
use ffi;

use object::{Object, Upcast};
use super::tree_model::{TreeIter, TreeModel};

///////////////////////////////////////////////////////////////////////////////

pub type ListStore = Object<ffi::GtkListStore>;

impl ListStore {
    pub fn new(column_types: &[Type]) -> ListStore {
        unsafe {
            from_glib_full(
                ffi::gtk_list_store_newv(column_types.len() as c_int,
                    column_types.to_glib_none().0))
        }
    }

    pub fn set_column_types(&self, column_types: &[Type]) {
        unsafe {
            ffi::gtk_list_store_set_column_types(self.to_glib_none().0, column_types.len() as c_int,
                column_types.to_glib_none().0)
        }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        unsafe {
            let text: Stash<*const c_char, _> = text.to_glib_none();
            ffi::gtk_list_store_set(self.to_glib_none().0, iter.unwrap_pointer(), column,
                text.0, -1)
        }
    }

    pub fn remove(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_remove(self.to_glib_none().0, iter.unwrap_pointer()))
        }
    }

    pub fn insert(&self, iter: &mut TreeIter, position: i32) {
        unsafe {
            ffi::gtk_list_store_insert(self.to_glib_none().0, iter.unwrap_pointer(), position)
        }
    }

    pub fn insert_before(&self, iter: &mut TreeIter, sibling: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_insert_before(self.to_glib_none().0, iter.unwrap_pointer(),
                sibling.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn insert_after(&self, iter: &mut TreeIter, sibling: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_insert_after(self.to_glib_none().0, iter.unwrap_pointer(),
                sibling.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn prepend(&self, iter: &mut TreeIter) {
        unsafe { ffi::gtk_list_store_prepend(self.to_glib_none().0, iter.unwrap_pointer()) }
    }

    pub fn append(&self, iter: &mut TreeIter) {
        unsafe { ffi::gtk_list_store_append(self.to_glib_none().0, iter.unwrap_pointer()) }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_list_store_clear(self.to_glib_none().0) }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_list_store_iter_is_valid(self.to_glib_none().0, iter.unwrap_pointer()))
        }
    }

    pub fn reorder(&self, new_order: &[i32]) {
        unsafe { ffi::gtk_list_store_reorder(self.to_glib_none().0, new_order.to_glib_none().0) }
    }

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_list_store_swap(self.to_glib_none().0, a.unwrap_pointer(), b.unwrap_pointer())
        }
    }

    pub fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_before(self.to_glib_none().0, iter.unwrap_pointer(),
                position.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_before(self.to_glib_none().0, iter.unwrap_pointer(),
                position.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &Value) {
        unsafe {
            ffi::gtk_list_store_set_value(self.to_glib_none().0, iter.unwrap_pointer(),
                column, value.as_ptr() as *mut _);
        }
    }
}

impl StaticType for ListStore {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_list_store_get_type()) }
    }
}

unsafe impl Upcast<TreeModel> for ListStore { }
unsafe impl Upcast<::builder::Buildable> for ListStore { }

///////////////////////////////////////////////////////////////////////////////

pub type TreeStore = Object<ffi::GtkTreeStore>;

impl TreeStore {
    pub fn new(column_types: &[Type]) -> TreeStore {
        unsafe {
            from_glib_full(
                ffi::gtk_tree_store_newv(column_types.len() as c_int,
                    column_types.to_glib_none().0))
        }
    }

    pub fn set_column_types(&self, column_types: &[Type]) {
        unsafe {
            ffi::gtk_tree_store_set_column_types(self.to_glib_none().0, column_types.len() as c_int,
                column_types.to_glib_none().0)
        }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        unsafe {
            let text: Stash<*const c_char, _> = text.to_glib_none();
            ffi::gtk_tree_store_set(self.to_glib_none().0, iter.unwrap_pointer(), column,
                text.0, -1)
        }
    }

    pub fn remove(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_remove(self.to_glib_none().0, iter.unwrap_pointer()))
        }
    }

    pub fn insert(&self, iter: &mut TreeIter, parent: Option<&TreeIter>, position: i32) {
        unsafe {
            ffi::gtk_tree_store_insert(self.to_glib_none().0, iter.unwrap_pointer(),
                parent.map_or(ptr::null_mut(), |p| p.unwrap_pointer()), position)
        }
    }

    pub fn insert_before(&self, iter: &mut TreeIter, parent: Option<&TreeIter>,
            sibling: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_insert_before(self.to_glib_none().0, iter.unwrap_pointer(),
                parent.map_or(ptr::null_mut(), |p| p.unwrap_pointer()),
                sibling.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn insert_after(&self, iter: &mut TreeIter, parent: Option<&TreeIter>,
            sibling: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_insert_after(self.to_glib_none().0, iter.unwrap_pointer(),
                parent.map_or(ptr::null_mut(), |p| p.unwrap_pointer()),
                sibling.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn prepend(&self, iter: &mut TreeIter, parent: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_prepend(self.to_glib_none().0, iter.unwrap_pointer(),
                parent.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn append(&self, iter: &mut TreeIter, parent: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_append(self.to_glib_none().0, iter.unwrap_pointer(),
                parent.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_store_is_ancestor(self.to_glib_none().0, iter.unwrap_pointer(),
                    descendant.unwrap_pointer()))
        }
    }

    pub fn iter_depth(&self, iter: &TreeIter) -> i32 {
        unsafe { ffi::gtk_tree_store_iter_depth(self.to_glib_none().0, iter.unwrap_pointer()) }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_tree_store_clear(self.to_glib_none().0) }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_store_iter_is_valid(self.to_glib_none().0, iter.unwrap_pointer()))
        }
    }

    pub fn reorder(&self, parent: &TreeIter, new_order: &[i32]) {
        unsafe {
            ffi::gtk_tree_store_reorder(self.to_glib_none().0, parent.unwrap_pointer(),
                new_order.to_glib_none().0)
        }
    }

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_tree_store_swap(self.to_glib_none().0, a.unwrap_pointer(), b.unwrap_pointer())
        }
    }

    pub fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_before(self.to_glib_none().0, iter.unwrap_pointer(),
                position.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_before(self.to_glib_none().0, iter.unwrap_pointer(),
                position.map_or(ptr::null_mut(), |p| p.unwrap_pointer()))
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &Value) {
        unsafe {
            ffi::gtk_tree_store_set_value(self.to_glib_none().0, iter.unwrap_pointer(),
                column, value.as_ptr() as *mut _);
        }
    }

    /*pub fn set_valuesv<T: ::traits::GValuePrivate>(&self, iter: &::TreeIter, columns: &[i32], values: &[T]) {
        let mut tmp_values = Vec::with_capacity(values.len());

        for value in values {
            tmp_values.push(value.get_gvalue());
        }
        unsafe { ffi::gtk_tree_store_set_valuesv(::cast::GTK_TREE_MODEL_FROM_TREE_STORE(self.to_glib_none().0), iter.unwrap_pointer(),
            columns.as_ptr(), tmp_values.as_slice().as_ptr()) }
    }*/
}

impl StaticType for TreeStore {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_tree_store_get_type()) }
    }
}

unsafe impl Upcast<TreeModel> for TreeStore { }
unsafe impl Upcast<::builder::Buildable> for TreeStore { }
