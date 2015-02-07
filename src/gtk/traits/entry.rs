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

use libc::{c_int, c_float, c_double};
use std::ffi::CString;
use libc;

use gtk::{EntryIconPosition, ImageType, InputPurpose, InputHints};
use gtk::cast::GTK_ENTRY;
use gtk::{self, ffi};
use gtk::ffi::{to_bool, to_gboolean};

pub trait EntryTrait: gtk::WidgetTrait {
    fn get_buffer(&self) -> gtk::EntryBuffer {
        let tmp_pointer = unsafe { ffi::gtk_entry_get_buffer(GTK_ENTRY(self.get_widget())) };

        gtk::EntryBuffer::wrap_pointer(tmp_pointer)
    }

    fn set_buffer(&mut self, buffer: &gtk::EntryBuffer) -> () {
        unsafe {
            ffi::gtk_entry_set_buffer(GTK_ENTRY(self.get_widget()), buffer.get_pointer())
        }
    }

    fn set_text(&mut self, text: String) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

            ffi::gtk_entry_set_text(GTK_ENTRY(self.get_widget()), c_str.as_ptr())
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_entry_get_text(GTK_ENTRY(self.get_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_text_length(&self) -> i16 {
        unsafe {
            ffi::gtk_entry_get_text_length(GTK_ENTRY(self.get_widget())) as i16
        }
    }

    fn set_visibility(&mut self, visible: bool) -> () {
        unsafe { ffi::gtk_entry_set_visibility(GTK_ENTRY(self.get_widget()), to_gboolean(visible)); }
    }

    fn set_invisible_char(&mut self, ch: i32) -> () {
        unsafe {
            ffi::gtk_entry_set_invisible_char(GTK_ENTRY(self.get_widget()), ch as c_int);
        }
    }

    fn unset_invisible_char(&mut self) -> () {
        unsafe {
            ffi::gtk_entry_unset_invisible_char(GTK_ENTRY(self.get_widget()));
        }
    }

    fn set_max_length(&mut self, max_length: i32) -> () {
        unsafe {
            ffi::gtk_entry_set_max_length(GTK_ENTRY(self.get_widget()), max_length as c_int);
        }
    }

    fn get_activates_default(&self) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_activates_default(GTK_ENTRY(self.get_widget()))) }
    }

    fn get_has_frame(&self) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_has_frame(GTK_ENTRY(self.get_widget()))) }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_width_chars(GTK_ENTRY(self.get_widget()))
        }
    }

    fn set_activates_default(&mut self, setting: bool) {
        unsafe { ffi::gtk_entry_set_activates_default(GTK_ENTRY(self.get_widget()), to_gboolean(setting)); }
    }

    fn set_has_frame(&mut self, setting: bool) {
        unsafe { ffi::gtk_entry_set_has_frame(GTK_ENTRY(self.get_widget()), to_gboolean(setting)); }
    }

    fn set_width_chars(&mut self, n_chars: i32) -> () {
        unsafe {
            ffi::gtk_entry_set_width_chars(GTK_ENTRY(self.get_widget()), n_chars as c_int);
        }
    }

    fn get_invisible_char(&self) -> u32 {
        unsafe {
            ffi::gtk_entry_get_invisible_char(GTK_ENTRY(self.get_widget())) as u32
        }
    }

    fn set_alignment(&mut self, x_align: f32) -> () {
        unsafe {
            ffi::gtk_entry_set_alignment(GTK_ENTRY(self.get_widget()), x_align as c_float);
        }
    }

    fn get_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_entry_get_alignment(GTK_ENTRY(self.get_widget())) as f32
        }
    }

    fn set_placeholder(&mut self, text: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

            ffi::gtk_entry_set_placeholder_text(GTK_ENTRY(self.get_widget()), c_str.as_ptr())
        }
    }

    fn placeholder(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_entry_get_placeholder_text(GTK_ENTRY(self.get_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_overwrite_mode(&self) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_overwrite_mode(GTK_ENTRY(self.get_widget()))) }
    }

    fn set_overwrite_mode(&mut self, overwrite: bool) {
        unsafe { ffi::gtk_entry_set_overwrite_mode(GTK_ENTRY(self.get_widget()), to_gboolean(overwrite)); }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;

        unsafe {
            ffi::gtk_entry_get_layout_offsets(GTK_ENTRY(self.get_widget()), &x, &y);
        }
        (x, y)
    }

    fn layout_index_to_text_index(&self, layout_index: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_layout_index_to_text_index(GTK_ENTRY(self.get_widget()), layout_index as c_int) as i32
        }
    }

    fn text_index_to_layout_index(&self, text_index: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_text_index_to_layout_index(GTK_ENTRY(self.get_widget()), text_index as c_int) as i32
        }
    }

    fn get_max_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_max_length(GTK_ENTRY(self.get_widget())) as i32
        }
    }

    fn get_visibility(&self) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_visibility(GTK_ENTRY(self.get_widget()))) }
    }

    fn set_cursor_hadjustment(&mut self, adjustment: &gtk::Adjustment) -> () {
        unsafe {
            ffi::gtk_entry_set_cursor_hadjustment(GTK_ENTRY(self.get_widget()), adjustment.get_pointer())
        }
    }

    fn get_cursor_hadjustment(&self) -> gtk::Adjustment {
        unsafe {
            gtk::Adjustment::wrap_pointer(ffi::gtk_entry_get_cursor_hadjustment(GTK_ENTRY(self.get_widget())))
        }
    }

    fn set_progress_fraction(&mut self, fraction: f64) -> () {
        unsafe {
            ffi::gtk_entry_set_progress_fraction(GTK_ENTRY(self.get_widget()), fraction as c_double);
        }
    }

    fn get_progress_fraction(&mut self) -> f64 {
        unsafe {
            ffi::gtk_entry_get_progress_fraction(GTK_ENTRY(self.get_widget())) as f64
        }
    }

    fn set_progress_pulse_step(&mut self, pulse_step: f64) -> () {
        unsafe {
            ffi::gtk_entry_set_progress_pulse_step(GTK_ENTRY(self.get_widget()), pulse_step as c_double);
        }
    }

    fn get_progress_pulse_step(&mut self) -> f64 {
        unsafe {
            ffi::gtk_entry_get_progress_pulse_step(GTK_ENTRY(self.get_widget())) as f64
        }
    }

    fn progress_pulse(&mut self) -> () {
        unsafe {
            ffi::gtk_entry_progress_pulse(GTK_ENTRY(self.get_widget()));
        }
    }

    fn reset_im_context(&mut self) -> () {
        unsafe {
            ffi::gtk_entry_reset_im_context(GTK_ENTRY(self.get_widget()));
        }
    }

    fn set_icon_from_stock(&mut self, icon_pos: EntryIconPosition, stock_id: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(stock_id.as_bytes());

            ffi::gtk_entry_set_icon_from_stock(GTK_ENTRY(self.get_widget()), icon_pos, c_str.as_ptr());
        }
    }

    fn set_icon_from_icon_name(&mut self, icon_pos: EntryIconPosition, icon_name: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(icon_name.as_bytes());

            ffi::gtk_entry_set_icon_from_icon_name(GTK_ENTRY(self.get_widget()), icon_pos, c_str.as_ptr())
        }
    }

    fn get_icon_storage_type(&self, icon_pos: EntryIconPosition) -> ImageType {
        unsafe {
            ffi::gtk_entry_get_icon_storage_type(GTK_ENTRY(self.get_widget()), icon_pos)
        }
    }

    fn get_icon_stock(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_entry_get_icon_stock(GTK_ENTRY(self.get_widget()), icon_pos);

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_icon_name(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_entry_get_icon_name(GTK_ENTRY(self.get_widget()), icon_pos);
            
            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_icon_activatable(&self, icon_pos: EntryIconPosition) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_icon_activatable(GTK_ENTRY(self.get_widget()), icon_pos)) }
    }

    fn set_icon_activatable(&mut self, icon_pos: EntryIconPosition, activatable: bool) {
        unsafe { ffi::gtk_entry_set_icon_activatable(GTK_ENTRY(self.get_widget()), icon_pos, to_gboolean(activatable)); }
    }

    fn get_icon_sensitive(&self, icon_pos: EntryIconPosition) -> bool {
        unsafe { to_bool(ffi::gtk_entry_get_icon_sensitive(GTK_ENTRY(self.get_widget()), icon_pos)) }
    }

    fn set_icon_sensitive(&mut self, icon_pos: EntryIconPosition, sensitive: bool) {
        unsafe { ffi::gtk_entry_set_icon_sensitive(GTK_ENTRY(self.get_widget()), icon_pos, to_gboolean(sensitive)); }
    }

    fn get_icon_at_pos(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_get_icon_at_pos(GTK_ENTRY(self.get_widget()), x, y) as i32
        }
    }

    fn set_icon_tooltip_text(&mut self, icon_pos: EntryIconPosition, tooltip: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(tooltip.as_bytes());

            ffi::gtk_entry_set_icon_tooltip_text(GTK_ENTRY(self.get_widget()), icon_pos, c_str.as_ptr())
        }
    }

    fn get_icon_tooltip_text(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_entry_get_icon_tooltip_text(GTK_ENTRY(self.get_widget()), icon_pos);
            let ret = if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            };

            ::libc::funcs::c95::stdlib::free(c_str as *mut libc::c_void);
            ret
        }
    }

    fn set_icon_tooltip_markup(&mut self, icon_pos: EntryIconPosition, tooltip: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(tooltip.as_bytes());

            ffi::gtk_entry_set_icon_tooltip_markup(GTK_ENTRY(self.get_widget()), icon_pos, c_str.as_ptr())
        }
    }

    fn get_icon_tooltip_markup(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_entry_get_icon_tooltip_markup(GTK_ENTRY(self.get_widget()), icon_pos);
            let ret = if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            };

            ::libc::funcs::c95::stdlib::free(c_str as *mut libc::c_void);
            ret
        }
    }

    fn get_current_icon_draw_source(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_current_icon_drag_source(GTK_ENTRY(self.get_widget())) as i32
        }
    }

    fn set_input_purpose(&mut self, purpose: InputPurpose) -> () {
        unsafe {
            ffi::gtk_entry_set_input_purpose(GTK_ENTRY(self.get_widget()), purpose)
        }
    }

    fn get_input_purpose(&self) -> InputPurpose {
        unsafe {
            ffi::gtk_entry_get_input_purpose(GTK_ENTRY(self.get_widget()))
        }
    }

    fn set_input_hints(&mut self, hints: InputHints) -> () {
        unsafe {
            ffi::gtk_entry_set_input_hints(GTK_ENTRY(self.get_widget()), hints)
        }
    }

    fn get_input_hints(&self) -> InputHints {
        unsafe {
            ffi::gtk_entry_get_input_hints(GTK_ENTRY(self.get_widget()))
        }
    }
}
