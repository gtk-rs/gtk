// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkEntryCompletion — Completion functionality for GtkEntry

use ffi;
use TreeModel;
use cast::GTK_ENTRY_COMPLETION;
use glib::translate::{from_glib_none, ToGlibPtr};

struct_Widget!(EntryCompletion);

impl EntryCompletion {
    pub fn new() -> Option<EntryCompletion> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_entry_completion_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(EntryCompletion {
                    pointer: tmp_pointer as *mut ffi::GtkWidget
                }
            )
        }
    }

    pub unsafe fn get_entry<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = ffi::gtk_entry_completion_get_entry(GTK_ENTRY_COMPLETION(self.pointer));

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_model(&self, model: &TreeModel) {
        unsafe { ffi::gtk_entry_completion_set_model(GTK_ENTRY_COMPLETION(self.pointer), model.unwrap_pointer()) }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            let ptr = ffi::gtk_entry_completion_get_model(GTK_ENTRY_COMPLETION(self.pointer));
            if ptr.is_null() {
                None
            } else {
                Some(TreeModel::wrap_pointer(ptr))
            }
        }
    }

    pub fn set_minimum_key_length(&self, length: i32) {
        unsafe { ffi::gtk_entry_completion_set_minimum_key_length(GTK_ENTRY_COMPLETION(self.pointer), length) }
    }

    pub fn get_minimum_key_length(&self) -> i32 {
        unsafe { ffi::gtk_entry_completion_get_minimum_key_length(GTK_ENTRY_COMPLETION(self.pointer)) }
    }

    pub fn compute_prefix(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_compute_prefix(
                    GTK_ENTRY_COMPLETION(self.pointer),
                    key.to_glib_none().0))
        }
    }

    pub fn complete(&self) {
        unsafe { ffi::gtk_entry_completion_complete(GTK_ENTRY_COMPLETION(self.pointer)) }
    }

    pub fn get_completion_prefix(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_entry_completion_get_completion_prefix(GTK_ENTRY_COMPLETION(self.pointer)))
        }
    }

    pub fn insert_prefix(&self) {
        unsafe { ffi::gtk_entry_completion_insert_prefix(GTK_ENTRY_COMPLETION(self.pointer)) }
    }

    pub fn insert_action_text(&self, index_: i32, text: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_text(GTK_ENTRY_COMPLETION(self.pointer), index_, text.to_glib_none().0)
        }
    }

    pub fn insert_action_markup(&self, index_: i32, markup: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_markup(GTK_ENTRY_COMPLETION(self.pointer), index_, markup.to_glib_none().0)
        }
    }

    pub fn delete_action(&self, index_: i32) {
        unsafe { ffi::gtk_entry_completion_delete_action(GTK_ENTRY_COMPLETION(self.pointer), index_) }
    }

    pub fn set_text_column(&self, column: i32) {
        unsafe { ffi::gtk_entry_completion_set_text_column(GTK_ENTRY_COMPLETION(self.pointer), column) }
    }

    pub fn get_text_column(&self) -> i32 {
        unsafe { ffi::gtk_entry_completion_get_text_column(GTK_ENTRY_COMPLETION(self.pointer)) }
    }

    pub fn set_inline_completion(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_inline_completion(GTK_ENTRY_COMPLETION(self.pointer), match inline_completion {
            true => 1,
            false => 0
        }) }
    }

    pub fn get_inline_completion(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_inline_completion(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_inline_selection(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_inline_selection(GTK_ENTRY_COMPLETION(self.pointer), match inline_completion {
            true => 1,
            false => 0
        }) }
    }

    pub fn get_inline_selection(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_inline_selection(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_popup_completion(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_inline_selection(GTK_ENTRY_COMPLETION(self.pointer), match inline_completion {
            true => 1,
            false => 0
        }) }
    }

    pub fn get_popup_completion(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_inline_selection(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_popup_set_width(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_popup_set_width(GTK_ENTRY_COMPLETION(self.pointer), match inline_completion {
            true => 1,
            false => 0
        }) }
    }

    pub fn get_popup_set_width(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_popup_set_width(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_popup_single_match(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_popup_single_match(GTK_ENTRY_COMPLETION(self.pointer),
                                                                  match inline_completion {
                                                                      true => 1,
                                                                      false => 0
                                                                  })
        }
    }

    pub fn get_popup_single_match(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_popup_single_match(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }
}

impl_drop!(EntryCompletion);
impl_TraitWidget!(EntryCompletion);

impl ::CellLayoutTrait for EntryCompletion {}
