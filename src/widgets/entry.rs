// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A single line text entry field

use std::ptr;

use glib::translate::*;
use glib::types;
use ffi;

use adjustment::Adjustment;
use object::{Object, Downcast, Upcast};
use super::editable::Editable;
use super::widget::Widget;

use {EntryIconPosition, ImageType, InputPurpose, InputHints};

///////////////////////////////////////////////////////////////////////////////

/// EntryBuffer — Text buffer for ::Entry

pub type EntryBuffer = Object<ffi::GtkEntryBuffer>;

impl EntryBuffer {
    pub fn new() -> EntryBuffer {
        unsafe {
            from_glib_full(ffi::gtk_entry_buffer_new(ptr::null(), 0))
        }
    }

    pub fn from_str(s: &str) -> EntryBuffer {
        unsafe {
            from_glib_full(ffi::gtk_entry_buffer_new(s.to_glib_none().0, -1))
        }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_buffer_get_text(self.to_glib_none().0))
        }
    }

    pub fn set_text(&self, text: &str) {
        unsafe {
            ffi::gtk_entry_buffer_set_text(self.to_glib_none().0, text.to_glib_none().0, -1);
        }
    }

    pub fn get_bytes(&self) -> i64 {
        unsafe {
            ffi::gtk_entry_buffer_get_bytes(self.to_glib_none().0) as i64
        }
    }

    pub fn get_length(&self) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_get_length(self.to_glib_none().0)
        }
    }

    pub fn get_max_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_buffer_get_max_length(self.to_glib_none().0)
        }
    }

    pub fn set_max_length(&self, max_length: i32) {
        unsafe {
            ffi::gtk_entry_buffer_set_max_length(self.to_glib_none().0, max_length)
        }
    }

    pub fn insert_text(&self, position: u32, text: &str) {
        unsafe {
            ffi::gtk_entry_buffer_insert_text(self.to_glib_none().0, position,
                                              text.to_glib_none().0, -1);
        }
    }

    pub fn delete_text(&self, position: u32, n_chars: u32) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_delete_text(self.to_glib_none().0, position, n_chars as i32)
        }
    }

    pub fn emit_deleted_test(&self, position: u32, n_chars: u32) {
        unsafe {
            ffi::gtk_entry_buffer_emit_deleted_text(self.to_glib_none().0, position, n_chars)
        }
    }

    pub fn emit_inserted_text(&self, position: u32, text: &str) {
        unsafe {
            ffi::gtk_entry_buffer_emit_inserted_text(self.to_glib_none().0, position,
                                                     text.to_glib_none().0, -1);
        }
    }
}

impl types::StaticType for EntryBuffer {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_entry_buffer_get_type()) }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Entry — A single line text entry field

pub type Entry = Object<ffi::GtkEntry>;

impl Entry {
    pub fn new() -> Entry {
        unsafe { Widget::from_glib_none(ffi::gtk_entry_new()).downcast_unchecked() }
    }

    pub fn new_with_buffer(buffer: &EntryBuffer) -> Entry {
        unsafe {
            Widget::from_glib_none(ffi::gtk_entry_new_with_buffer(buffer.to_glib_none().0))
                .downcast_unchecked()
        }
    }
}

