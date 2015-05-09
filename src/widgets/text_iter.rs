// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTextIter — Text buffer iterator

use ffi;
use glib::{to_bool, to_gboolean};
use glib::translate::{from_glib_none};

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

    pub fn get_buffer(&self) -> Option<::TextBuffer> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_get_buffer(self.pointer as *const ffi::C_GtkTextIter) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn copy(&self) -> Option<TextIter> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_copy(self.pointer as *const ffi::C_GtkTextIter) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextIter {
                pointer: tmp_pointer
            })
        }
    }

    pub fn assign(&self, other: &TextIter) {
        unsafe { ffi::gtk_text_iter_assign(self.pointer, other.pointer as *const ffi::C_GtkTextIter) }
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
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_slice(
                    self.pointer as *const ffi::C_GtkTextIter,
                    end.pointer as *const ffi::C_GtkTextIter))
        }
    }

    pub fn get_text(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_text(
                    self.pointer as *const ffi::C_GtkTextIter,
                    end.pointer as *const ffi::C_GtkTextIter))
        }
    }

    pub fn get_visible_slice(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_visible_slice(
                    self.pointer as *const ffi::C_GtkTextIter,
                    end.pointer as *const ffi::C_GtkTextIter))
        }
    }

    pub fn get_visible_text(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_visible_text(
                    self.pointer as *const ffi::C_GtkTextIter,
                    end.pointer as *const ffi::C_GtkTextIter))
        }
    }

    pub fn get_child_anchor(&self) -> Option<::TextChildAnchor> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_get_child_anchor(self.pointer as *const ffi::C_GtkTextIter) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::TextChildAnchor::wrap_pointer(tmp_pointer))
        }
    }

    pub fn begins_tag(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_begins_tag(self.pointer as *const ffi::C_GtkTextIter, tag.unwrap_pointer())) }
    }

    pub fn ends_tag(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_tag(self.pointer as *const ffi::C_GtkTextIter, tag.unwrap_pointer())) }
    }

    pub fn toggles_tag(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_tag(self.pointer as *const ffi::C_GtkTextIter, tag.unwrap_pointer())) }
    }

    pub fn has_tag(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_tag(self.pointer as *const ffi::C_GtkTextIter, tag.unwrap_pointer())) }
    }

    pub fn editable(&self, default_setting: bool) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_editable(self.pointer as *const ffi::C_GtkTextIter, to_gboolean(default_setting))) }
    }

    pub fn can_insert(&self, default_setting: bool) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_can_insert(self.pointer as *const ffi::C_GtkTextIter, to_gboolean(default_setting))) }
    }

    pub fn starts_word(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_starts_word(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn ends_word(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_word(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn inside_word(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_word(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn starts_line(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_starts_line(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn ends_line(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_line(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn starts_sentence(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_starts_sentence(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn ends_sentence(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_sentence(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn inside_sentence(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_inside_sentence(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn is_cursor_position(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_is_cursor_position(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn get_chars_in_line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_chars_in_line(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_bytes_in_line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_bytes_in_line(self.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn get_attributes(&self, values: &::TextAttributes) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_get_attributes(self.pointer as *const ffi::C_GtkTextIter, values.unwrap_pointer())) }
    }

    pub fn is_end(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_is_end(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn is_start(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_is_start(self.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn forward_char(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_char(self.pointer)) }
    }

    pub fn backward_char(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_char(self.pointer)) }
    }

    pub fn forward_chars(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_chars(self.pointer, count as ::libc::c_int)) }
    }

    pub fn backward_chars(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_chars(self.pointer, count as ::libc::c_int)) }
    }

    pub fn forward_line(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_line(self.pointer)) }
    }

    pub fn backward_line(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_line(self.pointer)) }
    }

    pub fn forward_lines(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_lines(self.pointer, count as ::libc::c_int)) }
    }

    pub fn backward_lines(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_lines(self.pointer, count as ::libc::c_int)) }
    }

    pub fn forward_word_ends(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_word_ends(self.pointer, count as ::libc::c_int)) }
    }

    pub fn forward_word_starts(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_word_starts(self.pointer, count as ::libc::c_int)) }
    }

    pub fn forward_word_end(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_word_end(self.pointer)) }
    }

    pub fn backward_word_start(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_word_start(self.pointer)) }
    }

    pub fn forward_cursor_position(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_cursor_position(self.pointer)) }
    }

    pub fn backward_cursor_position(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_cursor_position(self.pointer)) }
    }

    pub fn forward_cursor_positions(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_cursor_positions(self.pointer, count as ::libc::c_int)) }
    }

    pub fn backward_cursor_positions(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_cursor_positions(self.pointer, count as ::libc::c_int)) }
    }

    pub fn backward_sentence_start(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_sentence_start(self.pointer)) }
    }

    pub fn backward_sentence_starts(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_sentence_starts(self.pointer, count as ::libc::c_int)) }
    }

    pub fn backward_sentence_end(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_sentence_end(self.pointer)) }
    }

    pub fn backward_sentence_ends(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_sentence_ends(self.pointer, count as ::libc::c_int)) }
    }

    pub fn forward_visible_word_ends(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_word_ends(self.pointer, count as ::libc::c_int)) }
    }

    pub fn forward_visible_word_starts(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_word_starts(self.pointer, count as ::libc::c_int)) }
    }

    pub fn forward_visible_word_end(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_word_end(self.pointer)) }
    }

    pub fn forward_visible_word_start(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_word_start(self.pointer)) }
    }

    pub fn forward_visible_cursor_position(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_cursor_position(self.pointer)) }
    }

    pub fn backward_visible_cursor_position(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_cursor_position(self.pointer)) }
    }

    pub fn forward_visible_cursor_positions(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_cursor_positions(self.pointer, count as ::libc::c_int)) }
    }

    pub fn backward_visible_cursor_positions(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_cursor_positions(self.pointer, count as ::libc::c_int)) }
    }

    pub fn forward_visible_line(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_line(self.pointer)) }
    }

    pub fn backward_visible_line(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_line(self.pointer)) }
    }

    pub fn forward_visible_lines(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_lines(self.pointer, count as ::libc::c_int)) }
    }

    pub fn backward_visible_lines(&self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_lines(self.pointer, count as ::libc::c_int)) }
    }

    pub fn set_offset(&self, char_offset: i32) {
        unsafe { ffi::gtk_text_iter_set_offset(self.pointer, char_offset as ::libc::c_int) }
    }

    pub fn set_line(&self, line_number: i32) {
        unsafe { ffi::gtk_text_iter_set_line(self.pointer, line_number as ::libc::c_int) }
    }

    pub fn set_line_offset(&self, char_on_line: i32) {
        unsafe { ffi::gtk_text_iter_set_line_offset(self.pointer, char_on_line as ::libc::c_int) }
    }

    pub fn set_line_index(&self, byte_on_line: i32) {
        unsafe { ffi::gtk_text_iter_set_line_index(self.pointer, byte_on_line as ::libc::c_int) }
    }

    pub fn set_visible_line_index(&self, byte_on_line: i32) {
        unsafe { ffi::gtk_text_iter_set_visible_line_index(self.pointer, byte_on_line as ::libc::c_int) }
    }

    pub fn set_visible_line_offset(&self, char_on_line: i32) {
        unsafe { ffi::gtk_text_iter_set_visible_line_offset(self.pointer, char_on_line as ::libc::c_int) }
    }

    pub fn forward_to_end(&self) {
        unsafe { ffi::gtk_text_iter_forward_to_end(self.pointer) }
    }

    pub fn forward_to_line_end(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_to_line_end(self.pointer)) }
    }

    pub fn forward_to_tag_toggle(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_to_tag_toggle(self.pointer, tag.unwrap_pointer())) }
    }

    pub fn backward_to_tag_toggle(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_to_tag_toggle(self.pointer, tag.unwrap_pointer())) }
    }

    pub fn is_equal_to(&self, other: &TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_equal(self.pointer as *const ffi::C_GtkTextIter,
            other.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn compare_to(&self, other: &TextIter) -> i32 {
        unsafe { ffi::gtk_text_iter_compare(self.pointer as *const ffi::C_GtkTextIter, other.pointer as *const ffi::C_GtkTextIter) }
    }

    pub fn in_range(&self, start: &TextIter, end: &TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_in_range(self.pointer as *const ffi::C_GtkTextIter,
            start.pointer as *const ffi::C_GtkTextIter, end.pointer as *const ffi::C_GtkTextIter)) }
    }

    pub fn order(&self, second: &TextIter) {
        unsafe { ffi::gtk_text_iter_order(self.pointer, second.pointer) }
    }

    pub fn drop(&mut self) {
        unsafe { ffi::gtk_text_iter_free(self.pointer) };
        self.pointer = ::std::ptr::null_mut();
    }
}

impl_GObjectFunctions!(TextIter, C_GtkTextIter);
impl_TraitObject!(TextIter, C_GtkTextIter);
