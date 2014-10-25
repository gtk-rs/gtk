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

//! GtkTextIter — Text buffer iterator

use gtk::ffi;

pub struct TextIter {
    pointer: *mut ffi::C_GtkTextIter
}

impl TextIter {
    pub fn new() -> Option<TextIter> {
        let iter = TextIter {
            pointer: unsafe { ::std::mem::uninitialized() }
        };
        iter.copy()
    }

    pub fn get_buffer(&self) -> Option<gtk::TextBuffer> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_iter_get_buffer(self.pointer)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn copy(&self) -> Option<TextIter> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_copy(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreeIter {
                pointer: tmp_pointer
            })
        }
    }

    pub fn assign(&self, other: &TextIter) {
        unsafe { ffi::gtk_text_iter_assign(self.pointer, other.pointer) }
    }

    pub fn get_offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_offset(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_line_offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line_offset(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_line_index(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line_index(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_visible_line_index(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_visible_line_index(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_visible_line_offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_visible_line_offset(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_char(&self) -> u32 {
        unsafe { ffi::gtk_text_iter_get_char(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_slice(&self, end: &TextIter) -> Option<String> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_get_slice(self.pointer, end.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe { Some(::std::string::raw::from_buf(tmp_pointer as *const u8)) }
        }
    }

    pub fn get_text(&self, end: &TextIter) -> Option<String> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_get_text(self.pointer, end.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe { Some(::std::string::raw::from_buf(tmp_pointer as *const u8)) }
        }
    }

    pub fn get_visible_slice(&self, end: &TextIter) -> Option<String> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_get_visible_slice(self.pointer, end.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe { Some(::std::string::raw::from_buf(tmp_pointer as *const u8)) }
        }
    }

    pub fn get_visible_text(&self, end: &TextIter) -> Option<String> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_get_visible_text(self.pointer, end.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe { Some(::std::string::raw::from_buf(tmp_pointer as *const u8)) }
        }
    }

    pub fn begins_tag(&self, tag: &gtk::TextTag) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_iter_begins_tag(self.pointer as *const ffi::C_GtkTextIter, tag.get_pointer())) }
    }

    pub fn ends_tag(&self, tag: &gtk::TextTag) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_iter_ends_tag(self.pointer as *const ffi::C_GtkTextIter, tag.get_pointer())) }
    }

    pub fn toggles_tag(&self, tag: &gtk::TextTag) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_iter_ends_tag(self.pointer as *const ffi::C_GtkTextIter, tag.get_pointer())) }
    }

    pub fn drop(&mut self) {
        unsafe { ffi::gtk_text_iter_free(self.pointer) };
        self.pointer = ::std::ptr::null_mut();
    }
}

impl_TraitGObject!(TextIter, C_GtkTextIter)