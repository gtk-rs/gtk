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
use gtk;
use gtk::ffi;

pub struct ListStore {
    pointer: *mut ffi::C_GtkListStore
}

impl ListStore {
    pub fn new(column_types: Vec<GType>) -> Option<ListStore> {
        let tmp_pointer = unsafe { ffi::gtk_list_store_newv(column_types.len().to_i32().unwrap(), column_types.as_slice()) };
        check_pointer!(tmp_pointer, ListStore)
    }

    pub fn set_column_types(&self, column_types: Vec<GType>) {
        unsafe { ffi::gtk_list_store_set_column_types(self.pointer, column_types.len().to_i32().unwrap(), column_types.as_slice()) }
    }

    pub fn set_column_text(&self, iter: &mut ffi::C_GtkTreeIter, column: i32, text: &str) {
        let text_c = text.to_c_str();
        unsafe { ffi::gtk_list_store_set(self.pointer, iter, column, text_c.as_ptr(), -1i) }
    }

    pub fn remove(&self, iter: &mut ffi::C_GtkTreeIter) -> bool {
        unsafe { ffi::gtk_list_store_remove(self.pointer, iter) }
    }

    pub fn insert(&self, iter: &mut ffi::C_GtkTreeIter, position: i32) {
        unsafe { ffi::gtk_list_store_insert(self.pointer, iter, position) }
    }

    pub fn insert_before(&self, iter: &mut ffi::C_GtkTreeIter, sibling: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_list_store_insert_before(self.pointer, iter, sibling) }
    }

    pub fn insert_after(&self, iter: &mut ffi::C_GtkTreeIter, sibling: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_list_store_insert_after(self.pointer, iter, sibling) }
    }

    pub fn prepend(&self, iter: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_list_store_prepend(self.pointer, iter) }
    }

    pub fn append(&self, iter: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_list_store_append(self.pointer, iter) }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_list_store_clear(self.pointer) }
    }

    pub fn iter_is_valid(&self, iter: &mut ffi::C_GtkTreeIter) -> bool {
        unsafe { ffi::gtk_list_store_iter_is_valid(self.pointer, iter) }
    }

    pub fn reorder(&self, new_order: *mut i32) {
        unsafe { ffi::gtk_list_store_reorder(self.pointer, new_order) }
    }

    pub fn swap(&self, a: &mut ffi::C_GtkTreeIter, b: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_list_store_swap(self.pointer, a, b) }
    }

    pub fn move_before(&self, iter: &mut ffi::C_GtkTreeIter, position: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_list_store_move_before(self.pointer, iter, position) }
    }

    pub fn move_after(&self, iter: &mut ffi::C_GtkTreeIter, position: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_list_store_move_before(self.pointer, iter, position) }
    }

    pub fn get_model(&self) -> Option<gtk::TreeModel> {
        if self.pointer.is_null() {
            None
        } else {
            Some(gtk::TreeModel::wrap_pointer(gtk::cast::GTK_TREE_MODEL_FROM_LIST_STORE(self.pointer)))
        }
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
