// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_char;
use glib::{Value, Type};
use glib::translate::*;
use ffi;
use TreeIter;
use glib_ffi::GType;
use cast::GTK_TREE_MODEL;

pub struct TreeStore {
    pointer: *mut ffi::GtkTreeStore
}

impl TreeStore {
    pub fn new(column_types: &[Type]) -> Option<TreeStore> {
        let column_types_ffi: Vec<GType> = column_types.iter().map(|n| n.to_glib()).collect();
        let tmp_pointer = unsafe { ffi::gtk_tree_store_newv(column_types.len() as i32, column_types_ffi.as_ptr() as *mut GType) };
        check_pointer!(tmp_pointer, TreeStore, G_OBJECT_FROM_TREE_STORE)
    }

    pub fn set_column_types(&self, column_types: &[Type]) {
        let column_types_ffi: Vec<GType> = column_types.iter().map(|n| n.to_glib()).collect();
        unsafe { ffi::gtk_tree_store_set_column_types(self.pointer, column_types.len() as i32, column_types_ffi.as_ptr() as *mut GType) }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        let text: Stash<*const c_char, _> = text.to_glib_none();
        unsafe {
            ffi::gtk_tree_store_set(self.pointer, iter.to_glib_none().0 as *mut _, column,
                text.0, -1)
        }
    }

    pub fn remove(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_remove(self.pointer, iter.to_glib_none_mut().0))
        }
    }

    pub fn insert(&self, parent: Option<&TreeIter>, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert(self.pointer, iter.to_glib_none_mut().0,
                parent.to_glib_none().0 as *mut _, position);
            iter
        }
    }

    pub fn insert_before(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_before(self.pointer,
                                              iter.to_glib_none_mut().0,
                                              parent.to_glib_none().0 as *mut _,
                                              sibling.to_glib_none().0 as *mut _);
            iter
        }
    }

    pub fn insert_after(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_after(self.pointer,
                                             iter.to_glib_none_mut().0,
                                             parent.to_glib_none().0 as *mut _,
                                             sibling.to_glib_none().0 as *mut _);
            iter
        }
    }

    pub fn prepend(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_prepend(self.pointer,
                                        iter.to_glib_none_mut().0,
                                        parent.to_glib_none().0 as *mut _);
            iter
        }
    }

    pub fn append(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_append(self.pointer,
                                       iter.to_glib_none_mut().0,
                                       parent.to_glib_none().0 as *mut _);
            iter
        }
    }

    pub fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_is_ancestor(self.pointer,
                                                      iter.to_glib_none().0 as *mut _,
                                                      descendant.to_glib_none().0 as *mut _))
        }
    }

    pub fn iter_depth(&self, iter: &TreeIter) -> i32 {
        unsafe { ffi::gtk_tree_store_iter_depth(self.pointer, iter.to_glib_none().0 as *mut _) }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_tree_store_clear(self.pointer) }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_iter_is_valid(self.pointer,
                                                        iter.to_glib_none().0 as *mut _))
        }
    }

    pub fn reorder(&self, parent: Option<&TreeIter>, new_order: &[u32]) {
        unsafe {
            let count = ffi::gtk_tree_model_iter_n_children(GTK_TREE_MODEL(self.pointer as *mut _),
                                                            parent.to_glib_none().0 as *mut _);
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
                ffi::gtk_tree_store_reorder(self.pointer,
                                            parent.to_glib_none().0 as *mut _,
                                            new_order.as_ptr() as *mut _);
            }
        }
    }

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_tree_store_swap(self.pointer,
                                     a.to_glib_none().0 as *mut _,
                                     b.to_glib_none().0 as *mut _)
        }
    }

    pub fn move_before(&self, iter: &mut TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_tree_store_move_before(self.pointer, iter.to_glib_none_mut().0,
                                                 position.to_glib_none().0 as *mut _) }
    }

    pub fn move_after(&self, iter: &mut TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_tree_store_move_before(self.pointer, iter.to_glib_none_mut().0,
                                                 position.to_glib_none().0 as *mut _) }
    }

    pub fn get_model(&self) -> Option<::TreeModel> {
        if self.pointer.is_null() {
            None
        } else {
            let tmp = ::cast::GTK_TREE_MODEL_FROM_TREE_STORE(self.pointer);

            unsafe { ::gobject_ffi::g_object_ref(tmp as *mut _) };
            Some(::TreeModel::wrap_pointer(tmp))
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &Value) {
        unsafe {
            ffi::gtk_tree_store_set_value(self.pointer, iter.to_glib_none().0 as *mut _,
                column, value.as_ptr() as *mut _);
        }
    }

    /*pub fn set_valuesv<T: ::traits::GValuePrivate>(&self, iter: &::TreeIter, columns: &[i32], values: &[T]) {
        let mut tmp_values = Vec::with_capacity(values.len());

        for value in values {
            tmp_values.push(value.get_gvalue());
        }
        unsafe { ffi::gtk_tree_store_set_valuesv(::cast::GTK_TREE_MODEL_FROM_TREE_STORE(self.pointer), iter.to_glib_none_mut().0,
            columns.as_ptr(), tmp_values.as_slice().as_ptr()) }
    }*/

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkTreeStore {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treestore: *mut ffi::GtkTreeStore) -> TreeStore {
        TreeStore {
            pointer: c_treestore
        }
    }
}

impl_drop!(TreeStore, GTK_TREE_STORE);
