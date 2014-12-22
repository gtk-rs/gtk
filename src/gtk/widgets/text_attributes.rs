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

//! GtkTextTag — A tag that can be applied to text in a GtkTextBuffer

use gtk::ffi;

#[deriving(Copy)]
pub struct TextAttributes {
    pointer: *mut ffi::C_GtkTextAttributes
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

impl_GObjectFunctions!(TextAttributes, C_GtkTextAttributes);