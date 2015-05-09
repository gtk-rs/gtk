// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::{to_bool, Value, Type};
use glib::translate::ToGlib;
use ffi;
use TreeIter;
use glib::translate::ToGlibPtr;
use glib_ffi::{self, GType};
use libc::c_void;

pub struct ListStore {
    pointer: *mut ffi::C_GtkListStore
}

impl ListStore {
    pub fn new(column_types: &[Type]) -> Option<ListStore> {
        let column_types_ffi: Vec<GType> = column_types.iter().map(|n| n.to_glib()).collect();
        let tmp_pointer = unsafe { ffi::gtk_list_store_newv(column_types.len() as i32, column_types_ffi.as_ptr() as *mut GType) };
        check_pointer!(tmp_pointer, ListStore, G_OBJECT_FROM_LIST_STORE)
    }

    pub fn set_column_types(&self, column_types: &[Type]) {
        let column_types_ffi: Vec<GType> = column_types.iter().map(|n| n.to_glib()).collect();
        unsafe { ffi::gtk_list_store_set_column_types(self.pointer, column_types.len() as i32, column_types_ffi.as_ptr() as *mut GType) }
    }

    pub fn set_string(&self, iter: &TreeIter, column: i32, text: &str) {
        unsafe {
            ffi::gtk_list_store_set(self.pointer, iter.unwrap_pointer(), column, text.borrow_to_glib().0, -1)
        }
    }

    pub fn remove(&self, iter: &TreeIter) -> bool {
        unsafe { to_bool(ffi::gtk_list_store_remove(self.pointer, iter.unwrap_pointer())) }
    }

    pub fn insert(&self, iter: &mut TreeIter, position: i32) {
        unsafe { ffi::gtk_list_store_insert(self.pointer, iter.unwrap_pointer(), position) }
    }

    pub fn insert_before(&self, iter: &mut TreeIter, sibling: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_insert_before(self.pointer, iter.unwrap_pointer(),
                                                   if sibling.is_none() { ::std::ptr::null_mut()} else { sibling.unwrap().unwrap_pointer() }) }
    }

    pub fn insert_after(&self, iter: &mut TreeIter, sibling: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_insert_after(self.pointer, iter.unwrap_pointer(),
                                                  if sibling.is_none() { ::std::ptr::null_mut()} else { sibling.unwrap().unwrap_pointer() }) }
    }

    pub fn prepend(&self, iter: &mut TreeIter) {
        unsafe { ffi::gtk_list_store_prepend(self.pointer, iter.unwrap_pointer()) }
    }

    pub fn append(&self, iter: &mut TreeIter) {
        unsafe { ffi::gtk_list_store_append(self.pointer, iter.unwrap_pointer()) }
    }

    pub fn clear(&self) {
        unsafe { ffi::gtk_list_store_clear(self.pointer) }
    }

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe { to_bool(ffi::gtk_list_store_iter_is_valid(self.pointer, iter.unwrap_pointer())) }
    }

    pub fn reorder(&self, new_order: *mut i32) {
        unsafe { ffi::gtk_list_store_reorder(self.pointer, new_order) }
    }

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe { ffi::gtk_list_store_swap(self.pointer, a.unwrap_pointer(), b.unwrap_pointer()) }
    }

    pub fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_move_before(self.pointer, iter.unwrap_pointer(),
                                                 if position.is_none() { ::std::ptr::null_mut() } else { position.unwrap().unwrap_pointer() }) }
    }

    pub fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe { ffi::gtk_list_store_move_before(self.pointer, iter.unwrap_pointer(),
                                                 if position.is_none() { ::std::ptr::null_mut() } else { position.unwrap().unwrap_pointer() }) }
    }

    pub fn get_model(&self) -> Option<::TreeModel> {
        if self.pointer.is_null() {
            None
        } else {
            let tmp = ::cast::GTK_TREE_MODEL_FROM_LIST_STORE(self.pointer);

            unsafe { glib_ffi::g_object_ref(tmp as *mut c_void) };
            Some(::TreeModel::wrap_pointer(tmp))
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: i32, value: &Value) {
        unsafe {
            ffi::gtk_list_store_set_value(self.pointer, iter.unwrap_pointer(),
                column, value.as_ptr() as *mut _);
        }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkListStore {
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
