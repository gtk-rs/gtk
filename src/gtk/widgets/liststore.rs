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

    pub fn set_column_text(&self, iter: &gtk::TreeIter, column: i32, text: &str) {
        let text_c = text.to_c_str();
        unsafe { ffi::gtk_list_store_set(self.pointer, iter.get_pointer(), column, text_c.as_ptr(), -1i) }
    }

    // TODO: remove

    pub fn insert(&self, iter: &gtk::TreeIter, position: i32) {
        unsafe { ffi::gtk_list_store_insert(self.pointer, iter.get_pointer(), position) }
    }

    // TODO: insert_before

    // TODO: insert_after

    // TODO: prepend

    pub fn append(&self, iter: &gtk::TreeIter) {
        unsafe { ffi::gtk_list_store_append(self.pointer, iter.get_pointer()) }
    }

    // TODO: clear

    // TODO: iter_is_valid

    // TODO: reorder

    // TODO: swap

    // TODO: move_before

    // TODO: move_after

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
