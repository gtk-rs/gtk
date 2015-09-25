// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_int, c_uint, c_float, c_double};
use glib::translate::{from_glib_none, from_glib_full, ToGlibPtr};

use {EntryCompletion, EntryIconPosition, FFIWidget, ImageType, InputPurpose, InputHints};
use cast::{GTK_ENTRY, GTK_ENTRY_COMPLETION};
use ffi;
use glib::{to_bool, to_gboolean};

pub trait EntryTrait: ::WidgetTrait {
    fn get_buffer(&self) -> ::EntryBuffer {
        let tmp_pointer = unsafe { ffi::gtk_entry_get_buffer(GTK_ENTRY(self.unwrap_widget())) };

        ::EntryBuffer::wrap_pointer(tmp_pointer)
    }

    fn set_buffer(&self, buffer: &::EntryBuffer) -> () {
        unsafe {
            ffi::gtk_entry_set_buffer(GTK_ENTRY(self.unwrap_widget()), buffer.unwrap_pointer())
        }
    }

    fn set_text(&self, text: &str) {
        unsafe {
            ffi::gtk_entry_set_text(GTK_ENTRY(self.unwrap_widget()), text.to_glib_none().0)
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_text(GTK_ENTRY(self.unwrap_widget())))
        }
    }

    fn get_text_length(&self) -> i16 {
        unsafe {
            ffi::gtk_entry_get_text_length(GTK_ENTRY(self.unwrap_widget())) as i16
        }
    }

    fn set_visibility(&self, visible: bool) -> () {
        unsafe { ffi::gtk_entry_set_visibility(GTK_ENTRY(self.unwrap_widget()), to_gboolean(visible)); }
    }

    fn set_invisible_char(&self, ch: char) -> () {
        unsafe {
            ffi::gtk_entry_set_invisible_char(GTK_ENTRY(self.unwrap_widget()), ch as c_uint);
        }
    }

    fn unset_invisible_char(&self) -> () {
        unsafe {
            ffi::gtk_entry_unset_invisible_char(GTK_ENTRY(self.unwrap_widget()));
        }
    }

    fn set_max_length(&self, max_length: i32) -> () {
        unsafe {
            ffi::gtk_entry_set_max_length(GTK_ENTRY(self.unwrap_widget()), max_length as c_int);
        }
    }

    fn get_activates_default(&self) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_activates_default(GTK_ENTRY(self.unwrap_widget()))) }
    }

    fn get_has_frame(&self) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_has_frame(GTK_ENTRY(self.unwrap_widget()))) }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_width_chars(GTK_ENTRY(self.unwrap_widget()))
        }
    }

    fn set_activates_default(&self, setting: bool) {
        unsafe { ffi::gtk_entry_set_activates_default(GTK_ENTRY(self.unwrap_widget()), to_gboolean(setting)); }
    }

    fn set_has_frame(&self, setting: bool) {
        unsafe { ffi::gtk_entry_set_has_frame(GTK_ENTRY(self.unwrap_widget()), to_gboolean(setting)); }
    }

    fn set_width_chars(&self, n_chars: i32) -> () {
        unsafe {
            ffi::gtk_entry_set_width_chars(GTK_ENTRY(self.unwrap_widget()), n_chars as c_int);
        }
    }

    fn get_invisible_char(&self) -> u32 {
        unsafe {
            ffi::gtk_entry_get_invisible_char(GTK_ENTRY(self.unwrap_widget())) as u32
        }
    }

    fn set_alignment(&self, x_align: f32) -> () {
        unsafe {
            ffi::gtk_entry_set_alignment(GTK_ENTRY(self.unwrap_widget()), x_align as c_float);
        }
    }

    fn get_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_entry_get_alignment(GTK_ENTRY(self.unwrap_widget())) as f32
        }
    }

    fn set_placeholder(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_entry_set_placeholder_text(GTK_ENTRY(self.unwrap_widget()), text.to_glib_none().0)
        }
    }

    fn placeholder(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_placeholder_text(GTK_ENTRY(self.unwrap_widget())))
        }
    }

    fn get_overwrite_mode(&self) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_overwrite_mode(GTK_ENTRY(self.unwrap_widget()))) }
    }

    fn set_overwrite_mode(&self, overwrite: bool) {
        unsafe { ffi::gtk_entry_set_overwrite_mode(GTK_ENTRY(self.unwrap_widget()), to_gboolean(overwrite)); }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;

        unsafe {
            ffi::gtk_entry_get_layout_offsets(GTK_ENTRY(self.unwrap_widget()), &mut x, &mut y);
        }
        (x, y)
    }

    fn layout_index_to_text_index(&self, layout_index: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_layout_index_to_text_index(GTK_ENTRY(self.unwrap_widget()), layout_index as c_int) as i32
        }
    }

    fn text_index_to_layout_index(&self, text_index: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_text_index_to_layout_index(GTK_ENTRY(self.unwrap_widget()), text_index as c_int) as i32
        }
    }

    fn get_max_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_max_length(GTK_ENTRY(self.unwrap_widget())) as i32
        }
    }

    fn get_visibility(&self) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_visibility(GTK_ENTRY(self.unwrap_widget()))) }
    }

    fn set_completion(&self, completion: &EntryCompletion) -> () {
        unsafe {
            ffi::gtk_entry_set_completion(GTK_ENTRY(self.unwrap_widget()), GTK_ENTRY_COMPLETION(completion.unwrap_widget()));
        }
    }

    fn get_completion(&self) -> Option<EntryCompletion> {
        let tmp_pointer = unsafe { ffi::gtk_entry_get_completion(GTK_ENTRY(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(EntryCompletion::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    fn set_cursor_hadjustment(&self, adjustment: &::Adjustment) -> () {
        unsafe {
            ffi::gtk_entry_set_cursor_hadjustment(GTK_ENTRY(self.unwrap_widget()), adjustment.unwrap_pointer())
        }
    }

    fn get_cursor_hadjustment(&self) -> ::Adjustment {
        unsafe {
            ::Adjustment::wrap_pointer(ffi::gtk_entry_get_cursor_hadjustment(GTK_ENTRY(self.unwrap_widget())))
        }
    }

    fn set_progress_fraction(&self, fraction: f64) -> () {
        unsafe {
            ffi::gtk_entry_set_progress_fraction(GTK_ENTRY(self.unwrap_widget()), fraction as c_double);
        }
    }

    fn get_progress_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_entry_get_progress_fraction(GTK_ENTRY(self.unwrap_widget())) as f64
        }
    }

    fn set_progress_pulse_step(&self, pulse_step: f64) -> () {
        unsafe {
            ffi::gtk_entry_set_progress_pulse_step(GTK_ENTRY(self.unwrap_widget()), pulse_step as c_double);
        }
    }

    fn get_progress_pulse_step(&self) -> f64 {
        unsafe {
            ffi::gtk_entry_get_progress_pulse_step(GTK_ENTRY(self.unwrap_widget())) as f64
        }
    }

    fn progress_pulse(&self) -> () {
        unsafe {
            ffi::gtk_entry_progress_pulse(GTK_ENTRY(self.unwrap_widget()));
        }
    }

    fn reset_im_context(&self) -> () {
        unsafe {
            ffi::gtk_entry_reset_im_context(GTK_ENTRY(self.unwrap_widget()));
        }
    }

    fn set_icon_from_stock(&self, icon_pos: EntryIconPosition, stock_id: &str) -> () {
        unsafe {
            ffi::gtk_entry_set_icon_from_stock(GTK_ENTRY(self.unwrap_widget()), icon_pos, stock_id.to_glib_none().0);
        }
    }

    fn set_icon_from_icon_name(&self, icon_pos: EntryIconPosition, icon_name: &str) -> () {
        unsafe {
            ffi::gtk_entry_set_icon_from_icon_name(GTK_ENTRY(self.unwrap_widget()), icon_pos, icon_name.to_glib_none().0)
        }
    }

    fn get_icon_storage_type(&self, icon_pos: EntryIconPosition) -> ImageType {
        unsafe {
            ffi::gtk_entry_get_icon_storage_type(GTK_ENTRY(self.unwrap_widget()), icon_pos)
        }
    }

    fn get_icon_stock(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_icon_stock(GTK_ENTRY(self.unwrap_widget()), icon_pos))
        }
    }

    fn get_icon_name(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_icon_name(GTK_ENTRY(self.unwrap_widget()), icon_pos))
        }
    }

    fn get_icon_activatable(&self, icon_pos: EntryIconPosition) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_icon_activatable(GTK_ENTRY(self.unwrap_widget()), icon_pos)) }
    }

    fn set_icon_activatable(&self, icon_pos: EntryIconPosition, activatable: bool) {
        unsafe { ffi::gtk_entry_set_icon_activatable(GTK_ENTRY(self.unwrap_widget()), icon_pos, to_gboolean(activatable)); }
    }

    fn get_icon_sensitive(&self, icon_pos: EntryIconPosition) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_icon_sensitive(GTK_ENTRY(self.unwrap_widget()), icon_pos)) }
    }

    fn set_icon_sensitive(&self, icon_pos: EntryIconPosition, sensitive: bool) {
        unsafe { ffi::gtk_entry_set_icon_sensitive(GTK_ENTRY(self.unwrap_widget()), icon_pos, to_gboolean(sensitive)); }
    }

    fn get_icon_at_pos(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_get_icon_at_pos(GTK_ENTRY(self.unwrap_widget()), x, y) as i32
        }
    }

    fn set_icon_tooltip_text(&self, icon_pos: EntryIconPosition, tooltip: &str) -> () {
        unsafe {
            ffi::gtk_entry_set_icon_tooltip_text(GTK_ENTRY(self.unwrap_widget()), icon_pos, tooltip.to_glib_none().0)
        }
    }

    fn get_icon_tooltip_text(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_entry_get_icon_tooltip_text(GTK_ENTRY(self.unwrap_widget()),
                                                     icon_pos))
        }
    }

    fn set_icon_tooltip_markup(&self, icon_pos: EntryIconPosition, tooltip: &str) -> () {
        unsafe {
            ffi::gtk_entry_set_icon_tooltip_markup(GTK_ENTRY(self.unwrap_widget()), icon_pos, tooltip.to_glib_none().0)
        }
    }

    fn get_icon_tooltip_markup(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_entry_get_icon_tooltip_markup(GTK_ENTRY(self.unwrap_widget()),
                                                       icon_pos))
        }
    }

    fn get_current_icon_draw_source(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_current_icon_drag_source(GTK_ENTRY(self.unwrap_widget())) as i32
        }
    }

    fn set_input_purpose(&self, purpose: InputPurpose) -> () {
        unsafe {
            ffi::gtk_entry_set_input_purpose(GTK_ENTRY(self.unwrap_widget()), purpose)
        }
    }

    fn get_input_purpose(&self) -> InputPurpose {
        unsafe {
            ffi::gtk_entry_get_input_purpose(GTK_ENTRY(self.unwrap_widget()))
        }
    }

    fn set_input_hints(&self, hints: InputHints) -> () {
        unsafe {
            ffi::gtk_entry_set_input_hints(GTK_ENTRY(self.unwrap_widget()), hints)
        }
    }

    fn get_input_hints(&self) -> InputHints {
        unsafe {
            ffi::gtk_entry_get_input_hints(GTK_ENTRY(self.unwrap_widget()))
        }
    }
}
