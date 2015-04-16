// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkComboBox — A widget used to choose from a list of items

use ffi;
use cast::GTK_COMBO_BOX_TEXT;
use glib::translate::{FromGlibPtr, ToGlibPtr};

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
            ffi::gtk_combo_box_text_append(GTK_COMBO_BOX_TEXT(self.pointer),
                                           id.borrow_to_glib().0,
                                           text.borrow_to_glib().0)
        }
    }

    pub fn prepend(&self, id: &str, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend(GTK_COMBO_BOX_TEXT(self.pointer),
                                            id.borrow_to_glib().0,
                                            text.borrow_to_glib().0)
        }
    }

    pub fn insert(&self, position: i32, id: &str, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert(GTK_COMBO_BOX_TEXT(self.pointer),
                                           position,
                                           id.borrow_to_glib().0,
                                           text.borrow_to_glib().0)
        }
    }

    pub fn append_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append_text(GTK_COMBO_BOX_TEXT(self.pointer), text.borrow_to_glib().0)
        }
    }

    pub fn prepend_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend_text(GTK_COMBO_BOX_TEXT(self.pointer), text.borrow_to_glib().0)
        }
    }

    pub fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert_text(GTK_COMBO_BOX_TEXT(self.pointer), position, text.borrow_to_glib().0)
        }
    }

    pub fn remove(&self, position: i32) {
        unsafe { ffi::gtk_combo_box_text_remove(GTK_COMBO_BOX_TEXT(self.pointer), position) }
    }

    pub fn remove_all(&self) {
        unsafe { ffi::gtk_combo_box_text_remove_all(GTK_COMBO_BOX_TEXT(self.pointer)) }
    }

    pub fn get_active_text(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_combo_box_text_get_active_text(GTK_COMBO_BOX_TEXT(self.pointer)))
        }
    }
}

impl_drop!(ComboBoxText);
impl_TraitWidget!(ComboBoxText);

impl ::ContainerTrait for ComboBoxText {}
impl ::BinTrait for ComboBoxText {}
impl ::ComboBoxTrait for ComboBoxText {}

impl_widget_events!(ComboBoxText);
