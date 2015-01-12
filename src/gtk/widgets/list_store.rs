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

use glib::ffi::GType;
use gtk::{self, ffi};
use gtk::TreeIter;
use std::ffi::CString;
use std::num::ToPrimitive;

pub struct ListStore {
    pointer: *mut ffi::C_GtkListStore
}

impl ListStore {
    pub fn new(column_types: &[GType]) -> Option<ListStore> {
        let tmp_pointer = unsafe { ffi::gtk_list_store_newv(column_types.len().to_i32().unwrap(), column_types) };
        check_pointer!(tmp_pointer, ListStore, G_OBJECT_FROM_LIST_STORE)
    }

    pub fn set_column_types(&self, column_types: &[GType]) {
        unsafe { ffi::gtk_list_store_set_column_types(self.pointer, column_types.len().to_i32().unwrap(), column_types) }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        let text_c = text.to_c_str();
        unsafe { ffi::gtk_list_store_set(self.pointer, iter.get_pointer(), column, text_c.as_ptr(), -1is) }
    }

    pub fn remove(&self, iter: &TreeIter) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_list_store_remove(self.pointer, iter.get_pointer())) }
    }

    pub fn insert(&self, iter: &mut TreeIter, position: i32) {
        unsafe { ffi::gtk_list_store_insert(self.pointer, iter.get_pointer(), position) }
    }

    pub fn insert_before(&self, iter: &mut TreeIter, sibling: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_insert_before(self.pointer, iter.get_pointer(),
                                                   if sibling.is_none() { ::std::ptr::null_mut()} else { sibling.unwrap().get_pointer() }) }
    }

    pub fn insert_after(&self, iter: &mut TreeIter, sibling: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_insert_after(self.pointer, iter.get_pointer(),
                                                  if sibling.is_none() { ::std::ptr::null_mut()} else { sibling.unwrap().get_pointer() }) }
    }

    pub fn prepend(&self, iter: &mut TreeIter) {
        unsafe { ffi::gtk_list_store_prepend(self.pointer, iter.get_pointer()) }
    }

    pub fn append(&self, iter: &mut TreeIter) {
        unsafe { ffi::gtk_list_store_append(self.pointer, iter.get_pointer()) }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_list_store_clear(self.pointer) }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_list_store_iter_is_valid(self.pointer, iter.get_pointer())) }
    }

    pub fn reorder(&self, new_order: *mut i32) {
        unsafe { ffi::gtk_list_store_reorder(self.pointer, new_order) }
    }

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe { ffi::gtk_list_store_swap(self.pointer, a.get_pointer(), b.get_pointer()) }
    }

    pub fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_move_before(self.pointer, iter.get_pointer(),
                                                 if position.is_none() { ::std::ptr::null_mut() } else { position.unwrap().get_pointer() }) }
    }

    pub fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_move_before(self.pointer, iter.get_pointer(),
                                                 if position.is_none() { ::std::ptr::null_mut() } else { position.unwrap().get_pointer() }) }
    }

    pub fn get_model(&self) -> Option<gtk::TreeModel> {
        if self.pointer.is_null() {
            None
        } else {
            Some(gtk::TreeModel::wrap_pointer(gtk::cast::GTK_TREE_MODEL_FROM_LIST_STORE(self.pointer)))
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &gtk::GValue) {
        unsafe { ffi::gtk_list_store_set_value(self.pointer, iter.get_pointer(), column, value.unwrap_pointer()) }
    }

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *mut ffi::C_GtkListStore {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_liststore: *mut ffi::C_GtkListStore) -> ListStore {
        ListStore {
            pointer: c_liststore
        }
    }
}

impl_drop!(ListStore, GTK_LIST_STORE);
