// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTextIter — Text buffer iterator

use ffi;
use glib::{to_bool, to_gboolean};
use glib::boxed::{Boxed, BoxedMemoryManager};
use glib::translate::{Stash, StashMut, ToGlib, ToGlibPtr, ToGlibPtrMut, from_glib, from_glib_none};

#[doc(hidden)]
pub struct MM;

impl BoxedMemoryManager<ffi::GtkTextIter> for MM {
    #[inline]
    unsafe fn copy(ptr: *const ffi::GtkTextIter) -> *mut ffi::GtkTextIter {
        ffi::gtk_text_iter_copy(ptr)
    }

    #[inline]
    unsafe fn free(ptr: *mut ffi::GtkTextIter) {
        ffi::gtk_text_iter_free(ptr)
    }
}

pub struct TextIter(Boxed<ffi::GtkTextIter, MM>);

impl<'a> ToGlibPtr<'a, *const ffi::GtkTextIter> for &'a TextIter {
    type Storage = &'a Boxed<ffi::GtkTextIter, MM>;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const ffi::GtkTextIter, Self> {
        let stash = (&self.0).to_glib_none();
        Stash(stash.0, stash.1)
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::GtkTextIter> for TextIter {
    type Storage = &'a mut Boxed<ffi::GtkTextIter, MM>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GtkTextIter, Self> {
        let stash = self.0.to_glib_none_mut();
        StashMut(stash.0, stash.1)
    }
}

impl Clone for TextIter {
    fn clone(&self) -> Self {
        TextIter(self.0.clone())
    }
}

impl TextIter {
    pub unsafe fn uninitialized() -> TextIter {
        TextIter(Boxed::uninitialized())
    }

    pub fn get_buffer(&self) -> Option<::TextBuffer> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_get_buffer(self.to_glib_none().0) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    pub fn assign(&mut self, other: &TextIter) {
        unsafe { ffi::gtk_text_iter_assign(self.to_glib_none_mut().0, other.to_glib_none().0) }
    }

