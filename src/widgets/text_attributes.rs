// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTextTag — A tag that can be applied to text in a GtkTextBuffer

use ffi;

pub struct TextAttributes {
    pointer: *mut ffi::GtkTextAttributes
}

impl TextAttributes {
    pub fn new() -> Option<TextAttributes> {
        let tmp_pointer = unsafe { ffi::gtk_text_attributes_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextAttributes { pointer : tmp_pointer })
        }
    }

    pub fn copy(&self) -> Option<TextAttributes> {
        let tmp_pointer = unsafe { ffi::gtk_text_attributes_copy(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextAttributes { pointer : tmp_pointer })
        }
    }

    pub fn copy_values_from(&self, src: &TextAttributes) {
        unsafe { ffi::gtk_text_attributes_copy_values(src.pointer, self.pointer) }
    }

    pub fn unref(&self) {
        unsafe { ffi::gtk_text_attributes_unref(self.pointer) }
    }

    pub fn _ref(&self) -> Option<TextAttributes> {
        let tmp_pointer = unsafe { ffi::gtk_text_attributes_ref(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextAttributes { pointer : tmp_pointer })
        }
    }
}

impl_GObjectFunctions!(TextAttributes, GtkTextAttributes);
impl_TraitObject!(TextAttributes, GtkTextAttributes);