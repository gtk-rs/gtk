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

use libc::{c_int, c_double};
use std::str;

use gtk::ffi;
use gtk::traits::Widget;
use gtk::enums::Justification;
use gtk::cast::GTK_LABEL;

pub trait Label : Widget {
    fn set_label(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_label_set_label(GTK_LABEL(self.get_widget()), c_str)
            });
        }
    }

    fn set_text(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_label_set_text(GTK_LABEL(self.get_widget()), c_str)
            });
        }
    }

    fn set_justify(&mut self, jtype: Justification) -> () {
        unsafe {
            ffi::gtk_label_set_justify(GTK_LABEL(self.get_widget()), jtype);
        }
    }

    fn set_markup(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_label_set_markup(GTK_LABEL(self.get_widget()), c_str)
            });
        }
    }

    fn set_markup_with_mnemonic(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_label_set_markup_with_mnemonic(GTK_LABEL(self.get_widget()), c_str)
            });
        }
    }

    fn set_pattern(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_label_set_pattern(GTK_LABEL(self.get_widget()), c_str)
            });
        }
    }

    fn set_text_with_mnemonic(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_label_set_text_with_mnemonic(GTK_LABEL(self.get_widget()), c_str)
            });
        }
    }

    fn set_width_chars(&mut self, n_chars: i32) -> () {
        unsafe {
            ffi::gtk_label_set_width_chars(GTK_LABEL(self.get_widget()), n_chars as c_int);
        }
    }

    fn set_max_width_chars(&mut self, n_chars: i32) -> () {
        unsafe {
            ffi::gtk_label_set_max_width_chars(GTK_LABEL(self.get_widget()), n_chars as c_int);
        }
    }

    fn set_line_wrap(&mut self, wrap: bool) -> () {
        match wrap {
            true    => unsafe { ffi::gtk_label_set_line_wrap(GTK_LABEL(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_label_set_line_wrap(GTK_LABEL(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_line_wrap(&self) -> bool {
        match unsafe { ffi::gtk_label_get_line_wrap(GTK_LABEL(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_lines(&mut self, lines: i32) -> () {
        unsafe {
            ffi::gtk_label_set_lines(GTK_LABEL(self.get_widget()), lines as c_int);
        }
    }

    fn get_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_lines(GTK_LABEL(self.get_widget())) as c_int
        }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;
        unsafe {
            ffi::gtk_label_get_layout_offsets(GTK_LABEL(self.get_widget()), &x, &y);
        }
        (x, y)
    }

    fn get_mnemonic_keyval(&self) -> u32 {
        unsafe {
            ffi::gtk_label_get_mnemonic_keyval(GTK_LABEL(self.get_widget())) as u32
        }
    }

    fn set_selectable(&mut self, selectable: bool) -> () {
        match selectable {
            true    => unsafe { ffi::gtk_label_set_selectable(GTK_LABEL(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_label_set_selectable(GTK_LABEL(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_selectable(&self) -> bool {
        match unsafe { ffi::gtk_label_get_selectable(GTK_LABEL(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_use_markup(&mut self, use_markup: bool) -> () {
        match use_markup {
            true    => unsafe { ffi::gtk_label_set_use_markup(GTK_LABEL(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_label_set_use_markup(GTK_LABEL(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_use_markup(&self) -> bool {
        match unsafe { ffi::gtk_label_get_use_markup(GTK_LABEL(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_use_underline(&mut self, use_underline: bool) -> () {
        match use_underline {
            true    => unsafe { ffi::gtk_label_set_use_underline(GTK_LABEL(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_label_set_use_underline(GTK_LABEL(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_use_underline(&self) -> bool {
        match unsafe { ffi::gtk_label_get_use_underline(GTK_LABEL(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_single_line_mode(&mut self, single_line_mode: bool) -> () {
        match single_line_mode {
            true    => unsafe { ffi::gtk_label_set_single_line_mode(GTK_LABEL(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_label_set_single_line_mode(GTK_LABEL(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_single_line_mode(&self) -> bool {
        match unsafe { ffi::gtk_label_get_single_line_mode(GTK_LABEL(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_track_visited_links(&mut self, track_visited_links: bool) -> () {
        match track_visited_links {
            true    => unsafe { ffi::gtk_label_set_track_visited_links(GTK_LABEL(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_label_set_track_visited_links(GTK_LABEL(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_track_visited_links(&self) -> bool {
        match unsafe { ffi::gtk_label_get_track_visited_links(GTK_LABEL(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_label_get_text(GTK_LABEL(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(str::raw::from_c_str(c_str))
            }
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_label_get_label(GTK_LABEL(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(str::raw::from_c_str(c_str))
            }
        }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_label_get_current_uri(GTK_LABEL(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(str::raw::from_c_str(c_str))
            }
        }
    }

    fn select_region(&mut self, start_offset: i32, end_offset: i32) -> () {
        unsafe {
            ffi::gtk_label_select_region(GTK_LABEL(self.get_widget()), start_offset as c_int, end_offset as c_int);
        }
    }

    fn get_justify(&self) -> Justification {
        unsafe {
            ffi::gtk_label_get_justify(GTK_LABEL(self.get_widget()))
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_width_chars(GTK_LABEL(self.get_widget())) as i32
        }
    }

    fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_max_width_chars(GTK_LABEL(self.get_widget())) as i32
        }
    }

    fn get_selection_bounds(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;
        unsafe {
            ffi::gtk_label_get_selection_bounds(GTK_LABEL(self.get_widget()), &x, &y);
        }
        (x, y)
    }

    fn set_angle(&mut self, angle: f64) -> () {
        unsafe {
            ffi::gtk_label_set_angle(GTK_LABEL(self.get_widget()), angle as c_double);
        }
    }

    fn get_angle(&self) -> f64 {
        unsafe {
            ffi::gtk_label_get_angle(GTK_LABEL(self.get_widget())) as f64
        }
    }
}

