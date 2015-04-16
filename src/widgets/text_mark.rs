// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTextMark — A position in the buffer preserved across buffer modifications

use ffi;
use glib::translate::{FromGlibPtr, ToGlibPtr};
use glib::{to_bool, to_gboolean};

pub struct TextMark {
    pointer: *mut ffi::C_GtkTextMark
}

impl TextMark {
    pub fn new(name: &str, left_gravity: bool) -> Option<TextMark> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_mark_new(name.borrow_to_glib().0, to_gboolean(left_gravity))
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextMark { pointer: tmp_pointer })
        }
    }

    pub fn set_visible(&self, setting: bool) {
        unsafe { ffi::gtk_text_mark_set_visible(self.pointer, to_gboolean(setting)) }
    }

    pub fn get_visible(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_mark_get_visible(self.pointer)) }
    }

    pub fn get_deleted(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_mark_get_deleted(self.pointer)) }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_text_mark_get_name(self.pointer))
        }
    }

    pub fn get_buffer(&self) -> Option<::TextBuffer> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_mark_get_buffer(self.pointer)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn get_left_gravity(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_mark_get_left_gravity(self.pointer)) }
    }
}

impl_GObjectFunctions!(TextMark, C_GtkTextMark);
impl_TraitObject!(TextMark, C_GtkTextMark);
impl_drop!(TextMark, GTK_TEXT_MARK);
