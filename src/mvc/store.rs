// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use libc::{c_char, c_int};

use glib::translate::*;
use glib::{Type, Value};
use ffi;

use {
    TreeIter,
    TreeModel,
};

///////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct ListStore(Object<ffi::GtkListStore>): TreeModel, ::Buildable;

    match fn {
        get_type => || ffi::gtk_list_store_get_type(),
    }
}

impl ListStore {
    pub fn new(column_types: &[Type]) -> ListStore {
        unsafe {
            let mut column_types = column_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
            from_glib_full(
                ffi::gtk_list_store_newv(column_types.len() as c_int,
                    column_types.as_mut_ptr()))
        }
    }

    pub fn set_column_types(&self, column_types: &[Type]) {
        unsafe {
            let mut column_types = column_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
            ffi::gtk_list_store_set_column_types(self.to_glib_none().0, column_types.len() as c_int,
                column_types.as_mut_ptr())
        }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        unsafe {
            let text: Stash<*const c_char, _> = text.to_glib_none();
            ffi::gtk_list_store_set(self.to_glib_none().0, mut_override(iter.to_glib_none().0),
                column, text.0, -1)
        }
    }

    pub fn remove(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_remove(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    pub fn insert(&self, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert(self.to_glib_none().0, iter.to_glib_none_mut().0, position);
            iter
        }
    }

    pub fn insert_before(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_before(self.to_glib_none().0,
                                              iter.to_glib_none_mut().0,
                                              mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    pub fn insert_after(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_after(self.to_glib_none().0,
                                             iter.to_glib_none_mut().0,
                                             mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    pub fn prepend(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_prepend(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn append(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_append(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_list_store_clear(self.to_glib_none().0) }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_list_store_iter_is_valid(self.to_glib_none().0,
                    mut_override(iter.to_glib_none().0)))
        }
    }

    pub fn reorder(&self, new_order: &[i32]) {
        unsafe {
            let count = ffi::gtk_tree_model_iter_n_children(self.to_glib_none().0, ptr::null_mut());
            let safe_count = count as usize == new_order.len();
            debug_assert!(safe_count,
                          "Incorrect `new_order` slice length. Expected `{}`, found `{}`.",
                          count,
                          new_order.len());
            let safe_values = new_order.iter()
                .max()
                .map(|&max| {
                    let max = max as i32;
                    max >= 0 && max < count
                })
                .unwrap_or(true);
            debug_assert!(safe_values,
                          "Some `new_order` slice values are out of range. Maximum safe value: \
                           `{}`. The slice contents: `{:?}`",
                          count - 1,
                          new_order);
            if safe_count && safe_values {
                ffi::gtk_list_store_reorder(self.to_glib_none().0, mut_override(new_order.as_ptr()));
            }
        }
    }

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_list_store_swap(self.to_glib_none().0, mut_override(a.to_glib_none().0),
                mut_override(b.to_glib_none().0))
        }
    }

    pub fn move_before(&self, iter: &mut TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_move_before(self.to_glib_none().0, iter.to_glib_none_mut().0,
                                                 mut_override(position.to_glib_none().0)) }
    }

    pub fn move_after(&self, iter: &mut TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_move_before(self.to_glib_none().0, iter.to_glib_none_mut().0,
                                                 mut_override(position.to_glib_none().0)) }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &Value) {
        unsafe {
            ffi::gtk_list_store_set_value(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0), column, mut_override(value.as_ptr()));
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct TreeStore(Object<ffi::GtkTreeStore>): TreeModel, ::Buildable;

    match fn {
        get_type => || ffi::gtk_tree_store_get_type(),
    }
}

impl TreeStore {
    pub fn new(column_types: &[Type]) -> TreeStore {
        unsafe {
            let mut column_types = column_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
            from_glib_full(
                ffi::gtk_tree_store_newv(column_types.len() as c_int,
                    column_types.as_mut_ptr()))
        }
    }

    pub fn set_column_types(&self, column_types: &[Type]) {
        unsafe {
            let mut column_types = column_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
            ffi::gtk_tree_store_set_column_types(self.to_glib_none().0, column_types.len() as c_int,
                column_types.as_mut_ptr())
        }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        unsafe {
            let text: Stash<*const c_char, _> = text.to_glib_none();
            ffi::gtk_tree_store_set(self.to_glib_none().0, mut_override(iter.to_glib_none().0), column,
                text.0, -1)
        }
    }

    pub fn remove(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_remove(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    pub fn insert(&self, parent: Option<&TreeIter>, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert(self.to_glib_none().0, iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0), position);
            iter
        }
    }

    pub fn insert_before(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_before(self.to_glib_none().0,
                                              iter.to_glib_none_mut().0,
                                              mut_override(parent.to_glib_none().0),
                                              mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    pub fn insert_after(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_after(self.to_glib_none().0,
                                             iter.to_glib_none_mut().0,
                                             mut_override(parent.to_glib_none().0),
                                             mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    pub fn prepend(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_prepend(self.to_glib_none().0,
                                        iter.to_glib_none_mut().0,
                                        mut_override(parent.to_glib_none().0));
            iter
        }
    }

    pub fn append(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_append(self.to_glib_none().0,
                                       iter.to_glib_none_mut().0,
                                       mut_override(parent.to_glib_none().0));
            iter
        }
    }

    pub fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_store_is_ancestor(self.to_glib_none().0, mut_override(iter.to_glib_none().0),
                    mut_override(descendant.to_glib_none().0)))
        }
    }

    pub fn iter_depth(&self, iter: &TreeIter) -> i32 {
        unsafe { ffi::gtk_tree_store_iter_depth(self.to_glib_none().0, mut_override(iter.to_glib_none().0)) }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_tree_store_clear(self.to_glib_none().0) }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_store_iter_is_valid(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    pub fn reorder(&self, parent: &TreeIter, new_order: &[i32]) {
        unsafe {
            let count = ffi::gtk_tree_model_iter_n_children(self.to_glib_none().0,
                                                            mut_override(parent.to_glib_none().0));
            let safe_count = count as usize == new_order.len();
            debug_assert!(safe_count,
                          "Incorrect `new_order` slice length. Expected `{}`, found `{}`.",
                          count,
                          new_order.len());
            let safe_values = new_order.iter()
                .max()
                .map(|&max| {
                    let max = max as i32;
                    max >= 0 && max < count
                })
                .unwrap_or(true);
            debug_assert!(safe_values,
                          "Some `new_order` slice values are out of range. Maximum safe value: \
                           `{}`. The slice contents: `{:?}`",
                          count - 1,
                          new_order);
            if safe_count && safe_values {
                ffi::gtk_tree_store_reorder(self.to_glib_none().0,
                                            mut_override(parent.to_glib_none().0),
                                            mut_override(new_order.as_ptr()));
            }
        }
    }

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_tree_store_swap(self.to_glib_none().0, mut_override(a.to_glib_none().0),
                mut_override(b.to_glib_none().0))
        }
    }

    pub fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_before(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                position.map_or(ptr::null_mut(), |p| mut_override(p.to_glib_none().0)))
        }
    }

    pub fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_before(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                position.map_or(ptr::null_mut(), |p| mut_override(p.to_glib_none().0)))
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &Value) {
        unsafe {
            ffi::gtk_tree_store_set_value(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                column, mut_override(value.as_ptr()));
        }
    }

    /*pub fn set_valuesv<T: ::traits::GValuePrivate>(&self, iter: &::TreeIter, columns: &[i32], values: &[T]) {
        let mut tmp_values = Vec::with_capacity(values.len());

        for value in values {
            tmp_values.push(value.get_gvalue());
        }
        unsafe { ffi::gtk_tree_store_set_valuesv(::cast::GTK_TREE_MODEL_FROM_TREE_STORE(self.to_glib_none().0), mut_override(iter.to_glib_none().0),
            columns.as_ptr(), tmp_values.as_slice().as_ptr()) }
    }*/
}