    pub fn get_offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_offset(self.to_glib_none().0) }
    }

    pub fn get_line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line(self.to_glib_none().0) }
    }

    pub fn get_line_offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line_offset(self.to_glib_none().0) }
    }

    pub fn get_line_index(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line_index(self.to_glib_none().0) }
    }

    pub fn get_visible_line_index(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_visible_line_index(self.to_glib_none().0) }
    }

    pub fn get_visible_line_offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_visible_line_offset(self.to_glib_none().0) }
    }

    pub fn get_char(&self) -> u32 {
        unsafe { ffi::gtk_text_iter_get_char(self.to_glib_none().0) }
    }

    pub fn get_slice(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_slice(
                    self.to_glib_none().0,
                    end.to_glib_none().0))
        }
    }

    pub fn get_text(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_text(
                    self.to_glib_none().0,
                    end.to_glib_none().0))
        }
    }

    pub fn get_visible_slice(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_visible_slice(
                    self.to_glib_none().0,
                    end.to_glib_none().0))
        }
    }

    pub fn get_visible_text(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_visible_text(
                    self.to_glib_none().0,
                    end.to_glib_none().0))
        }
    }

    pub fn get_child_anchor(&self) -> Option<::TextChildAnchor> {
        let tmp_pointer = unsafe { ffi::gtk_text_iter_get_child_anchor(self.to_glib_none().0) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::TextChildAnchor::wrap_pointer(tmp_pointer))
        }
    }

    pub fn begins_tag(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_begins_tag(self.to_glib_none().0, tag.unwrap_pointer())) }
    }

    pub fn ends_tag(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_tag(self.to_glib_none().0, tag.unwrap_pointer())) }
    }

    pub fn toggles_tag(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_tag(self.to_glib_none().0, tag.unwrap_pointer())) }
    }

    pub fn has_tag(&self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_tag(self.to_glib_none().0, tag.unwrap_pointer())) }
    }

    pub fn editable(&self, default_setting: bool) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_editable(self.to_glib_none().0, to_gboolean(default_setting))) }
    }

    pub fn can_insert(&self, default_setting: bool) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_can_insert(self.to_glib_none().0, to_gboolean(default_setting))) }
    }

    pub fn starts_word(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_starts_word(self.to_glib_none().0)) }
    }

    pub fn ends_word(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_word(self.to_glib_none().0)) }
    }

    pub fn inside_word(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_word(self.to_glib_none().0)) }
    }

    pub fn starts_line(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_starts_line(self.to_glib_none().0)) }
    }

    pub fn ends_line(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_line(self.to_glib_none().0)) }
    }

    pub fn starts_sentence(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_starts_sentence(self.to_glib_none().0)) }
    }

    pub fn ends_sentence(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_ends_sentence(self.to_glib_none().0)) }
    }

    pub fn inside_sentence(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_inside_sentence(self.to_glib_none().0)) }
    }

    pub fn is_cursor_position(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_is_cursor_position(self.to_glib_none().0)) }
    }

    pub fn get_chars_in_line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_chars_in_line(self.to_glib_none().0) }
    }

    pub fn get_bytes_in_line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_bytes_in_line(self.to_glib_none().0) }
    }

    pub fn get_attributes(&self, values: &::TextAttributes) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_get_attributes(self.to_glib_none().0, values.unwrap_pointer())) }
    }

    pub fn is_end(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_is_end(self.to_glib_none().0)) }
    }

    pub fn is_start(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_is_start(self.to_glib_none().0)) }
    }

    pub fn forward_char(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_char(self.to_glib_none_mut().0)) }
    }

    pub fn backward_char(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_char(self.to_glib_none_mut().0)) }
    }

    pub fn forward_chars(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_chars(self.to_glib_none_mut().0, count)) }
    }

    pub fn backward_chars(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_chars(self.to_glib_none_mut().0, count)) }
    }

    pub fn forward_line(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_line(self.to_glib_none_mut().0)) }
    }

    pub fn backward_line(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_line(self.to_glib_none_mut().0)) }
    }

    pub fn forward_lines(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_lines(self.to_glib_none_mut().0, count)) }
    }

    pub fn backward_lines(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_lines(self.to_glib_none_mut().0, count)) }
    }

    pub fn forward_word_ends(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_word_ends(self.to_glib_none_mut().0, count)) }
    }

    pub fn forward_word_starts(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_word_starts(self.to_glib_none_mut().0, count)) }
    }

    pub fn forward_word_end(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_word_end(self.to_glib_none_mut().0)) }
    }

    pub fn backward_word_start(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_word_start(self.to_glib_none_mut().0)) }
    }

    pub fn forward_cursor_position(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_cursor_position(self.to_glib_none_mut().0)) }
    }

    pub fn backward_cursor_position(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_cursor_position(self.to_glib_none_mut().0)) }
    }

    pub fn forward_cursor_positions(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_cursor_positions(self.to_glib_none_mut().0, count)) }
    }

    pub fn backward_cursor_positions(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_cursor_positions(self.to_glib_none_mut().0, count)) }
    }

    pub fn backward_sentence_start(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_sentence_start(self.to_glib_none_mut().0)) }
    }

    pub fn backward_sentence_starts(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_sentence_starts(self.to_glib_none_mut().0, count)) }
    }

    pub fn backward_sentence_end(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_sentence_end(self.to_glib_none_mut().0)) }
    }

    pub fn backward_sentence_ends(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_sentence_ends(self.to_glib_none_mut().0, count)) }
    }

    pub fn forward_visible_word_ends(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_word_ends(self.to_glib_none_mut().0, count)) }
    }

    pub fn forward_visible_word_starts(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_word_starts(self.to_glib_none_mut().0, count)) }
    }

    pub fn forward_visible_word_end(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_word_end(self.to_glib_none_mut().0)) }
    }

    pub fn forward_visible_word_start(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_word_start(self.to_glib_none_mut().0)) }
    }

    pub fn forward_visible_cursor_position(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_cursor_position(self.to_glib_none_mut().0)) }
    }

    pub fn backward_visible_cursor_position(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_cursor_position(self.to_glib_none_mut().0)) }
    }

    pub fn forward_visible_cursor_positions(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_cursor_positions(self.to_glib_none_mut().0, count)) }
    }

    pub fn backward_visible_cursor_positions(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_cursor_positions(self.to_glib_none_mut().0, count)) }
    }

    pub fn forward_visible_line(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_line(self.to_glib_none_mut().0)) }
    }

    pub fn backward_visible_line(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_line(self.to_glib_none_mut().0)) }
    }

    pub fn forward_visible_lines(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_visible_lines(self.to_glib_none_mut().0, count)) }
    }

    pub fn backward_visible_lines(&mut self, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_visible_lines(self.to_glib_none_mut().0, count)) }
    }

    pub fn set_offset(&mut self, char_offset: i32) {
        unsafe { ffi::gtk_text_iter_set_offset(self.to_glib_none_mut().0, char_offset) }
    }

    pub fn set_line(&mut self, line_number: i32) {
        unsafe { ffi::gtk_text_iter_set_line(self.to_glib_none_mut().0, line_number) }
    }

    pub fn set_line_offset(&mut self, char_on_line: i32) {
        unsafe { ffi::gtk_text_iter_set_line_offset(self.to_glib_none_mut().0, char_on_line) }
    }

    pub fn set_line_index(&mut self, byte_on_line: i32) {
        unsafe { ffi::gtk_text_iter_set_line_index(self.to_glib_none_mut().0, byte_on_line) }
    }

    pub fn set_visible_line_index(&mut self, byte_on_line: i32) {
        unsafe { ffi::gtk_text_iter_set_visible_line_index(self.to_glib_none_mut().0, byte_on_line) }
    }

    pub fn set_visible_line_offset(&mut self, char_on_line: i32) {
        unsafe { ffi::gtk_text_iter_set_visible_line_offset(self.to_glib_none_mut().0, char_on_line) }
    }

    pub fn forward_to_end(&mut self) {
        unsafe { ffi::gtk_text_iter_forward_to_end(self.to_glib_none_mut().0) }
    }

    pub fn forward_to_line_end(&mut self) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_to_line_end(self.to_glib_none_mut().0)) }
    }

    pub fn forward_to_tag_toggle(&mut self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_forward_to_tag_toggle(self.to_glib_none_mut().0, tag.unwrap_pointer())) }
    }

    pub fn backward_to_tag_toggle(&mut self, tag: &::TextTag) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_backward_to_tag_toggle(self.to_glib_none_mut().0, tag.unwrap_pointer())) }
    }

    pub fn is_equal_to(&self, other: &TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_equal(self.to_glib_none().0,
            other.to_glib_none().0)) }
    }

    pub fn compare_to(&self, other: &TextIter) -> i32 {
        unsafe { ffi::gtk_text_iter_compare(self.to_glib_none().0, other.to_glib_none().0) }
    }

    pub fn in_range(&self, start: &TextIter, end: &TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_iter_in_range(self.to_glib_none().0,
            start.to_glib_none().0, end.to_glib_none().0)) }
    }

    pub fn order(&mut self, second: &mut TextIter) {
        unsafe { ffi::gtk_text_iter_order(self.to_glib_none_mut().0, second.to_glib_none_mut().0) }
    }
}