impl types::StaticType for Entry {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_entry_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Entry { }
unsafe impl Upcast<super::editable::Editable> for Entry { }
unsafe impl Upcast<::mvc::cell_interfaces::CellEditable> for Entry { }
unsafe impl Upcast<::builder::Buildable> for Entry { }

pub trait EntryExt {
    fn get_buffer(&self) -> EntryBuffer;
    fn set_buffer(&self, buffer: &EntryBuffer);
    fn set_text(&self, text: &str);
    fn get_text(&self) -> Option<String>;
    fn get_text_length(&self) -> u16;
    fn set_visibility(&self, visible: bool);
    fn set_invisible_char(&self, ch: char);
    fn unset_invisible_char(&self);
    fn set_max_length(&self, max_length: i32);
    fn get_activates_default(&self) -> bool;
    fn get_has_frame(&self) -> bool;
    fn get_width_chars(&self) -> i32;
    fn set_activates_default(&self, setting: bool);
    fn set_has_frame(&self, setting: bool);
    fn set_width_chars(&self, n_chars: i32);
    fn get_invisible_char(&self) -> Option<char>;
    fn set_alignment(&self, x_align: f32);
    fn get_alignment(&self) -> f32;
    fn set_placeholder(&self, text: &str);
    fn placeholder(&self) -> Option<String>;
    fn get_overwrite_mode(&self) -> bool;
    fn set_overwrite_mode(&self, overwrite: bool);
    fn get_layout_offsets(&self) -> (i32, i32);
    fn layout_index_to_text_index(&self, layout_index: i32) -> i32;
    fn text_index_to_layout_index(&self, text_index: i32) -> i32;
    fn get_max_length(&self) -> i32;
    fn get_visibility(&self) -> bool;
    fn set_cursor_hadjustment(&self, adjustment: &Adjustment);
    fn get_cursor_hadjustment(&self) -> Adjustment;
    fn set_progress_fraction(&self, fraction: f64);
    fn get_progress_fraction(&self) -> f64;
    fn set_progress_pulse_step(&self, pulse_step: f64);
    fn get_progress_pulse_step(&self) -> f64;
    fn progress_pulse(&self);
    fn reset_im_context(&self);
    fn set_icon_from_stock(&self, icon_pos: EntryIconPosition, stock_id: &str);
    fn set_icon_from_icon_name(&self, icon_pos: EntryIconPosition, icon_name: &str);
    fn get_icon_storage_type(&self, icon_pos: EntryIconPosition) -> ImageType;
    fn get_icon_stock(&self, icon_pos: EntryIconPosition) -> Option<String>;
    fn get_icon_name(&self, icon_pos: EntryIconPosition) -> Option<String>;
    fn get_icon_activatable(&self, icon_pos: EntryIconPosition) -> bool;
    fn set_icon_activatable(&self, icon_pos: EntryIconPosition, activatable: bool);
    fn get_icon_sensitive(&self, icon_pos: EntryIconPosition) -> bool;
    fn set_icon_sensitive(&self, icon_pos: EntryIconPosition, sensitive: bool);
    fn get_icon_at_pos(&self, x: i32, y: i32) -> i32;
    fn set_icon_tooltip_text(&self, icon_pos: EntryIconPosition, tooltip: &str);
    fn get_icon_tooltip_text(&self, icon_pos: EntryIconPosition) -> Option<String>;
    fn set_icon_tooltip_markup(&self, icon_pos: EntryIconPosition, tooltip: &str);
    fn get_icon_tooltip_markup(&self, icon_pos: EntryIconPosition) -> Option<String>;
    fn get_current_icon_draw_source(&self) -> i32;
    fn set_input_purpose(&self, purpose: InputPurpose);
    fn get_input_purpose(&self) -> InputPurpose;
    fn set_input_hints(&self, hints: InputHints);
    fn get_input_hints(&self) -> InputHints;
}

impl<O: Upcast<Entry>> EntryExt for O {
    fn get_buffer(&self) -> EntryBuffer {
        unsafe { from_glib_none(ffi::gtk_entry_get_buffer(self.upcast().to_glib_none().0)) }
    }

    fn set_buffer(&self, buffer: &EntryBuffer) {
        unsafe {
            ffi::gtk_entry_set_buffer(self.upcast().to_glib_none().0, buffer.to_glib_none().0)
        }
    }

    fn set_text(&self, text: &str) {
        unsafe { ffi::gtk_entry_set_text(self.upcast().to_glib_none().0, text.to_glib_none().0) }
    }

