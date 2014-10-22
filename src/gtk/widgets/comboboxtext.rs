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

//! GtkComboBox — A widget used to choose from a list of items

use gtk::ffi;
use gtk::traits;
use gtk::cast::GTK_COMBO_BOX_TEXT;
use std::string;

struct_Widget!(ComboBoxText)

impl ComboBoxText {
    pub fn new() -> Option<ComboBoxText> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_text_new() };
        check_pointer!(tmp_pointer, ComboBoxText)
    }

    pub fn new_with_entry() -> Option<ComboBoxText> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_text_new_with_entry() };
        check_pointer!(tmp_pointer, ComboBoxText)
    }

    pub fn append(&self, id: &str, text: &str) {
        unsafe {
            id.with_c_str(|c_id| {
                text.with_c_str(|c_text| {
                    ffi::gtk_combo_box_text_append(GTK_COMBO_BOX_TEXT(self.pointer), c_id, c_text)
                })
            })
        }
    }

    pub fn prepend(&self, id: &str, text: &str) {
        unsafe {
            id.with_c_str(|c_id| {
                text.with_c_str(|c_text| {
                    ffi::gtk_combo_box_text_prepend(GTK_COMBO_BOX_TEXT(self.pointer), c_id, c_text)
                })
            })
        }
    }

    pub fn insert(&self, position: i32, id: &str, text: &str) {
        unsafe {
            id.with_c_str(|c_id| {
                text.with_c_str(|c_text| {
                    ffi::gtk_combo_box_text_insert(GTK_COMBO_BOX_TEXT(self.pointer), position, c_id, c_text)
                })
            })
        }
    }

    pub fn append_text(&self, text: &str) {
        unsafe {
            text.with_c_str(|c_text| {
                ffi::gtk_combo_box_text_append_text(GTK_COMBO_BOX_TEXT(self.pointer), c_text)
            })
        }
    }

    pub fn prepend_text(&self, text: &str) {
        unsafe {
            text.with_c_str(|c_text| {
                ffi::gtk_combo_box_text_prepend_text(GTK_COMBO_BOX_TEXT(self.pointer), c_text)
            })
        }
    }

    pub fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            text.with_c_str(|c_text| {
                ffi::gtk_combo_box_text_insert_text(GTK_COMBO_BOX_TEXT(self.pointer), position, c_text)
            })
        }
    }

    pub fn remove(&self, position: i32) {
        unsafe { ffi::gtk_combo_box_text_remove(GTK_COMBO_BOX_TEXT(self.pointer), position) }
    }

    pub fn remove_all(&self) {
        unsafe { ffi::gtk_combo_box_text_remove_all(GTK_COMBO_BOX_TEXT(self.pointer)) }
    }

    pub fn get_active_text(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_combo_box_text_get_active_text(GTK_COMBO_BOX_TEXT(self.pointer)) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(string::raw::from_buf(tmp as *const u8)) }
        }
    }
}

impl_drop!(ComboBoxText)
impl_TraitWidget!(ComboBoxText)

impl traits::Container for ComboBoxText {}
impl traits::Bin for ComboBoxText {}
impl traits::ComboBox for ComboBoxText {}
