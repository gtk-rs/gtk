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

use gtk::{self, ffi};
use gtk::cast::GTK_COMBO_BOX_TEXT;
use glib::translate::{ToGlibPtr, ToTmp};
use libc::c_char;

struct_Widget!(ComboBoxText);

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
            let mut tmp_id = id.to_tmp_for_borrow();
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_combo_box_text_append(GTK_COMBO_BOX_TEXT(self.pointer),
                                           tmp_id.to_glib_ptr(),
                                           tmp_text.to_glib_ptr())
        }
    }

    pub fn prepend(&self, id: &str, text: &str) {
        unsafe {
            let mut tmp_id = id.to_tmp_for_borrow();
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_combo_box_text_prepend(GTK_COMBO_BOX_TEXT(self.pointer),
                                            tmp_id.to_glib_ptr(),
                                            tmp_text.to_glib_ptr())
        }
    }

    pub fn insert(&self, position: i32, id: &str, text: &str) {
        unsafe {
            let mut tmp_id = id.to_tmp_for_borrow();
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_combo_box_text_insert(GTK_COMBO_BOX_TEXT(self.pointer),
                                           position,
                                           tmp_id.to_glib_ptr(),
                                           tmp_text.to_glib_ptr())
        }
    }

    pub fn append_text(&self, text: &str) {
        unsafe {
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_combo_box_text_append_text(GTK_COMBO_BOX_TEXT(self.pointer), tmp_text.to_glib_ptr())
        }
    }

    pub fn prepend_text(&self, text: &str) {
        unsafe {
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_combo_box_text_prepend_text(GTK_COMBO_BOX_TEXT(self.pointer), tmp_text.to_glib_ptr())
        }
    }

    pub fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_combo_box_text_insert_text(GTK_COMBO_BOX_TEXT(self.pointer), position, tmp_text.to_glib_ptr())
        }
    }

    pub fn remove(&self, position: i32) {
        unsafe { ffi::gtk_combo_box_text_remove(GTK_COMBO_BOX_TEXT(self.pointer), position) }
    }

    pub fn remove_all(&self) {
        unsafe { ffi::gtk_combo_box_text_remove_all(GTK_COMBO_BOX_TEXT(self.pointer)) }
    }

    pub fn get_active_text(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_combo_box_text_get_active_text(GTK_COMBO_BOX_TEXT(self.pointer)) as *const c_char };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp)).to_string()) }
        }
    }
}

impl_drop!(ComboBoxText);
impl_TraitWidget!(ComboBoxText);

impl gtk::ContainerTrait for ComboBoxText {}
impl gtk::BinTrait for ComboBoxText {}
impl gtk::ComboBoxTrait for ComboBoxText {}

impl_widget_events!(ComboBoxText);
