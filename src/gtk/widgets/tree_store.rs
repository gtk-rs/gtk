// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use glib::{to_bool, Value, Type};
use glib::translate::ToGlib;
use gtk::{self, ffi};
use gtk::TreeIter;
use std::ffi::CString;
use std::num::ToPrimitive;

pub struct TreeStore {
    pointer: *mut ffi::C_GtkTreeStore
}

impl TreeStore {
    pub fn new(column_types: &[Type]) -> Option<TreeStore> {
        let column_types_ffi: Vec<::glib_ffi::GType> = column_types.iter().map(|n| n.to_glib()).collect();
        let tmp_pointer = unsafe { ffi::gtk_tree_store_newv(column_types.len().to_i32().unwrap(), column_types_ffi.as_slice()) };
        check_pointer!(tmp_pointer, TreeStore, G_OBJECT_FROM_TREE_STORE)
    }

    pub fn set_column_types(&self, column_types: &[Type]) {
        let column_types_ffi: Vec<::glib_ffi::GType> = column_types.iter().map(|n| n.to_glib()).collect();
        unsafe { ffi::gtk_tree_store_set_column_types(self.pointer, column_types.len().to_i32().unwrap(), column_types_ffi.as_slice()) }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        let text_c = CString::from_slice(text.as_bytes());

        unsafe { ffi::gtk_tree_store_set(self.pointer, iter.unwrap_pointer(), column, text_c.as_ptr(), -1) }
    }

    pub fn remove(&self, iter: &TreeIter) -> bool {
        unsafe { to_bool(ffi::gtk_tree_store_remove(self.pointer, iter.unwrap_pointer())) }
    }

    pub fn insert(&self, iter: &mut TreeIter, parent: Option<&TreeIter>, position: i32) {
        unsafe { ffi::gtk_tree_store_insert(self.pointer,
                                            iter.unwrap_pointer(),
                                            if parent.is_none() { ::std::ptr::null_mut() } else { parent.unwrap().unwrap_pointer() },
                                            position) }
    }

    pub fn insert_before(&self, iter: &mut TreeIter, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) {
        unsafe { ffi::gtk_tree_store_insert_before(self.pointer,
                                                   iter.unwrap_pointer(),
                                                   if parent.is_none() { ::std::ptr::null_mut() } else { parent.unwrap().unwrap_pointer() },
                                                   if sibling.is_none() { ::std::ptr::null_mut() } else { sibling.unwrap().unwrap_pointer() }) }
    }

    pub fn insert_after(&self, iter: &mut TreeIter, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) {
        unsafe { ffi::gtk_tree_store_insert_after(self.pointer,
                                                  iter.unwrap_pointer(),
                                                  if parent.is_none() { ::std::ptr::null_mut() } else { parent.unwrap().unwrap_pointer() },
                                                  if sibling.is_none() { ::std::ptr::null_mut() } else { sibling.unwrap().unwrap_pointer() }) }
    }

    pub fn prepend(&self, iter: &mut TreeIter, parent: Option<&TreeIter>) {
        unsafe { ffi::gtk_tree_store_prepend(self.pointer, iter.unwrap_pointer(),
                                             if parent.is_none() { ::std::ptr::null_mut() } else { parent.unwrap().unwrap_pointer() }) }
    }

    pub fn append(&self, iter: &mut TreeIter, parent: Option<&TreeIter>) {
        unsafe { ffi::gtk_tree_store_append(self.pointer, iter.unwrap_pointer(),
                                            if parent.is_none() { ::std::ptr::null_mut() } else { parent.unwrap().unwrap_pointer() }) }
    }

    pub fn is_ancestor(&self, iter: &TreeIter, descendent: &TreeIter) -> bool {
        unsafe { to_bool(ffi::gtk_tree_store_is_ancestor(self.pointer, iter.unwrap_pointer(), descendent.unwrap_pointer())) }
    }

    pub fn iter_depth(&self, iter: &TreeIter) -> i32 {
        unsafe { ffi::gtk_tree_store_iter_depth(self.pointer, iter.unwrap_pointer()) }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_tree_store_clear(self.pointer) }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe { to_bool(ffi::gtk_tree_store_iter_is_valid(self.pointer, iter.unwrap_pointer())) }
    }

    pub fn reorder(&self, parent: &TreeIter, new_order: *mut i32) {
        unsafe { ffi::gtk_tree_store_reorder(self.pointer, parent.unwrap_pointer(), new_order) }
    }

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe { ffi::gtk_tree_store_swap(self.pointer, a.unwrap_pointer(), b.unwrap_pointer()) }
    }

    pub fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_tree_store_move_before(self.pointer, iter.unwrap_pointer(),
                                                 if position.is_none() { ::std::ptr::null_mut() } else { position.unwrap().unwrap_pointer() }) }
    }

    pub fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_tree_store_move_before(self.pointer, iter.unwrap_pointer(),
                                                 if position.is_none() { ::std::ptr::null_mut() } else { position.unwrap().unwrap_pointer() }) }
    }

    pub fn get_model(&self) -> Option<gtk::TreeModel> {
        if self.pointer.is_null() {
            None
        } else {
            Some(gtk::TreeModel::wrap_pointer(gtk::cast::GTK_TREE_MODEL_FROM_TREE_STORE(self.pointer)))
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &Value) {
        unsafe { ffi::gtk_tree_store_set_value(self.pointer, iter.unwrap_pointer(), column, value.unwrap_pointer()) }
    }

    /*pub fn set_valuesv<T: gtk::traits::GValuePrivate>(&self, iter: &gtk::TreeIter, columns: &[i32], values: &[T]) {
        let mut tmp_values = Vec::with_capacity(values.len());

        for value in values {
            tmp_values.push(value.get_gvalue());
        }
        unsafe { ffi::gtk_tree_store_set_valuesv(gtk::cast::GTK_TREE_MODEL_FROM_TREE_STORE(self.pointer), iter.unwrap_pointer(),
            columns.as_ptr(), tmp_values.as_slice().as_ptr()) }
    }*/

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkTreeStore {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treestore: *mut ffi::C_GtkTreeStore) -> TreeStore {
        TreeStore {
            pointer: c_treestore
        }
    }
}

impl_drop!(TreeStore, GTK_TREE_STORE);