    fn get_text(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gtk_entry_get_text(self.upcast().to_glib_none().0)) }
    }

    fn get_text_length(&self) -> u16 {
        unsafe {
            ffi::gtk_entry_get_text_length(self.upcast().to_glib_none().0)
        }
    }

    fn set_visibility(&self, visible: bool) {
        unsafe { ffi::gtk_entry_set_visibility(self.upcast().to_glib_none().0, visible.to_glib()); }
    }

    fn set_invisible_char(&self, ch: char) {
        unsafe { ffi::gtk_entry_set_invisible_char(self.upcast().to_glib_none().0, ch as u32); }
    }

    fn unset_invisible_char(&self) {
        unsafe { ffi::gtk_entry_unset_invisible_char(self.upcast().to_glib_none().0); }
    }

    fn set_max_length(&self, max_length: i32) {
        unsafe { ffi::gtk_entry_set_max_length(self.upcast().to_glib_none().0, max_length); }
    }

    fn get_activates_default(&self) -> bool {
        unsafe { from_glib(ffi::gtk_entry_get_activates_default(self.upcast().to_glib_none().0)) }
    }

    fn get_has_frame(&self) -> bool {
        unsafe { from_glib(ffi::gtk_entry_get_has_frame(self.upcast().to_glib_none().0)) }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe { ffi::gtk_entry_get_width_chars(self.upcast().to_glib_none().0) }
    }

    fn set_activates_default(&self, setting: bool) {
        unsafe {
            ffi::gtk_entry_set_activates_default(self.upcast().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_has_frame(&self, setting: bool) {
        unsafe { ffi::gtk_entry_set_has_frame(self.upcast().to_glib_none().0, setting.to_glib()); }
    }

    fn set_width_chars(&self, n_chars: i32) {
        unsafe { ffi::gtk_entry_set_width_chars(self.upcast().to_glib_none().0, n_chars); }
    }

    fn get_invisible_char(&self) -> Option<char> {
        let c = unsafe { ffi::gtk_entry_get_invisible_char(self.upcast().to_glib_none().0) };
        if c == 0 {
            None
        }
        else {
            ::std::char::from_u32(c)
        }
    }

    fn set_alignment(&self, x_align: f32) {
        unsafe {
            ffi::gtk_entry_set_alignment(self.upcast().to_glib_none().0, x_align);
        }
    }

    fn get_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_entry_get_alignment(self.upcast().to_glib_none().0)
        }
    }

    fn set_placeholder(&self, text: &str) {
        unsafe {
            ffi::gtk_entry_set_placeholder_text(self.upcast().to_glib_none().0,
                text.to_glib_none().0)
        }
    }

    fn placeholder(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_placeholder_text(self.upcast().to_glib_none().0))
        }
    }

    fn get_overwrite_mode(&self) -> bool {
        unsafe { from_glib(ffi::gtk_entry_get_overwrite_mode(self.upcast().to_glib_none().0)) }
    }

    fn set_overwrite_mode(&self, overwrite: bool) {
        unsafe {
            ffi::gtk_entry_set_overwrite_mode(self.upcast().to_glib_none().0, overwrite.to_glib());
        }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;

        unsafe {
            ffi::gtk_entry_get_layout_offsets(self.upcast().to_glib_none().0, &mut x, &mut y);
        }

        (x, y)
    }

    fn layout_index_to_text_index(&self, layout_index: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_layout_index_to_text_index(self.upcast().to_glib_none().0, layout_index)
        }
    }

    fn text_index_to_layout_index(&self, text_index: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_text_index_to_layout_index(self.upcast().to_glib_none().0, text_index)
        }
    }

    fn get_max_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_max_length(self.upcast().to_glib_none().0)
        }
    }

    fn get_visibility(&self) -> bool {
        unsafe { from_glib(ffi::gtk_entry_get_visibility(self.upcast().to_glib_none().0)) }
    }

    fn set_cursor_hadjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_entry_set_cursor_hadjustment(self.upcast().to_glib_none().0,
                adjustment.to_glib_none().0)
        }
    }

    fn get_cursor_hadjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none((ffi::gtk_entry_get_cursor_hadjustment(self.upcast().to_glib_none().0)))
        }
    }

    fn set_progress_fraction(&self, fraction: f64) {
        unsafe {
            ffi::gtk_entry_set_progress_fraction(self.upcast().to_glib_none().0, fraction);
        }
    }

    fn get_progress_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_entry_get_progress_fraction(self.upcast().to_glib_none().0)
        }
    }

    fn set_progress_pulse_step(&self, pulse_step: f64) {
        unsafe {
            ffi::gtk_entry_set_progress_pulse_step(self.upcast().to_glib_none().0, pulse_step);
        }
    }

    fn get_progress_pulse_step(&self) -> f64 {
        unsafe {
            ffi::gtk_entry_get_progress_pulse_step(self.upcast().to_glib_none().0)
        }
    }

    fn progress_pulse(&self) {
        unsafe {
            ffi::gtk_entry_progress_pulse(self.upcast().to_glib_none().0);
        }
    }

    fn reset_im_context(&self) {
        unsafe {
            ffi::gtk_entry_reset_im_context(self.upcast().to_glib_none().0);
        }
    }

    fn set_icon_from_stock(&self, icon_pos: EntryIconPosition, stock_id: &str) {
        unsafe {
            ffi::gtk_entry_set_icon_from_stock(self.upcast().to_glib_none().0, icon_pos,
                stock_id.to_glib_none().0);
        }
    }

    fn set_icon_from_icon_name(&self, icon_pos: EntryIconPosition, icon_name: &str) {
        unsafe {
            ffi::gtk_entry_set_icon_from_icon_name(self.upcast().to_glib_none().0, icon_pos,
                icon_name.to_glib_none().0)
        }
    }

    fn get_icon_storage_type(&self, icon_pos: EntryIconPosition) -> ImageType {
        unsafe {
            ffi::gtk_entry_get_icon_storage_type(self.upcast().to_glib_none().0, icon_pos)
        }
    }

    fn get_icon_stock(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_icon_stock(self.upcast().to_glib_none().0, icon_pos))
        }
    }

    fn get_icon_name(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_icon_name(self.upcast().to_glib_none().0, icon_pos))
        }
    }

    fn get_icon_activatable(&self, icon_pos: EntryIconPosition) -> bool {
        unsafe { from_glib(ffi::gtk_entry_get_icon_activatable(self.upcast().to_glib_none().0,
            icon_pos)) }
    }

    fn set_icon_activatable(&self, icon_pos: EntryIconPosition, activatable: bool) {
        unsafe { ffi::gtk_entry_set_icon_activatable(self.upcast().to_glib_none().0, icon_pos,
            activatable.to_glib()); }
    }

    fn get_icon_sensitive(&self, icon_pos: EntryIconPosition) -> bool {
        unsafe { from_glib(ffi::gtk_entry_get_icon_sensitive(self.upcast().to_glib_none().0,
            icon_pos)) }
    }

    fn set_icon_sensitive(&self, icon_pos: EntryIconPosition, sensitive: bool) {
        unsafe { ffi::gtk_entry_set_icon_sensitive(self.upcast().to_glib_none().0, icon_pos,
            sensitive.to_glib()); }
    }

    fn get_icon_at_pos(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_get_icon_at_pos(self.upcast().to_glib_none().0, x, y)
        }
    }

    fn set_icon_tooltip_text(&self, icon_pos: EntryIconPosition, tooltip: &str) {
        unsafe {
            ffi::gtk_entry_set_icon_tooltip_text(self.upcast().to_glib_none().0, icon_pos,
                tooltip.to_glib_none().0)
        }
    }

    fn get_icon_tooltip_text(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_entry_get_icon_tooltip_text(self.upcast().to_glib_none().0,
                                                     icon_pos))
        }
    }

    fn set_icon_tooltip_markup(&self, icon_pos: EntryIconPosition, tooltip: &str) {
        unsafe {
            ffi::gtk_entry_set_icon_tooltip_markup(self.upcast().to_glib_none().0, icon_pos,
                tooltip.to_glib_none().0)
        }
    }

    fn get_icon_tooltip_markup(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_entry_get_icon_tooltip_markup(self.upcast().to_glib_none().0,
                                                       icon_pos))
        }
    }

    fn get_current_icon_draw_source(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_current_icon_drag_source(self.upcast().to_glib_none().0)
        }
    }

    fn set_input_purpose(&self, purpose: InputPurpose) {
        unsafe {
            ffi::gtk_entry_set_input_purpose(self.upcast().to_glib_none().0, purpose)
        }
    }

    fn get_input_purpose(&self) -> InputPurpose {
        unsafe {
            ffi::gtk_entry_get_input_purpose(self.upcast().to_glib_none().0)
        }
    }

    fn set_input_hints(&self, hints: InputHints) {
        unsafe {
            ffi::gtk_entry_set_input_hints(self.upcast().to_glib_none().0, hints)
        }
    }

    fn get_input_hints(&self) -> InputHints {
        unsafe {
            ffi::gtk_entry_get_input_hints(self.upcast().to_glib_none().0)
        }
    }
}
